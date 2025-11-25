# RustShadowDungeon Development Guidelines

Auto-generated from all feature plans. Last updated: [DATE]

## Active Technologies

- **Language**: Rust (stable, latest)
- **Game Engine**: Bevy (0.12+)
- **Physics**: bevy_rapier2d
- **Level Editor**: LDtk or Tiled (bevy_ecs_ldtk)
- **Build Tool**: Cargo
- **Testing**: cargo test, criterion (benchmarks)

## Project Structure

```text
src/
├── main.rs              # Game entry point
├── lib.rs               # Library root (for testing)
├── plugins/             # Bevy plugins (one per major system)
├── components/          # ECS components (pure data)
├── systems/             # ECS systems (behavior)
├── resources/           # Global game state
└── events/              # Event definitions

tests/
├── integration/         # Full system tests
└── unit/                # Component/system unit tests

assets/
├── sprites/             # Pixel art (16x16 grid)
├── audio/
├── levels/
└── data/                # RON config files

benches/                 # Performance benchmarks
```

## Commands

```bash
# Development
cargo run                        # Run game
cargo run --release              # Run optimized build
cargo test                       # Run all tests
cargo test --test [test_name]   # Run specific test
cargo bench                      # Run benchmarks
cargo clippy                     # Linting
cargo fmt                        # Format code

# Bevy-specific
RUST_LOG=debug cargo run        # Run with debug logging
RUST_LOG=info,wgpu=warn cargo run # Run with filtered logs
cargo run --features bevy/dynamic_linking # Fast compile times
```

## Code Style

**Rust Idioms**:
- Use `Result` and `Option` instead of panicking
- Avoid `unwrap()` and `expect()` in production code (dev/test okay with justification)
- Prefer `if let` and `match` over forced unwrapping
- Use descriptive variable names (no single letters except iterators)
- Document public API with `///` doc comments

**Bevy ECS Patterns**:
- Components are pure data (derive `Component`)
- Systems are functions with Query parameters
- Use `Commands` for deferred spawn/despawn
- Use `EventReader`/`EventWriter` for inter-system communication
- Resources for global state (use sparingly)
- Organize into plugins for modularity

**Performance**:
- Avoid allocations in hot loops
- Use `Query::iter()` efficiently
- Batch entity spawning with `Commands`
- Profile before optimizing (`cargo flamegraph`)

**Testing**:
- Write tests first (TDD)
- Test components and systems independently
- Use `App::new()` in tests for isolated system testing
- Mock resources/events for unit tests

## Recent Changes

[LAST 3 FEATURES AND WHAT THEY ADDED]

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
