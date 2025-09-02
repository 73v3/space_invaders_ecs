// assets.rs
use crate::components::GameState;
use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub alien_texture: Handle<Image>,
    pub player_texture: Handle<Image>,
    pub bullet_texture: Handle<Image>,
    pub shield_texture: Handle<Image>,
    pub font: Handle<Font>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(GameAssets {
        alien_texture: asset_server.load("alien.png"),
        player_texture: asset_server.load("player.png"),
        bullet_texture: asset_server.load("bullet.png"),
        shield_texture: asset_server.load("shield.png"),
        font: asset_server.load("fonts/space_invaders/space-invaders-full-version.ttf"),
    });
    next_state.set(GameState::Title);
}
