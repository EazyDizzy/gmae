use bevy::prelude::*;

use crate::player::entity::Player;

pub fn keyboard_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<Player>,
    mut transforms: Query<&mut Transform>,
    audio: Res<Audio>,
) {
    keyboard_input.get_pressed()
        .for_each(|k| {
            match k {
                KeyCode::Up => player.move_forward(),
                KeyCode::Down => player.move_back(),
                KeyCode::Right => player.move_right(),
                KeyCode::Left => player.move_left(),
                _ => {}
            }
        });

    player.move_model(transforms);

    // let music = asset_server.load("sounds/heartbeat.wav");
    // audio.play(music);
    //

    if keyboard_input.just_pressed(KeyCode::Space) {}
}