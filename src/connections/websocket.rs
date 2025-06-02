use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpStream, TcpListener};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

// Store the clients connected to the WebSocket server
pub static CLIENTS: Lazy<Arc<Mutex<Vec<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub async fn create_intra_websocket() -> Result<(), String> {
    // Load the url
    let url = "127.0.0.1:8080";

    // Set up a TCP listener
    let listener = TcpListener::bind(url)
        .await
        .map_err(|e| format!("Unable to bind TCP listener: {}", e))?;

    // Start the WebSocket server
    // and accept incoming WebSocket connections
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        let (write, mut read) = ws_stream.split();

        // Add the new client to the list of clients
        // Use an Arc and Mutex to share the client between tasks
        // and ensure thread safety
        let client = Arc::new(Mutex::new(write));
        add_client(client.clone()).await;

        // Handle incoming messages from the WebSocket client
        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(_)) => {
                   let mut write = client.lock().await;
                    write.send(Message::Text("You can't send messages to this server".into()))
                        .await
                        .expect("Error sending message");

                    break;
                }
                Ok(Message::Close(_)) => {
                    // Handle the close message
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);

                    break;
                }
                _ => {
                    let mut write = client.lock().await;
                    write.send(Message::Text("You can't send messages to this server".into()))
                        .await
                        .expect("Error sending message");

                    break;
                }
            }
        }

        // Remove the client from the list of clients
        remove_client(client.clone()).await;
    }

    Ok(())
}

// Add a new client to the list of clients
// Separate function to avoid long locks
pub async fn add_client(client: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) {
    let mut clients = CLIENTS.lock().await;
    clients.push(client);
}

// Same thing as above for removing a client
pub async fn remove_client(client: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>) {
    let mut clients = CLIENTS.lock().await;
    clients.retain(|c| !Arc::ptr_eq(c, &client));
}

// Send a message to all connected clients
pub async fn send_message_to_clients(message: &String) -> Result<(), String> {
    let clients = CLIENTS.lock().await;

    for client in clients.iter() {
        let mut write = client.lock().await;
        write.send(Message::Text(message.into()))
            .await
            .map_err(|e| format!("Error sending message: {}", e))?;
    }

    Ok(())
}