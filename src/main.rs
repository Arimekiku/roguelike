pub mod components;

use bevy::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_test)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Name("Player".to_string()), Position { x: 0., y: 0. }));
    commands.spawn((Name("Enemy".to_string()), Position { x: 100., y: 0. }));
}

fn update_test(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut person_position: Query<(&mut Position, &Name)>,
) {
    for (mut position, name) in &mut person_position {
        if keyboard_input.pressed(KeyCode::D) {
            position.x -= 150. * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::A) {
            position.x += 150. * time.delta_seconds();
        }

        println!("{0:?} position: {1}", name, position.x);
    }
}
