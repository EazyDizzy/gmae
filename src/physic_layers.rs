use heron::prelude::*;
use heron::CollisionLayers;

#[derive(PhysicsLayer)]
pub enum GamePhysicsLayer {
    World,
    Player,
    Creature,
    Projectile,
    Sensor,
}

pub fn is_sensor(layers: CollisionLayers) -> bool {
    layers.contains_group(GamePhysicsLayer::Sensor)
        && !layers.contains_group(GamePhysicsLayer::Player)
        && !layers.contains_group(GamePhysicsLayer::Projectile)
        && !layers.contains_group(GamePhysicsLayer::World)
        && !layers.contains_group(GamePhysicsLayer::Creature)
}
