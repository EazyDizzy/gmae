use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::math::vec3;
use bevy::prelude::*;
use heron::AxisAngle;

use crate::player::entity::Player;
use crate::Velocity;

pub fn player_track_keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &PhysiologyDescription, With<Player>)>,
) {
    let (mut velocity, phys, ..) = if let Some(a) = player_query.iter_mut().next() {
        a
    } else {
        return;
    };
    // TODO move to game settings and allow rebinding
    let mut move_buttons = 0;
    let mut x_velocity = 0.;
    let mut z_velocity = 0.;
    keyboard_input.get_pressed().for_each(|k| {
        let move_angular = match k {
            KeyCode::Up => Some((1.0, 1.0)),
            KeyCode::Down => Some((-1.0, -1.0)),
            KeyCode::Left => Some((1.0, -1.0)),
            KeyCode::Right => Some((-1.0, 1.0)),
            _ => None,
        };
        if let Some((x, z)) = move_angular {
            move_buttons += 1;
            x_velocity += x;
            z_velocity += z;
        }
    });

    if move_buttons > 0 {
        // TODO add jumping
        // TODO no overwrite to not block physics engine
        *velocity = velocity.with_linear(vec3(
            x_velocity / move_buttons as f32 * phys.movement_speed,
            velocity.linear.y,
            z_velocity / move_buttons as f32 * phys.movement_speed,
        ));
    }

    if move_buttons == 0
        && keyboard_input.any_just_released([
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Right,
            KeyCode::Left,
        ])
    {
        *velocity = velocity.with_linear(vec3(0., 0., 0.));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {}
}
