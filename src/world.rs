use crate::block::{Block, BlockType};

use bevy::prelude::*;

/// Sets up the world with all the blocks
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // create block mesh and material

    let cube_mesh = meshes.add(Mesh::from(Cuboid::default()));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.5, 0.5, 1.0),
        ..default()
    });

    // spawn blocks

    for j in 0..16 {
        for i in 0..16 {
            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: cube_material.clone(),
                    transform: Transform {
                        scale: Vec3::new(32.0, 32.0, 32.0),
                        translation: Vec3::new((32.0 * i as f32) - 256.0, 0.0, -32.0 * j as f32),
                        ..default()
                    },
                    ..default()
                },
                Block {
                    block_type: BlockType::Solid,
                },
            ));
        }
    }

}
