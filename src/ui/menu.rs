use crate::ui::{MenuState, settings};
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Ui};
use bevy_egui::{egui, EguiContext};
use lib::util::game_settings::GameSettings;
use std::process::exit;

pub fn render(
    mut egui_context: ResMut<EguiContext>,
    game_settings: ResMut<GameSettings>,
    menu_state: ResMut<State<MenuState>>,
) {
    egui::Window::new("Menu")
        .anchor(egui::Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| match menu_state.current() {
            MenuState::Main => render_main_menu(ui, menu_state),
            MenuState::GameSettings => render_game_settings_menu(ui, menu_state, game_settings),
        });
}

fn render_main_menu(ui: &mut Ui, mut menu_state: ResMut<State<MenuState>>) {
    let settings_button = ui.button("Settings");
    if settings_button.clicked() {
        menu_state.set(MenuState::GameSettings).unwrap();
    }

    // TODO normal padding?
    let exit_button = ui.button("    Exit    ");
    if exit_button.clicked() {
        // TODO fire exit event to be able to save player position etc
        exit(0);
    }
}

fn render_game_settings_menu(
    ui: &mut Ui,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_settings: ResMut<GameSettings>,
) {
    ui.add(egui::Slider::from_get_set(0.0..=1.0, settings::update_background_music(&mut game_settings)).text("Background music volume"));
    ui.add(egui::Slider::from_get_set(0.01..=1.0, settings::update_player_camera_sensitivity(game_settings)).text("Camera sensitivity"));

    let back_button = ui.button("Back");
    if back_button.clicked() {
        menu_state.set(MenuState::Main).unwrap();
    }
}
