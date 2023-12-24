use bevy::app::AppExit;
use bevy::prelude::*;

use crate::MainMenuState;

use super::MainMenuButtion;

pub fn update_main_menu_system(
    mut exit_event: EventWriter<AppExit>,
    buttion_query: Query<(&Interaction, &MainMenuButtion), Changed<Interaction>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (interaction, button) in buttion_query.iter() {
        if let Interaction::Pressed = *interaction {
            match button {
                MainMenuButtion::OptionsButtion => next_menu_state.set(MainMenuState::Options),
                MainMenuButtion::ExitGameButton => exit_event.send(AppExit),
            }
        }
    }
}
