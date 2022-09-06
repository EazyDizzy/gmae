use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use crate::system::fly_camera::FlyCamera;
use crate::GameState;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::util::debug_settings::DebugSettings;

use crate::player::PlayerMarker;

const CAMERA_HEIGHT: f32 = 15.0;

#[derive(Component)]
pub struct PlayerCamera {
    rotation_angle: f32,
    enabled: bool,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            rotation_angle: -FRAC_PI_2 - FRAC_PI_4,
            enabled: true,
        }
    }
}

// TODO refactor
fn camera_track_mouse_motion(
    mut cameras: Query<(&mut PlayerCamera, &mut Transform)>,
    player_position: Query<&Transform, (With<PlayerMarker>, Without<PlayerCamera>)>,
) {
    for (options, mut transform) in cameras.iter_mut() {
        if !options.enabled {
            continue;
        }

        if let Some(player_transform) = player_position.iter().next() {
            let player_position = player_transform.translation;
            let x = player_position.x + CAMERA_HEIGHT * options.rotation_angle.cos();
            let z = player_position.z + CAMERA_HEIGHT * options.rotation_angle.sin();
            transform.translation = vec3(x, player_position.y + CAMERA_HEIGHT, z);

            // idk how it works, but big Y fixes the camera angle problem TODO normal way?
            let up_target = Vec3::new(player_position.x, 10000.0, player_position.z);
            transform.look_at(player_position, up_target);
        }
    }
}

fn setup_player_camera(mut commands: Commands, settings: Res<DebugSettings>) {
    if settings.fly_camera {
        commands
            .spawn()
            // spawn remove hardcode, relatively to Player position
            .insert_bundle(Camera3dBundle {
                transform: Transform::from_xyz(22.0, 8.0, 22.0),
                ..Default::default()
            })
            // TODO move sensitivity to game settings
            .insert(FlyCamera {
                sensitivity: 1.0,
                max_speed: 3.0,
                ..Default::default()
            });
    } else {
        commands
            .spawn()
            .insert_bundle(Camera3dBundle {
                ..Default::default()
            })
            .insert(PlayerCamera::default());
    }
}

fn ui_enable_all_cameras(fly: Query<&mut FlyCamera>, player: Query<&mut PlayerCamera>) {
    toggle_cameras(fly, player, true);
}

fn ui_disable_all_cameras(fly: Query<&mut FlyCamera>, player: Query<&mut PlayerCamera>) {
    toggle_cameras(fly, player, false);
}

fn toggle_cameras(
    mut fly: Query<&mut FlyCamera>,
    mut player: Query<&mut PlayerCamera>,
    enabled: bool,
) {
    fly.iter_mut()
        .for_each(|mut camera| camera.enabled = enabled);
    player
        .iter_mut()
        .for_each(|mut camera| camera.enabled = enabled);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player_camera)
            .add_system(camera_track_mouse_motion)
            .add_system_set(
                SystemSet::on_update(GameState::Pause).with_system(ui_disable_all_cameras),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(ui_enable_all_cameras),
            );
    }
}
