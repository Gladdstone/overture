# Overture App Launcher

A lightweight, custom application launcher, built using the gpui library.


---

## Features

- Fast application search and launch
- Search currently driven by fuzzy-rs
- Built with the `gpui` library
- Keyboard-driven workflow

> This project is under active development.

---

## Requirements

- Rust (stable toolchain)

---

## Installation

### Build from Source

```bash
git clone https://github.com/Gladdstone/overture.git
cd overture
cargo build --release
```
Currently the application must be initialized with the following terminal command in order to invoke the dbus show method:
```bash
gdbus call --session \
  --dest org.example.App \
  --object-path /org/example/App \
  --method org.example.App.Show
```
