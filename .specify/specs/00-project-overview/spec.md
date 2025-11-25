# Project Specification: 锈影地下城 (RustShadowDungeon)

**Project Branch**: `main` (master specification)
**Created**: 2025-11-24
**Status**: Authoritative - Single Source of Truth
**Constitution**: v1.0.1 (all NON-NEGOTIABLE and MANDATORY rules enforced)
**Language**: 中文（本规范使用中文，代码使用英文）

---

## Executive Summary

RustShadowDungeon (锈影地下城) is a DNF-inspired 2D side-scrolling action RPG that combines fast-paced combo-based combat with dungeon exploration, character progression, and cooperative multiplayer. Built with Rust and Bevy engine, the game targets 60 FPS performance across all major platforms while maintaining pixel-perfect 16×16 art consistency.

**Core Value Proposition**: Deliver the visceral, combo-driven combat feel of Dungeon Fighter Online in a modern, open-source Rust game engine with cross-platform support and mod-friendly architecture.

---

## 1. Project Vision & Core Loop

### Vision Statement

Create a fast-paced 2D action RPG that captures the essence of DNF's combat system—fluid combos, satisfying hit feedback, and skill-based gameplay—while being fully open-source, community-driven, and technically excellent through Rust's safety guarantees and Bevy's ECS architecture.

### Core Gameplay Loop

1. **Select Character & Skills**: Choose character class, equip skills on hotbar
2. **Enter Dungeon**: Load into side-scrolling dungeon room
3. **Engage in Combat**: 
   - Execute combos using directional inputs + attack buttons
   - Chain skills with normal attacks
   - Dodge enemy attacks using i-frames and movement
   - Manage skill cooldowns and resource costs
4. **Clear Room & Progress**: Defeat all enemies, move to next room
5. **Collect Loot**: Gather equipment, consumables, crafting materials
6. **Boss Fight**: Face challenging boss with unique mechanics
7. **Complete Dungeon**: Return to hub, process rewards
8. **Character Progression**: 
   - Level up and allocate skill points
   - Equip better gear
   - Unlock new skills and combos
9. **Repeat**: Enter next dungeon with increased difficulty

### DNF-Inspired Mechanics

- **Combo System**: Attacks can be canceled into other attacks or skills
- **Juggling**: Launch enemies into air and continue combos
- **Super Armor**: Certain attacks cannot be interrupted
- **Hitstun & Hitfreeze**: Enemy reactions create satisfying combat feel
- **Skill Tree**: Multiple skill paths per character class
- **Equipment Enhancement**: Gear can be upgraded and enchanted
- **Dungeon Fatigue**: Daily dungeon entry limits (optional for single-player)

### Game Modes (Prioritized)

**Priority 1 (v1.0 MVP)**:
- Solo Adventure Mode
- Practice/Training Room

**Priority 2 (v1.1+)**:
- 2-4 Player Co-op
- Boss Rush Mode

**Priority 3 (v2.0+)**:
- PvP Arena (1v1, team battles)
- Endless Tower Mode
- Seasonal Events

---

## 2. Target Platforms

### Tier 1 Platforms (v1.0 Launch)

All Tier 1 platforms MUST achieve 60 FPS on specified minimum hardware:

| Platform | Minimum Hardware | Target Resolution | Input Methods |
|----------|------------------|-------------------|---------------|
| **Windows** | Intel Core i5-7400 / AMD Ryzen 3 1200, 8GB RAM, GTX 1050 | 1920×1080 (windowed/fullscreen) | Keyboard, Gamepad, Mouse (UI only) |
| **Linux** | Same as Windows | 1920×1080 (windowed/fullscreen) | Keyboard, Gamepad, Mouse (UI only) |
| **macOS** | M1 chip or Intel i5-8250U, 8GB RAM | 1920×1080 (native scaling) | Keyboard, Gamepad, Mouse (UI only) |
| **Web (WASM)** | Modern browser (Chrome 90+, Firefox 88+, Safari 14+) | Responsive (min 1280×720) | Keyboard, Mouse, Touch (mobile browsers) |
| **Android** | Snapdragon 750G / equivalent, 4GB RAM | 1080p (adaptive scaling) | Touch (virtual joystick), Gamepad |

### Tier 2 Platforms (v1.2+)

