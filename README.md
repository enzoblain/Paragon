# 🧠 Paragon

**Paragon** is a next-gen Rust-powered platform for building, testing, and deploying advanced Smart Money Concept (SMC) trading strategies. With a sleek graphical interface, a built-in DSL for strategy design, and optional AI decision models, Paragon is designed to become your all-in-one trading lab.

From structure detection to strategy fusion via machine learning, Paragon turns abstract trading ideas into robust, testable, and executable systems.

---

## 🚧 Current Status

> 🛠️ Actively building the core detection engine for SMC structures like FVG, OB, CHoCH...  

---

## ✅ Completed Features

- [x] Real-time candle aggregation (1m → 5m, 15m, etc.)   
- [x] WebSocket server to stream candles to clients  

---

## 🎯 Key Features (Roadmap)

### 🔎 Core Engine *(ongoing)*
- Automatic detection of SMC patterns (Fair Value Gaps, Order Blocks, Break of Structure, CHoCH, ...)
- High-performance Rust backend with zero-cost abstractions

### 🧠 AI Strategy Selector *(planned)*
- Combine multiple strategies into a decision layer
- Choose from built-in models:  
  - Neural Network  
  - Random Forest  
  - ...
- Train on historical performance data to optimize signal blending

### 🧾 Strategy DSL *(planned)*
- Domain-specific language to define strategies
- Safe, sandboxed, and beginner-friendly

### ⚙️ Backtesting & Live Trading *(planned)*
- Fast, accurate backtesting with custom date ranges
- Connect to brokers for live execution (e.g. MetaTrader, Binance)
- Unified engine for both simulated and real trades

### 🖥️ Graphical Interface *(planned)*
- Real-time backtest visualizations and trade analytics
- Strategy library management

---

## 🧰 Tech Stack

- **Rust** — safe, fast, and concurrent core  
- **AI Models** *(planned)* — via rust  
- **DSL Parser** *(planned)* — custom strategy language compiler (in Rust)  
- **Tauri** *(planned)* — native GUI across platforms  

---

## 🚀 Getting Started

```
git clone https://github.com/enzoblain/Paragon
cd Paragon
cargo run
```

---

## 📊 Example: Strategy & Execution Flow (future)

1. User defines 3 strategies: OB Breakout, FVG Reversal, Liquidity Sweep  
2. User picks an AI model: Neural Network  
3. Paragon feeds historical stats into the model  
4. During backtest or live trading:
   - All 3 strategies run  
   - Model picks the optimal action (entry/skip/reject)  
   - Execution engine fires orders with risk management  

---

## 👤 Author

**Enzo Blain**  
GitHub: [https://github.com/enzoblain](https://github.com/enzoblain)

---

## 📄 License

Paragon is released under the MIT License.  
See the [LICENSE](./LICENSE) file for more information.

---

## 🧑‍💻 Contributing

Paragon is currently in private development. If you'd like to collaborate, feel free to open an issue or fork the project for experimentation.