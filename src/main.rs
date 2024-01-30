use std::{time::Duration};
use rand::{thread_rng, Rng};

use bevy::prelude::*;

const PLAYER_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const ROTATION_SPEED: f32 = 2.0;

// Enemy
const ENEMY_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const ENEMY_SPAWN_SECONDS: u64 = 2;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Resource)]
struct Window;

#[derive(Component)]
struct MyCameraMarker;

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct Player {
    speed: f32,
    acceleration: f32,
    velocity: Vec3,
    rotation_velocity: f32,
    rotation_acceleration: f32,
}

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn Player
    let player_texture = asset_server.load("sprites/rocket.png");
    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            ..default()
        },
        Player { 
            speed: 100.0, 
            acceleration: 100.0, 
            velocity: Vec3::ZERO,
            rotation_velocity: 0.0,
            rotation_acceleration: 0.5,
        },
    ));

    // Enemy spawn timer
    commands.insert_resource(EnemySpawnTimer(Timer::new(Duration::from_secs(ENEMY_SPAWN_SECONDS), TimerMode::Repeating)));
}

fn player_movement(
    mut player: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in player.iter_mut() {
        
        // Rotation acceleration and deceleration
        if input.pressed(KeyCode::Left) {
            player.rotation_velocity += player.rotation_acceleration * time.delta_seconds();
        } else if input.pressed(KeyCode::Right) {
            player.rotation_velocity -= player.rotation_acceleration * time.delta_seconds();;
        }
        // Clamp to max rotation speed
        player.rotation_velocity = player.rotation_velocity.clamp(-ROTATION_SPEED, ROTATION_SPEED);
        // Apply rotation
        transform.rotate(Quat::from_rotation_z(player.rotation_velocity * time.delta_seconds()));



        // Get forward vector
        let forward = transform.rotation.mul_vec3(Vec3::Y);
        let acceleration = player.acceleration;
        // Acceleration        
        if input.pressed(KeyCode::Down) {
            player.velocity += forward * acceleration * time.delta_seconds();
        } else { // Deceleration
            let decel = 0.1 * time.delta_seconds();
            let velocity = player.velocity;
            if player.velocity.length() > decel {
                player.velocity -= velocity.normalize() * decel;
            } else {
                player.velocity = Vec3::ZERO;
            }
        }

        player.velocity = player.velocity.clamp_length_max(player.speed); // Clamp to max speed

        // Apply velocity to position
        transform.translation += player.velocity * time.delta_seconds();

    }
}


