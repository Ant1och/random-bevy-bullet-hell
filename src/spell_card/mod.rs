use std::{str::FromStr, time::Duration};

use bevy::prelude::*;
use thiserror::Error;

pub mod circles_of_fifth;
use circles_of_fifth::{
    circle_of_fifth_setup, circles_of_fifth, CirclesOfFifthBundle, CirclesOfFifthParams,
};

#[derive(Debug, Error)]
#[error("the given SpellCard doesn't exist")]
pub struct NoSuchSpellCard;

pub enum SpellCard {
    CirclesOfFifth(CirclesOfFifthBundle),
}

impl FromStr for SpellCard {
    type Err = NoSuchSpellCard;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use SpellCard::*;
        match string {
            "CirclesOfFifth" => Ok(CirclesOfFifth(CirclesOfFifthBundle::from_params(
                CirclesOfFifthParams {
                    frequency: Duration::from_secs_f64(5.),
                },
            ))),
            _ => Err(NoSuchSpellCard),
        }
    }
}

pub struct SpellCardPlugin;

impl Plugin for SpellCardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, circle_of_fifth_setup)
            .add_systems(Update, circles_of_fifth);
    }
}
