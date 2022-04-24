use bevy::prelude::*;
use lib::entity::level::Level;

use crate::player::entity::Player;

pub fn keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    lvl: Res<Level>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = player_query.iter_mut().next().unwrap();
    keyboard_input.get_pressed()
        .for_each(|k| {
            match k {
                KeyCode::Up => player.move_forward(&lvl),
                KeyCode::Down => player.move_back(&lvl),
                KeyCode::Right => player.move_right(&lvl),
                KeyCode::Left => player.move_left(&lvl),
                KeyCode::Space | KeyCode::Apostrophe => player.jump(&lvl),
                _ => {}
            }
        });

    player.gravity_move(&lvl);
    player.move_model(&mut transform);

    if keyboard_input.just_pressed(KeyCode::Space) {}
}