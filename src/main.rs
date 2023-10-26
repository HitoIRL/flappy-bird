mod physics;
mod pipes;
mod bird;
mod score;
mod animation;
mod main_menu;
mod game_over;

use animation::AnimationPlugin;
use bevy::{prelude::{App, Commands, Startup, Res, AssetServer, Camera2dBundle, PluginGroup, ClearColor, Color, Transform, Vec3, States}, DefaultPlugins, sprite::SpriteBundle, window::{WindowPlugin, Window, WindowResolution}};
use bird::BirdPlugin;
use game_over::GameOverPlugin;
use main_menu::MainMenuPlugin;
use physics::PhysicsPlugin;
use pipes::PipePlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Bird".to_string(),
                    resolution: WindowResolution::new(576.0, 1024.0).with_scale_factor_override(2.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugin,
            PipePlugin,
            BirdPlugin,
            ScorePlugin,
            AnimationPlugin,
            MainMenuPlugin,
            GameOverPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // default background
    commands.spawn(SpriteBundle {
        texture: assets.load("sprites/background-day.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
