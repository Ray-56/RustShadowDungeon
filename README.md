# RustShadowDungeon

锈影地下城 是一个基于 Bevy 引擎的 2D 横版动作 RPG 游戏，使用 Rust 语言开发。

**English**: A DNF-inspired 2D side-scrolling action RPG built with Rust + Bevy engine, featuring fast-paced combat, dungeon exploration, and cooperative multiplayer.

## Core Features

- **Fast-Paced Combat**: DNF-style combo system with skills, cooldowns, and status effects
- **Dungeon System**: Multi-room dungeon exploration with room clearing, door unlocking, and progress tracking
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

# Run performance benchmarks
cargo bench
```

## Dungeon System

The dungeon system provides multi-room exploration with the following features:

- **Room Management**: Load and unload rooms dynamically as players explore
- **Enemy Spawning**: Spawn enemies based on room state (cleared rooms don't respawn enemies)
- **Room Clearing**: Automatically detect when all enemies in a room are defeated
- **Door Unlocking**: Unlock doors to adjacent rooms when a room is cleared
- **Room Transitions**: Seamless transitions between rooms with proper cleanup
- **Progress Tracking**: Track cleared rooms to prevent enemy respawning

### Usage

1. **Enter a Dungeon**: The dungeon system automatically loads the starting room
2. **Defeat Enemies**: Clear all enemies in a room to unlock doors
3. **Interact with Doors**: Press `E` near an unlocked door to transition to the next room
4. **Explore**: Progress through multiple rooms, with cleared rooms staying cleared

### Configuration

Dungeon configurations are stored in `assets/data/dungeons/` as RON files. See `assets/data/dungeons/test_dungeon.ron` for an example.

For detailed developer documentation, see [docs/dungeon-system.md](docs/dungeon-system.md).

## Enemy AI System

The enemy AI system provides intelligent enemy behavior with state machines, perception, and combat logic:

- **State Machine**: Enemies transition between Patrol, Chase, Attack, and Return states
- **Perception System**: Enemies detect players within range using distance and line-of-sight checks
- **Aggro System**: Simplified target locking mechanism with aggro drop when players leave range
- **Attack Behavior**: Enemies attack when players enter attack range, with cooldown management
- **Patrol Behavior**: Enemies patrol using random points or predefined waypoints when no target is detected

### Features

- **Detection & Chase**: Enemies detect players within detection range (with line-of-sight) and chase them
- **Attack System**: Enemies attack when players enter attack range, with configurable cooldowns and hit frames
- **Patrol Modes**: Support for random point patrol or waypoint-based patrol paths
- **Performance Optimized**: Perception checks throttled to every 3-5 frames (~50-100ms) to meet <2ms frame budget

### Configuration

Enemy AI configurations are stored in `assets/data/enemies.ron`. Each enemy type can have:
- `ai_config`: Detection ranges, aggro drop range, patrol settings
- `attack_config`: Attack range, cooldown, damage, hit frame timing

Example:
```ron
slime: (
    ai_config: (
        detection_range: 200.0,
        attack_range: 32.0,
        aggro_drop_range: 400.0,
        patrol_radius: 100.0,
        wait_time: 2.0,
    ),
    attack_config: (
        attack_cooldown: 1.5,
        attack_hit_frame: 0.2,
    ),
)
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