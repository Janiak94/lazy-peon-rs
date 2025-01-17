# Lazy Peon

Simulates random io events. Blazingly fast!

Still a work in progress, more will hopefully come.

- [x] CLI
- [x] Random mouse events
    - [x] Support multiple backends
        - [x] enigo (https://docs.rs/enigo/latest/enigo/)
        - [x] mouse-rs (https://docs.rs/mouse-rs/latest/mouse_rs/)
- [x] Random keyboard events
    - [ ] Support multiple backends
        - [x] enigo (https://docs.rs/enigo/latest/enigo/)
- [ ] Complex io (`alt+tab` for example)

## Installation

Just run

```bash
cargo install lazy-peon-rs
```

or alternatively directly via the github

```bash
cargo install --git https://github.com/Janiak94/lazy-peon-rs
```

The binary can then be run with

```bash
lazy-peon-rs
```

Use `--help` flag to see available options.