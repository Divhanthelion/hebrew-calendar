# Implementation Plan: Hebrew-Gregorian Calendar App

Generated: 2024-01-26
Status: Ready for autonomous execution

## Phase 1: Project Setup

- [ ] Initialize Tauri project with `cargo create-tauri-app`
- [ ] Add hebrew-calendar crate to Cargo.toml dependencies
- [ ] Create basic HTML structure in src-tauri/index.html with two calendar containers
- [ ] Verify app launches with `cargo tauri dev`

## Phase 2: Calendar Data Layer (Rust)

- [ ] Create src-tauri/src/calendar.rs module
- [ ] Implement function to get days in a Gregorian month
- [ ] Implement function to convert Gregorian date to Hebrew date
- [ ] Implement function to get Hebrew month name (in Hebrew and transliterated)
- [ ] Implement function to get days in a Hebrew month
- [ ] Add unit tests for date conversion accuracy
- [ ] Expose calendar functions as Tauri commands

## Phase 3: Frontend Calendar Grid

- [ ] Create CSS grid layout for calendar (7 columns for days)
- [ ] Implement JavaScript function to render Gregorian calendar month
- [ ] Implement JavaScript function to render Hebrew calendar month
- [ ] Add day-of-week headers (Sun-Sat and Hebrew equivalents)
- [ ] Style today's date with highlight
- [ ] Ensure Hebrew text displays right-to-left correctly

## Phase 4: Navigation

- [ ] Add Previous/Next month buttons to UI
- [ ] Implement Rust command to get previous/next month data
- [ ] Wire up buttons to update both calendars
- [ ] Add "Today" button to return to current month
- [ ] Keep both calendars synchronized when navigating

## Phase 5: Date Selection & Conversion

- [ ] Make calendar dates clickable
- [ ] Add info panel below calendars for selected date details
- [ ] When Gregorian date clicked, show Hebrew equivalent in panel
- [ ] When Hebrew date clicked, show Gregorian equivalent in panel
- [ ] Display selected date in multiple formats

## Phase 6: Holiday Data

- [ ] Create holidays.rs with major Jewish holiday definitions
- [ ] Implement function to check if a Hebrew date is a holiday
- [ ] Add holiday names (Hebrew and English)
- [ ] Mark Shabbat (Saturday) on all weeks
- [ ] Add visual indicator (dot or color) for holidays on calendar
- [ ] Show holiday name on hover/click

## Phase 7: Polish & Testing

- [ ] Test date conversions against hebcal.com for accuracy
- [ ] Add window resizing support
- [ ] Improve typography and spacing
- [ ] Add app icon
- [ ] Build release binary with `cargo tauri build`
- [ ] Test on clean system

---

Total Tasks: 31
Estimated Iterations: ~50 (accounting for retries)
