# Data Model: 战斗系统核心 (Combat System Core)

**Feature**: 002-combat-core  
**Date**: 2025-11-25  
**Phase**: Phase 1 - Design & Contracts

---

## 数据模型概览

战斗系统数据模型分为两层：

1. **领域层（Domain Layer）**：纯数据结构，零 Bevy 依赖
2. **基础设施层（Infrastructure Layer）**：Bevy Components，桥接 ECS

---

## Domain Layer Entities（纯数据，零 Bevy 依赖）

### DamageResult

**描述**: 伤害计算结果

**用途**: `calculate_damage()` 纯函数的返回值

**字段**:

| 字段名 | 类型 | 描述 | 约束 |
|--------|------|------|------|
| `final_damage` | `f32` | 最终伤害值 | 1.0..=9999.0 |
| `is_critical` | `bool` | 是否暴击 | - |
| `element` | `Element` | 元素类型 | Physical, Fire, Ice, Lightning |

**示例**:
```rust
pub struct DamageResult {
    pub final_damage: f32,
    pub is_critical: bool,
    pub element: Element,
}
```

---

### Stats

**描述**: 战斗属性（攻击者/防御者共用）

**用途**: 伤害计算输入参数

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `attack` | `f32` | 攻击力 | 10.0 |
| `defense` | `f32` | 防御力 | 0.0 |
| `crit_rate` | `f32` | 暴击率 | 0.1 (10%) |
| `crit_multiplier` | `f32` | 暴击倍率 | 2.0 |
| `element_resistances` | `HashMap<Element, f32>` | 元素抗性 | 空（无抗性） |

**验证规则**:
- `crit_rate`: 0.0..=1.0（0%-100%）
- `crit_multiplier`: 1.0..=10.0（1x-10x）
- `element_resistances`: -1.0..=1.0（-100% 弱点 to +100% 免疫）

**示例**:
```rust
pub struct Stats {
    pub attack: f32,
    pub defense: f32,
    pub crit_rate: f32,
    pub crit_multiplier: f32,
    pub element_resistances: HashMap<Element, f32>,
}

impl Stats {
    pub fn new(attack: f32, defense: f32) -> Self {
        Self {
            attack,
            defense,
            crit_rate: 0.1,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        }
    }
}
```

---

### Element

**描述**: 元素类型枚举

**用途**: 伤害计算、元素修正

**变体**:
- `Physical`: 物理伤害（无元素）
- `Fire`: 火元素（对冰弱点 1.5x）
- `Ice`: 冰元素（对火弱点 1.5x）
- `Lightning`: 雷元素（对水弱点 1.5x，本版本无水元素）

**示例**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Physical,
    Fire,
    Ice,
    Lightning,
}
```

---

### ComboState

**描述**: 连击状态枚举

**用途**: 连击系统状态机

**变体**:
- `Idle`: 无连击状态
- `FirstHit`: 第 1 击（轻击，10 伤害）
- `SecondHit`: 第 2 击（轻击，10 伤害）
- `ThirdHit`: 第 3 击（重击，20 伤害）

**状态转换**:
```
Idle → FirstHit (按攻击键)
FirstHit → SecondHit (连击窗口内按攻击键)
SecondHit → ThirdHit (连击窗口内按攻击键)
ThirdHit → Idle (连击结束)
FirstHit/SecondHit → Idle (连击窗口超时)
```

**示例**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboState {
    Idle,
    FirstHit,
    SecondHit,
    ThirdHit,
}
```

---

### Rect

**描述**: 矩形碰撞区域（AABB）

**用途**: HitBox / HurtBox 碰撞检测

**字段**:

| 字段名 | 类型 | 描述 | 单位 |
|--------|------|------|------|
| `x` | `f32` | 左上角 X 坐标 | 像素 |
| `y` | `f32` | 左上角 Y 坐标 | 像素 |
| `width` | `f32` | 宽度 | 像素 |
| `height` | `f32` | 高度 | 像素 |

**辅助函数**:
```rust
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
    
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}
```

---

## Infrastructure Layer Components（Bevy Components）

### HitBox

**描述**: 攻击判定框（Component）

**用途**: 检测攻击是否命中敌人

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `rect` | `Rect` | 碰撞区域（相对父实体） | - |
| `damage` | `f32` | 基础伤害 | 10.0 |
| `element` | `Element` | 元素类型 | Physical |
| `lifetime_frames` | `u32` | 持续帧数 | 10 (167ms) |
| `can_pierce` | `bool` | 是否穿透 | false |
| `hit_entities` | `HashSet<Entity>` | 已命中的实体（去重） | 空 |

