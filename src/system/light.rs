use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[allow(unused)]
pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle { ..Default::default() });
}

pub fn spawn_orange_light_source_inside(commands: &mut EntityCommands) {
    spawn_point_light_source_inside(PointLight {
        color: Color::ORANGE,
        intensity: 500.0,
        range: 10.0,
        ..Default::default()
    }, commands);
}

pub fn spawn_blue_light_source_inside(commands: &mut EntityCommands) {
    spawn_point_light_source_inside(PointLight {
        color: Color::MIDNIGHT_BLUE,
        intensity: 1000.0,
        range: 50.0,
        ..Default::default()
    }, commands);
}

fn spawn_point_light_source_inside(point_light: PointLight, commands: &mut EntityCommands) {
    commands.with_children(|b| {
        b.spawn_bundle(
            PointLightBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                point_light,
                ..Default::default()
            }
        );
    });
}