| Platform | Status | Notes |
|----------|--------|-------|
| **iOS** | Post-v1.0 | Pending Apple Store approval process and Bevy iOS maturity |

### Platform-Specific Considerations

- **Windows/Linux/macOS**: Full feature parity, 60 FPS minimum
- **Web (WASM)**: Reduced particle effects if needed for performance, offline-capable PWA
- **Android**: Optimized touch controls, optional on-screen gamepad overlay, cloud save sync
- **iOS**: Same as Android when implemented

---

## 3. High-Level DDD Bounded Contexts

The game is organized into seven domain-driven bounded contexts, each with clear responsibilities and interfaces:

### 3.1 Player Context

**Responsibility**: Manage player character state, input handling, movement, and basic interactions.

**Core Entities**:
- Player (character identity, stats, position, state machine)
- PlayerInput (input buffering, command mapping)
- MovementState (grounded, airborne, dashing, stunned)

**Key Behaviors**:
- Process directional input and movement
- Handle jumps, dashes, and air control
- Manage player state transitions
- Handle collision with terrain

**Events Published**:
- PlayerMoved
- PlayerJumped
- PlayerDashed
- PlayerStateChanged

**Events Consumed**:
- CombatHitConfirmed (for hitstun/knockback)
- SkillActivated (for animation locks)

### 3.2 Combat Context

**Responsibility**: Handle all combat mechanics including damage calculation, hit detection, status effects, and combat feedback.

**Core Entities**:
- Attack (hitbox, damage, element, properties)
- HitResult (damage dealt, critical, status applied)
- StatusEffect (type, duration, stacks)
- ComboCounter (hits, damage, time window)

**Key Behaviors**:
- Detect hitbox collisions
- Calculate damage (base + modifiers + critical)
- Apply status effects (stun, burn, freeze, poison)
- Track combos and juggling
- Manage invincibility frames
- Provide hit feedback (hitstop, screen shake, particles)

**Events Published**:
- DamageDealt
- StatusEffectApplied
- ComboExtended
- CombatHitConfirmed

**Events Consumed**:
- AttackInitiated (from Skills Context)
- EnemyDefeated (from Dungeon Context)

### 3.3 Skills Context

**Responsibility**: Manage skill system including cooldowns, resource costs, skill effects, and skill trees.

**Core Entities**:
- Skill (ID, resource cost, cooldown, effects)
- SkillHotbar (equipped skills, quick slots)
- SkillTree (unlocked skills, skill points)
- ResourcePool (MP, SP, rage, etc.)

**Key Behaviors**:
- Validate skill usage (cooldown, resource, state)
- Consume resources and trigger cooldowns
- Execute skill effects (damage, buffs, summons)
- Handle skill animations and canceling
- Track skill experience and mastery

**Events Published**:
- SkillActivated
- SkillCooldownStarted
- SkillUnlocked
- AttackInitiated

**Events Consumed**:
- PlayerStateChanged (for skill availability)
- CombatHitConfirmed (for skill procs/resets)

### 3.4 Dungeon Context

**Responsibility**: Manage dungeon structure, room progression, enemy spawning, and environmental interactions.

**Core Entities**:
- Dungeon (ID, difficulty, room layout)
- Room (enemies, triggers, exits)
- Enemy (type, AI, stats, loot table)
- EnvironmentalHazard (traps, destructibles)

**Key Behaviors**:
- Load and unload dungeon rooms
- Spawn enemies in waves
- Manage room clear conditions
- Control room transitions
- Handle environmental hazards
- Manage boss encounters

**Events Published**:
- RoomCleared
- EnemySpawned
- EnemyDefeated
- BossEncounterStarted
- DungeonCompleted

**Events Consumed**:
- DamageDealt (for enemy health tracking)
- PlayerStateChanged (for room entry triggers)

### 3.5 Inventory Context

**Responsibility**: Manage equipment, consumables, crafting materials, and item interactions.

**Core Entities**:
- Equipment (weapon, armor, accessories)
- Consumable (potions, buffs, scrolls)
- Material (crafting, enhancement)
- InventorySlot (item reference, stack count)

**Key Behaviors**:
- Store and organize items
- Equip/unequip gear
- Apply item stats to player
- Handle item usage (consumables)
- Manage inventory capacity
- Process item drops and pickup

**Events Published**:
- ItemEquipped
- ItemUsed
- ItemAcquired
- EquipmentStatsChanged

