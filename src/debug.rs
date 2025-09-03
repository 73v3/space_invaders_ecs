use crate::alien::Alien;
use crate::components::GameState;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, test_wave_clear.run_if(in_state(GameState::Playing)));
    }
}

fn test_wave_clear(
    mut commands: Commands,
    alien_query: Query<Entity, With<Alien>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::End) {
        for entity in alien_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
