use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_fly_camera::FlyCamera;
use lib::util::game_settings::GameSettings;

use crate::player::entity::Player;

const CAMERA_DISTANCE: f32 = 10.0;

#[derive(Component)]
pub struct PlayerCamera {
    /// The sensitivity of the Camera's motion based on mouse movement. Defaults to `3.0`
    pub sensitivity: f32,

    rotation_angle: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            sensitivity: 3.0,
            rotation_angle: 0.0,
        }
    }
}

fn player_rotation_system(
    camera_query: Query<&PlayerCamera>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (_, mut transform) = player_query.iter_mut().next().unwrap();

    for camera in camera_query.iter() {
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, -camera.rotation_angle - PI / 2.0, 0.0);
    }
}

fn camera_rotation_system(
    time: Res<Time>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerCamera, &mut Transform)>,
    player_query: Query<&Player>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        let new_angle = options.rotation_angle + delta.x * options.sensitivity * time.delta_seconds();
        let clamped_angle: f32 = new_angle.clamp(-PI * 2.0, PI * 2.0);
        options.rotation_angle = if clamped_angle.abs() == PI * 2.0 {
            0.0
        } else {
            clamped_angle
        };

        let player_position = player_query.iter().next().unwrap().position();
        let x = player_position.x + CAMERA_DISTANCE * options.rotation_angle.cos();
        let z = player_position.z + CAMERA_DISTANCE * options.rotation_angle.sin();
        transform.translation = vec3(x, player_position.y + CAMERA_DISTANCE, z);

        let target = Vec3::new(player_position.x, player_position.y, player_position.z);
        let up_target = Vec3::new(player_position.x, player_position.y + 200.0, player_position.z);
        transform.look_at(target, up_target);
    }
}

fn setup_player_camera(mut commands: Commands, settings: Res<GameSettings>) {
    if settings.fly_camera {
        commands
            .spawn()
            .insert_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(5.0, 8.0, 20.0),
                ..Default::default()
            })
            .insert(FlyCamera {
                sensitivity: 1.0,
                max_speed: 2.0,
                ..Default::default()
            });
    } else {
        commands
            .spawn()
            .insert_bundle(PerspectiveCameraBundle {
                ..Default::default()
            })
            .insert(PlayerCamera::default());
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_player_camera)
            .add_system(camera_rotation_system)
            .add_system(player_rotation_system)
        ;
    }
}
