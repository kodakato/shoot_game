use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(vec: Vec3) -> Self {
        Velocity { value: vec }
    }
}

#[derive(Component, Default)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(vec: Vec3) -> Self {
        Acceleration { value: vec }
    }
}


#[derive(Component, Default)]
pub struct AngularVelocity {
    pub value: f32,
}

impl AngularVelocity {
    pub fn new(value: f32) -> Self {
        AngularVelocity { value }
    }
}

#[derive(Component, Default)]
pub struct AngularAcceleration {
    pub value: f32,
}

impl AngularAcceleration {
    pub fn new(value: f32) -> Self {
        AngularAcceleration { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub angular_velocity: AngularVelocity,
    pub angular_acceleration: AngularAcceleration,
    pub sprite: SpriteBundle,
}