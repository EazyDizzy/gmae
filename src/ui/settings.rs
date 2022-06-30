use lib::util::game_settings::GameSettings;
use crate::ResMut;

pub fn update_background_music<'settings>(game_settings: &'settings mut ResMut<GameSettings>) -> Box<dyn FnMut(Option<f64>) -> f64 +'settings> {
   Box::new(move |new_value: Option<f64>| -> f64 {
      match new_value {
         None => {}
         Some(new_value) => {
            game_settings.update_background_music_volume(new_value);
         }
      }

      game_settings.get_background_music_volume()
   })
}

pub fn update_player_camera_sensitivity<'settings>(mut game_settings: ResMut<'settings, GameSettings>) -> Box<dyn FnMut(Option<f64>) -> f64 +'settings> {
   Box::new(move |new_value: Option<f64>| -> f64 {
      match new_value {
         None => {}
         Some(new_value) => {
            game_settings.update_player_camera_sensitivity(new_value);
         }
      }

      game_settings.get_player_camera_sensitivity()
   })
}