**Events Consumed**:
- EnemyDefeated (for loot drops)
- DungeonCompleted (for rewards)

### 3.6 Networking Context

**Responsibility**: Handle multiplayer synchronization, client-server communication, and network state management.

**Core Entities**:
- Session (host, players, session ID)
- NetworkPlayer (remote player state, input prediction)
- SyncState (authoritative state, client predictions)

**Key Behaviors**:
- Establish and maintain connections
- Synchronize player positions and states
- Handle client-side prediction and rollback
- Validate and process remote inputs
- Resolve combat events authoritatively
- Manage session lifecycle (join, leave, disconnect)

**Events Published**:
- PlayerJoined
- PlayerLeft
- NetworkStateUpdated
- DesyncDetected

**Events Consumed**:
- ALL combat and player events (for synchronization)

**Note**: Tier 2 feature—single-player must work perfectly first.

### 3.7 UI Context

**Responsibility**: Render and manage all user interface elements including HUD, menus, inventory screens, and feedback.

**Core Entities**:
- HUD (health bar, skill cooldowns, combo counter)
- Menu (main menu, pause, settings)
- Dialog (NPC, quest, tutorial)
- Notification (damage numbers, item pickups)

**Key Behaviors**:
- Display real-time game state (health, resources, cooldowns)
- Handle menu navigation and input
- Render damage numbers and floating text
- Show inventory and equipment screens
- Display tutorials and tooltips
- Handle settings and configuration

**Events Published**:
- MenuItemSelected
- SettingChanged
- TutorialDismissed

**Events Consumed**:
- ALL game events (for UI updates and feedback)

---

## 4. Global Architecture Rules

### Pure Domain Layer (Zero Bevy Dependencies)

**Rule 1**: All domain logic MUST reside in `src/domain/` with ZERO dependencies on Bevy, bevy_rapier2d, or any game engine crates.

**Rationale**: Domain models represent pure game rules (damage calculation, status effects, skill logic). These must be testable without engine context and potentially portable to other engines or tools.

**Structure**:
```
src/domain/
├── combat/
│   ├── damage.rs       # Pure damage calculation functions
│   ├── status.rs       # Status effect rules
│   └── combo.rs        # Combo system logic
├── skills/
│   ├── skill_data.rs   # Skill definitions (data)
│   └── cooldown.rs     # Cooldown management
├── player/
│   └── stats.rs        # Stat calculations
└── dungeon/
    └── progression.rs  # Room clear logic
```

**Example**:
```rust
// ✅ GOOD: Pure domain function
pub fn calculate_damage(
    base: f32,
    attacker_stats: &Stats,
    defender_stats: &Stats,
    element: Element,
) -> DamageResult {
    // Pure calculation, no Bevy types
}

// ❌ BAD: Domain coupled to engine
pub fn calculate_damage(
    attacker: &Query<&AttackComponent>,
    defender: Entity,
) -> DamageResult {
    // Depends on Bevy Query - NOT ALLOWED
}
```

### Infrastructure Bridge Layer

**Rule 2**: All Bevy integration MUST happen in `src/systems/` and `src/plugins/` which act as bridges between domain logic and engine.

**Responsibilities**:
- Query ECS components
- Extract data from components
- Call pure domain functions
- Update components with results
- Publish Bevy events

**Structure**:
```
src/systems/
├── combat_systems.rs   # Bridges to domain/combat
├── skill_systems.rs    # Bridges to domain/skills
└── player_systems.rs   # Bridges to domain/player

src/plugins/
├── combat.rs           # Registers combat systems + resources
├── skills.rs           # Registers skill systems + resources
└── player.rs           # Registers player systems + resources
```

**Example**:
```rust
// ✅ GOOD: System bridges domain to ECS
pub fn apply_damage_system(
    mut commands: Commands,
    attackers: Query<(&Attack, &Stats)>,
    mut defenders: Query<(&mut Health, &Stats, &Transform)>,
) {
    for (attack, attacker_stats) in attackers.iter() {
        // Extract data from ECS
        let base_damage = attack.damage;
        
        // Call pure domain function
        let result = domain::combat::calculate_damage(
            base_damage,
            attacker_stats,
            defender_stats,
            attack.element,
        );
        
        // Update ECS components
        defender_health.current -= result.final_damage;
    }
}
```

### Component Design Rules

