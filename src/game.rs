use bevy::prelude::*;

use crate::asset_loader::ImageAssets;

use self::{
    laser::{despawn_laser, laser_control, move_laser},
    meteor::{despawn_meteor, move_meteor, spawn_meteor},
    player::{move_player, Spaceship},
};

mod laser;
mod meteor;
mod player;

#[derive(Component, Default)]
pub struct Meteor {
    position: Vec3,
}

#[derive(Component)]
pub struct SpaceshipLaser;

#[derive(Resource)]
struct MeteorSpawnTimer {
    timer: Timer,
}

#[derive(Resource, Default)]
pub struct BaseHP {
    hp: usize,
}

#[derive(Component)]
struct BaseHPText;

const PLAYER_MOVING_SPEED: f32 = 400.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MeteorSpawnTimer {
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        })
        .insert_resource(BaseHP { hp: 100 })
        .add_systems(Startup, (spawn_player, set_ui))
        .add_systems(
            Update,
            (
                move_player,
                spawn_meteor,
                despawn_meteor,
                move_meteor,
                laser_control,
                move_laser,
                despawn_laser,
                update_score_text,
            ),
        );
    }
}

fn set_ui(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        },))
        .with_children(|node| {
            node.spawn((
                BaseHPText,
                TextBundle::from_section(
                    "Base HP:100",
                    TextStyle {
                        font_size: 80.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            ));
        });
}

fn spawn_player(mut commands: Commands, assets: Res<ImageAssets>) {
    commands.spawn((
        Spaceship::default(),
        SpriteBundle {
            texture: assets.spaceship.clone(),
            ..default()
        },
    ));
}

fn update_score_text(mut query: Query<&mut Text, With<BaseHPText>>, score: Res<BaseHP>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        text.sections[0].value = format!("Base HP:{}", score.hp.to_string());
    }
}
