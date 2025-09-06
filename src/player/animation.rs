use super::state::PlayerState;
use crate::physics::looking_direction::LookDir;
use crate::player::state::ChangePlayerState;
use crate::player::{LookingDirection, Player};
use crate::state::AppState;
use bevy::prelude::*;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteAsset};

pub mod sprites {
    use bevy_mod_aseprite::aseprite;
    aseprite!(pub Player, "reimu.aseprite");
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct AsepriteHandles(Vec<Handle<AsepriteAsset>>);

fn looking_direction(player: Single<(&mut Sprite, &LookingDirection), With<Player>>) {
    let (mut sprite, looking_direction) = player.into_inner();
    sprite.flip_x = match looking_direction.0 {
        LookDir::Right => false,
        LookDir::Left => true,
    };
}

fn player_animation(
    player: Single<(&PlayerState, &mut Aseprite), With<Player>>,
    aseprites: Res<Assets<AsepriteAsset>>,
    mut events: EventReader<ChangePlayerState>,
) {
    let (state, mut ase) = player.into_inner();
    for _ in events.read() {
        let info = aseprites.get(&ase.asset).unwrap().info();
        ase.anim = AsepriteAnimation::new(info, state.animation_tag());
    }
}

fn load_assets(mut ase_handles: ResMut<AsepriteHandles>, server: Res<AssetServer>) {
    let handle = server.load(sprites::Player::PATH);
    ase_handles.push(handle);
}

fn set_player_sprite(
    player: Single<(&mut Transform, Entity), Added<Player>>,
    ase_handles: ResMut<AsepriteHandles>,
    ase_assets: Res<Assets<AsepriteAsset>>,
    mut cmd: Commands,
) {
    let (mut transform, player) = player.into_inner();

    // Resize player and their sprite
    transform.scale.y *= 18. / 48.;

    let ase_handle = &ase_handles[0];
    let ase_asset = ase_assets.get(ase_handle).unwrap();
    let anim = AsepriteAnimation::new(ase_asset.info(), PlayerState::default().animation_tag());
    cmd.entity(player).insert((
        Sprite {
            image: ase_asset.texture().clone_weak(),
            texture_atlas: Some(TextureAtlas {
                index: anim.current_frame(),
                layout: ase_asset.layout().clone_weak(),
            }),
            ..default()
        },
        Aseprite {
            anim,
            asset: ase_handle.clone_weak(),
        },
    ));
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsepriteHandles>()
            .add_systems(OnEnter(AppState::LoadingLevelAssets), load_assets)
            .add_systems(
                Update,
                set_player_sprite.run_if(in_state(AppState::Playing)),
            )
            .add_systems(Update, player_animation)
            .add_systems(Update, looking_direction);
    }
}