**Rule 3**: Components MUST be pure data structures (no methods except constructors and simple getters).

**Rule 4**: Components MUST NOT contain logic or mutable methods that change game state.

```rust
// ✅ GOOD: Pure data component
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

// ✅ ACCEPTABLE: Simple constructor
impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

// ❌ BAD: Component with game logic
impl Health {
    pub fn take_damage(&mut self, amount: f32, armor: f32) {
        let reduced = amount * (1.0 - armor);
        self.current -= reduced; // Logic in component - NOT ALLOWED
    }
}
```

### Testing Strategy by Layer

**Rule 5**: Each layer has specific testing requirements:

- **Domain Layer**: Unit tests with 85%+ coverage, no mocking needed (pure functions)
- **Systems Layer**: Integration tests using `App::new()` with minimal setup
- **Plugins Layer**: Smoke tests to verify registration and basic lifecycle

---

## 5. Performance Targets

### 60 FPS Frame Budget (16.67ms)

The game MUST maintain 60 FPS on all Tier 1 platforms under specified load conditions.

### Frame Budget Breakdown

| System Category | Budget (ms) | Budget (%) | Monitored Systems |
|-----------------|-------------|------------|-------------------|
| **Input Processing** | 0.5 | 3% | Input collection, command buffering |
| **Physics & Collision** | 3.0 | 18% | bevy_rapier2d update, custom hitbox checks |
| **Combat Logic** | 2.5 | 15% | Damage calculation, status effects, combo tracking |
| **AI & Dungeon Logic** | 2.0 | 12% | Enemy AI updates, room transitions |
| **Animation & State** | 1.5 | 9% | Sprite animation, state machine updates |
| **Rendering Preparation** | 4.0 | 24% | Transform updates, sprite sorting, camera |
| **Rendering (GPU)** | 2.5 | 15% | Sprite batching, shader execution |
| **Audio** | 0.5 | 3% | Sound effect triggering, music streaming |
| **Buffer** | 0.17 | 1% | Margin for frame variance |

**Total**: 16.67ms (60 FPS)

### Load Conditions for Testing

Performance MUST be validated under these conditions:

- **Baseline**: 1 player, 5 enemies, 1 room, 30 particle effects active
- **Heavy Combat**: 1 player, 15 enemies, 2 rooms loaded, 100 particles, 3 DOT effects active
- **Multiplayer**: 4 players, 10 enemies, 200 particles total, networked
- **Boss Fight**: 1 player, 1 boss (complex AI), 50 particles, 5 environmental hazards

### Performance Monitoring

- **Dev Builds**: Display frame time graph in top-right corner (toggleable)
- **CI Pipeline**: Run `cargo bench` on critical paths, fail PR if >10% regression
- **Profiling**: Use `bevy_framepace` and `tracing-chrome` for detailed profiling
- **Metrics Tracked**:
  - Frame time (avg, p95, p99)
  - System execution time (per-system breakdown)
  - Entity count
  - Particle count
  - Physics body count

### Memory Budget

| Platform | Target | Hard Limit | Notes |
|----------|--------|------------|-------|
| Desktop (Win/Linux/macOS) | 512 MB | 1 GB | Includes assets, entities, buffers |
| Web (WASM) | 256 MB | 512 MB | Browser memory constraints |
| Android | 384 MB | 768 MB | Low-end device support |

### Asset Loading Performance

- **Initial Load**: <5 seconds from launch to main menu
- **Dungeon Load**: <2 seconds from selection to gameplay
- **Room Transition**: <500ms between rooms (no loading screen)
- **Asset Streaming**: Background loading for next room while in current room

---

## 6. Art & Audio Standards

### Pixel Art Standards

**Grid System**: 16×16 pixel base unit (character sprites, tiles, UI elements)

**Color Palette**:
- Fixed 64-color palette (DawnBringer 32 extended to 64 colors)
- Consistent across ALL sprites
- No gradients or anti-aliasing on sprite edges
- Dithering allowed for texture depth

**Sprite Specifications**:

