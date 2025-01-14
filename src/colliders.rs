use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::prelude::*;

const PLAYER_WIDTH: f32 = 5.;
const PLAYER_HEIGHT: f32 = 16.;

#[derive(Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub restitution: Restitution,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(PLAYER_WIDTH, PLAYER_HEIGHT),
                rigid_body: RigidBody::Dynamic,
                restitution: Restitution {
                    coefficient: 0.,
                    combine_rule: CoefficientCombineRule::Max,
                },
                friction: Friction {
                    coefficient: 0.,
                    combine_rule: CoefficientCombineRule::Min,
                },
                gravity_scale: GravityScale(0.),
                rotation_constraints,
                density: ColliderMassProperties::Mass(0.),
                ..Default::default()
            },

            _ => ColliderBundle::default(),
        }
    }
}
