use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Hash, PartialEq, Eq, Debug, Reflect, Copy, Clone)]
pub enum DebugAction {
    #[cfg(debug_assertions)]
    GuiToggle,
}

pub(super) fn setup_debug_input_map(mut cmd: Commands) {
    use DebugAction as A;
    use KeyCode as K;
    cmd.spawn(InputMap::new([(A::GuiToggle, K::F6)]));
}
