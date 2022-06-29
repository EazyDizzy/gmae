use crate::entity::point::Point;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    relation: CreatureRelation,
    pub name: CreatureName,
    pub position: Point,
}

impl Creature {
    pub fn neytral(name: CreatureName, position: Point) -> Creature {
        Creature {
            relation: CreatureRelation::Neutral,
            name,
            position,
        }
    }
    pub fn enemy(name: CreatureName, position: Point) -> Creature {
        Creature {
            relation: CreatureRelation::Enemy,
            name,
            position,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CreatureRelation {
    Neutral,
    Friend,
    Enemy,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CreatureName {
    Dummy,
    Pizza,
}
