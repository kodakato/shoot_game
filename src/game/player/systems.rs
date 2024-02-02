use bevy::prelude::*;

use super::components::*;

pub fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    // Rotation acceleration and deceleration
    if input.pressed(KeyCode::Left) {
        player.rotation_velocity += player.rotation_acceleration * time.delta_seconds();
    } else if input.pressed(KeyCode::Right) {
        player.rotation_velocity -= player.rotation_acceleration * time.delta_seconds();
    } else { // Deceleration
        let decel = ROTATION_DECELERATION * time.delta_seconds();
        if player.rotation_velocity > decel {
            player.rotation_velocity -= decel;
        } else if player.rotation_velocity < -decel {
            player.rotation_velocity += decel;
        } else {
            player.rotation_velocity = 0.0;
        }
    }

    // Clamp to max rotation speed
    player.rotation_velocity = player.rotation_velocity.clamp(-MAX_ROTATION_SPEED, MAX_ROTATION_SPEED);
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
    // Apply movement
    transform.translation += player.velocity * time.delta_seconds();
    
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawn Player
    let player_texture = asset_server.load("sprites/rocket.png");
    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform {
                scale: PLAYER_SCALE,
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player { 
            max_speed: MAX_SPEED, 
            acceleration: ACCELERATION, 
            velocity: Vec3::ZERO,
            rotation_velocity: 0.0,
            rotation_acceleration: ROTATION_ACCELERATION,
            health: 100,
        },
    ));
}