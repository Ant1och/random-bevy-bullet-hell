use crate::{
    input::Action,
    player::{attack::melee::MeleePlugin, Player},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

mod melee;

fn attack(input: Single<&ActionState<Action>, With<Player>>) {
    let keys = input.into_inner();
    if keys.just_pressed(&Action::Attack) {}
}

pub(super) struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeleePlugin).add_systems(Update, attack);
    }
}
