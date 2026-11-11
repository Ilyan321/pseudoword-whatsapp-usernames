# 🌌 Divine Pseudoword WhatsApp Username Generator

A lightning-fast, terminal-based Rust tool that generates elegant, pronounceable, and completely meaningless **Absolute Pseudowords** for WhatsApp handles using LLMs (Groq and Google Gemini).

## ✨ Features
- **Strict Master Directive**: Implements the Omniscient Linguist Directive preventing real roots, slang, or existing meanings.
- **Dual AI Providers**:
  - **Groq Cloud**: Powered by `llama-3.3-70b-versatile` (~500 tps).
  - **Google Gemini**: Powered by `gemini-2.5-flash`.
- **Interactive Terminal Workflow**:
  - Generates 2 pseudowords at a time with phonetic guides.
  - One-click `[Enter]` or `r` to instantly retry/regenerate.
  - Switch provider or key in-flight with `p`.

## 🚀 Quick Start

### 1. Prerequisites
- Rust & Cargo installed:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```

### 2. Environment Variables (Optional)
You can pre-configure your API keys or enter them interactively upon startup:
```bash
# For Groq
export GROQ_API_KEY="gsk_..."

# Or for Gemini
export GEMINI_API_KEY="AIzaSy..."
```

### 3. Run
```bash
cargo run
```
