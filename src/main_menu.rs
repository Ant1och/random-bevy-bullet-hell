use crate::state::AppState;
use bevy::prelude::*;

pub(super) struct MainMenuPlugin;

fn main_menu(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::LoadingLevel);
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu);
    }
}
