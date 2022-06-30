use std::f32::consts::{PI, TAU};

use crate::GameState;
use bevy::input::mouse::MouseMotion;
use bevy::math::vec3;
use bevy::prelude::*;
use crate::system::fly_camera::FlyCamera;
use lib::util::debug_settings::DebugSettings;
use lib::util::game_settings::GameSettings;
use crate::creature::component::movement::locomotivity::Locomotivity;

use crate::player::entity::Player;

const CAMERA_HEIGHT: f32 = 10.0;

#[derive(Component)]
pub struct PlayerCamera {
    sensitivity: f32,

    rotation_angle: f32,
    enabled: bool,
}

impl PlayerCamera {
    pub fn angle(&self) -> f32 {
        self.rotation_angle
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.3,
            rotation_angle: 0.0,
            enabled: true,
        }
    }
}

fn camera_rotate_player_model(
    camera_query: Query<&PlayerCamera>,
    mut player_query: Query<(&mut Transform, With<Player>)>,
) {
    let (mut transform, ..) = player_query.iter_mut().next().unwrap();

    for camera in camera_query.iter() {
        transform.rotation =
            Quat::from_euler(EulerRot::XYZ, 0.0, -camera.rotation_angle - PI / 2.0, 0.0);
    }
}

fn camera_track_mouse_motion(
    time: Res<Time>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerCamera, &mut Transform)>,
    player_query: Query<(&Locomotivity, With<Player>)>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }
        let mut new_angle =
            options.rotation_angle + delta.x * options.sensitivity * time.delta_seconds();
        if new_angle < 0.0 {
            // angle should always be positive. -1.28 === 5.0
            new_angle += TAU;
        }
        let clamped_angle: f32 = new_angle.clamp(0.0, TAU);
        options.rotation_angle = if clamped_angle.abs() == PI * 2.0 {
            0.0
        } else {
            clamped_angle
        };

        let (locomotivity, ..) = player_query.iter().next().unwrap();
        let player_position = locomotivity.position();
        let x = player_position.x + CAMERA_HEIGHT * options.rotation_angle.cos();
        let z = player_position.z + CAMERA_HEIGHT * options.rotation_angle.sin();
        transform.translation = vec3(x, player_position.y + CAMERA_HEIGHT, z);

        let target = Vec3::new(
            player_position.x,
            player_position.y + 3.0,
            player_position.z,
        );
        // idk how it works, but big Y fixes the camera angle problem TODO normal way?
        let up_target = Vec3::new(player_position.x, 10000.0, player_position.z);
        transform.look_at(target, up_target);
    }
}

fn setup_player_camera(mut commands: Commands, settings: Res<DebugSettings>, game_settings: Res<GameSettings>) {
    if settings.fly_camera {
        commands
            .spawn()
            // spawn remove hardcode, relatively to Player position
            .insert_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(5.0, 8.0, 20.0),
                ..Default::default()
            })
            .insert(FlyCamera {
                sensitivity: 1.0,
                max_speed: 3.0,
                ..Default::default()
            });
    } else {
        commands
            .spawn()
            .insert_bundle(PerspectiveCameraBundle {
                ..Default::default()
            })
            .insert(PlayerCamera {
                sensitivity: game_settings.get_mouse_sensitivity() as f32,
                ..Default::default()
            });
    }
}

fn ui_enable_all_cameras(fly: Query<&mut FlyCamera>, player: Query<&mut PlayerCamera>) {
    toggle_cameras(fly, player, true);
}

fn ui_disable_all_cameras(fly: Query<&mut FlyCamera>, player: Query<&mut PlayerCamera>) {
    toggle_cameras(fly, player, false);
}

fn apply_mouse_sensitivity_change(game_settings: Res<GameSettings>, mut player: Query<&mut PlayerCamera>) {
    if game_settings.is_changed() {
        player.iter_mut()
            .for_each(|mut camera| camera.sensitivity = game_settings.get_mouse_sensitivity() as f32);
    }
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
            .add_system(camera_rotate_player_model)
            .add_system(apply_mouse_sensitivity_change)
            .add_system_set(SystemSet::on_update(GameState::Pause).with_system(ui_disable_all_cameras))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(ui_enable_all_cameras));
    }
}
