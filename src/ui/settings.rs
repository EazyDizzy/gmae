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

pub fn update_mouse_sensitivity<'settings>(game_settings: &'settings mut ResMut<GameSettings>) -> Box<dyn FnMut(Option<f64>) -> f64 +'settings> {
   Box::new(move |new_value: Option<f64>| -> f64 {
      match new_value {
         None => {}
         Some(new_value) => {
            game_settings.update_mouse_sensitivity(new_value);
         }
      }

      game_settings.get_mouse_sensitivity()
   })
}