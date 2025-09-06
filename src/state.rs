use crate::player::{animation::AsepriteHandles, ControlsEnabled, PhysicsEnabled};
use bevy::{asset::LoadState, prelude::*};

#[derive(States, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    LoadingLevel,
    LoadingLevelAssets,
    Playing,
    Pause,
}

fn check_assets(
    ase_handles: ResMut<AsepriteHandles>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<AppState>>,
) {
    ase_handles
        .iter()
        .all(|handle| {
            matches!(
                asset_server.get_load_state(handle.id()),
                Some(LoadState::Loaded)
            )
        })
        .then(|| {
            state.set(AppState::Playing);
        });
}

fn enable_player(mut controls: ResMut<ControlsEnabled>, mut physics: ResMut<PhysicsEnabled>) {
    controls.0 = true;
    physics.0 = true;
}

fn debug(state: Res<State<AppState>>) {
    info!("entered AppState: {state:?}");
}

pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(
                Update,
                (
                    check_assets.run_if(in_state(AppState::LoadingLevelAssets)),
                    debug.run_if(state_changed::<AppState>),
                ),
            )
            .add_systems(OnEnter(AppState::Playing), enable_player);
    }
}
