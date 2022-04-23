use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;

use crate::player::entity::Player;

#[derive(Component)]
pub struct PlayerCamera {
    /// The current pitch of the Camera in degrees.
    pub pitch: f32,
    /// The current pitch of the Camera in degrees.
    pub yaw: f32,
    /// The sensitivity of the Camera's motion based on mouse movement. Defaults to `3.0`
    pub sensitivity: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 3.0,
        }
    }
}

fn mouse_motion_system(
    time: Res<Time>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerCamera, &mut Transform)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
        // options.pitch += delta.y * options.sensitivity * time.delta_seconds();

        // options.pitch = options.pitch.clamp(-89.0, 89.9);
        // println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();
        dbg!(yaw_radians, pitch_radians);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
            * Quat::from_axis_angle(-Vec3::X, pitch_radians);
    }
}

fn camera_movement_system(
    mut camera_query: Query<(&mut PlayerCamera, &mut Transform)>,
    player_query: Query<&Player>,
) {
    let (options, mut transform) = camera_query.iter_mut().next().unwrap();
    let player_position = player_query.iter().next().unwrap().position();
    transform.translation = vec3(player_position.x, player_position.y - 10.0, player_position.z + 15.0);

    let yaw_radians = options.yaw.to_radians();
    let pitch_radians = options.pitch.to_radians();

    transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
        * Quat::from_axis_angle(-Vec3::X, pitch_radians);
}

fn setup_player_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(5.0, 8.0, 20.0),
            ..Default::default()
        })
        .insert(PlayerCamera {
            pitch: 0.0,
            yaw: 0.0,
            ..Default::default()
        });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_player_camera)
            .add_system(camera_movement_system)
            .add_system(mouse_motion_system);
    }
}
