use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollissionDetection,
    DespawnEntities,
}