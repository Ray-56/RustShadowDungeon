use crate::domain::combat::{collision::Rect, ComboState, Element};
/// Combat-related ECS components
/// 战斗相关 ECS 组件
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

/// Stats component - combat statistics for entities
///
/// This is a Component wrapper around domain::combat::damage::Stats
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    /// Attack power
    pub attack: f32,
    /// Defense value
    pub defense: f32,
    /// Critical hit rate (0.0 to 1.0)
    pub crit_rate: f32,
    /// Critical hit multiplier
    pub crit_multiplier: f32,
    /// Element resistances
    pub element_resistances: HashMap<Element, f32>,
}

impl Stats {
    /// Create new Stats
    pub fn new(attack: f32, defense: f32) -> Self {
        Self {
            attack,
            defense,
            crit_rate: 0.1,
            crit_multiplier: 2.0,
            element_resistances: HashMap::new(),
        }
    }

    /// Convert to domain Stats
    pub fn to_domain(&self) -> crate::domain::combat::damage::Stats {
        crate::domain::combat::damage::Stats {
            attack: self.attack,
            defense: self.defense,
            crit_rate: self.crit_rate,
            crit_multiplier: self.crit_multiplier,
            element_resistances: self.element_resistances.clone(),
        }
    }

    /// Create from domain Stats
    pub fn from_domain(stats: &crate::domain::combat::damage::Stats) -> Self {
        Self {
            attack: stats.attack,
            defense: stats.defense,
            crit_rate: stats.crit_rate,
            crit_multiplier: stats.crit_multiplier,
            element_resistances: stats.element_resistances.clone(),
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new(10.0, 0.0)
    }
}

/// HitBox component - attack hitbox that can damage enemies
///
/// HitBox 组件 - 可伤害敌人的攻击判定框
///
/// Spawned by attack systems. Lasts for `lifetime_frames` frames.
/// Collision with HurtBox triggers damage calculation.
#[derive(Component, Debug, Clone)]
pub struct HitBox {
    /// Collision rectangle (relative to entity transform)
    pub rect: Rect,
    /// Base damage value
    pub damage: f32,
    /// Element type of attack
    pub element: Element,
    /// How many frames this HitBox persists
    pub lifetime_frames: u32,
    /// Whether this HitBox can pierce through enemies (hit multiple)
    pub can_pierce: bool,
    /// Set of entities already hit (for deduplication)
    pub hit_entities: HashSet<Entity>,
}

impl HitBox {
    /// Create a new HitBox with default values
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

    /// Create a new HitBox with custom element
    pub fn with_element(mut self, element: Element) -> Self {
        self.element = element;
        self
    }

    /// Create a new HitBox with custom lifetime
    pub fn with_lifetime(mut self, lifetime_frames: u32) -> Self {
        self.lifetime_frames = lifetime_frames;
        self
    }

    /// Check if this HitBox has already hit the target entity
    pub fn has_hit(&self, target: Entity) -> bool {
        self.hit_entities.contains(&target)
    }

    /// Mark an entity as hit
    pub fn mark_hit(&mut self, target: Entity) {
        self.hit_entities.insert(target);
    }
}

/// HurtBox component - entity can be damaged when hit
///
/// HurtBox 组件 - 可被攻击命中的实体
///
/// Attached to entities that can take damage (player, enemies).
#[derive(Component, Debug, Clone)]
pub struct HurtBox {
    /// Collision rectangle (relative to entity transform)
    pub rect: Rect,
    /// Whether entity is currently invincible (ignores damage)
    pub is_invincible: bool,
}

impl HurtBox {
    /// Create a new HurtBox
    pub fn new(rect: Rect) -> Self {
        Self { rect, is_invincible: false }
    }
}

/// Combo component - tracks combo state for player
///
/// Combo 组件 - 追踪玩家连击状态
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Combo {
    /// Current combo state
    pub state: ComboState,
    /// Remaining time in combo window (seconds)
    pub window_remaining: f32,
    /// Total hit count in current combo
    pub hit_count: u32,
}

impl Combo {
    /// Create new Combo in Idle state
    pub fn new() -> Self {
        Self { state: ComboState::Idle, window_remaining: 0.0, hit_count: 0 }
    }

    /// Advance combo to next state
    pub fn advance(&mut self, window_duration: f32) {
        use crate::domain::combat::combo::advance_combo;

        let (new_state, new_window) = advance_combo(self.state, window_duration);
        self.state = new_state;
        self.window_remaining = new_window;

        if new_state != ComboState::Idle {
            self.hit_count += 1;
        } else {
            self.hit_count = 0;
        }
    }

    /// Reset combo to Idle
    pub fn reset(&mut self) {
        use crate::domain::combat::combo::reset_combo;

        let (new_state, new_window, new_count) = reset_combo();
        self.state = new_state;
        self.window_remaining = new_window;
        self.hit_count = new_count;
    }