**生命周期**:
- HitBox 在攻击动画特定帧生成（如第 3 帧）
- 持续 `lifetime_frames` 帧后自动销毁
- 每帧检测与所有 HurtBox 的碰撞

**示例**:
```rust
#[derive(Component)]
pub struct HitBox {
    pub rect: Rect,
    pub damage: f32,
    pub element: Element,
    pub lifetime_frames: u32,
    pub can_pierce: bool,
    pub hit_entities: HashSet<Entity>,
}

impl HitBox {
    pub fn new(rect: Rect, damage: f32) -> Self {
        Self {
            rect,
            damage,
            element: Element::Physical,
            lifetime_frames: 10,
            can_pierce: false,
            hit_entities: HashSet::new(),
        }
    }
}
```

---

### HurtBox

**描述**: 受击判定框（Component）

**用途**: 标记实体可被攻击

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `rect` | `Rect` | 碰撞区域（相对父实体） |
| `is_invincible` | `bool` | 是否无敌（忽略碰撞） |

**示例**:
```rust
#[derive(Component)]
pub struct HurtBox {
    pub rect: Rect,
    pub is_invincible: bool,
}

impl HurtBox {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            is_invincible: false,
        }
    }
}
```

---

### Health

**描述**: 生命值（Component）

**用途**: 追踪实体生命值，判定死亡

**字段**:

| 字段名 | 类型 | 描述 | 约束 |
|--------|------|------|------|
| `current` | `f32` | 当前生命值 | 0.0..=max |
| `max` | `f32` | 最大生命值 | >0 |

**辅助方法**:
```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
    
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
    
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn health_percentage(&self) -> f32 {
        self.current / self.max
    }
}
```

---

### Combo

**描述**: 连击状态（Component）

**用途**: 追踪玩家连击进度

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `state` | `ComboState` | 当前连击状态 | Idle |
| `window_remaining` | `f32` | 连击窗口剩余时间（秒） | 0.0 |
| `hit_count` | `u32` | 当前连击数 | 0 |

**状态转换逻辑**:
```rust
#[derive(Component)]
pub struct Combo {
    pub state: ComboState,
    pub window_remaining: f32,
    pub hit_count: u32,
}

impl Combo {
    pub fn new() -> Self {
        Self {
            state: ComboState::Idle,
            window_remaining: 0.0,
            hit_count: 0,
        }
    }
    
    pub fn advance(&mut self, window_duration: f32) {
        self.state = match self.state {
            ComboState::Idle => ComboState::FirstHit,
            ComboState::FirstHit => ComboState::SecondHit,
            ComboState::SecondHit => ComboState::ThirdHit,
            ComboState::ThirdHit => ComboState::Idle,
        };
        self.window_remaining = window_duration;
        self.hit_count += 1;
    }
    
    pub fn reset(&mut self) {
        self.state = ComboState::Idle;
        self.window_remaining = 0.0;
        self.hit_count = 0;
    }
}
```

---

### Skill

**描述**: 技能数据（Component）

**用途**: 存储技能状态（冷却、MP 消耗）

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `skill_id` | `String` | 技能 ID（如 "fireball"） | - |
| `cooldown` | `f32` | 冷却时间（秒） | 5.0 |
| `remaining_cooldown` | `f32` | 剩余冷却（秒） | 0.0 |
| `mp_cost` | `f32` | MP 消耗 | 20.0 |
| `damage` | `f32` | 伤害值 | 30.0 |
| `element` | `Element` | 元素类型 | Fire |

**辅助方法**:
```rust
#[derive(Component)]
pub struct Skill {
    pub skill_id: String,
    pub cooldown: f32,
    pub remaining_cooldown: f32,
    pub mp_cost: f32,
    pub damage: f32,
    pub element: Element,
}

impl Skill {
    pub fn is_ready(&self) -> bool {
        self.remaining_cooldown <= 0.0
    }
    
    pub fn activate(&mut self) {
        self.remaining_cooldown = self.cooldown;
    }
    
    pub fn update(&mut self, delta: f32) {
        if self.remaining_cooldown > 0.0 {
            self.remaining_cooldown -= delta;
            if self.remaining_cooldown < 0.0 {
                self.remaining_cooldown = 0.0;
            }
        }
    }
}
```

---

### Invincibility

**描述**: 无敌帧（Component）

**用途**: 玩家受击后短暂无敌

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `remaining` | `f32` | 剩余无敌时间（秒） | 0.0 |
| `flash_timer` | `f32` | 闪烁计时器（秒） | 0.0 |

