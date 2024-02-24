use bevy::prelude::*;

pub mod components;
mod systems;

use components::*;
use systems::*;

use super::{enemy::components::Enemy, player::components::Player, schedule::InGameSet};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
        .add_systems(Update, (
            collision_detection.in_set(InGameSet::CollisionDetection),
        ))
        .add_systems(Update, 
            (
                handle_collisions::<Enemy>,
                handle_collisions::<Player>,
            )
            .chain()
            .in_set(InGameSet::EntityUpdates)
        );
    }
}

