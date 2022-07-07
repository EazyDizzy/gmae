use std::time::Instant;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use heron::prelude::*;
use bevy::render::renderer::RenderDevice;
use lib::entity::level::Level;
use lib::entity::voxel::Shape;
use lib::util::debug_settings::DebugSettings;

use crate::level::render::material::TEXTURE_SIZE;
use crate::level::render::mesh::merge_voxels;
use crate::level::render::named_materials::NamedMaterials;
use crate::level::render::shape::cube::create_cube_bundle_batch;
use crate::level::render::shape::triangle_prism::create_triangle;
use crate::level::render::voxel_sequence::VoxelSequence;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};
use crate::Material;

pub mod material;
mod mesh;
pub(super) mod named_materials;
pub mod shape;
mod voxel_sequence;

// will be fully rewritten in future
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut named_materials: ResMut<NamedMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    render_device: Res<RenderDevice>,
    settings: Res<DebugSettings>,
) {
    commands
        .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
        .insert(RigidBody::Static)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(160., 160.),
            heights: vec![vec![0., 0.], vec![0., 0.]],
        });
    let limits = render_device.limits().max_texture_dimension_2d;
    // This is needed because of wgpu limitation. It can't render a texture which breaks the limits in some dimension
    let max_voxels_per_dimension = limits / TEXTURE_SIZE;
    // dbg!(max_voxels_per_dimension);

    let merged_voxels = merge_voxels(level.voxel_stack(), max_voxels_per_dimension);

    let start = Instant::now();

    for sequence in &merged_voxels {
        match sequence.shape() {
            Shape::Cube => {
                spawn_cube_sequence(
                    &mut commands,
                    &mut meshes,
                    &mut named_materials,
                    &mut materials,
                    &mut images,
                    sequence,
                    &merged_voxels,
                    &settings,
                );
            }
            Shape::TrianglePrism(properties) => {
                create_triangle(&mut commands, &asset_server, sequence, properties);
            }
        }
    }

    debug!("world initialization: {:?}", start.elapsed());
}
// will be fully rewritten in future
#[allow(clippy::too_many_arguments)]
fn spawn_cube_sequence(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    named_materials: &mut ResMut<NamedMaterials>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    sequence: &VoxelSequence,
    merged_voxels: &[VoxelSequence],
    settings: &Res<DebugSettings>,
) {
    let batch = create_cube_bundle_batch(
        meshes,
        named_materials,
        materials,
        images,
        sequence,
        merged_voxels,
        settings,
    );
    let mut light_spawned = false;

    for bundle in batch {
        let mut entity_commands = commands.spawn_bundle(bundle);

        if !light_spawned {
            light_spawned = spawn_light(&mut entity_commands, sequence.example_material());
        }
    }
}

fn spawn_light(entity_commands: &mut EntityCommands, material: Material) -> bool {
    if material == Material::OrangeLight {
        spawn_orange_light_source_inside(entity_commands);
        true
    } else if material == Material::BlueLight {
        spawn_blue_light_source_inside(entity_commands);
        true
    } else {
        false
    }
}
