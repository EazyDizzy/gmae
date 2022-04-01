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
    if keyboard_input.just_pressed(KeyCode::Up) {
        player.move_forward();
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        player.move_back();
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        player.move_right();
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        player.move_left();
    }

    player.move_model(transforms);

    // let music = asset_server.load("sounds/heartbeat.wav");
    // audio.play(music);
    //

    if keyboard_input.just_pressed(KeyCode::Space) {}
}