use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

#[derive(Resource)]
pub struct Health {
    pub amount: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Health {
        Health {
            amount: max,
            max,
        }
    }
}

impl Default for Health {
    fn default() -> Health {
        Health {
            amount: 100,
            max: 100,
        }
    }
}