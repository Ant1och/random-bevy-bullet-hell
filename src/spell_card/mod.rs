use std::{str::FromStr, time::Duration};

use bevy::prelude::*;
use thiserror::Error;

pub mod circle_of_fifth;
use circle_of_fifth::CirclesOfFifthPlugin;

#[derive(Debug, Error)]
#[error("the given SpellCard doesn't exist")]
pub struct NoSuchSpellCard;

pub enum SpellCard {
    CirclesOfFifth,
}

impl FromStr for SpellCard {
    type Err = NoSuchSpellCard;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use SpellCard::*;
        match string {
            "CirclesOfFifth" => Ok(CirclesOfFifth),
            _ => Err(NoSuchSpellCard),
        }
    }
}

#[derive(Component, Default)]
pub struct SpellCardTimer(Timer);

impl SpellCardTimer {
    fn new(duration: Duration) -> Self {
        SpellCardTimer(Timer::new(duration, TimerMode::Once))
    }

    fn tick(mut timers: Query<(&mut SpellCardTimer, Entity)>, time: Res<Time>, mut cmd: Commands) {
        for (mut timer, card) in &mut timers {
            if timer.0.tick(time.delta()).finished() {
                cmd.entity(card).despawn_recursive();
            }
        }
    }
}

pub struct SpellCardPlugin;

impl Plugin for SpellCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CirclesOfFifthPlugin)
            .add_systems(Update, SpellCardTimer::tick);
    }
}
