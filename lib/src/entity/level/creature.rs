use crate::entity::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Creature {
    relation: CreatureRelation,
    name: CreatureName,
    position: Point,
}

impl Creature {
    pub fn neytral(name: CreatureName, position: Point) -> Creature {
        Creature {
            relation: CreatureRelation::Neutral,
            name,
            position,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreatureRelation {
    Neutral,
    Friend,
    Enemy,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreatureName {
    Dummy,
}
