use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use lib::entity::level::Level;

pub fn setup(mut commands: Commands, level: Res<Level>) {
    if level.is_day() {
        // TODO normal light
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(5.0, 9.0, 5.0),
            point_light: PointLight {
                intensity: 5500.0,
                range: 30.0,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn spawn_orange_light_source_inside(commands: &mut EntityCommands) {
    spawn_point_light_source_inside(
        PointLight {
            color: Color::ORANGE,
            intensity: 500.0,
            range: 10.0,
            ..Default::default()
        },
        commands,
    );
}

pub fn spawn_blue_light_source_inside(commands: &mut EntityCommands) {
    spawn_point_light_source_inside(
        PointLight {
            color: Color::MIDNIGHT_BLUE,
            intensity: 1000.0,
            range: 50.0,
            ..Default::default()
        },
        commands,
    );
}

fn spawn_point_light_source_inside(point_light: PointLight, commands: &mut EntityCommands) {
    commands.with_children(|b| {
        b.spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            point_light,
            ..Default::default()
        });
    });
}
