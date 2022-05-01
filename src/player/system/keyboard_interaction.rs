use bevy::prelude::*;
use lib::entity::level::Level;

use crate::player::entity::Player;
use crate::player::system::camera::PlayerCamera;

pub fn keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    lvl: Res<Level>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    camera_query: Query<&PlayerCamera>,
) {
    let (mut player, mut transform) = player_query.iter_mut().next().unwrap();
    let angle = camera_query.iter().next().map_or(0.0, PlayerCamera::angle);
    keyboard_input.get_pressed()
        .for_each(|k| {
            match k {
                KeyCode::Up => player.move_forward(&lvl, angle),
                KeyCode::Down => player.move_back(&lvl, angle),
                KeyCode::Right => player.move_right(&lvl, angle),
                KeyCode::Left => player.move_left(&lvl, angle),
                KeyCode::Space | KeyCode::Apostrophe => player.jump(&lvl),
                _ => {}
            }
        });

    player.gravity_move(&lvl);
    player.move_model(&mut transform);

    if keyboard_input.just_pressed(KeyCode::Space) {}
}