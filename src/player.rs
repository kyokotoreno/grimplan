use bevy::{prelude::*, input::mouse::MouseMotion};

#[derive(Component, Debug)]
struct Player {
    name: String
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".into()
        }
    }
}

fn setup_player(mut commands: Commands) {
    // spawn camera for player

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 45.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 32.0, 0.0)
            .looking_at(Vec3::new(0.0, 32.0, -1.0), Vec3::Y),
            ..default()
        },
        Player::default()
    ));
}

fn update_player(
    mut query: Query<&mut Transform, With<Player>>,
    mut mouse_event: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = query.single_mut();

    for pressed in keyboard_input.get_pressed() {
        match pressed {
            KeyCode::KeyW => {
                transform.translation.x += 1.0;
            },
            KeyCode::KeyA => {
                transform.translation.z -= 1.0;
            },
            KeyCode::KeyS => {
                transform.translation.x -= 1.0;
            },
            KeyCode::KeyD => {
                transform.translation.z += 1.0;
            },
            KeyCode::Space => {
                transform.translation.y += 1.0;
            },
            KeyCode::ShiftLeft => {
                transform.translation.y -= 1.0;
            },
            _ => {}
        }
    }

    for mouse_input in mouse_event.read() {
        //transform.rotate_x(mouse_input.delta.y * 0.01);
        transform.rotate_y(-1.0 * mouse_input.delta.x * time.delta_seconds());
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}
