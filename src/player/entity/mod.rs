use bevy::ecs::system::EntityCommands;
use lib::entity::point::Point;

use crate::{Entity, Query, Transform};

pub struct Player {
    id: Entity,
    position: Point,
}

impl Player {
    pub fn new(id: Entity) -> Player {
        Player {
            id,
            position: Point::new(3.0, 3.0, 0.0),
        }
    }

    pub fn move_back(&mut self) {
        self.position.y -= 1.0;
    }
    pub fn move_forward(&mut self) {
        self.position.y += 1.0;
    }
    pub fn move_left(&mut self) {
        self.position.x -= 1.0;
    }
    pub fn move_right(&mut self) {
        self.position.x += 1.0;
    }

    pub fn move_model(&self, mut transforms: Query<&mut Transform>) {
        let mut position = transforms.get_mut(self.id).unwrap();
        *position = Transform::from_xyz(self.position.x, self.position.y, self.position.z - 0.5)
            .with_rotation(position.rotation);
    }
}