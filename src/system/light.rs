use bevy::prelude::*;

pub fn spawn_light_source(x: f32, y: f32, z: f32, commands: &mut Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(x - 1.0, y - 1.0, z + 1.0),
        ..Default::default()
    });
}