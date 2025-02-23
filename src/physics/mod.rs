use crate::{config::ldtk::LDTK_VECTOR_SCALE, shared::ldtk_to_bevy_vec2};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod looking_direction;
pub mod movement;
use movement::MovementTypePlugin;

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);

impl Acceleration {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        let acceleration = match entity_instance.get_point_field("acceleration") {
            Ok(vec) => ldtk_to_bevy_vec2(vec - entity_instance.grid),
            Err(_) => Vec2::ZERO,
        };

        Acceleration(acceleration * LDTK_VECTOR_SCALE)
    }
}

fn physics_acceleration(
    mut query: Query<(&mut Velocity, &Acceleration), Without<RigidBodyDisabled>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (mut velocity, accel) in &mut query {
        velocity.linvel += accel.0 * delta;
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementTypePlugin)
            .add_systems(Update, physics_acceleration);
    }
}
