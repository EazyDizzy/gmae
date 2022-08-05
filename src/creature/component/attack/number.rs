use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI};
use std::time::Instant;

#[derive(Component)]
pub struct DamageNumber {
    spawned_at: Instant,
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

pub fn spawn(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    position: &Transform,
    number: u16,
) {
    let mut p = position.clone();
    p.translation.y += 2.5;
    let mut rng = rand::thread_rng();
    let x_bonus = rng.gen_range(-5..5);
    p.translation.x += x_bonus as f32 / 10.;
    let z_bonus = rng.gen_range(-5..5);
    p.translation.z += z_bonus as f32 / 10.;

    p.rotation = Quat::from_euler(EulerRot::XYZ, -FRAC_PI_6, -(PI - FRAC_PI_6), 0.);

    let scale = rng.gen_range(75..125) as f32 / 100.;
    commands
        .spawn_bundle((p, GlobalTransform::identity()))
        .insert(DamageNumber {
            spawned_at: Instant::now(),
        })
        .with_children(|builder| {
            spawn_number(builder, materials, &asset_server, number, scale);
        });
}

fn spawn_number(
    builder: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    number: u16,
    scale: f32,
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
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..colors.len());
    let color = colors[index];

    if number <= 9 {
        spawn_single_number(builder, materials, asset_server, number, 0.0, color, scale);
    } else {
        let n = number
            .to_string()
            .chars()
            .map(|v| v.to_string().parse::<u16>())
            .flatten()
            .collect::<Vec<u16>>();

        spawn_single_number(builder, materials, asset_server, n[0], -0.25, color, scale);
        spawn_single_number(builder, materials, asset_server, n[1], 0.25, color, scale);
    };
}

fn spawn_single_number(
    builder: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    number: u16,
    x_bonus: f32,
    color: Color,
    scale: f32,
) -> Entity {
    let path = ["mesh/numbers.gltf#Mesh", &number.to_string(), "/Primitive0"].concat();
    let scene = asset_server.load(path.as_str());
    let material = StandardMaterial {
        base_color: color,
        unlit: true,
        ..Default::default()
    };
    let material_handle = materials.add(material);
    let mut t = Transform::from_xyz(0. + x_bonus, 0., 0.).with_scale(vec3(scale, scale, scale));

    let entity = builder
        .spawn_bundle(PbrBundle {
            mesh: scene,
            material: material_handle,
            transform: t,
            ..Default::default()
        })
        .id();

    entity
}
