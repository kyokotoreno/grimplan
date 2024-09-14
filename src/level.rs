use std::f32::consts::PI;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_level);
    }
}

/// Sets up the world with all the blocks
fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/level.glb#Scene0"),
        ..default()
    });

    commands.spawn(SpotLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 50., 0.)).looking_at(Vec3::ZERO, Vec3::Y),
        spot_light: SpotLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.,
            shadow_depth_bias: 0.2,
            inner_angle: PI / 4. * 7.,
            outer_angle: PI / 4. * 8.,
            ..default()
        },
        ..default()
    });
}
