use bevy::{prelude::*, window::PresentMode};

use super::{OptionsBackComp, OptionsComp, OptionsInput};

pub fn show_options_system(mut commands: Commands, window_query: Query<&Window>) {
    commands
        .spawn((
            Name::new("OptionsRoot"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::percent(50.0), GridTrack::percent(50.0)],
                    ..default()
                },
                background_color: BackgroundColor::from(Color::GRAY),
                ..default()
            },
            OptionsComp,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "VSync:",
                    TextStyle {
                        font_size: 24.0,
                        ..default()
                    },
                ),
                background_color: BackgroundColor::from(Color::BLACK),
                ..default()
            });
            let present_mode = window_query.single().present_mode;
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        if matches!(present_mode, PresentMode::AutoNoVsync) {
                            "Yes"
                        } else {
                            "No"
                        },
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    background_color: BackgroundColor::from(Color::BLACK),
                    ..default()
                },
                OptionsInput::VSync,
                Interaction::default(),
            ));
            // back
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Back",
                        TextStyle {
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    background_color: BackgroundColor::from(Color::BLACK),
                    ..default()
                },
                OptionsBackComp,
                Interaction::default(),
            ));
        });
}
