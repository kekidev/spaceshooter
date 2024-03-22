use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::asset_loader::ImageAssets;

use super::{BaseHP, Meteor, MeteorSpawnTimer};

const SPAWN_RANGE_X: Range<f32> = -625.0..625.0;
const SPAWN_RANGE_Y: Range<f32> = 150.0..250.0;

pub(super) fn spawn_meteor(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut spawn_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        rng.gen_range(SPAWN_RANGE_Y),
        0.0,
    );

    commands.spawn((
        Meteor {
            position: translation,
        },
        SpriteBundle {
            texture: image_assets.meteor.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    ));
}

pub(super) fn despawn_meteor(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Meteor>>,
    mut basehp: ResMut<BaseHP>,
) {
    for (entity, transform) in &query {
        if basehp.hp == 0 {
            commands.entity(entity).despawn_recursive();
            basehp.hp = 100;
            return;
        }
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn_recursive();
            basehp.hp -= 10;
        }
    }
}

pub(super) fn move_meteor(
    mut meteor: Query<(&mut Transform, &Meteor), With<Meteor>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut meteor {
        transform.translation.y -= 50.0 * time.delta_seconds();
    }
}
