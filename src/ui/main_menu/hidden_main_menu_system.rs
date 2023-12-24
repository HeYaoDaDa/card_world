use bevy::prelude::*;

use super::MainMenuComp;

pub fn hidden_main_menu_system(
    mut commands: Commands,
    main_menu: Query<Entity, With<MainMenuComp>>,
) {
    commands.entity(main_menu.single()).despawn_recursive();
}
