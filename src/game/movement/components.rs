use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
    pub max: f32,
}

impl Velocity {
    // Default constructor
    pub fn new() -> Self {
        Self::default()
    }

    // Constructor with Vec3 value
    pub fn with_value(mut self, value: Vec3) -> Self {
        self.value = value;
        self
    }

    // Constructor with max value
    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity { 
            value: Vec3::new(0.0, 0.0, 0.0),
            max: MAX_VELOCITY, 
        }
    }
}

#[derive(Component)]
pub struct Acceleration {
    pub value: Vec3,
    pub max: f32,
}

impl Acceleration {
    pub fn new() -> Self {
        self::default()
    }

    pub fn with_value(mut self, value: Vec3) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl Default for Acceleration {
    fn default() -> Acceleration {
        Acceleration { 
            value: Vec3::new(0.0, 0.0, 0.0),
            max: ACCELERATION, 
        }
    }
}


#[derive(Component)]
pub struct AngularVelocity {
    pub value: f32,
    pub max: f32,
}

impl AngularVelocity {
    pub fn new() -> Self {
        self::default()
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl Default for AngularVelocity {
    fn default() -> AngularVelocity {
        AngularVelocity { 
            value: 0.0,
            max: MAX_ANGULAR_VELOCITY,
         }
    }
}

#[derive(Component)]
pub struct AngularAcceleration {
    pub value: f32,
    pub max: f32,
}

impl AngularAcceleration {
    pub fn new() -> Self {
        self::default()
    }

    pub fn with_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl Default for AngularAcceleration {
    fn default() -> AngularAcceleration {
        AngularAcceleration { 
            value: 0.0, 
            max: ANGULAR_ACCELERATION,
        }
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