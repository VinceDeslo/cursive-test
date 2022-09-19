# Messing around with Cursive

This repo is a playground to mess around with Cursive in Rust.  

### Objectives
- Build out a small terminal UI in Rust.   
- Learning some basics for eventual tui applications.

### Requirements
- Rust
- Cursive

### Binaries
- `tui`: a Cursive implementation that displays a [Hyperion Cantos](https://hyperioncantos.fandom.com/wiki/Hyperion_Cantos_Wiki) quote Dialog and ASCII art.

### Top level Make commands:
- `make run-tui` : Executes the tui binary directly with cargo.

### Structure
```
cursive-test
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── README.md
├── src
│   ├── assets
│   |   ├── ** ASCII art **
│   └── tui
│       ├── ** Tui code using Cursive **
├── target
│   ├── ** Tui binary and debug content **
└── themes
    └── ** TOML files for tui theming **
```
