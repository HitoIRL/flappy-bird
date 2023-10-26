use bevy::{prelude::{Plugin, Update, App, Resource, ResMut, Res, Commands, AssetServer, Transform, Vec3, Query, Component, Entity, With, IntoSystemConfigs, in_state, OnEnter}, time::{Timer, TimerMode, Time}, sprite::{SpriteBundle, Sprite}, window::Window};
use rand::{thread_rng, Rng};

use crate::GameState;

#[derive(Resource)]
struct PipeTimer(Timer);

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PipeTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(OnEnter(GameState::Playing), reset_pipes)
            .add_systems(
                Update,
                (spawn_pipes, move_pipes).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Pipe(pub bool); // bool represents if the pipe was scored

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PipeTimer>,
    assets: Res<AssetServer>,
    window: Query<&Window>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let window = window.single();
    let width = window.resolution.width();

    // pipe positions
    let pipe_x = width / 2.0 + 16.0;
    let upper_pipe_y = thread_rng().gen_range(150.0..=300.0);
    let lower_pipe_y = upper_pipe_y - 320.0 - 150.0;

    commands.spawn((
        SpriteBundle {
            texture: assets.load("sprites/pipe-green.png"),
            transform: Transform {
                translation: Vec3::new(pipe_x, lower_pipe_y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Pipe(false),
    ));

    commands.spawn((
        SpriteBundle {
            texture: assets.load("sprites/pipe-green.png"),
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(pipe_x, upper_pipe_y, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Pipe(true), // if we have two pipe's on the same X axis marked as "unscored" then the player will get two points after passing these pipes (lower and upper)
    ));
}

fn reset_pipes(
    mut commands: Commands,
    mut query: Query<Entity, With<Pipe>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

fn move_pipes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Pipe>>,
    window: Query<&Window>,
    time: Res<Time>,
) {
    let window_width = window.single().resolution.width();

    for (entity, mut transform) in query.iter_mut() {
        // move pipe to the left
        transform.translation.x -= 250.0 * time.delta_seconds();

        // offscreen pipe deletion
        if transform.translation.x < -window_width / 2.0 - 32.0 { // -32 so it seems more natural than sudden disappearance
            commands.entity(entity).despawn();
        }
    }
}
