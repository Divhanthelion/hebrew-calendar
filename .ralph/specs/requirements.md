# Hebrew-Gregorian Calendar Desktop App

## Overview
A simple desktop application that displays a Hebrew calendar and Gregorian calendar side-by-side, allowing users to see the correspondence between dates in both systems.

## Target User
Jewish users who need to look up Hebrew dates for holidays, yahrzeits, bar/bat mitzvahs, or general date conversion.

## Technical Stack
- **Framework**: Tauri (Rust backend + web frontend)
- **UI**: HTML/CSS/vanilla JS (keep it simple, no React)
- **Hebrew Calendar Library**: `hcal` or `hebrew-calendar` crate
- **Platform**: macOS (primary), cross-platform capable

## Core Features

### Feature 1: Dual Calendar Display
- Show current month in both Gregorian and Hebrew format
- Side-by-side layout
- Today's date highlighted in both calendars
- Hebrew date shown in Hebrew letters (א׳ ניסן) with English transliteration option

### Feature 2: Date Navigation
- Previous/Next month buttons
- Click on "Today" to return to current date
- Click any date to see its Hebrew/Gregorian equivalent

### Feature 3: Date Conversion
- Click a Gregorian date → show Hebrew equivalent
- Click a Hebrew date → show Gregorian equivalent
- Display result in a panel below calendars

### Feature 4: Holiday Indicators
- Mark major Jewish holidays on the Hebrew calendar
- Shabbat highlighted each week
- Hover/click to see holiday name

## Out of Scope (v1)
- Zmanim (prayer times)
- Multiple calendar views (week, year)
- Event creation/storage
- Notifications/reminders
- Custom themes

## Acceptance Criteria
- App launches and displays current month
- Both calendars scroll in sync
- Hebrew dates are accurate (verify against hebcal.com)
- Holidays appear on correct dates
- Works offline (no API dependencies)

## UI Design Notes
- Clean, minimal design
- Hebrew text should be properly right-to-left
- Responsive to window resizing
- Dark mode support (nice to have)
