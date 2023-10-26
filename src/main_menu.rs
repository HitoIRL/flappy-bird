use bevy::{prelude::{Plugin, App, Commands, AssetServer, Res, Color, Transform, Vec3, BuildChildren, Resource, Entity, ResMut, DespawnRecursiveExt, NextState, Input, KeyCode, OnEnter, OnExit, Update, in_state, IntoSystemConfigs}, text::{Text2dBundle, Text, TextStyle}};

use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(OnExit(GameState::MainMenu), cleanup)
            .add_systems(Update, handle_input.run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(Resource)]
struct MainMenu(pub Entity);

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let font = assets.load("flappy-font.ttf");
    let text = commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Flappy Bird",
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::GREEN,
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
                    "Press \"Space\" to start the game!",
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

    commands.insert_resource(MainMenu(text));
}

fn cleanup(mut commands: Commands, menu: ResMut<MainMenu>) {
    commands.entity(menu.0).despawn_recursive();
}

fn handle_input(mut state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing);
    }
}
