# Project-Level Acceptance Tests: RustShadowDungeon

**Created**: 2025-11-24
**Specification**: [spec.md](./spec.md)
**Purpose**: Define high-level acceptance criteria for v1.0 public demo release

---

## 1. Platform & Performance Tests

### Test 1.1: 60 FPS on All Tier 1 Platforms

**Priority**: P1 (Blocking)

**Given**: Game running on minimum spec hardware for each platform
**When**: Player completes a full dungeon run with 15+ enemies active
**Then**: 
- ≥95% of frames render in ≤16.67ms
- No stutters or frame drops during combat
- Frame time graph (dev build) shows consistent performance

**Validation**:
- Run on Windows (GTX 1050), Linux (GTX 1050), macOS (M1), WASM (Chrome), Android (Snapdragon 750G)
- Record frame times using Bevy diagnostic plugin
- Generate performance report for each platform

---

### Test 1.2: Memory Budget Compliance

**Priority**: P1 (Blocking)

**Given**: Game running full demo content (3 dungeons, 2 classes)
**When**: Player navigates through all content without restarting
**Then**:
- Desktop: Memory usage ≤512 MB
- WASM: Memory usage ≤256 MB
- Android: Memory usage ≤384 MB
- No memory leaks (usage stable over 1 hour play session)

**Validation**:
- Monitor process memory using platform tools
- Run 1-hour automated test session
- Verify no continuous growth in memory usage

---

### Test 1.3: Load Time Requirements

**Priority**: P2

**Given**: Cold start (no cached assets)
**When**: User launches game
**Then**:
- Initial load to main menu: <5 seconds
- Dungeon selection to gameplay: <2 seconds
- Room transition: <500ms (no loading screen)

**Validation**:
- Instrument loading code with timestamps
- Test on minimum spec hardware
- Average 10 runs per platform

---

## 2. Combat System Tests

### Test 2.1: Damage Calculation Accuracy

**Priority**: P1 (Blocking)

**Given**: Player with known stats (100 base damage, no modifiers)
**When**: Player attacks enemy with 0 armor
**Then**: Enemy takes exactly 100 damage (±0 variance in test mode)

**Given**: Player with 100 base damage + 50% elemental bonus
**When**: Player attacks enemy weak to that element
**Then**: Enemy takes exactly 150 damage

**Given**: Player with 100 base damage, 50% crit rate, 2× crit multiplier
**When**: Player attacks 100 times
**Then**: 
- ~50 hits are critical (45-55 hits acceptable)
- Critical hits deal exactly 200 damage
- Non-critical hits deal exactly 100 damage

**Validation**:
- Automated unit tests in `tests/unit/combat/damage_test.rs`
- Deterministic RNG for test reproducibility
- ≥85% coverage of damage calculation code

---

### Test 2.2: Hit Detection Precision

**Priority**: P1 (Blocking)

**Given**: Player attack with 32×32 hitbox at position (100, 100)
**When**: Enemy hitbox overlaps at position (116, 116) (1 pixel overlap)
**Then**: Hit is registered and damage applied

**Given**: Player attack with 32×32 hitbox at position (100, 100)
**When**: Enemy hitbox is at position (132, 100) (0 pixel overlap)
**Then**: Hit is NOT registered

**Given**: Player executes 3-hit combo
**When**: All hits connect with enemy
**Then**: Exactly 3 damage events fired, enemy hit 3 times

**Validation**:
- Automated integration tests with mock hitboxes
- Visual hitbox debugging mode (dev builds)
- Frame-perfect collision checks

---

### Test 2.3: Status Effect Behavior

**Priority**: P1 (Blocking)

**Given**: Enemy with no status effects
**When**: Player applies "Burn" (5 damage/sec, 3 sec duration)
**Then**:
- Enemy takes 5 damage per second for exactly 3 seconds (15 total)
- Status icon displays above enemy
- Effect expires after 3 seconds

**Given**: Enemy with "Burn" (1 stack)
**When**: Player applies "Burn" again (stackable, max 3)
**Then**: Enemy has 2 stacks, damage doubled, duration refreshed

**Given**: Enemy with "Freeze" (immobilized, 2 sec)
**When**: Enemy takes damage
**Then**: Freeze breaks immediately (unless freeze is damage-persistent type)

**Validation**:
- Automated unit tests for each status effect
- Frame-accurate duration tracking
- Test all stacking/refresh/immunity rules

---

### Test 2.4: Invincibility Frames

