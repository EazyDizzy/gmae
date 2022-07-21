use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerAnimations {
    run: Handle<AnimationClip>,
}

pub fn animation_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PlayerAnimations {
        run: asset_server.load("mesh/player.glb#Animation0"),
    });
}

pub fn animation_run_on_move(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let is_moving =
        keyboard_input.any_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]);

    if is_moving {
        for mut player in players.iter_mut() {
            if player.is_paused() {
                player.play(animations.run.clone()).repeat();
            }
        }
    } else {
        for mut player in players.iter_mut() {
            player.pause();
        }
    }
}
