use crate::creature::component::moving::Moving;
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use std::f32::consts::{FRAC_PI_2, PI};

use crate::player::entity::Player;
use crate::player::system::camera::PlayerCamera;

pub fn keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    lvl: Res<Level>,
    mut player_query: Query<(
        &mut Transform,
        &mut Moving,
        &PhysiologyDescription,
        With<Player>,
    )>,
    camera_query: Query<&PlayerCamera>,
) {
    let (mut transform, mut moving, phys, ..) = player_query.iter_mut().next().unwrap();
    let angle = camera_query.iter().next().map_or(0.0, PlayerCamera::angle);
    // TODO move to game settings and allow rebinding
    keyboard_input.get_pressed().for_each(|k| {
        let future_position = match k {
            KeyCode::Up => Some(translate_forward(&moving, angle, phys.movement_speed)),
            KeyCode::Down => Some(translate_back(&moving, angle, phys.movement_speed)),
            KeyCode::Right => Some(translate_right(&moving, angle, phys.movement_speed)),
            KeyCode::Left => Some(translate_left(&moving, angle, phys.movement_speed)),
            _ => None,
        };
        if let Some((future_x, future_z)) = future_position {
            moving.go_to(future_x, future_z, &lvl, phys);
        }

        if *k == KeyCode::Space || *k == KeyCode::Apostrophe {
            moving.jump(&lvl, phys);
        }
    });

    moving.gravity_move(&lvl, phys);
    // TODO get rid of this line at all
    transform.translation = vec3(
        moving.position().x,
        moving.position().y + 1.0,
        moving.position().z,
    );

    if keyboard_input.just_pressed(KeyCode::Space) {}
}

fn translate_forward(moving: &Moving, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
    let future_x = moving.position().x + x_modifier * movement_speed;
    let future_z = moving.position().z + z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_back(moving: &Moving, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
    let future_x = moving.position().x - x_modifier * movement_speed;
    let future_z = moving.position().z - z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_left(moving: &Moving, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
    let future_x = moving.position().x + x_modifier * movement_speed;
    let future_z = moving.position().z + z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_right(moving: &Moving, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
    let future_x = moving.position().x - x_modifier * movement_speed;
    let future_z = moving.position().z - z_modifier * movement_speed;

    (future_x, future_z)
}

#[derive(Copy, Clone, Debug)]
enum PiePiece {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

fn angle_to_pie_piece(angle: f32) -> (f32, PiePiece) {
    if angle <= FRAC_PI_2 {
        (angle, PiePiece::BottomLeft)
    } else if angle <= PI {
        (angle - FRAC_PI_2, PiePiece::TopLeft)
    } else if angle <= PI + FRAC_PI_2 {
        (angle - PI, PiePiece::TopRight)
    } else {
        (angle - (PI + FRAC_PI_2), PiePiece::BottomRight)
    }
}

fn angle_to_forward_x_z_modifiers(angle: f32) -> (f32, f32) {
    let (diff, piece) = angle_to_pie_piece(angle);

    match piece {
        PiePiece::TopLeft => {
            let x = diff / FRAC_PI_2;
            (x, -(1.0 - x))
        }
        PiePiece::TopRight => {
            let z = diff / FRAC_PI_2;
            ((1.0 - z), z)
        }
        PiePiece::BottomRight => {
            let x = diff / FRAC_PI_2;
            (-x, (1.0 - x))
        }
        PiePiece::BottomLeft => {
            let z = diff / FRAC_PI_2;
            (-(1.0 - z), -z)
        }
    }
}

fn angle_to_left_x_z_modifiers(angle: f32) -> (f32, f32) {
    let (diff, piece) = angle_to_pie_piece(angle);

    match piece {
        PiePiece::TopLeft => {
            let z = diff / FRAC_PI_2;
            (-(1.0 - z), -z)
        }
        PiePiece::TopRight => {
            let x = diff / FRAC_PI_2;
            (x, -(1.0 - x))
        }
        PiePiece::BottomRight => {
            let z = diff / FRAC_PI_2;
            ((1.0 - z), z)
        }
        PiePiece::BottomLeft => {
            let x = diff / FRAC_PI_2;
            (-x, (1.0 - x))
        }
    }
}
