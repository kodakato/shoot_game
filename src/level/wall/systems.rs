use bevy::{prelude::*, transform::commands};

fn player_hits_wall(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Player)>,
    wall_query: Query<(&Transform, &Wall)>,
)