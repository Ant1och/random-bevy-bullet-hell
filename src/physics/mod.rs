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
pub struct IsOutOfBounds(pub bool);

#[derive(Component, Default)]
pub struct DespawnIfOutOfBounds;

fn update_out_of_bounds(
    levels: Query<(&LevelIid, &Transform)>,
    mut entities: Query<(&GlobalTransform, &mut IsOutOfBounds)>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
) -> Result {
    let mut bounds: Option<Rect> = None;
    for (level_iid, level_transform) in &levels {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single()?)
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
            return Ok(());
        }
    };

    let out_of_bounds = |pos: Vec2| {
        pos.x > bounds.max.x || pos.y > bounds.max.y || pos.x < bounds.min.x || pos.y < bounds.min.y
    };

    for (entity_tranform, mut is_out_of_bounds) in &mut entities {
        let entity_pos = entity_tranform.translation().truncate();
        is_out_of_bounds.0 = out_of_bounds(entity_pos);
    }
    Ok(())
}

fn add_out_of_bounds(
    entities: Query<Entity, (With<DespawnIfOutOfBounds>, Without<IsOutOfBounds>)>,
    mut cmd: Commands,
) {
    for entity in &entities {
        cmd.entity(entity).insert(IsOutOfBounds::default());
    }
}

fn despawn_out_of_bounds(
    entities: Query<(Entity, &IsOutOfBounds), With<DespawnIfOutOfBounds>>,
    mut cmd: Commands,
) {
    for (entity, out_of_bounds) in &entities {
        if out_of_bounds.0 {
            cmd.entity(entity).despawn();
        }
    }
}

#[derive(Component, Default)]
pub struct DespawnIfNoChildren;

pub fn despawn_no_children(
    entities: Query<Entity, (With<DespawnIfNoChildren>, Without<Children>)>,
    mut cmd: Commands,
) {
    for entity in &entities {
        cmd.entity(entity).despawn();
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
            .add_systems(Update, update_out_of_bounds)
            .add_systems(Update, add_out_of_bounds)
            .add_systems(Update, despawn_no_children)
            .add_systems(Update, despawn_out_of_bounds);
    }
}
