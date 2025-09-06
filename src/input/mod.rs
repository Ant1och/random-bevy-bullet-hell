use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{input::debug::DebugAction, player::Player, state::AppState};

#[cfg(debug_assertions)]
pub mod debug;

#[derive(Event)]
pub struct InputMapChange;

#[derive(Actionlike, Hash, PartialEq, Eq, Debug, Reflect, Copy, Clone)]
pub enum Action {
    Dash,
    Jump,
    Attack,
    SpecialAttack,
    #[actionlike(DualAxis)]
    Direction,
}

fn setup_player_input_map(input_map: Single<&mut InputMap<Action>, Added<Player>>) {
    let mut input_map = input_map.into_inner();

    use Action as A;
    // Default kbm controls
    use KeyCode as K;
    input_map.insert_multiple([
        (A::Jump, K::Space),
        (A::Dash, K::ShiftLeft),
        (A::Dash, K::ShiftRight),
    ]);
    input_map.insert_multiple([
        (A::Attack, MouseButton::Left),
        (A::SpecialAttack, MouseButton::Right),
    ]);
    input_map.insert_dual_axis(
        A::Direction,
        VirtualDPad::new(K::KeyW, K::KeyS, K::KeyA, K::KeyD),
    );

    // Keyboard only controls
    input_map.insert_multiple([
        (A::Jump, K::KeyZ),
        (A::Attack, K::KeyX),
        (A::SpecialAttack, K::KeyC),
    ]);
    input_map.insert_dual_axis(
        A::Direction,
        VirtualDPad::new(K::ArrowUp, K::ArrowDown, K::ArrowLeft, K::ArrowRight),
    );

    // Default gamepad controls
    use GamepadButton as GB;
    input_map.insert_multiple([
        (A::Jump, GB::South),
        (A::Dash, GB::RightTrigger),
        (A::Attack, GB::LeftTrigger),
        (A::SpecialAttack, GB::RightTrigger2),
    ]);
    input_map.insert_dual_axis(
        A::Direction,
        VirtualDPad::new(GB::DPadUp, GB::DPadDown, GB::Other(12), GB::DPadRight),
    );
}

fn update_player_input_map(events: EventReader<InputMapChange>) {
    if !events.is_empty() {
        todo!()
    }
}

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .add_event::<InputMapChange>()
            .add_systems(Update, setup_player_input_map)
            .add_systems(Update, update_player_input_map);

        #[cfg(debug_assertions)]
        app.add_plugins(InputManagerPlugin::<DebugAction>::default())
            .add_systems(OnEnter(AppState::Playing), debug::setup_debug_input_map);
    }
}
