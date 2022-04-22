use bevy::prelude::*;
use bevy_fly_camera::FlyCamera;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(initial_grab_cursor)
            .add_system(cursor_grab)
        ;
    }
}

#[allow(clippy::needless_pass_by_value)]
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
