//! Enemy AI domain logic
//!
//! Pure Rust functions for enemy AI state transitions and calculations.
//! ZERO Bevy dependencies - pure business logic.
//!
//! 敌人 AI 领域逻辑 - 纯 Rust 函数，零 Bevy 依赖

use bevy::math::Vec2;

/// AI 状态枚举
///
/// AI state enum for enemy behavior state machine.
/// 敌人行为状态机的 AI 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIState {
    /// 巡逻状态 - 敌人在无目标时移动
    Patrol,
    /// 追逐状态 - 敌人发现目标并追逐
    Chase,
    /// 攻击状态 - 敌人在攻击范围内执行攻击
    Attack,
    /// 返回状态 - 敌人脱战后返回初始位置
    Return,
}

/// 方向向量类型别名
///
/// Direction vector type alias for movement calculations.
/// 用于移动计算的方向向量类型别名
pub type Direction = Vec2;

/// 位置类型别名
///
/// Position type alias for coordinate calculations.
/// 用于坐标计算的位置类型别名
pub type Position = Vec2;

// ============================================================================
// State Transition Functions (User Story 2 - 发现与追逐)
// ============================================================================

/// 判断是否应该转换到追逐状态
///
/// Determines if enemy should transition to Chase state.
/// 判断敌人是否应该转换到追逐状态
///
/// # Arguments
///
/// * `distance` - 敌人到玩家的距离（像素）
/// * `detection_range` - 检测范围（像素）
/// * `has_line_of_sight` - 是否有视线（Line of Sight）
///
/// # Returns
///
/// `true` if enemy should transition to Chase state
pub fn should_transition_to_chase(
    distance: f32,
    detection_range: f32,
    has_line_of_sight: bool,
) -> bool {
    distance <= detection_range && has_line_of_sight
}

/// 判断是否应该放弃仇恨（脱战）
///
/// Determines if enemy should drop aggro and return to patrol.
/// 判断敌人是否应该放弃仇恨并返回巡逻
///
/// # Arguments
///
/// * `distance` - 敌人到目标的距离（像素）
/// * `aggro_drop_range` - 脱战范围（像素）
/// * `time_since_last_seen` - 自上次看到目标后的时间（秒）
/// * `max_time_without_sight` - 最大无视线时间（秒）
///
/// # Returns
///
/// `true` if enemy should drop aggro
pub fn should_drop_aggro(
    distance: f32,
    aggro_drop_range: f32,
    time_since_last_seen: f32,
    max_time_without_sight: f32,
) -> bool {
    distance > aggro_drop_range || time_since_last_seen > max_time_without_sight
}

/// 检查视线（步进式射线检测）
///
/// Line of sight check using step-based raycast to detect obstacles.
/// 使用步进式射线检测检查视线，检测障碍物
///
/// # Arguments
///
/// * `enemy_pos` - 敌人位置
/// * `target_pos` - 目标位置
/// * `obstacles` - 障碍物矩形列表（每个障碍物用 Rect 表示：x, y, width, height）
///
/// # Returns
///
/// `true` if there is a clear line of sight (no obstacles blocking)
///
/// # Implementation
///
/// Uses step-based raycast: samples points along the ray from enemy to target
/// at regular intervals (8 pixels) and checks if any point intersects with obstacles.
/// 使用步进式射线检测：在从敌人到目标的射线上以固定间隔（8 像素）采样点，
/// 检查是否有任何点与障碍物相交。
pub fn check_line_of_sight(
    enemy_pos: Vec2,
    target_pos: Vec2,
    obstacles: &[(f32, f32, f32, f32)], // (x, y, width, height) for each obstacle
) -> bool {
    let direction = target_pos - enemy_pos;
    let distance = direction.length();

    // If distance is very small, assume clear line of sight
    if distance < 1.0 {
        return true;
    }

    let normalized_dir = direction / distance;
    let step_size = 8.0; // Check every 8 pixels
    let steps = (distance / step_size).ceil() as usize;

    // Sample points along the ray
    for i in 0..=steps {
        let t = if steps > 0 { (i as f32) / (steps as f32) } else { 0.0 };
        let check_pos = enemy_pos + normalized_dir * (distance * t);

        // Check if this point intersects with any obstacle
        for obstacle in obstacles {
            let (obs_x, obs_y, obs_width, obs_height) = *obstacle;

            // Check if point is inside obstacle rectangle
            if check_pos.x >= obs_x
                && check_pos.x <= obs_x + obs_width
                && check_pos.y >= obs_y
                && check_pos.y <= obs_y + obs_height
            {
                return false; // Obstacle blocks line of sight
            }
        }
    }

    true // Clear line of sight
}

/// 计算追逐方向
///
/// Calculates the direction vector from enemy to target.
/// 计算从敌人到目标的方向向量
///
/// # Arguments
///
/// * `enemy_pos` - 敌人位置
/// * `target_pos` - 目标位置
///
/// # Returns
///
/// Normalized direction vector
pub fn calculate_chase_direction(enemy_pos: Vec2, target_pos: Vec2) -> Direction {
    let direction = target_pos - enemy_pos;
    let length = direction.length();
    if length > 0.0 {
        direction / length
    } else {
        Vec2::ZERO
    }
}

// ============================================================================
// State Transition Functions (User Story 3 - 攻击行为)
// ============================================================================

/// 判断是否应该转换到攻击状态
///
/// Determines if enemy should transition to Attack state.
/// 判断敌人是否应该转换到攻击状态
///
/// # Arguments
///
/// * `distance` - 敌人到目标的距离（像素）
/// * `attack_range` - 攻击范围（像素）
/// * `cooldown_ready` - 攻击冷却是否完成
///
/// # Returns
///
/// `true` if enemy should transition to Attack state
pub fn should_transition_to_attack(distance: f32, attack_range: f32, cooldown_ready: bool) -> bool {
    distance <= attack_range && cooldown_ready
}

// ============================================================================
// Patrol Functions (User Story 1 - 敌人巡逻)
// ============================================================================

/// 计算巡逻目标位置（随机点模式）
///
/// Calculates a random patrol target position within patrol radius.
/// 在巡逻半径内计算随机巡逻目标位置
///
/// # Arguments
///
/// * `current_pos` - 当前位置（未使用，保留用于未来扩展）
/// * `spawn_pos` - 生成位置（巡逻中心）
/// * `patrol_radius` - 巡逻半径（像素）
///
/// # Returns
///
/// Random position within patrol radius
pub fn calculate_patrol_target(
    _current_pos: Vec2,
    spawn_pos: Vec2,
    patrol_radius: f32,
) -> Position {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Random angle and distance
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let distance = rng.gen_range(0.0..patrol_radius);

    spawn_pos + Vec2::new(angle.cos() * distance, angle.sin() * distance)
}
