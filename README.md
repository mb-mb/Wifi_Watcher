# Wi‑Fi Watcher

A tiny Rust daemon that monitors connectivity (Wi‑Fi/Internet) and plays a sound when the link goes down.

## ✨ Features

- Periodic connectivity checks (default: every 2 s).
- Plays an MP3 (or WAV) alert and a short sine tone.
- Cross‑platform: Linux, macOS, and Windows.
- Low CPU usage thanks to the async Tokio loop.

## 📂 Project structure

```text
wifi_watcher/
├─ Cargo.toml        # dependencies and metadata
├─ src/
│  └─ main.rs       # main source file
└─ alert.mp3        # alert sound (can be WAV)
```

## 🔧 Building

1. Install the Rust toolchain:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone and build the project:
   ```sh
   git clone https://github.com/<your-username>/wifi_watcher.git
   cd wifi_watcher
   cargo build --release
   ```

The binary is generated at `target/release/wifi_watcher`.

### Cross‑compile for macOS (from Linux)
```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
# Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

## 🚀 Quick start

In the directory that contains `alert.mp3`:
```sh
./target/release/wifi_watcher
```

- Keep the terminal open and disable Wi‑Fi to hear the alert.
- To test without taking the network down, you can temporarily call `play_direct_alert(...)` directly inside `main()` (for testing only) and rebuild.

Notes:
- The default interval is hardcoded (2 s). Adjust the `INTERVAL` constant in `src/main.rs` if you need a different value.
- Log messages are printed to the terminal in Portuguese.


