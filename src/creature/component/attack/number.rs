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
    p.rotation = Quat::from_euler(EulerRot::XYZ, -FRAC_PI_6, -(PI - FRAC_PI_6), 0.);

    commands
        .spawn_bundle((p, GlobalTransform::identity()))
        .insert(DamageNumber {
            spawned_at: Instant::now(),
        })
        .with_children(|builder| {
            spawn_number(builder, materials, &asset_server, number);
        });
}

fn spawn_number(
    builder: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    number: u16,
) {
    let colors = [
        Color::rgb(0.8, 0.3, 0.3),
        Color::rgb(0.81, 0.3, 0.3),
        Color::rgb(0.82, 0.3, 0.3),
        Color::rgb(0.83, 0.3, 0.3),
        Color::rgb(0.84, 0.3, 0.3),
        Color::rgb(0.85, 0.3, 0.3),
        Color::rgb(0.86, 0.3, 0.3),
        Color::rgb(0.87, 0.3, 0.3),
        Color::rgb(0.88, 0.3, 0.3),
        Color::rgb(0.89, 0.3, 0.3),
        Color::rgb(0.90, 0.3, 0.3),
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..colors.len());

    let color = colors[index];
    if number <= 9 {
        spawn_single_number(builder, materials, asset_server, number, 0.0, color);
    } else {
        let n = number
            .to_string()
            .chars()
            .map(|v| v.to_string().parse::<u16>())
            .flatten()
            .collect::<Vec<u16>>();

        spawn_single_number(builder, materials, asset_server, n[0], -0.25, color);
        spawn_single_number(builder, materials, asset_server, n[1], 0.25, color);
    };
}

fn spawn_single_number(
    builder: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &Res<AssetServer>,
    number: u16,
    x_bonus: f32,
    color: Color,
) -> Entity {
    let path = ["mesh/numbers.gltf#Mesh", &number.to_string(), "/Primitive0"].concat();
    let scene = asset_server.load(path.as_str());
    let material = StandardMaterial {
        base_color: color,
        ..Default::default()
    };
    let material_handle = materials.add(material);
    let mut t = Transform::from_xyz(0. + x_bonus, 0., 0.);

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