| Asset Type | Dimensions | Frame Count | Notes |
|------------|------------|-------------|-------|
| Player Character | 32×32 (2×2 tiles) | 8-12 per animation | Idle, walk, jump, attack, hit, death |
| Small Enemy | 16×16 (1×1 tile) | 4-8 per animation | Idle, walk, attack, death |
| Medium Enemy | 32×32 (2×2 tiles) | 8-12 per animation | Same as player |
| Boss | 64×64 to 128×128 | 12-24 per animation | Complex attack animations |
| Terrain Tile | 16×16 (1×1 tile) | Static or 2-4 frames | Platforms, walls, hazards |
| Item/Loot | 16×16 (1×1 tile) | 1-2 frames | Optional idle animation |
| UI Elements | Multiples of 16 | Static | Buttons, panels, borders |
| Particles | 4×4 to 8×8 | 4-8 frames | Hit sparks, magic effects |

**Camera Settings**:
- **Pixel-Perfect Rendering**: Camera locked to 16×16 grid (no sub-pixel positioning)
- **Viewport**: Integer scaling only (1×, 2×, 3×, 4×)
- **Zoom**: Fixed zoom level per room type (no dynamic zoom)
- **Smoothing**: Nearest-neighbor filtering (no bilinear/trilinear)

**Animation Standards**:
- **Frame Rate**: 12 FPS for character animations (holds 5 frames at 60 FPS)
- **No Rotation**: Never rotate sprites (causes pixel distortion)
- **No Scaling**: Never scale sprites dynamically (pre-render multiple sizes if needed)
- **Flipping**: Horizontal flip allowed for directional sprites
- **Timing**: All animation timings in animation data files (RON format)

### Audio Standards

**Music**:
- Format: OGG Vorbis (streaming)
- Sample Rate: 44.1 kHz
- Bitrate: 128-192 kbps
- Loop Points: Seamless loop tags in metadata
- Layers: Combat intensity layers (add instruments as combat escalates)

**Sound Effects**:
- Format: WAV or OGG Vorbis
- Sample Rate: 44.1 kHz or 22.05 kHz
- Bit Depth: 16-bit
- Max Duration: <2 seconds per effect
- Categories: UI, footsteps, attacks, hits, skills, environment

**Audio Mixing**:
- Master Volume: 0 dB reference
- Music: -12 dB default (user adjustable)
- SFX: -6 dB default (user adjustable)
- Voices/Dialog: -3 dB default (user adjustable)
- Ducking: Music reduces -6 dB during intense combat or dialog

**Spatial Audio**:
- 2D panning based on horizontal position (left/right only)
- Volume falloff with distance (exponential curve)
- No vertical audio positioning

---

## 7. Testing Strategy

### Test-Driven Development (NON-NEGOTIABLE)

**Workflow**: All combat and movement features MUST follow TDD:
1. Write failing tests that verify specification
2. User approves tests
3. Implement feature to pass tests
4. Refactor with tests passing

### Coverage Requirements

| Layer | Min Coverage | Test Types | Priority |
|-------|--------------|------------|----------|
| **Domain Logic** | 85% | Unit tests (pure functions) | P1 (blocking) |
| **Combat Systems** | 85% | Unit + Integration | P1 (blocking) |
| **Movement Systems** | 85% | Unit + Integration | P1 (blocking) |
| **Skills Systems** | 75% | Unit + Integration | P1 (blocking) |
| **Infrastructure** | 60% | Integration + Smoke | P2 |
| **UI Systems** | 40% | Smoke tests | P3 |

### Combat Testing Requirements (Constitution Principle V)

Every combat mechanic MUST have tests for:

**Damage Calculation**:
- Base damage applied correctly
- Elemental modifiers (fire, ice, lightning, etc.)
- Critical hit calculation (rate and multiplier)
- Damage variance (random range)
- Armor/defense reduction
- Edge case: zero damage, overflow prevention

**Hit Detection**:
- Hitbox accuracy (position and size)
- Multi-hit attacks (count and timing)
- Piercing attacks (hit multiple enemies)
- I-frame interaction (hits ignored during invincibility)
- Edge case: simultaneous hits from multiple sources

**Status Effects**:
- Application conditions (chance, resist check)
- Duration tracking (frame-accurate)
- Stacking rules (stack count, max stacks, refresh)
- Immunity conditions
- Visual feedback presence
- Edge case: multiple conflicting effects

**Skill System**:
- Cooldown accuracy (frame-perfect timing)
- Resource costs (MP, SP, rage consumption)
- Combo interactions (canceling, chaining)
- Canceling windows (when skill can be interrupted)
- Edge case: skill use during stun, simultaneous skills

