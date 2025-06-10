use std::{collections::HashSet, time::Duration};

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

const JUST_GROUNDED_BUFFER: f64 = 0.008;

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}

#[derive(Default, Component)]
pub struct GroundDetection {
    pub grounded: bool,
    pub just_grounded: bool,
    pub just_grounded_timer: Timer,
}

pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {
                x: half_extents_x,
                y: half_extents_y,
            } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents_x / 2.0, 2.);

            let sensor_translation = Vec3::new(0., -half_extents_y, 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default())
                    .insert(GroundSensor {
                        ground_detection_entity: entity,
                        intersecting_ground_entities: HashSet::new(),
                    });
            });
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, (With<Collider>, Without<Sensor>)>,
) {
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn setup_ground_detection(
    mut ground_detectors: Query<&mut GroundDetection, Added<GroundDetection>>,
) {
    let duration = Duration::from_secs_f64(JUST_GROUNDED_BUFFER);
    for mut ground_detection in &mut ground_detectors {
        ground_detection
            .just_grounded_timer
            .set_mode(TimerMode::Once);
        ground_detection.just_grounded_timer.set_duration(duration);
    }
}

pub fn update_grounded(
    mut ground_detectors: Query<&mut GroundDetection>,
    ground_sensors: Query<&GroundSensor, Changed<GroundSensor>>,
) {
    for sensor in &ground_sensors {
        if let Ok(mut ground_detection) = ground_detectors.get_mut(sensor.ground_detection_entity) {
            ground_detection.grounded = !sensor.intersecting_ground_entities.is_empty();
        }
    }
}

pub fn update_just_grounded(mut ground_detectors: Query<&mut GroundDetection>, time: Res<Time>) {
    let delta = time.delta();
    for mut ground_detection in &mut ground_detectors {
        if ground_detection.grounded {
            ground_detection.just_grounded =
                !ground_detection.just_grounded_timer.tick(delta).finished();
        } else {
            ground_detection.just_grounded_timer.reset();
        }
    }
}

pub struct GroundDetectionPlugin;

impl Plugin for GroundDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_ground_detection)
            .add_systems(Update, spawn_ground_sensor)
            .add_systems(Update, ground_detection)
            .add_systems(Update, update_grounded)
            .add_systems(Update, update_just_grounded);
    }
}
