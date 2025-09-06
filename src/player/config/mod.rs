use crate::physics::looking_direction::LookDir;

pub mod collision;
pub mod physics;
pub mod stats;

pub const PLAYER_DEFAULT_LOOKING_DIRECTION: LookDir = LookDir::Left;
