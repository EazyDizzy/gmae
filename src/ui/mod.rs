use std::process::exit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_egui::egui::{Pos2, vec2};
use crate::GameState;

pub fn render(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    game_state: Res<State<GameState>>,
) {
    if game_state.current() != &GameState::Pause {
        return;
    }

    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let window_width = window.width();

    egui::Window::new("Menu")
        .fixed_size(vec2(500.0, 100.0))
        .collapsible(false)
        // TODO center
        .fixed_pos(Pos2 { x: window_width / 2.0, y: window_height / 2.0 })
        .show(egui_context.ctx_mut(), |ui| {
            let exit_button = ui.button("   Exit   ");

            if exit_button.clicked() {
                exit(0);
            }
        });
}