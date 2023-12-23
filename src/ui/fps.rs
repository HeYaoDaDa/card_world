use bevy::prelude::*;

#[derive(Component)]
struct FpsTextComp;

pub struct ShowFpsPlugin;

impl Plugin for ShowFpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

fn spawn_fps_text(mut commands: Commands) {
    commands.spawn((
        Name::new("FpsText"),
        TextBundle::default().with_background_color(Color::rgba(0.0, 0.5, 0.0, 0.2)),
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
