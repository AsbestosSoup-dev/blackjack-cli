# Blackjack CLI

**blackjack-cli** is a terminal-based Blackjack game written in Rust. It currently supports local singleplayer sessions against the house, with plans to expand into LAN and online multiplayer modes.

## Features

- 🎮 Command-line Blackjack gameplay (TUI)
- 🧠 Object-oriented architecture (transitioning to ECS)
- 🔒 Planned focus on private, anonymous multiplayer sessions
- 📡 Future support for LAN/online hosting & joining

## Roadmap

- [x] Local singleplayer vs house
- [ ] ECS-based architecture for performance
- [ ] LAN multiplayer sessions
- [ ] Secure, anonymous networking
- [ ] Online lobby system

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed:

```bash
# With Rust installed
cargo build --release
cargo run
```

## Development
```bash
# To run in debug mode
cargo build
cargo run
```

## Contributing
This is a personal WIP project. Contributions may be welcome later — stay tuned.

## License
[MIT](LICENSE)

---
Built with 🦀 Rust.