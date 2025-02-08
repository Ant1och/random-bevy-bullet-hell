use std::f32::consts::PI;
use std::ptr::replace;
use std::time::Duration;

use crate::bullet::Bullet;
use crate::bullet::BulletBundle;
use crate::colliders::SensorBundle;
use crate::physics::shared::MovementType;
use crate::physics::Acceleration;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(PartialEq, Debug, Default, Component)]
pub struct CirclePattern;

#[derive(PartialEq, Debug, Default, Component, Reflect)]
pub struct CirclePatternConstruction {
    pub bullets_amount: u32,
    pub timer: Timer,
    pub finished: bool,
}

#[derive(PartialEq, Debug, Default, Component)]
pub struct CirclePatternParams {
    pub movement_type: MovementType,
    pub radius: f64,
    pub bullets_max_amount: u32,
    pub construction_frequency: Duration,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct CirclePatternBundle {
    pub entity: CirclePattern,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[with(Acceleration::from_field)]
    pub accel: Acceleration,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[with(CirclePatternParams::from_field)]
    pub params: CirclePatternParams,
    pub construction: CirclePatternConstruction,
    // #[worldly]
    // pub worldly: Worldly,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub disabled: RigidBodyDisabled,
}

impl CirclePatternParams {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        CirclePatternParams {
            movement_type: MovementType::Circle {
                speed: *entity_instance
                    .get_float_field("rotation_speed")
                    .expect("CirclePattern should have rotation_speed defined"),
                accel: *entity_instance
                    .get_float_field("bullets_acceleration_scale")
                    .expect("CirclePattern should have bullets_acceleration_scale defined"),
            },
            radius: *entity_instance
                .get_float_field("radius")
                .expect("CirclePattern should have radius defined") as f64,
            bullets_max_amount: *entity_instance
                .get_int_field("bullets_max_amount")
                .expect("CirclePattern should have bullets_max_amount defined")
                as u32,
            construction_frequency: Duration::from_secs_f64(
                *entity_instance
                    .get_float_field("construction_speed")
                    .expect("CirclePattern should have construction_speed defined")
                    as f64,
            ),
            ..default()
        }
    }
}

impl CirclePatternBundle {
    pub fn from_params(params: CirclePatternParams, velocity: Vec2, accel: Vec2) -> Self {
        CirclePatternBundle {
            params,
            // velocity: Velocity::linear(velocity),
            accel: Acceleration(accel),
            ..default()
        }
    }
}

fn circle_construction_timer(
    mut patterns: Query<
        (&CirclePatternParams, &mut CirclePatternConstruction),
        Added<CirclePattern>,
    >,
) {
    for (params, mut construction) in &mut patterns {
        let duration = params.construction_frequency;
        construction.timer.set_duration(duration);
        construction.timer.set_mode(TimerMode::Repeating);
    }
}

fn circle_construction(
    mut cmd: Commands,
    mut patterns: Query<
        (&CirclePatternParams, &mut CirclePatternConstruction, Entity),
        With<CirclePattern>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta();

    for (params, mut construction, circle) in &mut patterns {
        if !construction.timer.tick(delta).just_finished() {
            continue;
        }

        if construction.bullets_amount >= params.bullets_max_amount {
            construction.finished = true;
        }

        if construction.finished {
            continue;
        }

        let translation = next_bullet_position(
            construction.bullets_amount,
            params.bullets_max_amount,
            params.radius,
        )
        .extend(0.);

        let bullet = cmd
            .spawn(BulletBundle {
                transform: Transform::from_translation(translation),
                ..Default::default()
            })
            .id();
        construction.bullets_amount += 1;

        cmd.entity(bullet).insert(RigidBodyDisabled);
        cmd.entity(circle).add_child(bullet);
    }
}

fn circle_finish_construction(
    mut patterns: Query<(&CirclePatternConstruction, Entity), With<CirclePattern>>,
    children: Query<&Children>,
    mut cmd: Commands,
) {
    for (construction, circle) in &mut patterns {
        if !construction.finished {
            continue;
        }
        for bullet in children.children(circle) {
            cmd.entity(*bullet).remove::<RigidBodyDisabled>();
        }
        cmd.entity(circle).remove::<RigidBodyDisabled>();
    }
}

fn circle_setup_bullets(
    mut patterns: Query<
        (&CirclePatternParams, &CirclePatternConstruction, Entity),
        With<CirclePattern>,
    >,
    children: Query<&Children>,
    mut bullet_query: Query<(&mut Velocity, &mut Acceleration, &Transform), With<Bullet>>,
) {
    for (params, construction, circle) in &mut patterns {
        if construction.finished {
            continue;
        }

        for bullet in children.children(circle) {
            let Ok((mut velocity, mut acceleration, transform)) = bullet_query.get_mut(*bullet)
            else {
                continue;
            };

            let position = transform.translation.truncate();
            velocity.linvel = params.movement_type.start_velocity(position);
            acceleration.0 = Vec2::ZERO;
        }
    }
}

fn circle_bullet_acceleration(
    mut patterns: Query<(&CirclePatternParams, Entity), With<CirclePattern>>,
    children: Query<&Children>,
    mut bullet_query: Query<(&mut Acceleration, &Transform), With<Bullet>>,
) {
    for (params, circle) in &mut patterns {
        for bullet in children.children(circle) {
            let Ok((mut acceleration, transform)) = bullet_query.get_mut(*bullet) else {
                continue;
            };

            let relative_position = transform.translation.truncate();
            acceleration.0 = params.movement_type.acceleration(relative_position);
        }
    }
}

fn next_bullet_position(current_amount: u32, max_amount: u32, radius: f64) -> Vec2 {
    Vec2::from_angle((current_amount as f32 * 2. * PI) / max_amount as f32) * radius as f32
}

pub struct CirclePatternPlugin;

impl Plugin for CirclePatternPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CirclePatternBundle>("CirclePattern")
            .add_systems(Update, circle_construction_timer)
            .add_systems(Update, circle_construction)
            .add_systems(Update, (circle_finish_construction, circle_setup_bullets))
            .add_systems(Update, circle_bullet_acceleration);
    }
}
