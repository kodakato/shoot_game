mod components;
mod resources;
mod systems;

use bevy::prelude::*;

use resources::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>();
    }
}