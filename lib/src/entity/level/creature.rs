use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    relation: CreatureRelation,
    pub name: CreatureName,
    pub position: Vec3,
}

impl Creature {
    pub fn neytral(name: CreatureName, position: Vec3) -> Creature {
        Creature {
            relation: CreatureRelation::Neutral,
            name,
            position,
        }
    }
    pub fn enemy(name: CreatureName, position: Vec3) -> Creature {
        Creature {
            relation: CreatureRelation::Enemy,
            name,
            position,
        }
    }

    pub fn is_enemy(&self) -> bool {
        self.relation == CreatureRelation::Enemy
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