**Invincibility Frames**:
- Duration (frame count)
- Interaction with status effects (apply vs ignore)
- Visual feedback (flashing sprite)
- Edge case: overlapping i-frame sources

**Edge Cases & Exploits**:
- Zero damage edge case
- Integer overflow in damage calculation
- Simultaneous deaths (player and enemy)
- Death during skill animation
- Negative health handling
- Resource underflow (negative MP/SP)

### Integration Testing

**System Interaction Tests**:
- Input → Movement → Animation pipeline
- Skill activation → Combat → Damage → Health update
- Enemy death → Loot drop → Inventory pickup
- Room clear → Transition → Next room load
- Equipment change → Stat update → Damage recalculation

**Performance Tests** (Benchmarks):
- Combat system with 15 enemies (must be <2.5ms avg)
- Hit detection with 50 active hitboxes (must be <1.0ms avg)
- Status effect updates with 20 effects active (must be <0.5ms avg)
- Particle system with 100 particles (must be <1.0ms avg)

### Continuous Integration (CI)

**On Every PR**:
- `cargo fmt --check` (formatting)
- `cargo clippy -- -D warnings` (linting, zero warnings allowed)
- `cargo test` (all unit and integration tests)
- `cargo bench --no-run` (verify benchmarks compile)

**On Main Branch**:
- Full `cargo bench` with result archiving
- Performance regression detection (>10% regression fails)
- WASM build verification

### Testing Tools

