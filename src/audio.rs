// audio.rs
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {
        // No additional setup needed, as AudioPlugin is included in DefaultPlugins
    }
}

/// Plays a sound effect by spawning an entity that will despawn automatically after playback.
/// This is efficient for one-shot SFX and handles cleanup to avoid entity buildup.
pub fn play(commands: &mut Commands, audio: Handle<AudioSource>) {
    commands.spawn((AudioPlayer::new(audio), PlaybackSettings::DESPAWN));
}
