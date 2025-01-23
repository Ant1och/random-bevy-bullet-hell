use crate::player::config::collision::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::prelude::*;

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
    pub collision_types: ActiveCollisionTypes,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub active_events: ActiveEvents,
    pub sensor: Sensor,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match (
            entity_instance.identifier.as_ref(),
            entity_instance.width as f32,
            entity_instance.height as f32,
        ) {
            ("Bullet", width, height) => SensorBundle {
                collider: Collider::cuboid(width / 2., height / 2.),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
                ..Default::default()
            },
            _ => SensorBundle::default(),
        }
    }
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
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
