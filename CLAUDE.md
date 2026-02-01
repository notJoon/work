# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool ("work") for managing daily work journals in plain text files. The journal uses a specific markdown-like format with dated sections, TODO lists, completion timestamps, and hierarchical bullet points with Korean time notation.

## Build Commands

```bash
cargo build          # Development build
cargo build --release # Release build with LTO
cargo run            # Run the CLI
cargo test           # Run tests
```

## Architecture

The codebase is organized into four modules:

- **md.rs** - Bullet point formatting. Implements alternating bullet pattern: `-` at odd depths (1, 3, 5...), `.` at even depths (2, 4, 6...). Any bullet characters (-, ., *) are normalized on save.

- **time.rs** - Date/time utilities. Returns dates as `YYYY-MM-DD` and times in Korean 12-hour format: `[오전 HH:MM]` (morning) or `[오후 HH:MM]` (afternoon).

- **io.rs** - File I/O and configuration. Respects environment variables:
  - `TODO_FILE` - Journal file path (default: `~/todo.txt`)
  - `EDITOR` / `VISUAL` - Editor command (default: `vim`)

- **editor.rs** - Today section handling. Manages date sections with `==========` underlines, newest dates at top of file.

## Journal File Format

```
YYYY-MM-DD
==========

TODO
 - Task (depth 1)
  . Subtask (depth 2)
   - Sub-subtask (depth 3)

[오전 10:30]
 - Completed item with timestamp

#TIL
 - Note with custom tag
```

Indentation is 1 space per level. Sections within a date: TODO, timestamped completions, and custom tags (#TIL, #memo, etc.).
