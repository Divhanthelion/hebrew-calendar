# Hebrew Calendar

A Hebrew–Gregorian calendar with zmanim, holidays, and weekly parsha. The conversion core is `hebrew_core` (Rata Die, tested). The desktop app is `hebrew_app` (Tauri GUI or an Axum API).

## Requirements

- [Rust](https://rustup.rs/)
- For the GUI: [WebKitGTK](https://v2.tauri.app/start/prerequisites/) on Linux, Xcode CLT on macOS

## Build

```bash
git clone https://github.com/Divhanthelion/hebrew-calendar.git
cd hebrew-calendar
cargo test -p hebrew_core
cargo run -p hebrew_app
```

GUI is the default. API mode:

```bash
cargo run -p hebrew_app -- --server --port 3000
```

`src-tauri/` is a leftover second Tauri shell. Use `hebrew_app`.

## License

MIT
