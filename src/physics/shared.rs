use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::shared::ldtk_to_bevy_vec2;

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

impl Acceleration {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        let acceleration = match entity_instance.get_point_field("acceleration") {
            Ok(vec) => ldtk_to_bevy_vec2(vec - entity_instance.grid),
            Err(_) => Vec2::ZERO,
        };

        Acceleration(acceleration)
    }
}

#[derive(Component, Default, Reflect, Debug, PartialEq, Clone)]
pub struct AccelerationScale(pub f64);
