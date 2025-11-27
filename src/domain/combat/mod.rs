/// Domain layer for combat system
///
/// This module contains pure business logic with zero Bevy dependencies.
/// All functions are deterministic and easily testable.
///
/// 领域层战斗系统
/// 包含纯业务逻辑，零 Bevy 依赖，所有函数确定性且易于测试
pub mod collision;
pub mod combo;
pub mod damage;
pub mod element;

// Re-export commonly used types
pub use collision::Rect;
pub use combo::ComboState;
pub use damage::{calculate_damage, DamageResult, Stats};
pub use element::Element;
