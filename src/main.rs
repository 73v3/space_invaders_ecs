use bevy::prelude::*;

//link our modules to our project
pub mod alien;
pub mod assets;
pub mod collate_src;
pub mod components;
pub mod custom_window_plugin;
pub mod game;
pub mod player;
pub mod projectile;
pub mod random;
pub mod resolution;
pub mod score;
pub mod shields;
pub mod title;

fn main() {
    App::new()
        .add_plugins((custom_window_plugin::CustomWindowPlugin, game::GamePlugin))
        .run();
}
