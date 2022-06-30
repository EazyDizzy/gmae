use bevy::prelude::*;
use lib::entity::point::Point;
use lib::entity::WorldSide;

#[derive(Component, Debug)]
pub struct PhysiologyDescription {
    pub model_height: f32,
    pub model_radius: f32,
    pub jump_height: f32,
    pub movement_speed: f32,
    pub gravity_speed: f32,
    pub eyes_height: f32,
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
            eyes_height: 1.0,
        }
    }
}

impl PhysiologyDescription {
    pub fn get_eyes_position(&self, transform: &Transform, pos: &Point) -> Point {
        let (_, y_rotation, _) = transform.rotation.to_euler(EulerRot::XYZ);

        let start_x = pos.x - self.model_radius * y_rotation.sin();
        let start_z = pos.z - self.model_radius * y_rotation.cos();
        let start_y = pos.y + self.eyes_height;

        Point::new(start_x, start_y, start_z)
    }
}
