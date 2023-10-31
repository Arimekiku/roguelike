use bevy::{prelude::*, render::camera::ScalingMode};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Setup camera
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.,
        min_height: 144.,
    };

    commands.spawn(camera);

    //Setup player
    let mut player = (
        SpriteBundle::default(),
        Player {
            movement_speed: 20.,
        },
    );

    player.0.sprite.custom_size = Some(Vec2::new(16., 16.));
    player.0.transform = Transform::from_xyz(50., 50., 0.);

    commands.spawn(player);
}

fn update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut characters: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.movement_speed * time.delta_seconds();
        let mut vector2_movement = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::D) {
            vector2_movement.x += movement_amount;
        }

        if keyboard_input.pressed(KeyCode::A) {
            vector2_movement.x -= movement_amount;
        }

        if keyboard_input.pressed(KeyCode::W) {
            vector2_movement.y += movement_amount;
        }

        if keyboard_input.pressed(KeyCode::S) {
            vector2_movement.y -= movement_amount;
        }

        if vector2_movement.length() != 0. {
            transform.translation += vector2_movement.normalize();
        }
    }
}
