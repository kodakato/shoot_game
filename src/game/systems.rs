use bevy::prelude::*;

use super::components::*;

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


