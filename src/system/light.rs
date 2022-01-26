use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn spawn_orange_light_source(x: f32, y: f32, z: f32, commands: &mut EntityCommands) {
    commands.insert_bundle(PointLightBundle {
        transform: Transform::from_xyz(x, y, z),
        point_light: PointLight {
            color: Color::ORANGE,
            intensity: 500.0,
            range: 10.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
pub fn spawn_blue_light_source(x: f32, y: f32, z: f32, commands: &mut EntityCommands) {
    commands.insert_bundle(PointLightBundle {
        transform: Transform::from_xyz(x, y, z),
        point_light: PointLight {
            color: Color::MIDNIGHT_BLUE,
            intensity: 1000.0,
            range: 50.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}