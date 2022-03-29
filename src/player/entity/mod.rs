use lib::entity::point::Point;

use crate::Entity;

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
}