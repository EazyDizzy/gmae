use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PhysiologyDescription {
    pub model_height: f32,
    pub model_radius: f32,
    pub jump_height: f32,
    pub movement_speed: f32,
    pub gravity_speed: f32,
}

impl Default for PhysiologyDescription {
    fn default() -> Self {
        PhysiologyDescription {
            // TODO use
            model_height: 2.0,
            model_radius: 0.5,
            // TODO use
            jump_height: 1.0,
            movement_speed: 0.1,
            gravity_speed: 0.1,
        }
    }
}
