use bevy::prelude::*;

use super::MainMenuButtion;

pub fn show_main_menu_system(mut commands: Commands) {
    commands
        .spawn((
            Name::new("MainMenuRoot"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor::from(Color::GRAY),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("ExitGameButton"),
                TextBundle {
                    text: Text::from_section(
                        "Exit",
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    background_color: BackgroundColor::from(Color::BLACK),
                    ..default()
                },
                MainMenuButtion::ExitGameButton,
                Interaction::default(),
            ));
        });
}
