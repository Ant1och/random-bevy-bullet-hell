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

#[derive(Component, Default)]
pub struct DespawnIfOutOfBounds;

fn despawn_out_of_bounds(
    levels: Query<(&LevelIid, &Transform)>,
    entities: Query<(Entity, &GlobalTransform), With<DespawnIfOutOfBounds>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    mut cmd: Commands,
) {
    let mut bounds: Option<Rect> = None;
    for (level_iid, level_transform) in &levels {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("spawned level should exist in ldtk project");

        if !level_selection.is_match(&LevelIndices::default(), level) {
            continue;
        }

        let level_translation = level_transform.translation.truncate();

        bounds = Some(Rect {
            min: Vec2::new(level_translation.x, level_translation.y),
            max: Vec2::new(
                level_translation.x + level.px_wid as f32,
                level_translation.y + level.px_hei as f32,
            ),
        });
    }

    let bounds = match bounds {
        Some(val) => val,
        None => {
            return;
        }
    };

    let out_of_bounds = |pos: Vec2| {
        pos.x > bounds.max.x || pos.y > bounds.max.y || pos.x < bounds.min.x || pos.y < bounds.min.y
    };

    for (entity, entity_tranform) in &entities {
        let entity_pos = entity_tranform.translation().truncate();
        if out_of_bounds(entity_pos) {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Default)]
pub struct DespawnIfNoChildren;

pub fn despawn_no_children(
    entities: Query<(Entity, &Children), With<DespawnIfNoChildren>>,

    mut cmd: Commands,
) {
    for (entity, children) in entities.iter() {
        if children.is_empty() {
            cmd.entity(entity).despawn();
        }
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
            .add_systems(Update, physics_acceleration)
            .add_systems(Update, despawn_no_children)
            .add_systems(Update, despawn_out_of_bounds);
    }
}
