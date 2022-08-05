use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};
use std::time::Instant;

#[derive(Component)]
pub struct DamageNumber {
    spawned_at: Instant,
}

pub struct DamageNumbers {
    materials: Vec<Handle<StandardMaterial>>,
    meshes: Vec<Handle<Mesh>>,
}

pub fn attack_setup_damage_numbers_assets(
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
        let path = ["mesh/numbers.gltf#Mesh", &number.to_string(), "/Primitive0"].concat();
        let scene: Handle<Mesh> = asset_server.load(path.as_str());
        meshes.push(scene);
    }

    commands.insert_resource(DamageNumbers {
        materials: mat,
        meshes,
    });
}

pub fn attack_animate_damage_numbers(
    mut numbers: Query<(&mut Transform, &DamageNumber, Entity)>,
    mut commands: Commands,
) {
    for (mut t, number, e) in numbers.iter_mut() {
        t.translation.y += 0.05;

        if number.spawned_at.elapsed().as_secs() >= 1 {
            commands.entity(e).despawn_recursive();
        }
    }
}

pub fn spawn(commands: &mut Commands, numbers: &DamageNumbers, position: &Transform, number: u16) {
    let mut p = position.clone();
    p.translation.y += 2.5;
    let mut rng = rand::thread_rng();
    let x_bonus = rng.gen_range(-5..5);
    p.translation.x += x_bonus as f32 / 10.;
    let z_bonus = rng.gen_range(-5..5);
    p.translation.z += z_bonus as f32 / 10.;

    // TODO more accurate rotation to the camera
    p.rotation = Quat::from_euler(EulerRot::XYZ, -FRAC_PI_6, -(PI - FRAC_PI_6), 0.);

    let scale = rng.gen_range(75..125) as f32 / 100.;
    commands
        .spawn_bundle((p, GlobalTransform::identity()))
        .insert(DamageNumber {
            spawned_at: Instant::now(),
        })
        .with_children(|builder| {
            spawn_number(builder, numbers, number, scale);
        });
}

fn spawn_number(builder: &mut ChildBuilder, numbers: &DamageNumbers, number: u16, scale: f32) {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..numbers.materials.len());
    let material = numbers.materials[index].clone();

    if number <= 9 {
        spawn_single_number(builder, number, 0.0, material, scale, numbers);
    } else {
        let n = number
            .to_string()
            .chars()
            .map(|v| v.to_string().parse::<u16>())
            .flatten()
            .collect::<Vec<u16>>();

        spawn_single_number(builder, n[0], -0.25, material.clone(), scale, numbers);
        spawn_single_number(builder, n[1], 0.25, material, scale, numbers);
    };
}

fn spawn_single_number(
    builder: &mut ChildBuilder,
    number: u16,
    x_bonus: f32,
    material: Handle<StandardMaterial>,
    scale: f32,
    numbers: &DamageNumbers,
) -> Entity {
    let mesh = numbers.meshes[number as usize].clone();
    let mut t = Transform::from_xyz(0. + x_bonus, 0., 0.).with_scale(vec3(scale, scale, scale));

    let entity = builder
        .spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: t,
            ..Default::default()
        })
        .id();

    entity
}
