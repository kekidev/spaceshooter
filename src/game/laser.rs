use bevy::prelude::*;

use crate::asset_loader::ImageAssets;

use super::{player::Spaceship, SpaceshipLaser};

pub(super) fn move_laser(mut query: Query<(&mut Transform, &SpaceshipLaser)>, time: Res<Time>) {
    for (mut transform, _) in &mut query {
        transform.translation.y += 500.0 * time.delta_seconds();
    }
}

pub(super) fn despawn_laser(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<SpaceshipLaser>>,
) {
    for (entity, transform) in &query {
        if transform.translation.y > 300.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub(super) fn laser_control(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    image_assets: Res<ImageAssets>,
) {
    let transform = query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            SpaceshipLaser,
            SpriteBundle {
                texture: image_assets.laser.clone(),
                transform: Transform::from_translation(
                    transform.translation + transform.forward() * 50.0,
                ),
                ..default()
            },
        ));
    }
}
