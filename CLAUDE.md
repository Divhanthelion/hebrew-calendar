# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Hebrew Calendar App — a Tauri desktop application and HTTP API server for Hebrew-Gregorian date conversion, Jewish holiday identification, Torah portions (parsha), and zmanim (halachic prayer times). Built in Rust with a workspace of two crates.

## Build & Run Commands

```bash
# Build (workspace root)
cargo build                        # Debug build
cargo build --release              # Release (optimized for size, LTO enabled)

# Run as Tauri desktop app (default)
cargo tauri dev                    # Dev mode with hot reload
cargo tauri build                  # Production bundle

# Run as HTTP API server
cargo run -p hebrew_app -- --server              # Default port 3000
cargo run -p hebrew_app -- --server -p 8080      # Custom port

# Tests
cargo test                         # All workspace tests
cargo test -p hebrew_core          # Core library tests only
cargo test -p hebrew_app           # App tests only
cargo test -p hebrew_core -- calendar::tests::test_leap_year  # Single test

# Lint & Format
cargo clippy                       # Lint
cargo fmt --check                  # Check formatting
cargo fmt                          # Auto-format
```

## Architecture

### Workspace Layout

- **`hebrew_core/`** — Pure library crate. No GUI or network dependencies. All calendar math lives here.
- **`hebrew_app/`** — Binary crate with two optional feature-gated modes:
  - `gui` feature → Tauri desktop app (default)
  - `server` feature → Axum HTTP API server
  - Both features are enabled by default

### hebrew_core Modules

| Module | Purpose |
|--------|---------|
| `calendar.rs` | Hebrew↔Gregorian conversion using Rata Die (R.D.) arithmetic from Reingold & Dershowitz |
| `holidays.rs` | 80+ holiday variants including Omer counting, Chanukah year-boundary handling |
| `parsha.rs` | 54 base Torah portions + 7 combined variants |
| `zmanim.rs` | NOAA solar algorithms for 14 halachic times (sunrise, sunset, candle lighting, etc.) |
| `lib.rs` | Public API: `HebrewCalendar::calculate_day()` returns `DailyData` |

### hebrew_app Modules

| Module | Purpose |
|--------|---------|
| `main.rs` | CLI arg parsing (clap). Launches Tauri GUI or Axum server based on `--server` flag |
| `gui/mod.rs` | 5 Tauri `#[command]` handlers with Mutex-protected `AppConfig` state |
| `api/mod.rs` | Axum routes under `/api/v1/` — calendar convert, date range, zmanim, upcoming holidays |
| `config.rs` | JSON config at `~/.config/hebrew-calendar/config.json` (location, candle offset, customs) |

### Frontend

Single HTML file at `hebrew_app/frontend/index.html` with inline CSS/JS. Auto-detects Tauri (`window.__TAURI__`) vs HTTP API mode. No build step.

## Key Types

- `HebrewDate` — year/month/day with `HebrewMonth` enum (13 months, Adar I for leap years)
- `DailyData` — complete daily output: both dates, holidays, parsha, zmanim, candle lighting
- `GeoLocation` — lat/long/elevation/timezone for zmanim calculations
- `CalendarError` — `DateOutOfRange`, `InvalidDateFormat`, `InvalidLatitude`, `InvalidLongitude`, `CalculationError`

## API Routes (server mode)

- `GET /api/v1/calendar/convert?date=YYYY-MM-DD&lat=LAT&long=LNG`
- `GET /api/v1/calendar/range?start=...&end=...&lat=LAT&long=LNG` (max 366 days)
- `GET /api/v1/zmanim?date=...&lat=LAT&long=LNG&elevation=M`
- `GET /api/v1/holidays/upcoming?year=YYYY`
- `GET /api/v1/health`

## Notes

- Hebrew leap year formula: `(7*y + 1) % 19 < 7`
- Valid date range: year 0 (1 BCE) to 2050 CE
- Default location: Jerusalem (31.7683, 35.2137)
- Default candle lighting offset: 18 minutes before sunset
- Release profile uses `opt-level = "z"` + LTO + strip for minimal binary size
