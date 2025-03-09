use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq, Debug)]
pub enum KeyType {
    Up,
    Down,
    Right,
    Left,
    Dash,
    Jump,
    #[cfg(debug_assertions)]
    GuiDebugToggle,
}

use KeyType::*;
pub const KEY_MAP: &[(KeyType, &[KeyCode], &[GamepadButton])] = &[
    (
        Dash,
        &[KeyCode::ShiftLeft, KeyCode::ShiftRight],
        &[GamepadButton::RightTrigger],
    ),
    (Jump, &[KeyCode::Space], &[GamepadButton::South]),
    (
        Up,
        &[KeyCode::KeyW, KeyCode::ArrowUp],
        &[GamepadButton::DPadUp],
    ),
    (
        Down,
        &[KeyCode::KeyS, KeyCode::ArrowDown],
        &[GamepadButton::DPadDown],
    ),
    (
        Right,
        &[KeyCode::KeyD, KeyCode::ArrowRight],
        &[GamepadButton::DPadRight],
    ),
    (
        Left,
        &[KeyCode::KeyA, KeyCode::ArrowLeft],
        &[GamepadButton::DPadLeft],
    ),
    (GuiDebugToggle, &[KeyCode::F6], &[]),
];
