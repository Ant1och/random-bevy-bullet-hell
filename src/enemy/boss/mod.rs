use bevy::prelude::*;
use yuyuko::YuyukoPlugin;

pub mod yuyuko;

#[derive(Component, Default)]
pub struct SpellCardList(Vec<String>);

pub trait Boss {
    fn spell_card();
}

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YuyukoPlugin);
    }
}
