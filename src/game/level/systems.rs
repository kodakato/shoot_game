use bevy::prelude::*;

use super::wall::components::*;

pub fn setup_level(
    mut commands: Commands,
) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
}