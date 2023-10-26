use bevy::{prelude::{Vec2, Component, Plugin, App, Update, Query, Transform, Res, IntoSystemConfigs, in_state, OnEnter}, time::Time};

use crate::GameState;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Gravity(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), reset_velocity)
            .add_systems(Update, update_physics.run_if(in_state(GameState::Playing)));
    }
}

fn reset_velocity(
    mut query: Query<&mut Velocity>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec2::new(0.0, 0.0);
    }
}

fn update_physics(
    mut query: Query<(&mut Transform, &mut Velocity, &Gravity)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, gravity) in query.iter_mut() {
        // update y velocity based on gravitation
        velocity.0.y -= gravity.0 * time.delta_seconds();

        // update transform based on velocity
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}
