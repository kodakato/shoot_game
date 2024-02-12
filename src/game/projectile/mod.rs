use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::schedule::InGameSet;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ((
            despawn_projectiles,
        ).in_set(InGameSet::DespawnEntities))
        .run_if(in_state(AppState::InGame)));
    }
}