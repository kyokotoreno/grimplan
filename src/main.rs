mod block;
mod world;

use bevy::{prelude::*, input::mouse::MouseMotion};

#[derive(Component)]
struct Player;

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
        Player
    ));
}

fn update_player(
    mut query: Query<&mut Transform, With<Player>>,
    mut mouse_event: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut transform = query.single_mut();

    let speed = 100.0 * time.delta_seconds();

    for pressed in keyboard_input.get_pressed() {
        match pressed {
            KeyCode::KeyW => {
                transform.translation.x += speed;
            },
            KeyCode::KeyA => {
                transform.translation.z -= speed;
            },
            KeyCode::KeyS => {
                transform.translation.x -= speed;
            },
            KeyCode::KeyD => {
                transform.translation.z += speed;
            },
            KeyCode::Space => {
                transform.translation.y += speed;
            },
            KeyCode::ShiftLeft => {
                transform.translation.y -= speed;
            },
            _ => {}
        }
    }

    for mouse_input in mouse_event.read() {
        let speed = 2.5 * time.delta_seconds();

        let yaw = -speed * mouse_input.delta.x;
        let pitch = -speed * mouse_input.delta.y;

        transform.rotate_y(yaw.to_radians());
        transform.rotate_local_x(pitch.to_radians());

        /*
        transform.rotate(Quat::from_euler(
            EulerRot::YXZ,
            yaw.to_radians(), 
            pitch.to_radians(),
            0.0
        ));

        let rot = transform.rotation;
        transform.rotation = Quat::from_xyzw(
            rot.x.clamp(-45.0, 45.0),
        );
        */
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    name: Some("Grimplan".to_string()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, (world::setup_world, setup_player))
        .add_systems(Update, update_player)
        .run();
}
