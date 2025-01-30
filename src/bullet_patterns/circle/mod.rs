use std::f32::consts::PI;
use std::time::Duration;

use crate::bullet::Bullet;
use crate::bullet::{BulletBundle, BulletPivot};
use crate::colliders::SensorBundle;
use crate::physics::shared::Acceleration;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

const DEFAULT_CONSTRUCTION_SPEED: f64 = 110.1;

#[derive(PartialEq, Debug, Default, Component)]
#[require(CirclePatternConstruction(|| CirclePatternConstruction {bullets_max_amount: 12, speed: Duration::from_secs_f64(DEFAULT_CONSTRUCTION_SPEED), radius: 10., ..Default::default()}))]
pub struct CirclePattern;

#[derive(PartialEq, Debug, Default, Component)]
pub struct CirclePatternConstruction {
    radius: f64,
    bullets_max_amount: u32,
    bullets_amount: u32,
    speed: Duration,
    timer: Timer,
    finished: bool,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct CirclePatternBundle {
    pub sprite: Sprite,
    pub animation: AseSpriteAnimation,
    pub acceleration: Acceleration,
    #[from_entity_instance]
    pub sensor_bundle: SensorBundle,
    pub pattern: CirclePattern,
    pub parameters: CirclePatternConstruction,
    #[worldly]
    pub worldly: Worldly,
    pub transform: Transform,
    #[from_entity_instance]
    pub entity_instance: EntityInstance,
}

fn circle_acceleration(mut patterns: Query<(&mut Velocity, &Acceleration), With<CirclePattern>>) {
    for (mut velocity, acceleration) in &mut patterns {
        velocity.linvel += acceleration.0;
    }
}

fn circle_construction_timer(
    mut patterns: Query<(Entity, &mut CirclePatternConstruction), Added<CirclePattern>>,
) {
    for (circle, mut construction) in &mut patterns {
        // let speed = construction.speed;
        // construction.timer.set_duration(speed);
        // let bullet = cmd.spawn(BulletBundle::default()).id();
        // cmd.entity(circle).add_child(bullet);

        construction
            .timer
            .set_duration(Duration::from_secs_f64(0.07));
        construction.timer.set_mode(TimerMode::Repeating);
    }
}

fn circle_construction(
    mut cmd: Commands,
    mut patterns: Query<(&Transform, &mut CirclePatternConstruction, Entity), With<CirclePattern>>,
    time: Res<Time>,
) {
    let delta = time.delta();

    for (transform, mut construction, circle) in &mut patterns {
        if !construction.timer.tick(delta).just_finished() {
            return;
        }

        if construction.bullets_amount >= 12 {
            construction.finished = true;
        }

        if construction.finished {
            return;
        }

        let translation = next_bullet_position(construction.bullets_amount, 12, 100.).extend(0.);

        let bullet = cmd
            .spawn(BulletBundle {
                pivot: BulletPivot(transform.clone()),
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
    for (construction, pattern) in &mut patterns {
        if !construction.finished {
            return;
        }
        for bullet in children.children(pattern) {
            cmd.entity(*bullet).remove::<RigidBodyDisabled>();
        }
    }
}

fn circle_setup_bullets(
    mut patterns: Query<(&CirclePatternConstruction, Entity), With<CirclePattern>>,
    children: Query<&Children>,
    mut bullet_query: Query<
        (&mut Velocity, &mut Acceleration, &BulletPivot, &Transform),
        With<Bullet>,
    >,
) {
    for (construction, pattern) in &mut patterns {
        if construction.finished {
            return;
        }

        for bullet in children.children(pattern) {
            let Ok((
                mut velocity,
                mut acceleration,
                BulletPivot(Transform {
                    translation: pivot,
                    rotation: _,
                    scale: _,
                }),
                Transform {
                    translation,
                    rotation: _,
                    scale: _,
                },
            )) = bullet_query.get_mut(*bullet)
            else {
                return;
            };
            {
                velocity.linvel = Vec2::from_angle(PI / 2.).rotate(translation.truncate());
                velocity.linvel = velocity.linvel / velocity.linvel.length() * 60.;
                acceleration.0 = Vec2::ZERO;
            }
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
            .add_systems(
                Update,
                (circle_finish_construction, circle_setup_bullets).chain(),
            )
            .add_systems(Update, circle_acceleration);
    }
}
