# 🧠 Paragon

**Paragon** is a next-gen Rust-powered platform for building, testing, and deploying advanced Smart Money Concept (SMC) trading strategies. With a sleek graphical interface, a built-in DSL for strategy design, and optional AI decision models, Paragon is designed to become your all-in-one trading lab. ⚡📈

From structure detection to strategy fusion via machine learning, Paragon turns abstract trading ideas into robust, testable, and executable systems. 🧠💡

## 🚧 Current Status

> 🛠️ Actively building the core detection engine for SMC structures like FVG, OB, CHoCH...  
> GUI, strategy scripting DSL, and AI model selection are in the roadmap.

## 🎯 Key Features (Roadmap)

### ✅ Core Engine *(ongoing)*
- Automatic detection of SMC patterns (Fair Value Gaps, Order Blocks, Break of Structure, CHoCH, ...)
- High-performance Rust backend with zero-cost abstractions

### 🖥️ Graphical Interface *(planned)*
- Real-time backtest visualizations and trade analytics
- Strategy library management

### ⚙️ Backtesting & Live Trading *(planned)*
- Fast, accurate backtesting with custom date ranges
- Connect to brokers for live execution (e.g. MetaTrader, Binance)
- Unified engine for both simulated and real trades

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

## 🧰 Tech Stack

- **Rust** — safe, fast, and concurrent core
- **Tauri** *(planned)* — native GUI across platforms
- **DSL Parser** *(planned)* — custom strategy language compiler (in Rust)
- **AI Models** *(planned)* — via rust

## 🚀 Getting Started

> Paragon is not yet publicly available. The first CLI-based dev builds are expected soon.

```sh
git clone https://github.com/enzoblain/Paragon
cd Paragon
cargo run
```

## 📊 Example: Strategy & Execution Flow (future)

1. User defines 3 strategies: OB Breakout, FVG Reversal, Liquidity Sweep  
2. User picks an AI model: Neural Network  
3. Paragon feeds historical stats into the model  
4. During backtest or live trading:
   - All 3 strategies run  
   - Model picks the optimal action (entry/skip/reject)  
   - Execution engine fires orders with risk management  

## 📄 License

Paragon is released under the MIT License.  
See the [LICENSE](./LICENSE) file for more information.

## 👤 Author

**Enzo Blain**  
GitHub: [https://github.com/enzoblain](https://github.com/enzoblain)

---

💬 Ideas, feedback, or contributions? Open an issue or discussion — Paragon is built to grow with its community!