**Priority**: P1 (Blocking)

**Given**: Player executes dodge (0.5 sec i-frames)
**When**: Enemy attacks during i-frame window
**Then**: Player takes no damage, sprite flashes white

**Given**: Player has 0.3 sec remaining on i-frames
**When**: Another i-frame source triggers (0.5 sec)
**Then**: Longer duration takes precedence (0.5 sec total from new trigger)

**Validation**:
- Automated tests with frame-accurate timing
- Visual verification (sprite flash)
- Test i-frame overlap behavior

---

### Test 2.5: Combo System

**Priority**: P1 (Blocking)

**Given**: Player on ground, no input
**When**: Player presses Attack → Attack → Attack within timing windows
**Then**: 
- 3-hit combo executes (light, light, heavy)
- Each hit cancels into next
- Combo counter displays "3 HIT COMBO"

**Given**: Player executes 2-hit combo
**When**: Player waits >1 second
**Then**: Combo resets, next attack starts new combo

**Given**: Player executes combo on enemy
**When**: All hits connect
**Then**: Enemy is juggled (stays airborne through combo)

**Validation**:
- Integration tests for combo timing
- Visual verification of animation canceling
- Test juggle physics

---

## 3. Movement & Physics Tests

### Test 3.1: Player Movement Controls

**Priority**: P1 (Blocking)

**Given**: Player on flat ground
**When**: Player holds right arrow key
**Then**: 
- Player moves right at constant speed
- Walk animation plays at correct frame rate
- No jittering or stuttering

**Given**: Player airborne
**When**: Player presses left/right
**Then**: Air control applies (reduced speed vs. ground)

**Given**: Player on ground
**When**: Player presses jump
**Then**: 
- Player launches upward with fixed velocity
- Jump cannot be pressed again until grounded
- Jump animation plays

**Validation**:
- Automated movement tests using bevy_tnua
- Record player position over time, verify velocity
- Test all movement states (ground, air, dash)

---

### Test 3.2: Collision Detection

**Priority**: P1 (Blocking)

**Given**: Player moving right toward wall
**When**: Player collides with wall
**Then**: Player stops, cannot move through wall

**Given**: Player airborne above platform
**When**: Player falls onto platform
**Then**: Player lands on platform, becomes grounded

**Given**: Player on platform edge
**When**: Player walks off edge
**Then**: Player becomes airborne, begins falling

**Validation**:
- Integration tests with bevy_rapier2d
- Test all collision layer interactions
- Verify no clipping through geometry

---

## 4. Dungeon & Progression Tests

### Test 4.1: Room Clear and Progression

**Priority**: P1 (Blocking)

**Given**: Player enters dungeon room with 5 enemies
**When**: Player defeats all 5 enemies
**Then**: 
- Room clear notification displays
- Exit door unlocks
- Room clear tracked in dungeon progress

**Given**: Room cleared
**When**: Player walks through exit door
**Then**: 
- Transition to next room (<500ms)
- New room loads with fresh enemies
- No assets from previous room leak

**Validation**:
- Integration test for full dungeon run
- Verify room state tracking
- Check memory cleanup between rooms

---

### Test 4.2: Enemy AI Behavior

**Priority**: P2

**Given**: Player visible to melee enemy (within 200 pixels)
**When**: Enemy has no active action
**Then**: Enemy chases player (moves toward player position)

**Given**: Player within melee range (32 pixels)
**When**: Enemy attack is off cooldown
**Then**: Enemy executes melee attack

**Given**: Ranged enemy with line-of-sight to player
**When**: Player is 100-300 pixels away
**Then**: Enemy fires ranged projectile toward player

**Validation**:
- Integration tests with mock player positions
- Verify AI state machine transitions
- Test pathfinding (if implemented)

---

### Test 4.3: Loot and Inventory

**Priority**: P2

**Given**: Enemy dies
**When**: Loot drop rolls succeed
**Then**: 
- Loot sprite spawns at enemy position
- Loot type matches loot table
- Loot is pickup-able by player

**Given**: Player walks over loot
**When**: Inventory has space
**Then**: 
- Loot added to inventory
- Pickup sound plays
- Loot sprite despawns

**Given**: Player opens inventory screen
**When**: Player equips weapon
**Then**: 
- Weapon equipped in slot
- Player stats update immediately
- Weapon sprite changes on character

**Validation**:
- Integration tests for loot pipeline
- Test inventory full condition
- Verify stat recalculation on equip

