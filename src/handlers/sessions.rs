use crate::{
    connections::database::add_session, utils::utils::is_in_timerange, Candle, ReferenceSession, Session, SESSIONS
};

use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Store the current session in a global state
pub static SESSION: Lazy<Mutex<Option<Session>>> = Lazy::new(|| Mutex::new(None));

pub async fn process_session(candle: Arc<Candle>, symbol: &'static str) -> Result<(), String> {
    // Check if the session is not initialized 
    // or if the current session is not the same as the candle's session
    // This prevent locking for too long
    let should_create_new_session = should_create_new_session(candle.clone()).await;

    if should_create_new_session {
        // Get the right session based on the candle's timestamp
        let session = get_right_session(candle.timestamp)?;

        // Get the start and end of the session
        let (start, end) = get_sessions_start_end(candle.timestamp, session);

        // Create a new session
        let new_session = Session {
            symbol: symbol,
            label: session.label,
            start,
            end,
            high: candle.high,
            low: candle.low,
            open: candle.open,
            close: candle.close,
            volume: candle.volume,
        };

        // Set the new session in the global state
        let mut session_guard = SESSION.lock().unwrap();
        *session_guard = Some(new_session);
    } else {
        // Get the current session
        // We can unwrap here because we checked if the session is Some
        let mut session_guard = SESSION.lock().unwrap();
        let current_session = session_guard.as_mut().unwrap();

        // Update the current session
        if candle.high > current_session.high {
            current_session.high = candle.high;
        }
        if candle.low < current_session.low {
            current_session.low = candle.low;
        }

        current_session.close = candle.close;
        current_session.volume += candle.volume;
    }

    Ok(())
}

pub async fn should_create_new_session(candle: Arc<Candle>) -> bool {
    // Check if the session is not initialized 
    // or if the current session is not the same as the candle's session
    // This prevent locking for too long
    let session = {
        let session_guard = SESSION.lock().unwrap();
        session_guard.clone()
    };

    // If the session is not initialized
    if session.is_none() {
        return true;
    }

    // If the candle is not in the current session (its timestamp is not in the session's start and end)
    if let Some(session) = session.as_ref() {
        if !is_same_session(session, candle) {
            // Check if errors occurred while adding the session to the database
            let Ok(()) = add_session(&session).await else {
                eprintln!("Failed to add session to the database");

                return true;
            };

            return true;
        }
    }

    return false;
}

// Checks if the candle's timestamp is within the session's start and end
pub fn is_same_session(session: &Session, candle: Arc<Candle>) -> bool {
    candle.timestamp >= session.start && candle.timestamp <= session.end
}

// Returns the right session based on the timestamp
// If the timestamp is not in any session, it returns an error (won't happen)
pub fn get_right_session(timestamp: DateTime<Utc>) -> Result<&'static ReferenceSession, String> {
    for session in SESSIONS.iter() {
        if is_in_timerange(session.start, session.end, timestamp.time()) {
            return Ok(session);
        }
    }
    
    Err(format!("No session found for timestamp: {}", timestamp))
}

// Rerturrns the end of the session base on the timestamp
pub fn get_sessions_start_end(timestamp: DateTime<Utc>, session: &ReferenceSession) -> (DateTime<Utc>, DateTime<Utc>) {
    let mut start_date = timestamp.date_naive();
    let mut end_date = timestamp.date_naive();

    // Because the asian session crosses the day
    // We need to check which day we are in
    if session.label == "Asian Session"{ 
        // If the timestamp is before 7am
        // The session starts the day before
        if timestamp.hour() < 7 {
            start_date = start_date.pred_opt().unwrap();
        } else { // Else the session end the next day
            end_date = end_date.succ_opt().unwrap();
        }
    }

    // Convert the date to NaiveDateTime
    let start = NaiveDateTime::new(
        start_date,
        session.start
    );
    let end = NaiveDateTime::new(
        end_date,
        session.end
    );

    (start.and_utc(), end.and_utc())
}