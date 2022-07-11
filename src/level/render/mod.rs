
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use heron::prelude::*;
use lib::entity::level::Level;
use lib::entity::voxel::Voxel;

use crate::level::render::merge::merge_voxels;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};
use crate::Material;

mod merge;
mod voxel_sequence;

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn init_world(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<Level>) {
    // let mesh = asset_server.load(&format!("lvl/{}/lvl.glb#Scene0", level.name));
    let lvl_width = level.width() as f32;

    commands
        .spawn_bundle((
            Transform::from_xyz(lvl_width / 2.0 - 1., -0.5, lvl_width / 2.0 - 1.),
            GlobalTransform::identity(),
        ))
        .insert(RigidBody::Static)
        .with_children(|parent| {
            // parent.spawn_scene(mesh);
        });

    let merged_voxels = merge_voxels(level.voxel_stack());
    for shape in merged_voxels {
        let x_width = shape.end_position().x - shape.start_position().x + 1.0;
        let z_width = shape.end_position().z - shape.start_position().z + 1.0;
        let y_height = shape.end_position().y - shape.start_position().y + 1.0;

        commands
            .spawn_bundle((
                Transform::from_xyz(
                    shape.end_position().x - x_width / 2.0 + 0.5,
                    shape.end_position().y - y_height / 2.0 + 0.5,
                    shape.end_position().z - z_width / 2.0 + 0.5,
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
