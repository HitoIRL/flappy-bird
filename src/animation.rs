use bevy::{prelude::{Plugin, App, Component, Query, Res, Update, Resource, ResMut, IntoSystemConfigs, in_state}, sprite::TextureAtlasSprite, time::{Time, Timer, TimerMode}};

use crate::GameState;

#[derive(Resource)]
struct AnimationTimer(Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)))
            .add_systems(Update, update_animations.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

fn update_animations(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut TextureAtlasSprite)>,
    mut timer: ResMut<AnimationTimer>,
) {
    if !timer.0.tick(time.delta()).finished() {
        return;
    }

    for (indices, mut sprite) in query.iter_mut() {
        sprite.index = if sprite.index == indices.last {
            indices.first
        } else {
            sprite.index + 1
        }
    }
}
