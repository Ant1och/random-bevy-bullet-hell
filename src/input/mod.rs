use bevy::prelude::*;
use std::collections::HashMap;

pub mod config;
use config::{KeyType, KEY_MAP};

pub mod prelude {
    pub use super::config::KeyType;
    pub use super::CustomInput;
    pub use super::Direction;
    pub use super::KeysPressed;
}

#[derive(Default, Debug)]
pub struct CustomKeyState {
    pressed: bool,
    just_pressed: bool,
}

#[derive(Component, Default)]
pub struct CustomInput;

#[derive(Component, Default, Debug)]
pub struct Direction(pub Vec2);

#[derive(Bundle, Default)]
pub struct CustomInputBundle {
    entity: CustomInput,
    direction: Direction,
    keys: KeysPressed,
}

#[derive(Component, Default, Debug)]
pub struct KeysPressed(HashMap<&'static KeyType, CustomKeyState>);

impl From<&'static [(KeyType, &'static [KeyCode], &'static [GamepadButton])]> for KeysPressed {
    fn from(key_map: &'static [(KeyType, &'static [KeyCode], &'static [GamepadButton])]) -> Self {
        let mut keys = HashMap::new();
        for (key_type, _, _) in key_map.iter() {
            keys.insert(key_type, CustomKeyState::default());
        }
        KeysPressed(keys)
    }
}

impl KeysPressed {
    pub fn pressed(&self, key: KeyType) -> bool {
        self.0.get(&key).unwrap().pressed
    }

    pub fn just_pressed(&self, key: KeyType) -> bool {
        self.0.get(&key).unwrap().just_pressed
    }
}

fn input_setup_keys(mut cmd: Commands) {
    let keys = KeysPressed::from(KEY_MAP);
    cmd.spawn(CustomInputBundle { keys, ..default() });
}

fn input_update_keys(
    mut input: Query<&mut KeysPressed, With<CustomInput>>,
    buttons: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
) {
    let Ok(mut keys) = input.get_single_mut() else {
        return;
    };

    // Ignore all gamepads except the first one
    let mut gamepad: Option<&Gamepad> = None;
    for i in &gamepads {
        gamepad = Option::from(i);
    }

    for (key_type, key_codes, gamepad_codes) in KEY_MAP {
        keys.0.get_mut(key_type);
        let Some(key_state) = keys.0.get_mut(key_type) else {
            return;
        };

        let mut pressed = key_codes.iter().any(|&key_code| buttons.pressed(key_code));

        let mut just_pressed = key_codes
            .iter()
            .any(|&key_code| buttons.just_pressed(key_code));

        if gamepad.is_some() {
            let gamepad = gamepad.unwrap();
            pressed = pressed
                || gamepad_codes
                    .iter()
                    .any(|&gamepad_code| gamepad.pressed(gamepad_code));
            just_pressed = just_pressed
                || gamepad_codes
                    .iter()
                    .any(|&gamepad_code| gamepad.just_pressed(gamepad_code));
        }
        *key_state = CustomKeyState {
            pressed,
            just_pressed,
        };
    }
}

pub fn input_direction(mut input: Query<(&mut Direction, &KeysPressed), With<CustomInput>>) {
    let Ok((mut direction, keys)) = input.get_single_mut() else {
        return;
    };

    use KeyType::*;
    let up = if keys.pressed(Up) { 1. } else { 0. };
    let down = if keys.pressed(Down) { 1. } else { 0. };
    let right = if keys.pressed(Right) { 1. } else { 0. };
    let left = if keys.pressed(Left) { 1. } else { 0. };

    direction.0.x = right - left;
    direction.0.y = up - down;
}

pub struct CustomInputPlugin;

impl Plugin for CustomInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_direction)
            .add_systems(Update, input_update_keys)
            .add_systems(Startup, input_setup_keys);
    }
}
