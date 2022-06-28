use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::math::vec3;
use bevy::prelude::*;
use lib::entity::level::Level;
use std::f32::consts::{FRAC_PI_2, PI};
use crate::creature::component::movement::locomotivity::Locomotivity;

use crate::player::entity::Player;
use crate::player::system::camera::PlayerCamera;

pub fn player_track_keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    lvl: Res<Level>,
    mut player_query: Query<(
        &mut Transform,
        &mut Locomotivity,
        &PhysiologyDescription,
        With<Player>,
    )>,
    camera_query: Query<&PlayerCamera>,
) {
    let (mut transform, mut locomotivity, phys, ..) = player_query.iter_mut().next().unwrap();
    let angle = camera_query.iter().next().map_or(0.0, PlayerCamera::angle);
    // TODO move to game settings and allow rebinding
    keyboard_input.get_pressed().for_each(|k| {
        // TODO sprint
        let future_position = match k {
            KeyCode::Up => Some(translate_forward(&locomotivity, angle, phys.movement_speed)),
            KeyCode::Down => Some(translate_back(&locomotivity, angle, phys.movement_speed)),
            KeyCode::Right => Some(translate_right(&locomotivity, angle, phys.movement_speed)),
            KeyCode::Left => Some(translate_left(&locomotivity, angle, phys.movement_speed)),
            _ => None,
        };
        if let Some((future_x, future_z)) = future_position {
            locomotivity.move_to(future_x, future_z, &lvl, phys);
        }

        if *k == KeyCode::Space || *k == KeyCode::Apostrophe {
            locomotivity.jump(&lvl, phys);
        }
    });

    // TODO get rid of this line at all
    transform.translation = vec3(
        locomotivity.position().x,
        locomotivity.position().y,
        locomotivity.position().z,
    );

    if keyboard_input.just_pressed(KeyCode::Space) {}
}

fn translate_forward(locomotivity: &Locomotivity, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
    let future_x = locomotivity.position().x + x_modifier * movement_speed;
    let future_z = locomotivity.position().z + z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_back(locomotivity: &Locomotivity, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_forward_x_z_modifiers(angle);
    let future_x = locomotivity.position().x - x_modifier * movement_speed;
    let future_z = locomotivity.position().z - z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_left(locomotivity: &Locomotivity, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
    let future_x = locomotivity.position().x + x_modifier * movement_speed;
    let future_z = locomotivity.position().z + z_modifier * movement_speed;

    (future_x, future_z)
}
fn translate_right(locomotivity: &Locomotivity, angle: f32, movement_speed: f32) -> (f32, f32) {
    let (x_modifier, z_modifier) = angle_to_left_x_z_modifiers(angle);
    let future_x = locomotivity.position().x - x_modifier * movement_speed;
    let future_z = locomotivity.position().z - z_modifier * movement_speed;

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
