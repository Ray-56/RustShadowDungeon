# Dungeon System Developer Documentation

## 地下城系统开发者文档

This document provides a comprehensive guide to the dungeon system architecture, implementation details, and extension methods.

本文档提供地下城系统的架构、实现细节和扩展方法的全面指南。

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Domain Layer](#domain-layer)
3. [Infrastructure Layer](#infrastructure-layer)
4. [System Execution Order](#system-execution-order)
5. [Events and Messages](#events-and-messages)
6. [Extending the System](#extending-the-system)
7. [Testing](#testing)
8. [Performance Considerations](#performance-considerations)

---

## Architecture Overview

The dungeon system follows **Domain-Driven Design (DDD)** principles, separating pure business logic from Bevy ECS integration.

地下城系统遵循**领域驱动设计（DDD）**原则，将纯业务逻辑与 Bevy ECS 集成分离。

### Layer Separation

```
┌─────────────────────────────────────┐
│   Domain Layer (Pure Rust)          │
│   src/domain/dungeon/               │
│   - Zero Bevy dependencies          │
│   - Pure functions                  │
│   - Testable in isolation           │
└─────────────────────────────────────┘
              ↕ (called by)
┌─────────────────────────────────────┐
│   Infrastructure Layer (Bevy ECS)  │
│   src/infrastructure/               │
│   - Components (data)               │
│   - Systems (behavior)              │
│   - Events (messages)              │
│   - Resources (global state)        │
└─────────────────────────────────────┘
```

### Key Principles

1. **Domain Logic is Pure**: All business rules live in `src/domain/dungeon/` with no Bevy dependencies
2. **Infrastructure Bridges**: Systems in `src/infrastructure/systems/dungeon.rs` call domain functions
3. **Event-Driven**: Systems communicate via Bevy events/messages
4. **State Consistency**: Room state is managed through `DungeonSession` resource

---

## Domain Layer

**Location**: `src/domain/dungeon/progression.rs`

The domain layer contains pure functions that implement dungeon progression logic.

### Core Functions

#### Room State Management

```rust
pub enum RoomState {
    Active,      // Room is currently loaded
    Cleared,     // All enemies defeated
    Uncleared,   // Not yet cleared
}

pub fn is_room_cleared(room_state: RoomState) -> bool
pub fn mark_room_cleared(room_state: RoomState) -> RoomState
```

#### Spawn Decision Logic

```rust
pub fn should_spawn_enemies(room_state: RoomState) -> bool
```

Returns `true` only for `Uncleared` rooms.

#### Room Clearing Logic

```rust
pub fn check_room_cleared(enemy_count: usize) -> bool
```

Returns `true` when `enemy_count == 0`.

#### Door Unlocking Logic

```rust
pub fn should_unlock_doors(room_state: RoomState) -> bool
```

Returns `true` for `Cleared` rooms.

#### Room Transition Logic

```rust
pub fn can_transition_to_room(door_state: DoorState) -> bool
pub fn get_target_room_entrance(entrance_position: Vec2) -> Vec2
```

### Testing Domain Logic

All domain functions are pure and can be tested without Bevy:

```rust
#[test]
fn test_room_cleared_check() {
    assert!(check_room_cleared(0));
    assert!(!check_room_cleared(1));
}
```

---

## Infrastructure Layer

### Components

**Location**: `src/infrastructure/components/dungeon.rs`

#### Room Component

```rust
#[derive(Component, Debug)]
pub struct Room {
    pub room_id: RoomId,
    pub state: RoomState,  // Re-exported from domain layer
    pub spawn_points: Vec<Vec2>,
}
```

#### Door Component

```rust
#[derive(Component, Debug)]
pub struct Door {
    pub door_id: DoorId,
    pub connected_room_id: RoomId,
    pub door_state: DoorState,
    pub entrance_position: Vec2,
}
```

#### DungeonManager Component

```rust
#[derive(Component, Debug)]
pub struct DungeonManager {
    pub current_room_id: RoomId,
    pub room_graph: HashMap<RoomId, Vec<(DoorId, RoomId)>>,
}
```

#### EnemySpawnPoint Component

```rust
#[derive(Component, Debug)]
pub struct EnemySpawnPoint {
    pub position: Vec2,
    pub enemy_type: String,
    pub spawn_on_activate: bool,
}
```

### Resources

**Location**: `src/infrastructure/resources/dungeon.rs`

#### DungeonSession Resource

```rust
#[derive(Resource, Debug, Default)]
pub struct DungeonSession {
    pub cleared_rooms: HashSet<RoomId>,
}
```

Tracks which rooms have been cleared to prevent enemy respawning.

### Events/Messages

**Location**: `src/infrastructure/events/dungeon.rs`

#### RoomEntered

```rust
#[derive(Message, Debug, Clone)]
pub struct RoomEntered {
    pub room_id: RoomId,
    pub player_position: Vec2,
}
```

Published when player enters a room.

#### RoomCleared

```rust
#[derive(Message, Debug, Clone)]
pub struct RoomCleared {
    pub room_id: RoomId,
    pub cleared_at: f64,
}
```

Published when all enemies in a room are defeated.

#### DoorUnlocked

```rust
#[derive(Message, Debug, Clone)]
pub struct DoorUnlocked {
    pub door_id: DoorId,
    pub room_id: RoomId,
}
```

Published when a door is unlocked.

#### RoomTransitioned

```rust
#[derive(Message, Debug, Clone)]
pub struct RoomTransitioned {
    pub from_room_id: RoomId,
    pub to_room_id: RoomId,
    pub player_position: Vec2,
}
```

Published when player transitions between rooms.

#### PlayerDeathInRoom

```rust
#[derive(Message, Debug, Clone)]
pub struct PlayerDeathInRoom {
    pub room_id: RoomId,
    pub death_position: Vec2,
}
```

Published when player dies in a dungeon room.

### Systems

**Location**: `src/infrastructure/systems/dungeon.rs`

#### Core Systems

1. **`load_dungeon_system`**: Initializes dungeon and first room
2. **`initialize_room_system`**: Creates room entity, doors, and spawn points
3. **`spawn_enemies_system`**: Spawns enemies based on room state
4. **`check_room_clear_system`**: Checks if room should be marked as cleared
5. **`handle_room_cleared_system`**: Updates room state and unlocks doors
6. **`unlock_doors_system`**: Updates door states to Unlocked
7. **`handle_door_interaction_system`**: Handles player interaction with doors
8. **`transition_to_room_system`**: Teleports player to new room
9. **`unload_current_room_system`**: Removes old room entities (batch optimized)
10. **`track_room_clear_system`**: Records cleared rooms in DungeonSession

#### Edge Case Systems

- **`handle_player_death_in_dungeon_system`**: Respawns player at room entrance
- **`handle_empty_room_system`**: Auto-clears rooms with no enemies
- **`ensure_state_consistency_system`**: Ensures doors match room state

---

## System Execution Order

The execution order is critical for correct dungeon behavior. Systems are registered in `DungeonPlugin` with explicit ordering:

### Update Schedule

```rust
// 1. Initialize room (runs once at start, and after room transitions)
initialize_room_system,

// 2. Spawn enemies (runs after room initialization)
spawn_enemies_system.after(initialize_room_system),

// 3. Check room clear (runs after enemies are spawned and death animations complete)
check_room_clear_system
    .after(enemy_death_animation_update_system)
    .after(spawn_enemies_system)
    .after(apply_deferred),

// 4. Track cleared rooms
track_room_clear_system.after(check_room_clear_system),

// 5. Handle room cleared event
handle_room_cleared_system,

// 6. Unlock doors
unlock_doors_system,

// 7. Handle door interactions (runs early to catch just_pressed events)
handle_door_interaction_system,

// 8. Transition to room
transition_to_room_system,

// 9. Unload previous room
unload_current_room_system.after(transition_to_room_system),

// 10. Restore room state (when returning to previously visited rooms)
restore_room_state_system,
```

### Why Order Matters

- **`check_room_clear_system`** must run after `spawn_enemies_system` and `apply_deferred` to ensure newly spawned enemies are available
- **`track_room_clear_system`** must run after `check_room_clear_system` to record cleared rooms
- **`unload_current_room_system`** must run after `transition_to_room_system` to ensure player is teleported before cleanup

---

## Events and Messages

The dungeon system uses Bevy's `Message` system (events) for decoupled communication.

### Event Flow Example: Room Clearing

```
1. Enemy dies → EnemyDefeated event
2. check_room_clear_system → checks enemy count
3. If count == 0 → RoomCleared event
4. track_room_clear_system → records in DungeonSession
5. handle_room_cleared_system → updates Room component
6. unlock_doors_system → updates Door components
7. DoorUnlocked events → published for each door
```

### Listening to Events

```rust
pub fn my_custom_system(
    mut room_cleared_events: MessageReader<RoomCleared>,
) {
    for event in room_cleared_events.read() {
        // Handle room cleared
    }
}
```

---

## Extending the System

### Adding a New Room Type

1. **Define room configuration** in `assets/data/dungeons/your_dungeon.ron`
2. **Update `load_dungeon_system`** to load from RON (currently hardcoded)
3. **Customize spawn points** in `initialize_room_system` based on room ID

### Adding Custom Room Behaviors

1. **Add domain logic** in `src/domain/dungeon/progression.rs`
2. **Create system** in `src/infrastructure/systems/dungeon.rs`
3. **Register system** in `DungeonPlugin` with proper ordering

### Adding New Events

1. **Define event** in `src/infrastructure/events/dungeon.rs`:
   ```rust
   #[derive(Message, Debug, Clone)]
   pub struct MyCustomEvent {
       pub room_id: RoomId,
       // ... fields
   }
   ```

2. **Publish event** in your system:
   ```rust
   my_events.write(MyCustomEvent { room_id, ... });
   ```

3. **Listen to event** in other systems:
   ```rust
   mut events: MessageReader<MyCustomEvent>,
   ```

### Customizing Enemy Spawns

Modify `spawn_enemies_system` to:
- Read from `EnemySpawnPoint` components
- Check `DungeonSession.cleared_rooms` to prevent respawning
- Spawn different enemy types based on room configuration

---

## Testing

### Unit Tests

**Location**: `tests/unit/dungeon/progression_test.rs`

Test domain logic in isolation:

```rust
#[test]
fn test_should_spawn_enemies() {
    assert!(should_spawn_enemies(RoomState::Uncleared));
    assert!(!should_spawn_enemies(RoomState::Cleared));
}
```

### Integration Tests

**Location**: `tests/integration/dungeon/`

Test full workflows:

- **`room_transition_test.rs`**: Tests complete room transition flow
- **`progress_tracking_test.rs`**: Tests room state persistence

### Performance Benchmarks

**Location**: `benches/dungeon_bench.rs`

Run with:
```bash
cargo bench --bench dungeon_bench
```

Target: <16ms per frame for room operations (60 FPS budget)

---

## Performance Considerations

### Optimizations Implemented

1. **Batch Entity Despawning**: `unload_current_room_system` collects all entities before despawning
2. **State Caching**: `DungeonSession` caches cleared rooms to avoid repeated checks
3. **Conditional Spawning**: Enemies only spawn for `Uncleared` rooms

### Future Optimizations

- **Lazy Loading**: Load rooms on-demand instead of all at once
- **Resource Pooling**: Reuse enemy entities instead of spawning/destroying
- **Spatial Partitioning**: Optimize door interaction detection with spatial queries

### Performance Budget

- **Room Loading**: <5ms
- **Room Transition**: <10ms (including unload + load)
- **Enemy Spawning**: <2ms per enemy
- **Total Frame Budget**: <16.67ms (60 FPS)

---

## Configuration

### Dungeon Configuration (RON)

**Location**: `assets/data/dungeons/test_dungeon.ron`

```ron
(
    dungeon_id: "test_dungeon",
    start_room_id: 1,
    rooms: [
        (
            room_id: 1,
            spawn_points: [
                [100.0, 0.0],
                [200.0, 0.0],
                [300.0, 0.0],
            ],
            doors: [
                (
                    door_id: 1,
                    connected_room_id: 2,
                    entrance_position: [100.0, 0.0],
                ),
            ],
        ),
        // ... more rooms
    ],
)
```

**Note**: Currently, `load_dungeon_system` uses hardcoded room graph. RON loading is a TODO.

---

## Troubleshooting

### Common Issues

1. **Rooms cleared immediately after spawning**
   - **Cause**: `check_room_clear_system` running before enemies are spawned
   - **Fix**: Ensure `check_room_clear_system.after(spawn_enemies_system).after(apply_deferred)`

2. **Doors not unlocking**
   - **Cause**: `RoomCleared` event not being published
   - **Fix**: Check `check_room_clear_system` is counting enemies correctly

3. **Player can't enter doors**
   - **Cause**: Interaction distance too small or door state incorrect
   - **Fix**: Check `INTERACTION_DISTANCE` in `handle_door_interaction_system` (currently 150.0)

4. **Enemies respawn in cleared rooms**
   - **Cause**: `DungeonSession` not being checked in `spawn_enemies_system`
   - **Fix**: Ensure `spawn_enemies_system` checks `dungeon_session.cleared_rooms`

---

## Summary

The dungeon system provides a robust, extensible foundation for multi-room dungeon exploration. Key takeaways:

1. **Domain logic is pure** - testable without Bevy
2. **Infrastructure bridges** - systems call domain functions
3. **Event-driven** - systems communicate via messages
4. **State consistency** - `DungeonSession` tracks progress
5. **Performance optimized** - batch processing, state caching

For questions or contributions, see the main [README.md](../README.md) and project [constitution](.specify/memory/constitution.md).

