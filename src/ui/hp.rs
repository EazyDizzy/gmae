use crate::entity::component::hp::HP;
use crate::player::entity::Player;
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Pos2, ProgressBar};
use bevy_egui::{egui, EguiContext};

pub fn render(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    player_query: Query<(&HP, With<Player>)>,
) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let player = player_query.iter().next();

    egui::Window::new("HP")
        .fixed_size(vec2(250.0, 60.0))
        .collapsible(false)
        .fixed_pos(Pos2 {
            x: 0.0,
            y: window_height - 60.0,
        })
        .show(egui_context.ctx_mut(), |ui| {
            if let Some((hp, ..)) = player {
                ui.add(ProgressBar::new(hp.percent()).show_percentage());
            }
        });
}
