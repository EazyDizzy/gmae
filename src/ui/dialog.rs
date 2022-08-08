use crate::ui::{settings, MenuState};
use bevy::prelude::*;
use bevy_egui::egui::{vec2, Ui, Color32, Stroke};
use bevy_egui::{egui, EguiContext};
use lib::util::game_settings::GameSettings;
use std::process::exit;

pub struct Images {
    pub dart_icon: Handle<Image>,
}


pub fn render0(
    mut egui_context: ResMut<EguiContext>,
    images: Res<Images>,
) {
    let bevy_texture_id = egui_context.add_image(images.dart_icon.clone_weak());
    egui::Window::new("dialog")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(egui::widgets::Image::new(bevy_texture_id, [300.0, 300.0]));
        });
}

pub fn render(
    mut egui_context: ResMut<EguiContext>
) {
    egui::Area::new("dialog")
        .anchor(egui::Align2::CENTER_BOTTOM, vec2(0.0, 0.0))
        // .resizable(false)
        // .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            let context = egui_context.ctx_mut();
            let mut style = (*context.style()).clone();
            let text_color = style.visuals.text_color();

            let rect = ui.max_rect();
            let painter = ui.painter();
            painter.rect(
                rect.shrink(1.0),
                10.0,
                style.visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );
        });
}
