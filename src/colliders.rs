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
    pub character_controller: KinematicCharacterController,
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
                character_controller: KinematicCharacterController {
                    // The character offset is set to 0.01.
                    offset: CharacterLength::Absolute(0.0),
                    ..default()
                },
                gravity_scale: GravityScale(0.),
                rotation_constraints,
                density: ColliderMassProperties::Mass(0.),
                // collision_types: (ActiveCollisionTypes::default()
                //     | ActiveCollisionTypes::KINEMATIC_STATIC
                //     | ActiveCollisionTypes::KINEMATIC_KINEMATIC),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}
