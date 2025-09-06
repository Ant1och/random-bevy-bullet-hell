use std::time::Duration;

use crate::{
    colliders::SensorBundle,
    player::{
        state::{PlayerState, QueuePlayerState},
        LookingDirection, Player,
    },
};
use bevy::prelude::*;
// use bevy_aseprite_ultra::prelude::{AnimationRepeat, AseAnimation};
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
    mut melee_events: EventReader<MeleeEvent>,
    mut state_event: EventWriter<QueuePlayerState>,
    mut cmd: Commands,
    player: Single<(Entity, &LookingDirection), With<Player>>,
) {
    let (player, LookingDirection(look_dir)) = player.into_inner();

    for MeleeEvent {
        area,
        offset,
        duration,
    } in melee_events.read()
    {
        let attack = cmd
            .spawn((
                Melee,
                MeleeTimer(Timer::new(*duration, TimerMode::Once)),
                SensorBundle {
                    collider: Collider::cuboid(area.half_size.x, area.half_size.y),
                    ..default()
                },
                Transform::from_xyz(offset.x * Into::<f32>::into(look_dir), offset.y, 10.),
            ))
            .id();

        cmd.entity(player).add_child(attack);
        println!("spawn!");
        state_event.write(QueuePlayerState(PlayerState::Attack));
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
            println!("despawn!");
        }
    }
}

pub(super) struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MeleeEvent>()
            .add_systems(Update, (spawn_attack, despawn_attack));
    }
}
