use crate::assets::GameAssets;
use crate::components::{GameEntity, GameState};
use crate::player::PlayerDied;
use bevy::prelude::*;
use bevy::state::app::AppExtStates;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Title), (spawn_title, cleanup_game))
            .add_systems(OnExit(GameState::Title), despawn_title)
            .add_systems(
                Update,
                handle_title_input.run_if(in_state(GameState::Title)),
            )
            .add_systems(
                Update,
                player_death_to_title.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
struct TitleText;

fn spawn_title(mut commands: Commands, game_assets: Res<GameAssets>) {
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
            TitleText,
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Text::new("SPACE INVADERS"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 60.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
        ));

        parent.spawn((
            Text::new("FIRE TO PLAY"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    });
}

fn despawn_title(mut commands: Commands, query: Query<Entity, With<TitleText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_title_input(
    mut next_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn player_death_to_title(
    mut next_state: ResMut<NextState<GameState>>,
    mut events: EventReader<PlayerDied>,
) {
    for _ in events.read() {
        next_state.set(GameState::Title);
    }
}
fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    //commands.remove_resource::<AlienManager>();
}
