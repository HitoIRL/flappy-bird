use bevy::{prelude::{Plugin, App, Commands, AssetServer, Res, Query, Color, Transform, Vec3, Startup, Update, IntoSystemConfigs, in_state, OnEnter, EventReader, Resource, ResMut, AudioBundle, PlaybackSettings, AudioSource, Handle}, window::Window, text::{TextStyle, Text2dBundle, Text}, audio::{Volume, VolumeLevel}};

use crate::{GameState, bird::AddScoreEvent};

#[derive(Resource)]
struct Score(pub usize);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), reset)
            .add_systems(Update, update.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
struct ScoreSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Query<&Window>,
) {
    // font
    let window_height = window.single().resolution.height();
    let font = assets.load("flappy-font.ttf");
    
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "0",
            TextStyle {
                font,
                font_size: 32.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform {
            translation: Vec3::new(0.0, window_height / 2.0 - 30.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // audio
    let sound = assets.load("audio/point.ogg");
    commands.insert_resource(ScoreSound(sound));
}

fn reset(
    mut score: ResMut<Score>,
    mut query: Query<&mut Text>,
) {
    score.0 = 0;
    let mut text = query.single_mut();
    text.sections[0].value = score.0.to_string();
}

fn update(
    mut commands: Commands,
    mut addscore_ev: EventReader<AddScoreEvent>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text>,
    sound: Res<ScoreSound>,
) {
    for _event in addscore_ev.iter() {
        score.0 += 1;
        let mut text = query.single_mut();
        text.sections[0].value = score.0.to_string();

        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.1)),
                ..Default::default()
            },
        });
    }
}