**辅助方法**:
```rust
#[derive(Component)]
pub struct Invincibility {
    pub remaining: f32,
    pub flash_timer: f32,
}

impl Invincibility {
    pub fn new(duration: f32) -> Self {
        Self {
            remaining: duration,
            flash_timer: 0.0,
        }
    }
    
    pub fn is_invincible(&self) -> bool {
        self.remaining > 0.0
    }
    
    pub fn update(&mut self, delta: f32) {
        if self.remaining > 0.0 {
            self.remaining -= delta;
            self.flash_timer += delta;
        }
    }
    
    pub fn should_flash(&self) -> bool {
        (self.flash_timer * 10.0) as i32 % 2 == 0 // 每 0.1 秒切换
    }
}
```

---

### Particle

**描述**: 粒子（Component）

**用途**: 打击特效粒子

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `lifetime` | `f32` | 剩余寿命（秒） |
| `velocity` | `Vec2` | 速度（像素/秒） |
| `color` | `Color` | 颜色（RGBA） |

**示例**:
```rust
#[derive(Component)]
pub struct Particle {
    pub lifetime: f32,
    pub velocity: Vec2,
    pub color: Color,
}
```

---

## Infrastructure Layer Resources（全局资源）

### CombatConfig

**描述**: 战斗配置（Resource）

**用途**: 全局战斗参数配置

**字段**:

| 字段名 | 类型 | 描述 | 默认值 |
|--------|------|------|--------|
| `combo_window` | `f32` | 连击窗口时间（秒） | 1.0 |
| `hitfreeze_light` | `f32` | 轻击定格时间（秒） | 0.05 (3 帧) |
| `hitfreeze_heavy` | `f32` | 重击定格时间（秒） | 0.083 (5 帧) |
| `hitfreeze_critical` | `f32` | 暴击定格时间（秒） | 0.117 (7 帧) |
| `invincibility_duration` | `f32` | 无敌帧时长（秒） | 0.5 |
| `damage_number_lifetime` | `f32` | 伤害数字显示时长（秒） | 1.0 |

**示例**:
```rust
#[derive(Resource)]
pub struct CombatConfig {
    pub combo_window: f32,
    pub hitfreeze_light: f32,
    pub hitfreeze_heavy: f32,
    pub hitfreeze_critical: f32,
    pub invincibility_duration: f32,
    pub damage_number_lifetime: f32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            combo_window: 1.0,
            hitfreeze_light: 0.05,
            hitfreeze_heavy: 0.083,
            hitfreeze_critical: 0.117,
            invincibility_duration: 0.5,
            damage_number_lifetime: 1.0,
        }
    }
}
```

---

### HitfreezeTimer

**描述**: 打击定格计时器（Resource）

**用途**: 控制全局时间缩放（hitfreeze）

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `remaining` | `f32` | 剩余定格时间（秒） |

**示例**:
```rust
#[derive(Resource)]
pub struct HitfreezeTimer {
    pub remaining: f32,
}

impl HitfreezeTimer {
    pub fn trigger(&mut self, duration: f32) {
        // 取最长的定格时间（多次命中时）
        self.remaining = self.remaining.max(duration);
    }
}
```

---

### SkillDatabase

**描述**: 技能数据库（Resource）

**用途**: 从 RON 文件加载技能配置

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `skills` | `HashMap<String, SkillData>` | 技能 ID → 技能数据 |

**示例**:
```rust
#[derive(Resource)]
pub struct SkillDatabase {
    pub skills: HashMap<String, SkillData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SkillData {
    pub id: String,
    pub name: String, // 中文名称（UI 显示）
    pub cooldown: f32,
    pub mp_cost: f32,
    pub damage: f32,
    pub element: Element,
}

// RON 文件示例（assets/data/skills.ron）
/*
{
    "fireball": (
        id: "fireball",
        name: "火球术",
        cooldown: 5.0,
        mp_cost: 20.0,
        damage: 30.0,
        element: Fire,
    ),
}
*/
```

---

### EnemyDatabase

**描述**: 敌人数据库（Resource）

**用途**: 从 RON 文件加载敌人配置

**字段**:

| 字段名 | 类型 | 描述 |
|--------|------|------|
| `enemies` | `HashMap<String, EnemyData>` | 敌人 ID → 敌人数据 |

**示例**:
```rust
#[derive(Resource)]
pub struct EnemyDatabase {
    pub enemies: HashMap<String, EnemyData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyData {
    pub id: String,
    pub name: String, // 中文名称
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub move_speed: f32,
}

// RON 文件示例（assets/data/enemies.ron）
/*
{
    "slime": (
        id: "slime",
        name: "史莱姆",
        health: 30.0,
        attack: 5.0,
        defense: 0.0,
        move_speed: 50.0,
    ),
}
*/
```

