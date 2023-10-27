use bevy::{prelude::{Plugin, App, Update, Res, Input, KeyCode, Component, With, Query, Startup, Commands, AssetServer, Vec2, Transform, Handle, Image, Assets, IntoSystemConfigs, in_state, ResMut, NextState, OnEnter, Event, EventWriter, Quat, AudioBundle, PlaybackSettings, Resource, AudioSource}, sprite::{collide_aabb::collide, TextureAtlas, SpriteSheetBundle, TextureAtlasSprite}, window::Window, time::Time, audio::{Volume, VolumeLevel}};

use crate::{physics::{Velocity, Gravity}, pipes::Pipe, GameState, animation::AnimationIndices};

#[derive(Resource)]
struct WingSound(Handle<AudioSource>);

#[derive(Resource)]
struct HitSound(Handle<AudioSource>);

#[derive(Resource)]
struct FallSound(Handle<AudioSource>);

#[derive(Event)]
pub struct AddScoreEvent;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AddScoreEvent>()
            .add_systems(Startup, spawn_bird)
            .add_systems(OnEnter(GameState::Playing), reset_bird_position)
            .add_systems(
                Update,
                (bird_input, collision_check, rotate_bird).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Bird;

fn spawn_bird(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet = assets.load("sprites/bird.png");
    let atlas = TextureAtlas::from_grid(
        spritesheet,
        Vec2::new(34.0, 24.0),
        3,
        1,
        None,
        None
    );
    let atlas_handle = texture_atlases.add(atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        },
        Bird,
        Velocity(Vec2::new(0.0, 0.0)),
        Gravity(1000.0),
        AnimationIndices {
            first: 0,
            last: 2,
        },
    ));

    // sounds
    let wing_sound = assets.load("audio/wing.ogg");
    commands.insert_resource(WingSound(wing_sound));

    let hit_sound = assets.load("audio/hit.ogg");
    commands.insert_resource(HitSound(hit_sound));

    let fall_sound = assets.load("audio/die.ogg");
    commands.insert_resource(FallSound(fall_sound));
}

fn reset_bird_position(mut query: Query<&mut Transform, With<Bird>>) {
    let mut transform = query.single_mut();
    transform.translation.y = 0.0;
}

fn bird_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Bird>>,
    mut commands: Commands,
    wing_sound: Res<WingSound>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut bird_velocity = query.single_mut();
        bird_velocity.0.y = 300.0;

        // play wing sound
        commands.spawn(AudioBundle {
            source: wing_sound.0.clone(),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.1)),
                ..Default::default()
            },
        });
    }
}

// check for collision with pipes and map borders, send score event
fn collision_check(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    window: Query<&Window>,
    mut next_state: ResMut<NextState<GameState>>,
    bird_query: Query<&Transform, With<Bird>>,
    mut pipe_query: Query<(&Transform, &Handle<Image>, &mut Pipe)>,
    mut addscore_ev: EventWriter<AddScoreEvent>,
    hit_sound: Res<HitSound>,
    fall_sound: Res<FallSound>,
) {
    let bird_transform = bird_query.single();

    for (pipe_transform, pipe_image, mut pipe) in pipe_query.iter_mut() {
        let pipe_size = images.get(pipe_image).unwrap().size() * pipe_transform.scale.truncate();

        // pipe collision
        let collision = collide(
            bird_transform.translation,
            Vec2::new(34.0, 24.0), // hardcoded...
            pipe_transform.translation,
            pipe_size,
        );
        
        if let Some(_) = collision {
            next_state.set(GameState::GameOver);

            // play pipe hit sound
            commands.spawn(AudioBundle {
                source: hit_sound.0.clone(),
                settings: PlaybackSettings {
                    volume: Volume::Relative(VolumeLevel::new(0.1)),
                    ..Default::default()
                },
            });
        }

        // check if player passed the pipe
        if !pipe.0 && pipe_transform.translation.x < 0.0 {
            pipe.0 = true;
            addscore_ev.send(AddScoreEvent);
        }
    }

    // screen bounds check
    let window_height = window.single().resolution.height();

    if bird_transform.translation.y > window_height / 2.0 || bird_transform.translation.y < -window_height / 2.0 {
        next_state.set(GameState::GameOver);

        // play flew out sound
        commands.spawn(AudioBundle {
            source: fall_sound.0.clone(),
            settings: PlaybackSettings {
                volume: Volume::Relative(VolumeLevel::new(0.1)),
                ..Default::default()
            },
        });
    }
}

fn rotate_bird(
    mut query: Query<(&mut Transform, &Velocity), With<Bird>>,
    time: Res<Time>,
) {
    let (mut transform, velocity) = query.single_mut();
    let v = (velocity.0.y * time.delta_seconds()).clamp(-1.0, 1.0);
    transform.rotation = Quat::from_rotation_z(f32::to_radians(45.0 * v));
}
