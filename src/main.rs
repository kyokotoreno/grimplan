mod menus;
mod player;
mod world;

use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_egui::EguiPlugin;

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
            EguiPlugin
        ))
        .run();
}
