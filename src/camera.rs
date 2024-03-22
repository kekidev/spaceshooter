use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {   
    commands.spawn(Camera2dBundle::default());
}