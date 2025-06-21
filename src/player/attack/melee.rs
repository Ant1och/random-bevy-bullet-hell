use std::time::Duration;

use crate::{
    colliders::SensorBundle,
    player::{LookingDirection, Player},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

#[derive(Event)]
pub struct MeleeEvent {
    area: Rectangle,
    offset: Vec2,
    duration: Duration,
}

impl MeleeEvent {
    pub fn new(area: Rectangle, offset: Vec2, duration: Duration) -> Self {
        MeleeEvent {
            area,
            offset,
            duration,
        }
    }
}

#[derive(Component)]
struct Melee;

#[derive(Component)]
struct MeleeTimer(pub Timer);

fn spawn_attack(
    mut events: EventReader<MeleeEvent>,
    mut cmd: Commands,
    player: Single<(Entity, &Transform, &LookingDirection), With<Player>>,
) {
    let (
        player,
        Transform {
            translation: player_pos,
            ..
        },
        LookingDirection(look_dir),
    ) = player.into_inner();

    for MeleeEvent {
        area,
        offset,
        duration,
    } in events.read()
    {
        cmd.entity(player).insert((
            Melee,
            MeleeTimer(Timer::new(*duration, TimerMode::Once)),
            SensorBundle {
                collider: Collider::cuboid(area.half_size.x, area.half_size.y),
                ..default()
            },
            Transform::from_xyz(
                player_pos.x as f32 + offset.x as f32 * look_dir.into(),
                player_pos.y + offset.y,
                10.,
            ),
        ));
    }
}

fn despawn_attack(
    mut cmd: Commands,
    mut attacks: Query<(Entity, &mut MeleeTimer), With<Melee>>,
    time: Res<Time>,
) {
    for (attack, mut timer) in &mut attacks {
        if timer.0.tick(time.delta()).finished() {
            cmd.entity(attack).despawn();
        }
    }
}

pub(super) struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_attack, despawn_attack));
    }
}
