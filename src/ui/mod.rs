mod hp;
pub mod menu;

use crate::GameState;
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Color32, Rgba};
use bevy_egui::egui::style::Margin;
use bevy_egui::EguiContext;

pub struct UIPlugin;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    Main,
    GameSettings,
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // TODO make MenuState Local?
        app.add_state(MenuState::Main)
            .add_startup_system(ui_setup_theme)
            .add_system(hp::ui_render_hp)
            .add_system(ui_track_menu_keyboard_interaction)
            .add_system_set(
                SystemSet::on_update(GameState::Pause)
                    .with_system(ui_show_cursor)
                    .with_system(menu::render),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(ui_hide_cursor));
    }
}

fn ui_track_menu_keyboard_interaction(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
) {
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

fn ui_show_cursor(windows: ResMut<Windows>) {
    toggle_cursor(windows, true);
}

fn ui_hide_cursor(windows: ResMut<Windows>) {
    toggle_cursor(windows, false);
}

fn toggle_cursor(mut windows: ResMut<Windows>, enabled: bool) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(!enabled);
    window.set_cursor_visibility(enabled);
}

fn ui_setup_theme(mut egui_context: ResMut<EguiContext>) {
    let context = egui_context.ctx_mut();
    let mut style = (*context.style()).clone();
    style.spacing.item_spacing = vec2(10.0, 10.0);
    style.spacing.button_padding = vec2(15.0, 10.0);
    style.spacing.window_margin = Margin::same(20.0);
    // hp bar color
    style.visuals.selection.bg_fill =
        Color32::from(Rgba::from_srgba_premultiplied(220, 20, 60, 255));

    context.set_style(style);
}
