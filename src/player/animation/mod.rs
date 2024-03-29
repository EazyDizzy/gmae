use crate::util::math::round;
use bevy::prelude::*;
use heron::Velocity;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct PlayerAnimations {
    run: Handle<AnimationClip>,
}

pub fn player_animation_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
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

pub fn animation_rotate_model_on_move(mut creatures: Query<(&Velocity, &mut Transform)>) {
    creatures.iter_mut().for_each(|(velocity, mut transform)| {
        if velocity.linear.x == 0. && velocity.linear.z == 0. {
            return;
        }

        let current_rotation: Quat = transform.rotation;
        let (_, mut y, _) = current_rotation.to_euler(EulerRot::XYZ);
        y = round(y, 2);
        let future_y: f32 = round(velocity.linear.x.atan2(velocity.linear.z), 2);

        if future_y != y {
            // let step_size = 0.1;
            // let mut y_step = if y < -FRAC_PI_2 && future_y > FRAC_PI_2 {
            //     -step_size
            // } else if future_y < -FRAC_PI_2 && y > FRAC_PI_2 {
            //     step_size
            // } else {
            //     (future_y - y).clamp(-step_size, step_size)
            // };
            // TODO catch bug & make smooth rotation
            // if !(-FRAC_PI_2..=FRAC_PI_2).contains(&future_y) {
            //     // println!("old {} -> {} ({})", y, future_y, y_step);
            //     y_step = future_y - y;
            // }
            // y_step = future_y - y;
            // println!("{} -> {} ({})", y, future_y, y_step);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., future_y, 0.);
        }
    });
}
