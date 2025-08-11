Wi-Fi Watch

A tiny Rust daemon that monitors network connectivity (Wi-Fi or Internet) and plays a sound whenever the link goes down.
✨ Features

    Periodic connectivity checks (default: every 10 s).

    Plays any short MP3/WAV as an alert.

    Cross-platform: Linux, macOS, Windows.

    Minimal CPU usage thanks to an async Tokio loop.

📂 Project Layout

wifi_watch/
├─ Cargo.toml        # dependencies & metadata
├─ src/
│   └─ main.rs       # main source file
└─ alert.mp3         # alert sound (can be WAV)


🔧 Building

1 - Install Rust toolchain:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


2 - Clone and build:
    bash
    git clone https://github.com/<your-user>/wifi_watch.git
    cd wifi_watch
    cargo build --release


The binary is created at target/release/wifi_watch.
Cross-compile for macOS (from Linux)

rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin   # Apple Silicon


🚀 Quick Start

    ./wifi_watch               # run in folder containing alert.mp3

Keep the terminal open, then disable Wi-Fi to hear the alert.
To test without cutting the network, make play_alert() run unconditionally or add a --test flag.


