use super::animation::sprites;
use bevy::prelude::*;
use bevy_mod_aseprite::{Aseprite, AsepriteAsset, AsepriteTag};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    input::Action,
    player::{ControlsEnabled, PhysicsEnabled, Player},
};

// #[derive(Default)]
// struct StateParams;

#[derive(Debug, Default, Clone, PartialEq, Component)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Attack,
}

#[derive(Debug, Component)]
struct PlayerStateBuf(PlayerState);

#[derive(Event, Debug)]
pub struct ChangePlayerState(pub PlayerState);

#[derive(Event, Debug)]
pub struct QueuePlayerState(pub PlayerState);

impl PlayerState {
    pub fn has_priority(&self) -> bool {
        use PlayerState as S;
        match self {
            S::Idle => false,
            S::Walk => false,
            S::Attack => true,
        }
    }
    pub fn has_controls_priority(&self) -> bool {
        use PlayerState as S;
        match self {
            S::Idle => false,
            S::Walk => false,
            S::Attack => true,
        }
    }
    // Overrides control priority
    pub fn has_physics_priority(&self) -> bool {
        use PlayerState as S;
        match self {
            S::Idle => false,
            S::Walk => false,
            S::Attack => true,
        }
    }
    pub fn animation_tag(&self) -> AsepriteTag {
        match &self {
            Self::Idle => sprites::Player::tags::IDLE,
            Self::Walk => sprites::Player::tags::WALK,
            Self::Attack => sprites::Player::tags::ATTACK,
        }
        .into()
    }
}

fn apply_state_buf(
    aseprites: Res<Assets<AsepriteAsset>>,
    mut change_event: EventWriter<ChangePlayerState>,
    player: Single<(Entity, &PlayerStateBuf, &PlayerState, &Aseprite), With<Player>>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    let (player, state_buf, state, ase) = player.into_inner();
    let ase_asset = aseprites.get(&ase.asset).unwrap();

    let remaining_frames = ase.anim.remaining_tag_frames(ase_asset.info()).unwrap();
    let frame_finished = ase.anim.frame_finished(time.delta());
    let a = remaining_frames == 0 && frame_finished;
    if state.has_priority() {
        if remaining_frames == 0 && frame_finished {
            change_event.write(ChangePlayerState(state_buf.0.clone()));
            cmd.entity(player).remove::<PlayerStateBuf>();
            println!("a: {a}");
            return;
        }
    } else {
        change_event.write(ChangePlayerState(state_buf.0.clone()));
        cmd.entity(player).remove::<PlayerStateBuf>();
        println!("a: {a}");
    }
}

fn queue_state(
    mut queue_events: EventReader<QueuePlayerState>,
    player: Single<Entity, With<Player>>,
    mut cmd: Commands,
) {
    let player = player.into_inner();
    for event in queue_events.read() {
        println!("{event:?}");
        cmd.entity(player).insert(PlayerStateBuf(event.0.clone()));
    }
}

fn change_state(
    mut events: EventReader<ChangePlayerState>,
    mut controls_enabled: ResMut<ControlsEnabled>,
    mut physics_enabled: ResMut<PhysicsEnabled>,
    player: Single<&mut PlayerState, With<Player>>,
) {
    let mut state = player.into_inner();
    for event in events.read() {
        *state = event.0.clone();
        controls_enabled.0 = !event.0.has_controls_priority();
        physics_enabled.0 = !event.0.has_physics_priority();
    }
}

fn walking_state(
    player: Single<(&ActionState<Action>, &PlayerState), With<Player>>,
    mut event: EventWriter<QueuePlayerState>,
) {
    let (input, state) = player.into_inner();

    let direction = input.axis_pair(&Action::Direction);
    use PlayerState as S;
    let new_state = match direction.x {
        0. => S::Idle,
        _ => S::Walk,
    };
    if new_state != *state {
        event.write(QueuePlayerState(new_state));
    }
}

pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<QueuePlayerState>>()
            .init_resource::<Events<ChangePlayerState>>()
            .add_systems(Update, walking_state)
            .add_systems(Update, queue_state)
            .add_systems(Update, change_state)
            .add_systems(Update, apply_state_buf);
    }
}
