// assets.rs
use crate::components::GameState;
use bevy::audio::AudioSource;
use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

#[derive(Resource, Clone)]
pub struct Palette {
    pub colors: Vec<Color>,
}

#[derive(Resource)]
pub struct GameAssets {
    pub alien_texture_a: Handle<Image>,
    pub alien_texture_b: Handle<Image>,
    pub player_texture: Handle<Image>,
    pub bullet_texture: Handle<Image>,
    pub shield_texture: Handle<Image>,
    pub explosion_texture: Handle<Image>,
    pub font: Handle<Font>,
    pub shoot_sfx: Handle<AudioSource>,
    pub alien_killed_sfx: Handle<AudioSource>,
    pub invader_move_1_sfx: Handle<AudioSource>,
    pub invader_move_2_sfx: Handle<AudioSource>,
    pub player_explosion_sfx: Handle<AudioSource>,
    pub palette: Palette,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let palette = Palette {
        // https://lospec.com/palette-list/gilt-8 by tomicit0
        colors: vec![
            Color::srgb(0.631, 0.224, 0.333),
            Color::srgb(0.761, 0.431, 0.522),
            Color::srgb(0.949, 0.729, 0.800),
            Color::srgb(1.000, 0.949, 0.918),
            Color::srgb(0.984, 0.906, 0.412),
            Color::srgb(0.894, 0.725, 0.169),
            Color::srgb(0.769, 0.416, 0.176),
            Color::srgb(0.506, 0.173, 0.137),
        ],
    };

    commands.insert_resource(GameAssets {
        alien_texture_a: asset_server.load("alien_a.png"),
        alien_texture_b: asset_server.load("alien_b.png"),
        player_texture: asset_server.load("player.png"),
        bullet_texture: asset_server.load("bullet.png"),
        shield_texture: asset_server.load("shield.png"),
        explosion_texture: asset_server.load("explosion.png"),
        font: asset_server.load("fonts/space_invaders/space-invaders-full-version.ttf"),
        shoot_sfx: asset_server.load("sfx/shoot.wav"),
        alien_killed_sfx: asset_server.load("sfx/invaderkilled.wav"),
        invader_move_1_sfx: asset_server.load("sfx/fastinvader1.wav"),
        invader_move_2_sfx: asset_server.load("sfx/fastinvader2.wav"),
        player_explosion_sfx: asset_server.load("sfx/player_explosion.ogg"),
        palette,
    });
    next_state.set(GameState::Title);
}
