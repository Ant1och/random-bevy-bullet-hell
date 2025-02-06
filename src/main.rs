use std::num::NonZero;

use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};

mod bullet;
mod bullet_pattern;
mod camera;
mod colliders;
mod config;
mod enemy;
mod ground_detection;
mod input;
mod physics;
mod player;
mod shared;
mod spell_card;
mod walls;
mod world;

fn spawn_context(mut commands: Commands) {
    let mut context = RapierContextSimulation::default();
    context.integration_parameters = IntegrationParameters {
        length_unit: 1000.,
        contact_damping_ratio: 0.,
        contact_natural_frequency: 0.,
        normalized_prediction_distance: 0.1,
        normalized_allowed_linear_error: 0.01,
        normalized_max_corrective_velocity: 1000.,
        num_solver_iterations: NonZero::new(4).unwrap(),
        ..default()
    };

    commands.spawn((Name::new("Rapier Context"), context, DefaultRapierContext));
}

fn main() {
    App::new()
        // Pixel art fix
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::with_custom_initialization(
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.),
                RapierContextInitialization::NoAutomaticRapierContext,
            ),
        ))
        .add_systems(PreStartup, spawn_context.before(PhysicsSet::SyncBackend))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(world::WorldPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(bullet_pattern::PatternsPlugin)
        .add_plugins(spell_card::SpellCardPlugin)
        .add_plugins(enemy::boss::BossPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(input::CustomInputPlugin)
        .run();
}
