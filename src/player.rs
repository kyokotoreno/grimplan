use bevy::{prelude::*, input::mouse::MouseMotion};

#[derive(Component)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub inventory: Vec<String>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            level: 1,
            inventory: vec!["sword".into(), "shield".into()],
        }
    }
}

fn setup_player(mut commands: Commands) {
    // spawn camera with player

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 60.,
                ..default()
            }),
            ..default()
        },
        Player::default()
    ));
}

fn update_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut evr_motion: EventReader<MouseMotion>,
    time: Res<Time>
) {
    let mut transform = player_query.single_mut();

    let speed = 100. * time.delta_seconds();

    for keycode in keyboard.get_pressed() {
        match keycode {
            KeyCode::KeyW => {
                transform.translation.z -= speed;
            },
            KeyCode::KeyA => {
                transform.translation.x -= speed;
            },
            KeyCode::KeyS => {
                transform.translation.z += speed;
            },
            KeyCode::KeyD => {
                transform.translation.x += speed;
            },
            KeyCode::KeyC => {
                transform.translation.y -= speed;
            },
            KeyCode::Space => {
                transform.translation.y += speed;
            },
            _ => {}
        }
    }

    let mut total_motion: Vec2 = evr_motion.read()
        .map(|ev| ev.delta).sum();

    total_motion.y = -total_motion.y;

    /*
    for mouse_input in mouse_event.read() {
        //transform.rotate_x(mouse_input.delta.y * 0.01);
        transform.rotate_y(-1.0 * mouse_input.delta.x * time.delta_seconds());
    }
    */
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}
