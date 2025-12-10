# Boss 遭遇战系统接口契约

**Created**: 2025-12-03
**Feature**: 006-boss-encounter
**Purpose**: 定义 Boss 系统与其他系统的接口契约

---

## 概述

本文档定义 Boss 遭遇战系统与其他系统（Enemy AI、Combat Core、Loot System）之间的接口契约，确保系统间解耦和可测试性。

---

## 系统接口

### 1. Enemy AI 系统集成

**依赖关系**: Boss AI 建立在 Enemy AI 基础架构之上（004-enemy-ai）

**接口方式**:
- Boss 实体继承 `Enemy` 组件
- `BossController` 组件扩展 Boss 特定行为
- Boss AI 系统在 Enemy AI 系统之后运行（可覆盖部分行为）

**接口契约**:
```rust
// Boss 实体必须包含 Enemy 组件
Entity {
    Enemy,              // 从 Enemy AI 系统继承
    Boss,               // Boss 标记组件
    BossController,     // Boss 特定控制器
    Health,             // 生命值组件（Combat Core）
    Transform,          // 位置变换
    // ... 其他 Enemy AI 组件
}
```

---

### 2. Combat Core 系统集成

**依赖关系**: 伤害判定和血量管理复用 Combat Core（002-combat-core）

**接口方式**:
- Boss 使用 `Health` 组件（来自 Combat Core）
- 伤害系统发布 `DamageDealt` 事件
- Boss 系统监听 `DamageDealt` 事件，检测血量阈值

**接口契约**:
```rust
// Boss 系统监听伤害事件
pub fn handle_boss_damage(
    mut events: EventReader<DamageDealt>,
    mut boss_query: Query<(&mut Health, &mut BossController), With<Boss>>,
) {
    for event in events.read() {
        // 检查是否伤害到 Boss
        // 检测血量阈值，触发阶段转换
    }
}

// Boss 系统发布阶段转换事件
pub fn trigger_phase_transition(
    boss_entity: Entity,
    from_phase: usize,
    to_phase: usize,
    mut events: EventWriter<BossPhaseTransition>,
) {
    events.send(BossPhaseTransition {
        boss_entity,
        from_phase,
        to_phase,
    });
}
```

---

### 3. Loot System 集成

**依赖关系**: Boss 死亡后的掉落逻辑复用 Loot System（005-loot-inventory）

**接口方式**:
- Boss 死亡时发布 `BossDefeated` 事件
- Loot System 监听 `BossDefeated` 事件，触发掉落逻辑

**接口契约**:
```rust
// Boss 系统发布死亡事件
pub fn handle_boss_death(
    boss_entity: Entity,
    boss_id: String,
    mut events: EventWriter<BossDefeated>,
) {
    events.send(BossDefeated {
        boss_entity,
        boss_id,
    });
}

// Loot System 监听事件（在其他系统中实现）
pub fn spawn_boss_loot(
    mut events: EventReader<BossDefeated>,
    // ... Loot System 逻辑
) {
    for event in events.read() {
        // 生成 Boss 掉落物
    }
}
```

---

## 事件契约

### BossEncounterStarted

**发布者**: Boss 系统  
**监听者**: UI 系统（显示 Boss 血条）、Dungeon 系统（房间锁定）

**数据结构**:
```rust
#[derive(Event)]
pub struct BossEncounterStarted {
    pub boss_entity: Entity,
    pub boss_id: String,
}
```

---

### BossPhaseTransition

**发布者**: Boss 系统  
**监听者**: UI 系统（更新阶段指示器）、动画系统（播放转换动画）

**数据结构**:
```rust
#[derive(Event)]
pub struct BossPhaseTransition {
    pub boss_entity: Entity,
    pub from_phase: usize,
    pub to_phase: usize,
}
```

---

### BossDefeated

**发布者**: Boss 系统  
**监听者**: Loot System（生成掉落）、Dungeon 系统（标记房间清理）

**数据结构**:
```rust
#[derive(Event)]
pub struct BossDefeated {
    pub boss_entity: Entity,
    pub boss_id: String,
}
```

---

## 配置加载接口

### BossConfig 资源

**加载时机**: 游戏启动时或进入地下城时

**加载方式**:
```rust
fn load_boss_config(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // 从 RON 文件加载 Boss 定义
    let config = load_ron_config::<BossDefinitions>("data/bosses.ron");
    commands.insert_resource(BossConfig {
        bosses: config.bosses,
    });
}
```

---

## 性能契约

- **Boss AI 处理**: <1.5ms per frame
- **阶段转换逻辑**: <0.5ms per frame
- **预警渲染**: <1.0ms per frame
- **总开销**: <3.2ms per frame（在战斗逻辑预算内）

---

## 测试接口

### 单元测试接口

```rust
// 领域层函数可直接测试（无 Bevy 依赖）
#[test]
fn test_phase_transition_logic() {
    let phases = vec![/* ... */];
    let result = check_phase_transition(0, 0.4, &phases, false);
    assert_eq!(result, PhaseTransitionResult::Transition { from_phase: 0, to_phase: 1 });
}
```

### 集成测试接口

```rust
// 使用 Bevy 测试工具创建测试 App
fn setup_boss_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugins(BossPlugin);
    app
}
```

---

**Document Status**: ✅ 系统接口契约定义完成






