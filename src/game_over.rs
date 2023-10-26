use bevy::{prelude::{Plugin, Entity, App, Resource, AssetServer, Commands, Res, Color, Transform, Vec3, BuildChildren, ResMut, DespawnRecursiveExt, NextState, KeyCode, Input, OnEnter, OnExit, Update, IntoSystemConfigs, in_state}, text::{Text2dBundle, Text, TextStyle}};

use crate::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), setup)
            .add_systems(OnExit(GameState::GameOver), cleanup)
            .add_systems(Update, handle_input.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Resource)]
struct GameOverMenu(pub Entity);

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // create text
    let font = assets.load("flappy-font.ttf");
    let text = commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Game Over",
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform {
                translation: Vec3::new(0.0, 32.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "Press \"Enter\" to try again",
                    TextStyle {
                        font,
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(0.0, -32.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    commands.insert_resource(GameOverMenu(text));
}

fn cleanup(mut commands: Commands, menu: ResMut<GameOverMenu>) {
    commands.entity(menu.0).despawn_recursive();
}

fn handle_input(
    mut state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing);
    }
}
