mod menus;
mod player;
mod level;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_editor_pls::prelude::*;

use player::PlayerPlugin;
use level::LevelPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some("Grimplan".into()),
                    ..default()
                }),
                ..default()
            }),
            EditorPlugin::default(),
            EguiPlugin,
            PlayerPlugin,
            LevelPlugin,
        ))
        .add_systems(Update, gizmos_test)
        .run();
}

fn gizmos_test(mut gizmos: Gizmos) {
    use std::f32::consts::PI;
    use bevy::color::palettes::tailwind;

    gizmos.grid(
        Vec3::ZERO,
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(20),
        Vec2::new(2., 2.),
        tailwind::GRAY_500,
    );
}
