# RustShadowDungeon

锈影地下城 是一个基于 Bevy 引擎的 2D 横版动作 RPG 游戏，使用 Rust 语言开发。

**English**: A DNF-inspired 2D side-scrolling action RPG built with Rust + Bevy engine, featuring fast-paced combat, dungeon exploration, and cooperative multiplayer.

## Core Features

- **Fast-Paced Combat**: DNF-style combo system with skills, cooldowns, and status effects
- **60 FPS Performance**: Optimized for smooth, responsive gameplay
- **Pixel Art**: Consistent 16x16 grid-based pixel art style
- **ECS Architecture**: Clean, modular design using Bevy's Entity Component System
- **Memory Safe**: Leveraging Rust's ownership system for crash-free gameplay
- **Open Source**: MIT licensed for community contributions

## Development Philosophy

This project follows a strict [constitution](.specify/memory/constitution.md) that enforces:
- Test-driven development for all combat mechanics
- Bevy ECS best practices (no OOP anti-patterns)
- Performance budgets (<16.67ms per frame)
- Modular plugin architecture
- Comprehensive documentation

## Technology Stack

- **Language**: Rust (stable)
- **Game Engine**: Bevy 0.12+
- **Physics**: bevy_rapier2d
- **Testing**: cargo test + criterion benchmarks

## Quick Start

```bash
# Clone the repository
git clone https://github.com/[username]/RustShadowDungeon.git
cd RustShadowDungeon

# Run the game (development mode)
cargo run

# Run tests
cargo test

# Run with performance monitoring
RUST_LOG=debug cargo run
```

## Contributing

Contributions are welcome! Please:
1. Read the [constitution](.specify/memory/constitution.md) for development principles
2. Follow the specification-driven workflow in `.specify/`
3. Ensure all tests pass and meet performance budgets
4. Follow Rust and Bevy best practices

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**Note**: The current LICENSE file is Apache 2.0. This will be updated to MIT to match the project constitution.