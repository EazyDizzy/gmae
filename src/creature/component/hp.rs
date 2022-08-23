use bevy::prelude::*;
use bevy::render::render_asset::RenderAsset;
use std::cmp;

#[derive(Component, Debug)]
pub struct HPMeshMarker;

#[derive(Component, Debug)]
pub struct HP {
    max: u16,
    current: u16,
}

pub struct HPColors {}

impl HP {
    pub fn full(max: u16) -> HP {
        HP { max, current: max }
    }
    pub fn percent(&self) -> f32 {
        f32::from(self.current) / f32::from(self.max)
    }

    pub fn max(&self) -> u16 {
        self.max
    }
    pub fn current(&self) -> u16 {
        self.current
    }
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn sub(&mut self, amount: u16) {
        self.current -= cmp::min(self.current, amount);
    }
}

pub fn creature_hp_change_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hp_materials: Query<(&mut Handle<StandardMaterial>, &Parent), With<HPMeshMarker>>,
    hps: Query<&HP>,
) {
    for (mut material, parent) in hp_materials.iter_mut() {
        if let Ok(hp) = hps.get(**parent) {
            let p = hp.percent();
            let mut a = materials.get(&material).unwrap().clone();
            a.base_color = Color::rgb(0.5 * (p * 2.), a.base_color.g(), 0.3 * (1. - p));
            let handle = materials.add(a);
            *material = handle;
        }
    }
    materials.set_changed();
}
