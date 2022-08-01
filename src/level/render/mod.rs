use crate::level::reader::read_level_collisions;
use bevy::prelude::*;
use heron::prelude::*;
use lib::entity::level::Level;
use lib::entity::voxel::Voxel;

use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};
use crate::Material;

#[allow(clippy::needless_pass_by_value)]
pub fn level_init(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<Level>) {
    let mesh = asset_server.load(&format!("lvl/{}/lvl.glb#Scene0", level.name));
    let collisions = read_level_collisions(&level.name);
    let lvl_width = level.width() as f32;

    commands
        .spawn_bundle((
            Transform::from_xyz(lvl_width / 2.0 - 1., -0.5, lvl_width / 2.0 - 1.),
            GlobalTransform::identity(),
        ))
        .insert(RigidBody::Static)
        .with_children(|parent| {
            parent.spawn_scene(mesh);
        });

    for shape in collisions {
        let x_width = shape.end.x as f32 - shape.start.x as f32 + 1.0;
        let z_width = shape.end.z as f32 - shape.start.z as f32 + 1.0;
        let y_height = shape.end.y as f32 - shape.start.y as f32 + 1.0;

        commands
            .spawn_bundle((
                Transform::from_xyz(
                    shape.end.x as f32 - x_width / 2.0 + 0.5,
                    shape.end.y as f32 - y_height / 2.0 + 0.5,
                    shape.end.z as f32 - z_width / 2.0 + 0.5,
                ),
                GlobalTransform::identity(),
            ))
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(x_width / 2.0, y_height / 2.0, z_width / 2.0),
                border_radius: None,
            });
    }
    for light in level.lights() {
        spawn_light(&mut commands, light);
    }
}

fn spawn_light(commands: &mut Commands, voxel: &Voxel) -> bool {
    if voxel.material == Material::OrangeLight {
        spawn_orange_light_source_inside(commands, voxel);
        true
    } else if voxel.material == Material::BlueLight {
        spawn_blue_light_source_inside(commands, voxel);
        true
    } else {
        false
    }
}
