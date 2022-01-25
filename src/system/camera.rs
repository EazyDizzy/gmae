use std::cmp;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_fly_camera::FlyCamera;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..Default::default()
        })
        // TODO disable "roll" camera rotation
        .insert(FlyCamera {
            sensitivity: 10.0,
            pitch: -70.0,
            yaw: -40.0,
            key_forward: KeyCode::LShift,
            key_backward: KeyCode::Space,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::W,
            key_down: KeyCode::S,
            ..Default::default()
        });
}

pub fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut FlyCamera>,
) {
    let window = windows.get_primary_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(window);
        query.iter_mut().for_each(|mut camera| camera.enabled = !window.cursor_visible());
    }
}

pub fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    toggle_grab_cursor(windows.get_primary_mut().unwrap());
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}
