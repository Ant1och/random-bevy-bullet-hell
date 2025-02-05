use std::f32::consts::PI;
use std::time::Duration;

use crate::bullet::Bullet;
use crate::bullet::BulletBundle;
use crate::colliders::SensorBundle;
use crate::physics::shared::Acceleration;
use crate::physics::shared::AccelerationScale;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(PartialEq, Debug, Default, Component)]
pub struct CirclePattern;

#[derive(PartialEq, Debug, Default, Component, Reflect)]
pub struct CirclePatternConstruction {
    pub radius: f64,
    pub bullets_max_amount: u32,
    pub bullets_acceleration_scale: AccelerationScale,
    pub speed: Duration,
    pub rotation_speed: f32,
    pub bullets_amount: u32,
    pub timer: Timer,
    pub finished: bool,
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct CirclePatternBundle {
    pub entity: CirclePattern,
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    #[with(Acceleration::from_field)]
    pub acceleration: Acceleration,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    #[with(CirclePatternConstruction::from_field)]
    pub parameters: CirclePatternConstruction,
    #[worldly]
    pub worldly: Worldly,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
    pub disabled: RigidBodyDisabled,
}

impl CirclePatternConstruction {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        CirclePatternConstruction {
            radius: *entity_instance
                .get_float_field("radius")
                .expect("CirclePattern should have radius defined") as f64,
            bullets_max_amount: *entity_instance
                .get_int_field("bullets_max_amount")
                .expect("CirclePattern should have bullets_max_amount defined")
                as u32,
            rotation_speed: *entity_instance
                .get_float_field("rotation_speed")
                .expect("CirclePattern should have rotation_speed defined"),
            bullets_acceleration_scale: AccelerationScale(
                *entity_instance
                    .get_float_field("bullets_acceleration_scale")
                    .expect("CirclePattern should have bullets_acceleration_scale defined")
                    as f64,
            ),
            speed: Duration::from_secs_f64(
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
    pub fn from_params(params: CirclePatternConstruction) -> Self {
        CirclePatternBundle {
            parameters: params,
            ..default()
        }
    }
}

fn circle_acceleration(
    mut patterns: Query<
        (&mut Velocity, &Acceleration, &CirclePatternConstruction),
        With<CirclePattern>,
    >,
) {
    for (mut velocity, acceleration, construction) in &mut patterns {
        if construction.finished {
            velocity.linvel += acceleration.0;
        }
    }
}

fn circle_construction_timer(
    mut patterns: Query<&mut CirclePatternConstruction, Added<CirclePattern>>,
) {
    for mut construction in &mut patterns {
        let speed = construction.speed;
        construction.timer.set_duration(speed);

        construction.timer.set_mode(TimerMode::Repeating);
    }
}

fn circle_construction(
    mut cmd: Commands,
    mut patterns: Query<(&mut CirclePatternConstruction, Entity), With<CirclePattern>>,
    time: Res<Time>,
) {
    let delta = time.delta();

    for (mut construction, circle) in &mut patterns {
        if !construction.timer.tick(delta).just_finished() {
            continue;
        }

        if construction.bullets_amount >= construction.bullets_max_amount {
            construction.finished = true;
        }

        if construction.finished {
            continue;
        }

        let translation = next_bullet_position(
            construction.bullets_amount,
            construction.bullets_max_amount,
            construction.radius,
        )
        .extend(0.);

        let acceleration_scale = construction.bullets_acceleration_scale.clone();

        let bullet = cmd
            .spawn(BulletBundle {
                transform: Transform::from_translation(translation),
                acceleration_scale,
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
    mut patterns: Query<(&CirclePatternConstruction, Entity), With<CirclePattern>>,
    children: Query<&Children>,
    mut bullet_query: Query<(&mut Velocity, &mut Acceleration, &Transform), With<Bullet>>,
) {
    for (construction, pattern) in &mut patterns {
        if construction.finished {
            continue;
        }

        for bullet in children.children(pattern) {
            let Ok((mut velocity, mut acceleration, Transform { translation, .. })) =
                bullet_query.get_mut(*bullet)
            else {
                continue;
            };

            velocity.linvel = Vec2::from_angle(PI / 2.).rotate(translation.truncate());
            velocity.linvel = velocity.linvel / velocity.linvel.length();
            velocity.linvel *= construction.rotation_speed;
            acceleration.0 = Vec2::ZERO;
        }
    }
}

fn next_bullet_position(current_amount: u32, max_amount: u32, radius: f64) -> Vec2 {
    Vec2::from_angle((current_amount as f32 * 2. * PI) / max_amount as f32) * radius as f32
}

fn circle_bullet_acceleration(
    mut patterns: Query<(&CirclePatternConstruction, Entity), With<CirclePattern>>,
    children: Query<&Children>,
    mut bullet_query: Query<(&mut Acceleration, &Transform), With<Bullet>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs_f64();

    for (construction, circle) in &mut patterns {
        for bullet in children.children(circle) {
            let Ok((mut acceleration, transform)) = bullet_query.get_mut(*bullet) else {
                continue;
            };

            let relative_position = transform.translation.truncate();
            let direction = -relative_position / relative_position.length();
            let accel_scale = construction.bullets_acceleration_scale.0;

            // Accelerate towards center (parent)
            acceleration.0 = direction * (accel_scale * delta) as f32;
        }
    }
}

pub struct CirclePatternPlugin;

impl Plugin for CirclePatternPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CirclePatternBundle>("CirclePattern")
            .add_systems(Update, circle_construction_timer)
            .add_systems(Update, circle_construction)
            .add_systems(Update, (circle_finish_construction, circle_setup_bullets))
            .add_systems(Update, circle_acceleration)
            .add_systems(Update, circle_bullet_acceleration);
    }
}
