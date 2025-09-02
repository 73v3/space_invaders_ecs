use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, States, Debug)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Playing,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component, PartialEq)]
pub struct Collider {
    pub radius: f32,
    pub source: ColliderSource,
}

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ColliderSource {
    Player,
    PlayerBullet,
    Alien,
    AlienBullet,
    Shield,
    None, // For entities that aren't projectiles, like the player or aliens themselves
}

#[derive(Resource)]
pub struct GameSpeed {
    pub value: f32,
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpeed { value: 1.0 }).add_systems(
            Update,
            (despawn_dead_entities, update_velocity)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn despawn_dead_entities(mut commands: Commands, dead_query: Query<Entity, With<Dead>>) {
    for entity in dead_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.velocity.x * time.delta_secs();
        transform.translation.y += velocity.velocity.y * time.delta_secs();
    }
}
