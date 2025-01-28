use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::bullet::config::animation::*;
use crate::bullet::Bullet;

pub fn set_bullet_sprite(
    mut bullet: Query<&mut AseSpriteAnimation, Added<Bullet>>,
    server: Res<AssetServer>,
) {
    for mut animation in &mut bullet {
        animation.aseprite = server.load("bullet.aseprite");
        animation.animation = Animation::tag(DEFAULT).with_speed(ANIMATION_SPEED);
    }
}

pub struct BulletAnimationPlugin;

impl Plugin for BulletAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_bullet_sprite);
    }
}
