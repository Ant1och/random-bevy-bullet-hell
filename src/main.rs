#![err(clippy::all)]

use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::{prelude::*, rapier::prelude::IntegrationParameters};
use std::num::NonZero;

mod bullet;
mod bullet_pattern;
mod camera;
mod colliders;
mod config;
mod enemy;
mod ground_detection;
mod gui;
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
        .insert_resource(LevelSelection::iid("7b660fe0-e920-11ef-8441-3da15693e03d"))
        // .insert_resource(LevelSelection::iid("a315ac10-66b0-11ec-9cd7-99f223ad6ade"))
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
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(bullet_pattern::PatternPlugin)
        .add_plugins(spell_card::SpellCardPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(input::CustomInputPlugin)
        .add_plugins(gui::GuiPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .run();
}
