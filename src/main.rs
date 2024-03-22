use asset_loader::AssetLoaderPlugin;
use bevy::{
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};
use camera::CameraPlugin;
use game::GamePlugin;

mod asset_loader;
mod camera;
mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space shooter"),
                        resolution: WindowResolution::new(1250.0, 750.0),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            minimize: false,
                            close: true,
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GamePlugin)
        .run();
}

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
