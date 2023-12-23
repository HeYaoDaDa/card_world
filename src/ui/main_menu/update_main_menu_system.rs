use bevy::app::AppExit;
use bevy::prelude::*;

use super::MainMenuButtion;

pub fn update_main_menu_system(
    mut exit_event: EventWriter<AppExit>,
    buttion_query: Query<(&Interaction, &MainMenuButtion), Changed<Interaction>>,
) {
    for (interaction, button) in buttion_query.iter() {
        if let Interaction::Pressed = *interaction {
            match button {
                MainMenuButtion::ExitGameButton => exit_event.send(AppExit),
            }
        }
    }
}
