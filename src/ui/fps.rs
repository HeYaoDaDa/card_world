use bevy::prelude::*;

#[derive(Component)]
struct FpsTextComp;

pub struct ShowFpsPlugin;

impl Plugin for ShowFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

fn show_fps_text(mut commands: Commands) {
    commands.spawn((
        Name::new("FpsText"),
        TextBundle {
            background_color: BackgroundColor::from(Color::rgba(0.0, 0.5, 0.0, 0.2)),
            z_index: ZIndex::Global(1000),
            ..default()
        },
        FpsTextComp,
    ));
}

fn update_fps_text(
    mut commands: Commands,
    time: Res<Time<Real>>,
    text_query: Query<Entity, With<FpsTextComp>>,
) {
    let delta_sec = time.delta_seconds_f64();
    let mut fps = 0.0;
    if delta_sec != 0.0 {
        fps = 1.0 / delta_sec;
    }
    commands
        .entity(text_query.single())
        .insert(Text::from_section(
            format!("FPS: {:.0}", fps),
            TextStyle::default(),
        ));
}