---

## 5. Cross-Platform & Input Tests

### Test 5.1: Keyboard Input

**Priority**: P1 (Blocking)

**Given**: Player using keyboard controls
**When**: Player presses WASD
**Then**: Character moves in corresponding direction

**When**: Player presses Space
**Then**: Character jumps

**When**: Player presses J/K/L
**Then**: Character attacks or uses skills

**Validation**:
- Test on Windows, Linux, macOS
- Verify key bindings work as expected
- Test key rebinding (if implemented)

---

### Test 5.2: Gamepad Input

**Priority**: P1 (Tier 1 for Desktop)

**Given**: Player using Xbox/PlayStation gamepad
**When**: Player moves left analog stick
**Then**: Character moves proportionally

**When**: Player presses A/Cross button
**Then**: Character jumps

**When**: Player presses X/Y/B buttons
**Then**: Character uses skills mapped to those buttons

**Validation**:
- Test with Xbox, PlayStation, generic gamepads
- Verify analog stick sensitivity
- Test button remapping (if implemented)

---

### Test 5.3: Touch Controls (Android/WASM Mobile)

**Priority**: P1 (Tier 1 for Android)

**Given**: Player on touchscreen device
**When**: Player drags virtual joystick
**Then**: Character moves in dragged direction

**When**: Player taps attack button
**Then**: Character attacks

**When**: Player taps skill button
**Then**: Skill executes (if off cooldown)

**Validation**:
- Test on Android devices (3+ models)
- Test on mobile browsers (Chrome, Safari)
- Verify touch responsiveness (<100ms latency)

---

## 6. User Experience Tests

### Test 6.1: Tutorial Comprehension

**Priority**: P2

**Given**: New player starts game for first time
**When**: Tutorial plays
**Then**: 
- Player learns movement within 30 seconds
- Player learns combat within 1 minute
- Player completes tutorial dungeon within 5 minutes

**Validation**:
- User testing with 5+ new players
- Measure time to complete tutorial
- Survey: "Did you understand controls?" (target ≥80% yes)

---

### Test 6.2: Session Engagement

**Priority**: P2

**Given**: Player completes tutorial
**When**: Player has access to demo content
**Then**: 
- Average session length ≥30 minutes
- ≥60% of players complete at least 1 full dungeon
- ≥50% return for 2nd session within 7 days

**Validation**:
- Telemetry data (optional, privacy-respecting)
- Survey feedback on engagement
- Measure via platform analytics

---

### Test 6.3: Accessibility

**Priority**: P2

**Given**: Player using any supported input method
**When**: Player plays demo content
**Then**: 
- Keyboard controls rated ≥7/10
- Gamepad controls rated ≥7/10
- Touch controls rated ≥6/10 (acceptable)

**Validation**:
- User testing across input methods
- Survey ratings from 10+ testers per method

---

## 7. Multiplayer Tests (v0.6+)

### Test 7.1: Lobby and Connection

**Priority**: P2 (Post-MVP)

**Given**: Host creates lobby
**When**: 3 clients join
**Then**: 
- All 4 players see each other in lobby
- Host can start game
- Game transitions to dungeon for all players

**Validation**:
- Integration test with simulated clients
- Test on LAN and internet
- Measure connection success rate (target ≥95%)

---

### Test 7.2: Combat Synchronization

**Priority**: P1 (Blocking for Multiplayer)

**Given**: 2 players in same dungeon
**When**: Player A attacks enemy
**Then**: 
- Player B sees attack animation
- Enemy takes damage on both clients
- Health bars synchronized (±5% tolerance)

**Given**: Player A and Player B attack same enemy simultaneously
**When**: Both hits register on server
**Then**: Both damage values apply (no lost hits)

**Validation**:
- Integration tests with networked clients
- Verify server-authoritative combat
- Test with artificial latency (50ms, 100ms, 150ms)

---

### Test 7.3: Latency Compensation

**Priority**: P1 (Blocking for Multiplayer)

**Given**: Player with 100ms latency
**When**: Player presses attack button
**Then**: 
- Local attack animation plays immediately (client prediction)
- Damage applies after server confirms (~100ms delay)
- No visible "rollback" for player movement

**Validation**:
- Test with artificial latency injection
- Survey: "Did combat feel responsive?" (target ≥7/10 at 100ms)

---

## 8. Constitution Compliance Tests

### Test 8.1: Rust Memory Safety

**Priority**: P1 (NON-NEGOTIABLE)

