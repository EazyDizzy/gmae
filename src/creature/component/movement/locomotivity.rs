use bevy::math::vec3;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use lib::entity::level::Level;
use lib::util::math::round_based;

pub fn creature_not_inside_blocks(
    x: f32,
    y: f32,
    z: f32,
    lvl: &Res<Level>,
    phys: &PhysiologyDescription,
) -> bool {
    let mut obstacles: Vec<Vec3> = get_touched_points(x, y, z, phys);

    obstacles = obstacles
        .iter()
        .map(|p| vec3(p.x, p.y + 1.0, p.z))
        .collect();

    obstacles.extend(
        obstacles
            .iter()
            .map(|p| vec3(p.x, p.y + 1.0, p.z))
            .collect::<Vec<Vec3>>(),
    );

    lvl.points_are_empty(&obstacles)
}

#[allow(dead_code)]
fn no_y_obstacles(x: f32, y: f32, z: f32, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
    let obstacles: Vec<Vec3> = get_touched_points(x, y, z, phys);

    lvl.points_are_empty(&obstacles)
}

#[allow(dead_code)]
pub fn has_y_obstacles_on_point(
    x: f32,
    y: f32,
    z: f32,
    lvl: &Res<Level>,
    phys: &PhysiologyDescription,
) -> bool {
    let obstacles: Vec<Vec3> = get_touched_points(x, y, z, phys);

    !lvl.points_are_empty(&obstacles)
}

fn get_touched_points(x: f32, y: f32, z: f32, phys: &PhysiologyDescription) -> Vec<Vec3> {
    let model_radius = phys.model_radius;
    let mut points: Vec<Vec3> = vec![vec3(x.floor(), y, z), vec3(x, y, z.floor())];

    let x_gap = round_based(x - x.floor(), 1);
    if x_gap > model_radius {
        points.push(vec3((x + model_radius).floor(), y, z.floor()));
    } else if x_gap < model_radius {
        points.push(vec3((x - model_radius).floor(), y, z.floor()));
    };

    let z_gap = round_based(z - z.floor(), 1);
    if z_gap > model_radius {
        points.push(vec3(x.floor(), y, (z + model_radius).floor()));
    } else if z_gap < model_radius {
        points.push(vec3(x.floor(), y, (z - model_radius).floor()));
    };

    if x_gap > model_radius && z_gap > model_radius {
        points.push(vec3(
            (x + model_radius).floor(),
            y,
            (z + model_radius).floor(),
        ));
    } else if x_gap < model_radius && z_gap < model_radius {
        points.push(vec3(
            (x - model_radius).floor(),
            y,
            (z - model_radius).floor(),
        ));
    } else if x_gap > model_radius && z_gap < model_radius {
        points.push(vec3(
            (x + model_radius).floor(),
            y,
            (z - model_radius).floor(),
        ));
    } else if x_gap < model_radius && z_gap > model_radius {
        points.push(vec3(
            (x - model_radius).floor(),
            y,
            (z + model_radius).floor(),
        ));
    }

    points
}
