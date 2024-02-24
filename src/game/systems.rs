use bevy::prelude::*;


use super::{components::*};

pub fn toggle_simulation (
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        match simulation_state.get() {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Pausing simulation");
            },
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("Resuming simulation");
            }
        }
    }
}

pub fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, health) in query.iter() {
        if health.amount <= 0 {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/explosion.ogg"),
                settings: PlaybackSettings::DESPAWN,
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}




