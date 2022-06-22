use crate::entity::component::hp::HP;
use crate::player::entity::Player;
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Color32, Pos2, ProgressBar, Rgba};
use bevy_egui::{egui, EguiContext};

pub fn render(
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
                        // TODO must be better way
                        ui.style_mut().visuals.selection.bg_fill =
                            Color32::from(Rgba::from_srgba_premultiplied(220, 20, 60, 255));
                        ui.add(ProgressBar::new(hp.percent()).text(format!(
                            "{} / {}",
                            hp.current(),
                            hp.max()
                        )));
                    }
                });
        });
}