---

## Entity Relationships（实体关系图）

```
Player (Entity)
├── Transform (Bevy 内置)
├── Stats (Component)
├── Health (Component)
├── HurtBox (Component)
├── Invincibility (Component, 可选)
├── Combo (Component)
└── Skill (Component, 多个技能)

Enemy (Entity)
├── Transform (Bevy 内置)
├── Stats (Component)
├── Health (Component)
├── HurtBox (Component)
└── EnemyAI (Component, M3 实现)

HitBox (Entity, 临时)
├── Transform (Bevy 内置)
├── HitBox (Component)
└── Lifetime (自定义 Component, 自动销毁)

Particle (Entity, 临时)
├── Transform (Bevy 内置)
├── Particle (Component)
├── Sprite (Bevy 内置)
└── Lifetime (自定义 Component, 自动销毁)

Fireball (Entity, 技能弹道)
├── Transform (Bevy 内置)
├── RigidBody (bevy_rapier2d)
├── Velocity (bevy_rapier2d)
├── Collider (bevy_rapier2d)
├── Sensor (bevy_rapier2d, 仅触发碰撞事件)
├── Fireball (Marker Component)
└── Lifetime (自定义 Component)
```

---

## Data Flow（数据流）

### 伤害计算流程

```
1. 玩家按攻击键
   ↓
2. input_system 检测输入 → 生成 AttackCommand 事件
   ↓
3. attack_system 处理事件 → 播放攻击动画 + 生成 HitBox 实体
   ↓
4. collision_detection_system 检测 HitBox vs HurtBox
   ↓
5. apply_damage_system 调用领域层 calculate_damage() → 更新 Health
   ↓
6. feedback_system 触发打击感反馈（hitfreeze, 震动, 粒子, 音效, 伤害数字）
   ↓
7. death_system 检测 Health <= 0 → 播放死亡动画 + despawn
```

### 连击流程

```
1. 玩家按攻击键
   ↓
2. combo_system 检查 Combo 组件 → advance() 状态转换
   ↓
3. 根据 ComboState 选择攻击动画（FirstHit, SecondHit, ThirdHit）
   ↓
4. 生成对应 HitBox（伤害值不同：10, 10, 20）
   ↓
5. 连击窗口计时器更新（window_remaining -= delta）
   ↓
6. 超时 → reset() 重置连击状态
```

### 技能施放流程

```
1. 玩家按技能键
   ↓
2. skill_input_system 检查冷却 + MP → 生成 SkillActivated 事件
   ↓
3. skill_system 处理事件 → 扣除 MP + 启动冷却 + 播放施法动画
   ↓
4. projectile_system 生成火球实体（RigidBody + Velocity + Sensor）
   ↓
5. 火球飞行（bevy_rapier2d 物理模拟）
   ↓
6. 碰撞事件触发 → 火球爆炸动画 + 伤害计算 + despawn
```

---

## Validation Rules（数据验证规则）

| 实体/组件 | 字段 | 验证规则 |
|-----------|------|---------|
| **Stats** | `crit_rate` | 0.0..=1.0（clamp） |
| **Stats** | `crit_multiplier` | 1.0..=10.0（clamp） |
| **Health** | `current` | 0.0..=max（clamp） |
| **Health** | `max` | >0.0（panic if <=0） |
| **DamageResult** | `final_damage` | 1.0..=9999.0（clamp） |
| **HitBox** | `lifetime_frames` | 1..=60（clamp） |
| **Combo** | `window_remaining` | >=0.0（clamp） |
| **Skill** | `remaining_cooldown` | >=0.0（clamp） |
| **Invincibility** | `remaining` | >=0.0（clamp） |

---

## Next Steps

1. ✅ **Data Model 完成**: 所有实体和组件已定义
2. ⏳ **Contracts 创建**: 定义事件契约（见 contracts/ 目录）
3. ⏳ **Quickstart 创建**: 开发者快速开始指南（见 quickstart.md）
4. ⏳ **Task Breakdown**: 运行 `/speckit.tasks` 生成详细任务列表

---

**Status**: ✅ Data Model Complete  
**Total Entities**: 9 (DamageResult, Stats, Element, ComboState, Rect, HitBox, HurtBox, Health, Combo, Skill, Invincibility, Particle)  
**Total Resources**: 5 (CombatConfig, HitfreezeTimer, SkillDatabase, EnemyDatabase)


