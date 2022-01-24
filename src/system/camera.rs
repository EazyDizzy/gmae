use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-10.0, -10.0, 10.0)
            .looking_at(Vec3::from([180.0, 180.0, -50.0]), Vec3::Z),
        ..Default::default()
    });

    commands.spawn_bundle(UiCameraBundle::default());
}