- **Unit Tests**: `cargo test`
- **Benchmarks**: `criterion` crate
- **Mocking**: Minimal (pure domain functions don't need mocks)
- **Integration**: Bevy `App::new()` with minimal plugin setup
- **Coverage**: `cargo-tarpaulin` or `cargo-llvm-cov`

---

## 8. Roadmap with Milestones

### v0.1 - Foundation (Weeks 1-3)

**Goal**: Prove core technical feasibility—player movement, basic combat, pixel-perfect rendering.

**Deliverables**:
- [ ] Player character spawns and renders (16×16 grid-locked)
- [ ] WASD movement with physics (grounded, airborne states)
- [ ] Basic attack (single hitbox, damage calculation)
- [ ] Dummy enemy takes damage and dies
- [ ] 60 FPS on test hardware
- [ ] TDD workflow established (tests for movement and damage)

**Success Criteria**:
- Player can move and attack enemy
- No frame drops below 60 FPS with 5 enemies
- All core systems have ≥85% test coverage

---

### v0.2 - Combat Feel (Weeks 4-6)

**Goal**: Implement DNF-style combat mechanics—combos, juggling, hit feedback.

**Deliverables**:
- [ ] Combo system (3-hit basic combo)
- [ ] Launching and juggling enemies
- [ ] Hitstop (hitfreeze on impact)
- [ ] Screen shake on heavy hits
- [ ] Particle effects (hit sparks, dust)
- [ ] Invincibility frames (dodging)
- [ ] 2-3 skills with cooldowns

**Success Criteria**:
- Combos feel responsive and satisfying
- Players can juggle enemies consistently
- Combat passes blind playtest feedback (5+ testers rate "fun" ≥7/10)
- All combat mechanics have ≥85% test coverage

---

### v0.3 - Dungeon & Progression (Weeks 7-10)

**Goal**: Build dungeon structure, room transitions, enemy AI, and basic progression.

**Deliverables**:
- [ ] Multi-room dungeon (3 rooms + boss room)
- [ ] Room transitions with door triggers
- [ ] Basic enemy AI (chase player, melee attack)
- [ ] Ranged enemy type
- [ ] Boss encounter (simple pattern)
- [ ] Experience and leveling
- [ ] Skill tree (unlock 5 skills)
- [ ] Loot drops and inventory

**Success Criteria**:
- Players can complete a full dungeon run (5-10 minutes)
- Enemy AI provides challenge without being unfair
- Progression feels rewarding (unlock new skills each level)
- Dungeon runs are repeatable and fun

---

### v0.4 - Platform Support (Weeks 11-13)

**Goal**: Ensure cross-platform compatibility and performance.

**Deliverables**:
- [ ] Windows build (tested on 3+ machines)
- [ ] Linux build (tested on Ubuntu, Arch)
- [ ] macOS build (tested on Intel + Apple Silicon)
- [ ] Web WASM build (deployed to test URL)
- [ ] Android build (tested on 2+ devices)
- [ ] Gamepad support (Xbox, PlayStation, generic)
- [ ] Touch controls (Android/WASM mobile)

**Success Criteria**:
- All Tier 1 platforms hit 60 FPS on minimum hardware
- Input feels responsive on all control schemes
- WASM build loads in <5 seconds

---

### v0.5 - Content & Polish (Weeks 14-17)

**Goal**: Add content variety and polish for public demo readiness.

**Deliverables**:
- [ ] 5 unique dungeons with themes
- [ ] 10+ enemy types
- [ ] 3 character classes (warrior, mage, rogue)
- [ ] 20+ skills across all classes
- [ ] Equipment system (weapons, armor, accessories)
- [ ] Crafting and enhancement
- [ ] Main menu and settings
- [ ] Save/load system
- [ ] Tutorial and tooltips

**Success Criteria**:
- 2+ hours of unique content per class
- Players can customize builds (skill + equipment variety)
- New players understand mechanics within 5 minutes (tutorial)

---

### v0.6 - Multiplayer (Weeks 18-22)

**Goal**: Implement 2-4 player cooperative multiplayer.

**Deliverables**:
- [ ] Lobby system (host, join)
- [ ] Client-side prediction and rollback
- [ ] Server-authoritative combat
- [ ] Synchronized enemy AI
- [ ] Loot distribution rules
- [ ] Latency compensation (<100ms feels good)
- [ ] Reconnection handling

**Success Criteria**:
- 4 players can complete dungeon together without desyncs
- Combat feels responsive up to 100ms latency
- No game-breaking exploits in multiplayer

---

### v0.7 - Community Features (Weeks 23-25)

**Goal**: Enable community engagement and modding.

**Deliverables**:
- [ ] Modding documentation (how to add skills, enemies, dungeons)
- [ ] Hot-reload for assets (faster iteration)
- [ ] Replay system (save and playback runs)
- [ ] Leaderboards (fastest clear times)
- [ ] Discord integration (rich presence)
- [ ] Feedback/bug report UI

**Success Criteria**:
- 3+ community members create and share mods
- Replay system used for speedrunning
- Leaderboards drive competition

---

### v0.8 - Performance Optimization (Weeks 26-28)

**Goal**: Optimize for lower-end hardware and mobile.

**Deliverables**:
- [ ] Profiling and bottleneck identification
- [ ] Entity pooling for bullets/particles
- [ ] Culling for off-screen entities
- [ ] LOD for distant sprites (if needed)
- [ ] Android-specific optimizations (battery, thermal)
- [ ] WASM bundle size reduction (<10 MB)

**Success Criteria**:
- Desktop targets 60 FPS on hardware 2 years older than minimum spec
- Android maintains 60 FPS on mid-range devices (Snapdragon 600 series)
- WASM loads in <3 seconds on 4G connection

---

### v0.9 - Beta Testing (Weeks 29-32)

**Goal**: Public beta with feedback iteration.

**Deliverables**:
- [ ] Open beta announcement
- [ ] Steam page and itch.io page
- [ ] Beta keys distributed to 100+ testers
- [ ] Feedback collection (surveys, Discord, in-game)
- [ ] Balance adjustments (skill damage, enemy health)
- [ ] Bug fixes (prioritize crashes, softlocks)
- [ ] Localization (English, Chinese)

**Success Criteria**:
- ≥80% of testers rate experience ≥7/10
- <5 critical bugs remaining
- Average session length ≥30 minutes
- Retention: 50% of testers play 3+ sessions

---

### v1.0 - Public Demo Release (Week 33)

**Goal**: Launch polished public demo on all Tier 1 platforms.

**Deliverables**:
- [ ] Demo content: 3 dungeons, 2 character classes, 1 boss
- [ ] All Tier 1 platforms released simultaneously
- [ ] Trailer and press kit
- [ ] Launch on Steam, itch.io, web, Google Play
- [ ] Community Discord server
- [ ] Roadmap for post-1.0 content

**Success Criteria (v1.0)**:
- ≥10,000 downloads in first month
- ≥4.0/5.0 average rating on platforms
- ≥60% completion rate for demo content
- <1% crash rate across all platforms
- Community engagement: ≥500 Discord members
- Press coverage: ≥5 gaming outlets feature the game
- Performance: 60 FPS on all Tier 1 platforms, 100% tested hardware
- Open source: ≥10 external contributors to GitHub repo

---

## 9. Success Metrics for v1.0

### Technical Metrics (Constitution Compliance)

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Frame Rate** | 60 FPS (≥95% of frames) | Telemetry from all platforms |
| **Memory Usage** | <512 MB (desktop), <256 MB (WASM), <384 MB (Android) | Profiling data |
| **Test Coverage** | ≥85% (domain + combat + movement) | `cargo-tarpaulin` report |
| **Crash Rate** | <1% of sessions | Error reporting (Sentry or similar) |
| **Load Time** | <5 sec (initial), <2 sec (dungeon) | Instrumented timing |

### User Engagement Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Downloads** | ≥10,000 (first month) | Platform analytics |
| **Session Length** | ≥30 min (average) | Telemetry (optional, privacy-respecting) |
| **Completion Rate** | ≥60% (demo content) | Telemetry (optional) |
| **Retention** | 50% play ≥3 sessions | Telemetry (optional) |
| **User Rating** | ≥4.0/5.0 | Platform reviews |

### Community Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Discord Members** | ≥500 | Discord server stats |
| **GitHub Stars** | ≥100 | GitHub insights |
| **Contributors** | ≥10 external contributors | GitHub contributors page |
| **Mods/Content** | ≥5 community mods | itch.io/GitHub/Discord |
| **Press Coverage** | ≥5 gaming outlets | PR tracking |

### Quality of Life Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Bug Reports** | <50 open issues at launch | GitHub issues |
| **Critical Bugs** | 0 at launch | Triage process |
| **User Feedback** | ≥7/10 "fun" rating | Surveys |
| **Accessibility** | Gamepad + keyboard + touch all rated ≥7/10 | Surveys |

---

## Appendices

### A. Technology Stack (Locked Versions)

| Dependency | Version | Purpose |
|------------|---------|---------|
| **Rust** | 1.91.1 (stable) | Language (memory safety, performance) |
| **Bevy** | 0.17.0 | Game engine (ECS, rendering, audio) |
| **bevy_rapier2d** | 0.29+ | Physics and collision |
| **bevy_tnua** | latest stable | Character controller (movement feel) |
| **leafwing-input-manager** | latest stable | Input abstraction (keyboard, gamepad, touch) |
| **bevy_ecs_ldtk** | latest stable | Level loading from LDtk editor |
| **serde** | latest | Serialization (save games, config) |
| **ron** | latest | Data format (skills, enemies, items) |
| **criterion** | latest | Benchmarking |

All crates must be latest stable compatible with Rust 1.91.1 as of November 2025.

### B. Glossary

- **DNF**: Dungeon Fighter Online, the inspiration for combat feel
- **ECS**: Entity Component System (Bevy's architecture)
- **TDD**: Test-Driven Development (write tests first)
- **I-Frames**: Invincibility frames (brief immunity during dodges/hits)
- **Hitstop**: Frame freeze on hit for impact feedback
- **Juggling**: Keeping enemy airborne with consecutive hits
- **Super Armor**: Attack that cannot be interrupted
- **Combo**: Chain of attacks that cannot be escaped
- **DDD**: Domain-Driven Design (separation of concerns)
- **PWA**: Progressive Web App (installable web version)

### C. References

- **Constitution**: `.specify/memory/constitution.md` (v1.0.1 - Language Separation Rule)
- **Bevy Docs**: https://bevyengine.org/learn/
- **Rust Book**: https://doc.rust-lang.org/book/
- **DNF Mechanics**: Namu Wiki, DFO Global Wiki

---

## Document Control

**Authoritative Version**: This document is the single source of truth for RustShadowDungeon.

**Amendments**: Changes to this spec require:
1. GitHub issue with proposed changes and rationale
2. Discussion with core team
3. Approval from project maintainer
4. Version bump and changelog entry
5. Update all dependent feature specs

**Version History**:
- v1.0.0 (2025-11-24): Initial authoritative specification
- v1.0.0.1 (2025-11-24): Updated to comply with Constitution v1.0.1 (Language Separation Rule)

**Related Documents**:
- `.specify/memory/constitution.md` (governance and principles)
- Feature specs in `.specify/specs/[###-feature-name]/`

**Status**: ✅ Ready for Feature Planning

All future feature specifications MUST reference and comply with this document.

