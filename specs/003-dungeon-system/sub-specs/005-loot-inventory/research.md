# Research: Loot and Inventory System

**Feature**: 005-loot-inventory
**Date**: 2025-01-27
**Status**: Complete

## Research Tasks

### 1. Loot Table Probability Calculation (Independent vs Mutually Exclusive)

**Question**: How should loot table probability be calculated - independent rolls or mutually exclusive selection?

**Decision**: Independent probability calculation (each item rolls independently)

**Rationale**:
- Matches common RPG design patterns (Diablo, Path of Exile)
- Allows multiple items to drop simultaneously, increasing reward variety
- Simpler implementation (no normalization needed)
- More flexible for game designers (can set individual drop rates without worrying about total probability)

**Alternatives Considered**:
- **Mutually exclusive (weighted selection)**: Only one item drops per enemy. Rejected because it limits reward variety and requires normalization of weights.
- **Guaranteed + bonus drops**: One guaranteed drop plus independent bonus drops. Considered but deferred to future enhancement (not in MVP scope).

**Implementation**:
```rust
// Domain layer: Pure function
pub fn calculate_loot_drops(
    loot_table: &LootTable,
    rng: &mut impl Rng,
) -> Vec<LootDrop> {
    loot_table.items
        .iter()
        .filter_map(|entry| {
            if rng.gen::<f32>() < entry.chance {
                Some(LootDrop {
                    item_id: entry.item_id,
                    quantity: rng.gen_range(entry.quantity_min..=entry.quantity_max),
                })
            } else {
                None
            }
        })
        .collect()
}
```

### 2. Inventory Slot Management Strategy

**Question**: How should inventory slots be managed - fixed array, Vec with capacity, or separate slot entity?

**Decision**: Fixed-size array (`[Option<InventorySlot>; 30]`) in `InventoryComponent`

**Rationale**:
- Simple and performant (no allocations during gameplay)
- Fixed size matches specification (30 slots)
- Easy to serialize for save games
- Clear memory layout (stack-allocated)

**Alternatives Considered**:
- **Vec with capacity**: More flexible but requires heap allocation. Rejected because specification requires fixed 30 slots.
- **Separate slot entities**: More ECS-idiomatic but adds complexity. Rejected because slots are tightly coupled to inventory (not independent entities).

**Implementation**:
```rust
// Component: Pure data
#[derive(Component)]
pub struct InventoryComponent {
    slots: [Option<InventorySlot>; 30],
}

#[derive(Clone)]
pub struct InventorySlot {
    item_id: ItemId,
    quantity: u32,
}
```

### 3. Item Stacking Implementation

**Question**: How should item stacking be handled - merge on add, separate stacking system, or component-based?

**Decision**: Merge on add with domain layer validation

**Rationale**:
- Keeps inventory state consistent
- Domain layer function validates stacking rules (same item ID, stackable flag, max stack)
- Infrastructure layer calls domain function before updating components

**Alternatives Considered**:
- **Separate stacking system**: Runs after add, requires two passes. Rejected for complexity.
- **Component-based stacking**: Each stackable item has a `Stackable` component. Rejected because stacking is a property of `ItemDefinition`, not runtime state.

**Implementation**:
```rust
// Domain layer: Pure function
pub fn can_stack_items(
    item1: &ItemDefinition,
    item2: &ItemDefinition,
) -> bool {
    item1.id == item2.id && item1.stackable
}

pub fn calculate_stack_result(
    current_quantity: u32,
    add_quantity: u32,
    max_stack: u32,
) -> StackResult {
    let total = current_quantity + add_quantity;
    if total <= max_stack {
        StackResult::Merge(total)
    } else {
        StackResult::Split {
            remaining: max_stack,
            overflow: total - max_stack,
        }
    }
}
```

### 4. Pickup Range Detection

**Question**: How should pickup range be detected - physics queries, distance calculation, or trigger zones?