**Given**: Full codebase
**When**: Code review conducted
**Then**: 
- Zero unsafe blocks without documented justification
- All unsafe blocks have safety comments
- All unsafe blocks have comprehensive tests

**Validation**:
- Automated: `rg "unsafe" --type rust` and check for comments
- Manual code review for any unsafe blocks

---

### Test 8.2: Bevy ECS Compliance

**Priority**: P1 (NON-NEGOTIABLE)

**Given**: All game systems
**When**: Code review conducted
**Then**: 
- Components are pure data (no methods with game logic)
- Systems operate on queries
- No direct entity references stored in components
- Resources used sparingly

**Validation**:
- Manual code review
- Automated: Check for `impl` blocks on components with business logic

---

### Test 8.3: Test Coverage

**Priority**: P1 (NON-NEGOTIABLE)

**Given**: Full codebase
**When**: Coverage report generated
**Then**:
- Domain logic: ≥85%
- Combat systems: ≥85%
- Movement systems: ≥85%
- Skills systems: ≥75%

**Validation**:
- Run `cargo-tarpaulin` or `cargo-llvm-cov`
- Generate HTML report
- Fail CI if coverage drops below thresholds

---

### Test 8.4: Pixel Art Consistency

**Priority**: P1 (MANDATORY)

**Given**: All sprite assets
**When**: Visual inspection conducted
**Then**:
- All sprites use 16×16 base grid
- No rotated or scaled sprites
- Color palette consistent across all assets
- Camera locked to pixel grid (no sub-pixel rendering)

**Validation**:
- Manual review of all sprites
- Automated: Check sprite dimensions are multiples of 16
- Visual testing: Verify no blurred pixels in-game

---

### Test 8.5: Language Separation Rule

**Priority**: P1 (NON-NEGOTIABLE)

**Given**: Full codebase and documentation
**When**: Code and documentation review conducted
**Then**:
- All code identifiers (types, functions, variables, files) use English
- All documentation (specs, README, comments `///`) uses Chinese (for Chinese projects)
- No mixed naming (e.g., `Player玩家.rs`, `玩家Controller`)
- No Chinese in Cargo.toml, mod.rs, pub API identifiers
- Error messages and logs use English (for international community)

**Validation**:
- Automated: `rg '[\p{Han}]' --type rust src/ | grep -v '///'` (no Chinese in code identifiers)
- Automated: Check all `.rs` file names contain no Chinese characters
- Manual review: Documentation uses Chinese consistently
- Manual review: Code comments use English or Chinese appropriately

---

## 9. Release Readiness Tests

### Test 9.1: Zero Critical Bugs

**Priority**: P1 (Blocking)

**Given**: v1.0 release candidate
**When**: Bug triage completed
**Then**: 
- 0 critical bugs (crashes, softlocks, data loss)
- <10 high-priority bugs
- <50 total open issues

**Validation**:
- GitHub Issues triage
- Beta tester feedback review

---

### Test 9.2: Localization

**Priority**: P2

**Given**: Game content
**When**: Language switched to Chinese
**Then**: 
- All UI text displays in Chinese
- No English text remains (except proper nouns)
- Text fits in UI elements (no overflow)

**Validation**:
- Manual testing in both languages
- Screenshot comparison

---

### Test 9.3: First-Time User Experience

**Priority**: P2

**Given**: Completely new player (never played)
**When**: Player launches game for first time
**Then**: 
- Player understands how to start (within 30 sec)
- Player completes tutorial (within 5 min)
- Player rates initial experience ≥7/10

**Validation**:
- User testing with 10+ new players
- Record session and identify friction points
- Survey feedback

---

## Test Execution Summary

### Phase 1: Unit & Integration (Continuous)
- Run on every PR via CI
- Automated tests for combat, movement, damage
- Target: ≥85% coverage maintained

### Phase 2: Platform Testing (Weekly during v0.4+)
- Test on all Tier 1 platforms
- Performance profiling on min-spec hardware
- Document any platform-specific issues

### Phase 3: User Testing (v0.9 Beta)
- Recruit 100+ beta testers
- Collect feedback via surveys + Discord
- Iterate on reported issues

### Phase 4: Release Validation (Pre-v1.0)
- Execute all P1 acceptance tests
- Verify all constitution compliance
- Final sign-off from maintainers

---

**Status**: ✅ Acceptance Criteria Defined

These tests MUST pass before v1.0 public demo release.

