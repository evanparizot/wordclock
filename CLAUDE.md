# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A DIY word clock that spells out the time using words (e.g., "IT IS FIVE PAST THREE IN THE AFTERNOON"). The repo contains:
- **`firmware/`** — Rust embedded firmware for the STM32F303 microcontroller
- **`schematics/`** — KiCad PCB schematics (two revisions: `wordclock-proto-r1` and `wordclock-r1`)
- **`graphics/`** — SVG layout for the clock face
- **`datasheets/`** — Reference PDFs for components (STM32F303, DS3231, MAX7219)

## Firmware Build & Flash

All commands run from `firmware/`:

```bash
# Build (default mode)
cargo build

# Build with a specific display mode
cargo build --no-default-features --features mode-am

# Select mode at runtime via env var
WORDCLOCK_MODE=am cargo build --features mode-default,mode-am

# Flash to device (requires OpenOCD running in a separate terminal)
./ocd.sh          # terminal 1: start OpenOCD with ST-Link v2-1
cargo run         # terminal 2: builds, flashes via gdb-multiarch + openocd.gdb
```

The `.cargo/config.toml` sets the build target to `thumbv7em-none-eabihf` and the runner to `gdb-multiarch -q -x openocd.gdb`. There are no tests (disabled in `Cargo.toml`).

## Architecture

### Hardware
- **MCU**: STM32F303xC at 64 MHz
- **RTC**: DS3231 over I2C (PB6/PB7, I2C1)
- **LED matrix driver**: 4× MAX7219 over SPI (PB12–PB15, SPI2)
- **Minute LEDs**: 4 discrete LEDs on PA1–PA4 (sub-5-minute indicators)
- **Buttons**: Hour (PA0/EXTI0), Minute (PB3/EXTI3), active-low with internal pull-ups

### Firmware Structure

The firmware uses **RTIC v2** (Real-Time Interrupt-driven Concurrency) with a systick monotonic timer.

| File | Purpose |
|------|---------|
| `main.rs` | RTIC app — shared state, tasks, button ISRs, render loop |
| `clock.rs` | `Clock` struct wrapping MAX7219 display + DS3231 RTC + active `TimeMode` |
| `setup.rs` | Hardware init (clocks, GPIO, SPI, I2C, heap allocator) |
| `times.rs` | `TimeMode` trait and `Phrase` type (`[[u8;8];4]` — 4 MAX7219 modules × 8 rows) |
| `modes.rs` | `REGISTRY` of named `TimeMode` factories; `make_by_name()` |
| `config/mod.rs` | Brightness constants: `DAY_INTENSITY`, `NIGHT_INTENSITY`, `NIGHT_START_HOUR`, `NIGHT_END_HOUR` |
| `config/default.rs` | `DefaultMode` — primary English word-clock layout |
| `config/am.rs` | `AdrianMorgan` — alternate layout |

### Display Model

A `Phrase` is `[[u8; 8]; 4]`: 4 MAX7219 modules, each with 8 rows of 8 bits. Each bit maps to one LED on the clock face. The `combine_phrases()` function OR-combines multiple phrases to produce the final frame written to the display.

`Clock::update_display_time()` calls `mode.get_time_arrays(hour, minutes)` then writes all 4 modules via SPI.

### Adding a New Display Mode

1. Create `firmware/src/config/your_mode.rs` implementing `TimeMode`.
2. Add `pub mod your_mode;` in `config/mod.rs`.
3. Add a `mode-your-mode` feature in `Cargo.toml`.
4. Register it in `modes.rs` `REGISTRY`.

### RTIC Tasks

| Task | Trigger | Role |
|------|---------|------|
| `render_tick` | Async loop, 500 ms | Adjusts brightness, re-renders on minute change, blinks heartbeat pixel and minute LEDs |
| `update_hour` | EXTI0 ISR | Debounced hour increment, spawns `hour_hold_repeat` on long press |
| `update_minutes` | EXTI3 ISR | Debounced minute increment, spawns `minute_hold_repeat` on long press |
| `hour_hold_repeat` | Async task | Repeats hour increment every 120 ms after 700 ms hold |
| `minute_hold_repeat` | Async task | Repeats minute increment every 120 ms after 700 ms hold |

### Key Constants (main.rs)
- `DEBOUNCE_MS = 200` — button debounce window
- `HOLD_MS = 700` — hold duration before auto-repeat
- `REPEAT_MS = 120` — auto-repeat interval
- `RENDER_PERIOD_MS = 500` — render loop tick

### Feature Flags
- `mode-default` (default) — enables `DefaultMode`
- `mode-am` — enables `AdrianMorgan` alternate mode

At least one `mode-*` feature must be enabled or the build will `compile_error!`.
