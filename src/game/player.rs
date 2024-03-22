use bevy::prelude::*;

use super::PLAYER_MOVING_SPEED;

#[derive(Component, Default)]
pub struct Spaceship {
    position: Vec3,
}

pub(super) fn move_player(
    mut player: Query<(&mut Transform, &mut Spaceship), With<Spaceship>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut spaceship) in &mut player {
        if keys.pressed(KeyCode::KeyW) {
            spaceship.position.y += PLAYER_MOVING_SPEED * time.delta_seconds();
        } else if keys.pressed(KeyCode::KeyS) {
            spaceship.position.y -= PLAYER_MOVING_SPEED * time.delta_seconds();
        }

        if keys.pressed(KeyCode::KeyA) {
            spaceship.position.x -= PLAYER_MOVING_SPEED * time.delta_seconds();
        } else if keys.pressed(KeyCode::KeyD) {
            spaceship.position.x += PLAYER_MOVING_SPEED * time.delta_seconds();
        }

        transform.translation = spaceship.position;
    }
}
