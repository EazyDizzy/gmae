use crate::util::component::ShortLife;
use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::{FRAC_PI_6, PI};
use std::time::Duration;

#[derive(Component)]
pub struct DamageNumberMarker;

pub struct DamageNumberAssets {
    materials: Vec<Handle<StandardMaterial>>,
    meshes: Vec<Handle<Mesh>>,
}

pub fn attack_setup_damage_number_assets(
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let colors = [
        Color::CRIMSON,
        Color::FUCHSIA,
        Color::MAROON,
        Color::ORANGE_RED,
        Color::ORANGE,
        Color::PINK,
        Color::PURPLE,
        Color::TOMATO,
    ];

    let mut mat = vec![];
    for color in colors {
        let material = StandardMaterial {
            base_color: color,
            unlit: true,
            ..Default::default()
        };

        let id = materials.add(material);
        mat.push(id);
    }

    let mut meshes = vec![];
    for number in 0..=9 {
        let path = ["mesh/numbers.glb#Mesh", &number.to_string(), "/Primitive0"].concat();
        let scene: Handle<Mesh> = asset_server.load(path.as_str());
        meshes.push(scene);
    }

    commands.insert_resource(DamageNumberAssets {
        materials: mat,
        meshes,
    });
}

pub fn attack_animate_damage_numbers(mut numbers: Query<&mut Transform, With<DamageNumberMarker>>) {
    for mut t in numbers.iter_mut() {
        t.translation.y += 0.05;
    }
}

pub fn spawn(
    commands: &mut Commands,
    numbers: &DamageNumberAssets,
    position: &Transform,
    number: u16,
) {
    let mut rng = rand::thread_rng();
    let x_bonus = rng.gen_range(-5..5);
    let z_bonus = rng.gen_range(-5..5);
    let transform = Transform::from_xyz(
        position.translation.x + x_bonus as f32 / 10.,
        position.translation.y + 2.5,
        position.translation.z + z_bonus as f32 / 10.,
    )
    // TODO more accurate rotation to the camera
    .with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        -FRAC_PI_6,
        -(PI - FRAC_PI_6),
        0.,
    ));

    let scale = rng.gen_range(45..100) as f32 / 100.;
    commands
        .spawn_bundle(TransformBundle::from_transform(transform))
        .insert_bundle(VisibilityBundle::default())
        .insert(DamageNumberMarker)
        .insert(ShortLife::new(Duration::from_millis(500)))
        .with_children(|builder| {
            spawn_number(builder, numbers, number, scale);
        });
}

fn spawn_number(builder: &mut ChildBuilder, numbers: &DamageNumberAssets, number: u16, scale: f32) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..numbers.materials.len());
    let material = numbers.materials[index].clone();

    if number <= 9 {
        spawn_single_number(builder, number, 0.0, material, scale, numbers);
    } else {
        let n = number
            .to_string()
            .chars()
            .flat_map(|v| v.to_string().parse::<u16>())
            .collect::<Vec<u16>>();

        // TODO bigger numbers? like 123
        spawn_single_number(builder, n[0], -0.2, material.clone(), scale, numbers);
        spawn_single_number(builder, n[1], 0.2, material, scale, numbers);
    };
}

fn spawn_single_number(
    builder: &mut ChildBuilder,
    number: u16,
    x_bonus: f32,
    material: Handle<StandardMaterial>,
    scale: f32,
    numbers: &DamageNumberAssets,
) {
    let mesh = numbers.meshes[number as usize].clone();
    let transform = Transform::from_xyz(0. + x_bonus, 0., 0.).with_scale(vec3(scale, scale, scale));

    builder.spawn_bundle(PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    });
}
