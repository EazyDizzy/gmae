use crate::creature::component::hp::HP;
use crate::player::entity::Player;
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Pos2, ProgressBar};
use bevy_egui::{egui, EguiContext};

pub fn ui_render_hp(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    player_query: Query<(&HP, With<Player>)>,
) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let player = player_query.iter().next();
    let context = egui_context.ctx_mut();

    egui::Area::new("HP")
        .fixed_pos(Pos2 {
            x: 20.0,
            y: window_height - 40.0,
        })
        .show(context, |ui| {
            egui::Resize::default()
                .fixed_size(vec2(250.0, 40.0))
                .show(ui, |ui| {
                    if let Some((hp, ..)) = player {
                        ui.add(ProgressBar::new(hp.percent()).text(format!(
                            "{} / {}",
                            hp.current(),
                            hp.max()
                        )));
                    }
                });
        });
}