    /// Check if combo window has expired
    pub fn is_expired(&self) -> bool {
        use crate::domain::combat::combo::is_combo_expired;
        is_combo_expired(self.window_remaining)
    }
}

impl Default for Combo {
    fn default() -> Self {
        Self::new()
    }
}

/// Skill component - represents a skill that can be activated
///
/// Skill 组件 - 可激活的技能
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Skill ID (e.g., "fireball")
    pub skill_id: String,
    /// Cooldown duration (seconds)
    pub cooldown: f32,
    /// Remaining cooldown (seconds)
    pub remaining_cooldown: f32,
    /// MP cost to activate
    pub mp_cost: f32,
    /// Base damage
    pub damage: f32,
    /// Element type
    pub element: Element,
}

impl Skill {
    /// Create a new skill
    pub fn new(
        skill_id: String,
        cooldown: f32,
        mp_cost: f32,
        damage: f32,
        element: Element,
    ) -> Self {
        Self { skill_id, cooldown, remaining_cooldown: 0.0, mp_cost, damage, element }
    }

    /// Check if skill is ready (cooldown finished)
    pub fn is_ready(&self) -> bool {
        self.remaining_cooldown <= 0.0
    }

    /// Activate skill (start cooldown)
    pub fn activate(&mut self) {
        self.remaining_cooldown = self.cooldown;
    }

    /// Update cooldown timer
    pub fn update(&mut self, delta: f32) {
        if self.remaining_cooldown > 0.0 {
            self.remaining_cooldown -= delta;
            if self.remaining_cooldown < 0.0 {
                self.remaining_cooldown = 0.0;
            }
        }
    }

    /// Get cooldown progress (0.0 = ready, 1.0 = just activated)
    pub fn cooldown_progress(&self) -> f32 {
        if self.cooldown <= 0.0 {
            return 0.0;
        }
        (self.remaining_cooldown / self.cooldown).clamp(0.0, 1.0)
    }
}

/// Invincibility component - entity is immune to damage
///
/// Invincibility 组件 - 无敌帧
///
/// Added to entities after taking damage. Prevents further damage.
#[derive(Component, Debug, Clone)]
pub struct Invincibility {
    /// Remaining invincibility duration (seconds)
    pub remaining: f32,
    /// Flash timer for visual feedback (toggles visibility)
    pub flash_timer: f32,
}

impl Invincibility {
    /// Create new Invincibility with duration
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration, flash_timer: 0.0 }
    }

    /// Check if still invincible
    pub fn is_active(&self) -> bool {
        self.remaining > 0.0
    }

    /// Update timers
    pub fn update(&mut self, delta: f32) {
        if self.remaining > 0.0 {
            self.remaining -= delta;
            self.flash_timer += delta;
        }
    }

    /// Check if entity should be visible (for flashing effect)
    /// Flashes every 0.1 seconds
    pub fn should_be_visible(&self) -> bool {
        (self.flash_timer * 10.0) as i32 % 2 == 0
    }
}

/// Particle component - visual effect particle
///
/// Particle 组件 - 视觉效果粒子
#[derive(Component, Debug, Clone)]
pub struct Particle {
    /// Remaining lifetime (seconds)
    pub lifetime: f32,
    /// Velocity vector (pixels per second)
    pub velocity: Vec2,
    /// Particle color
    pub color: Color,
}

impl Particle {
    /// Create a new particle
    pub fn new(lifetime: f32, velocity: Vec2, color: Color) -> Self {
        Self { lifetime, velocity, color }
    }

    /// Update particle (returns true if still alive)
    pub fn update(&mut self, delta: f32) -> bool {
        self.lifetime -= delta;
        self.lifetime > 0.0
    }

    /// Get alpha value based on remaining lifetime (fades out)
    pub fn get_alpha(&self, max_lifetime: f32) -> f32 {
        (self.lifetime / max_lifetime).clamp(0.0, 1.0)
    }
}

/// Lifetime component - automatically despawn entity after duration
///
/// Lifetime 组件 - 自动销毁实体
#[derive(Component, Debug, Clone)]
pub struct Lifetime {
    /// Remaining lifetime (seconds)
    pub remaining: f32,
}

impl Lifetime {
    /// Create a new Lifetime
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }

    /// Update lifetime (returns true if still alive)
    pub fn update(&mut self, delta: f32) -> bool {
        self.remaining -= delta;
        self.remaining > 0.0
    }
}

/// Fireball projectile component
///
/// 火球弹道组件
#[derive(Component, Debug, Clone)]
pub struct Fireball {
    /// Velocity vector (pixels per second)
    pub velocity: Vec2,
}

/// Attack animation state
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackAnimation {
    Idle,
    FirstAttack,
    SecondAttack,
    ThirdAttack,
}

impl Default for AttackAnimation {
    fn default() -> Self {
        AttackAnimation::Idle
    }
}
