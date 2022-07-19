use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Bullet {
    pub shift: Vec3,
    pub damage: u16,
}

impl Bullet {
    pub fn new(direction: Vec3, speed: f32, damage: u16) -> Bullet {
        let mut max = direction.x.abs();
        for a in direction.to_array() {
            if a.abs() > max {
                max = a.abs();
            }
        }
        let x_shift = direction.x / max;
        let y_shift = direction.y / max;
        let z_shift = direction.z / max;

        let shift = vec3(speed * x_shift, speed * y_shift, speed * z_shift);

        Bullet { shift, damage }
    }
}
