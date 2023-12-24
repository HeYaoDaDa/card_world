use bevy::prelude::*;
use bevy::window::PresentMode;

use super::OptionsInput;

pub fn update_options_system(
    input_query: Query<(&Interaction, &OptionsInput), Changed<Interaction>>,
    mut window_query: Query<&mut Window>,
) {
    for (interaction, input) in input_query.iter() {
        if let Interaction::Pressed = *interaction {
            let mut window = window_query.single_mut();
            match input {
                OptionsInput::VSync => {
                    if matches!(window.present_mode, PresentMode::AutoNoVsync) {
                        window.present_mode = PresentMode::AutoVsync;
                    } else {
                        window.present_mode = PresentMode::AutoNoVsync;
                    }
                }
            }
        }
    }
}