**Decision**: Distance calculation with spatial partitioning (throttled checks)

**Rationale**:
- Simple and performant (no physics queries needed)
- 2-meter range is straightforward distance check
- Can be throttled (check every 3-5 frames) to reduce cost
- Works well with Bevy's Transform component

**Alternatives Considered**:
- **Physics trigger zones**: More accurate but requires physics setup. Rejected for complexity (not needed for simple distance check).
- **Spatial hash grid**: Better for many items. Considered but deferred (optimization for future if needed).

**Implementation**:
```rust
// Domain layer: Pure function
pub fn is_within_pickup_range(
    item_pos: Vec2,
    player_pos: Vec2,
    range: f32,
) -> bool {
    item_pos.distance(player_pos) <= range
}

// Infrastructure layer: System (throttled)
fn pickup_range_check_system(
    mut query: Query<&mut WorldItem>,
    player_query: Query<&Transform, (With<Player>, Without<WorldItem>)>,
    time: Res<Time>,
) {
    // Throttle: Check every 3 frames
    if time.elapsed_seconds() % 0.05 < 0.016 { // ~3 frames at 60 FPS
        return;
    }
    // ... distance check logic
}
```

### 5. Manual vs Automatic Pickup

**Question**: How should manual and automatic pickup modes be implemented?

**Decision**: Configurable pickup mode with separate systems

**Rationale**:
- Separate systems for manual and automatic keep logic clear
- Pickup mode stored in `PickupConfig` resource
- Player can toggle mode (future enhancement, not in MVP)
- Both modes use same domain layer functions

**Alternatives Considered**:
- **Single system with mode check**: Simpler but less clear separation. Rejected for maintainability.
- **Event-based pickup**: Pickup triggered by events. Considered but automatic pickup needs per-frame checks anyway.

**Implementation**:
```rust
// Resource: Pickup configuration
#[derive(Resource)]
pub struct PickupConfig {
    pub mode: PickupMode,
}

pub enum PickupMode {
    Manual,
    Automatic,
}

// Systems: Separate for each mode
fn manual_pickup_system(
    input: Res<Input<KeyCode>>,
    // ... pickup logic
) {
    if input.just_pressed(KeyCode::E) {
        // Trigger pickup
    }
}

fn automatic_pickup_system(
    // ... pickup logic
) {
    // Auto-pickup when in range
}
```

### 6. Integration with Combat System

**Question**: How should loot drops be triggered from enemy death events?

**Decision**: Listen to `EnemyDefeated` event from combat/enemy systems

**Rationale**:
- Loose coupling via events (follows Bevy ECS best practices)
- No direct dependency on combat/enemy systems
- Can be tested independently with mock events

**Alternatives Considered**:
- **Direct system coupling**: Query enemy health directly. Rejected for tight coupling.
- **Component-based death flag**: Enemy has `Dead` component. Considered but events are cleaner for cross-system communication.

**Implementation**:
```rust
// Event: From combat/enemy system
pub struct EnemyDefeated {
    pub enemy_entity: Entity,
    pub enemy_type: EnemyType,
    pub position: Vec2,
}

// System: Listen to event
fn loot_drop_system(
    mut events: EventReader<EnemyDefeated>,
    loot_tables: Res<Assets<LootTable>>,
    // ... spawn world items
) {
    for event in events.read() {
        let loot_table = loot_tables.get(&event.enemy_type.loot_table_id)?;
        let drops = domain::loot::calculate_loot_drops(loot_table, &mut rng);
        // Spawn WorldItem entities
    }
}
```

## Summary

All technical decisions align with:
- DDD architecture (pure domain layer, infrastructure bridge)
- Bevy ECS patterns (components, systems, events, resources)
- Constitution compliance (no unsafe, modular, testable)
- Performance budget (<1ms per frame)
- Language separation (English code, Chinese docs)

No blocking technical unknowns remain. Ready for Phase 1 design.

