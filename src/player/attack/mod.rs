use std::time::Duration;

use crate::{
    input::Action,
    player::{
        attack::melee::{MeleeEvent, MeleePlugin},
        Player,
    },
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

mod melee;

fn attack(input: Single<&ActionState<Action>, With<Player>>, mut event: EventWriter<MeleeEvent>) {
    let keys = input.into_inner();
    if keys.just_pressed(&Action::Attack) {
        event.write(MeleeEvent::new(
            Rectangle::new(1., 1.),
            Vec2::new(1., 0.),
            Duration::from_secs_f64(1.),
        ));
    }
}

pub(super) struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeleePlugin).add_systems(Update, attack);
    }
}
