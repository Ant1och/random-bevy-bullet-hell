use crate::bullet::BulletBundle;
use crate::colliders::SensorBundle;
use crate::physics::movement::MovementType;
use crate::physics::Acceleration;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

pub mod construction;
use construction::ConstructionType;

#[derive(PartialEq, Debug, Default, Component)]
pub struct Pattern;

#[derive(PartialEq, Debug, Default, Component, Reflect)]
pub struct PatternConstruction {
    pub progress: u64,
    pub timer: Timer,
    pub finished: bool,
}

#[derive(PartialEq, Debug, Default, Component, Clone)]
pub struct PatternParams {
    pub scale: f64,
    pub bullet_amount: u64,
    pub bullet_movement: MovementType,
    pub construction_frequency: Duration,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct PatternBundle {
    pub entity: Pattern,
    pub name: Name,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[with(Acceleration::from_field)]
    pub accel: Acceleration,
    pub velocity: Velocity,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[with(PatternParams::from_field)]
    pub params: PatternParams,
    pub movement_type: MovementType,
    pub construction_type: ConstructionType,
    pub construction: PatternConstruction,
    // #[worldly]
    // pub worldly: Worldly,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub disabled: RigidBodyDisabled,
}

impl PatternParams {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        PatternParams {
            bullet_movement: MovementType::Circle {
                speed: *entity_instance
                    .get_float_field("rotation_speed")
                    .expect("CirclePattern should have rotation_speed defined"),
                accel: *entity_instance
                    .get_float_field("bullets_acceleration_scale")
                    .expect("CirclePattern should have bullets_acceleration_scale defined"),
            },
            scale: *entity_instance
                .get_float_field("radius")
                .expect("CirclePattern should have radius defined") as f64,
            bullet_amount: *entity_instance
                .get_int_field("bullets_max_amount")
                .expect("CirclePattern should have bullets_max_amount defined")
                as u64,
            construction_frequency: Duration::from_secs_f64(
                *entity_instance
                    .get_float_field("construction_speed")
                    .expect("CirclePattern should have construction_speed defined")
                    as f64,
            ),
        }
    }
}

impl PatternBundle {
    pub fn new(
        params: PatternParams,
        construction: ConstructionType,
        movement: MovementType,
    ) -> Self {
        PatternBundle {
            name: Name::from("Circle Pattern"),
            params,
            construction_type: construction,
            movement_type: movement,
            ..default()
        }
    }
}

fn construction_timer(
    mut patterns: Query<(&PatternParams, &mut PatternConstruction), Added<Pattern>>,
) {
    for (params, mut construction) in &mut patterns {
        let duration = params.construction_frequency;
        construction.timer.set_duration(duration);
        construction.timer.set_mode(TimerMode::Repeating);
    }
}

fn construction(
    mut cmd: Commands,
    mut patterns: Query<
        (
            &PatternParams,
            &ConstructionType,
            &mut PatternConstruction,
            Entity,
        ),
        With<Pattern>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta();

    for (params, construction_type, mut construction, circle) in &mut patterns {
        if !construction.timer.tick(delta).just_finished() {
            continue;
        }

        if construction.progress >= params.bullet_amount {
            construction.finished = true;
        }

        if construction.finished {
            continue;
        }

        let translation = construction_type
            .translation(construction.progress, params.bullet_amount)
            .extend(0.1)
            * params.scale as f32;

        let bullet = cmd
            .spawn(BulletBundle {
                name: Name::from("Bullet"),
                transform: Transform::from_translation(translation),
                movement: params.bullet_movement.clone(),
                ..default()
            })
            .id();
        construction.progress += 1;

        cmd.entity(bullet).insert(RigidBodyDisabled);
        cmd.entity(circle).add_child(bullet);
    }
}

fn finish_construction(
    mut patterns: Query<(&PatternConstruction, Entity), With<Pattern>>,
    children: Query<&Children>,
    mut cmd: Commands,
) {
    for (construction, circle) in &mut patterns {
        if !construction.finished {
            continue;
        }
        for bullet in children.children(circle) {
            let Some(mut bullet) = cmd.get_entity(*bullet) else {
                return;
            };
            bullet.remove::<RigidBodyDisabled>();
        }
        cmd.entity(circle).remove::<RigidBodyDisabled>();
    }
}

// fn setup_bullets(
//     mut patterns: Query<(&MovementType, &PatternConstruction, Entity), With<Pattern>>,
//     children: Query<&Children>,
//     mut bullet_query: Query<(&mut Velocity, &mut Acceleration, &Transform), With<Bullet>>,
// ) {
//     for (movement_type, construction, circle) in &mut patterns {
//         if construction.finished {
//             continue;
//         }

//         for bullet in children.children(circle) {
//             let Ok((mut velocity, mut acceleration, transform)) = bullet_query.get_mut(*bullet)
//             else {
//                 continue;
//             };

//             let position = transform.translation.truncate();
//             velocity.linvel = movement_type.start_velocity(position);
//             acceleration.0 = Vec2::ZERO;
//         }
//     }
// }

// fn bullet_acceleration(
//     mut patterns: Query<(&MovementType, Entity), With<Pattern>>,
//     children: Query<&Children>,
//     mut bullet_query: Query<(&mut Acceleration, &Transform), With<Bullet>>,
// ) {
//     for (movement_type, circle) in &mut patterns {
//         for bullet in children.children(circle) {
//             let Ok((mut acceleration, transform)) = bullet_query.get_mut(*bullet) else {
//                 continue;
//             };

//             let relative_position = transform.translation.truncate();
//             acceleration.0 = movement_type.acceleration(relative_position);
//         }
//     }
// }

pub struct PatternPlugin;

impl Plugin for PatternPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PatternBundle>("Pattern")
            .add_systems(Update, construction_timer)
            .add_systems(Update, (construction, finish_construction).chain())
        // .add_systems(Update, bullet_acceleration)
        ;
    }
}
