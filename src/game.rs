use bevy::prelude::*;

use crate::alien;
use crate::assets;
use crate::collate_src;
use crate::components;
use crate::player;
use crate::projectile;
use crate::random;
use crate::resolution;
use crate::score;
use crate::title;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collate_src::CollateSrcPlugin,
            components::ComponentsPlugin,
            alien::AlienPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            random::RandomPlugin,
            title::TitlePlugin,
            assets::AssetsPlugin,
            score::ScorePlugin,
        ))
        .add_systems(Startup, setup_scene);
    }
}
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
