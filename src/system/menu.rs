use bevy::prelude::*;
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(keyboard_interaction)
            .add_system_set(
                SystemSet::on_update(GameState::Pause)
                    .with_system(show_cursor)
            ).add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(hide_cursor)
        )
        ;
    }
}

fn keyboard_interaction(keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        match game_state.current() {
            GameState::Playing => {
                game_state.set(GameState::Pause).unwrap();
            }
            GameState::Pause => {
                game_state.set(GameState::Playing).unwrap();
            }
        }
    }
}

fn show_cursor(windows: ResMut<Windows>) {
    toggle_cursor(windows, true);
}

fn hide_cursor(windows: ResMut<Windows>) {
    toggle_cursor(windows, false);
}

fn toggle_cursor(mut windows: ResMut<Windows>, enabled: bool) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(!enabled);
    window.set_cursor_visibility(enabled);
}
