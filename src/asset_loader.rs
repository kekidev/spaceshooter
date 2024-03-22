use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct ImageAssets {
    pub meteor: Handle<Image>,
    pub spaceship: Handle<Image>,
    pub laser: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        spaceship: asset_server.load("sprites/redship.png"),
        laser: asset_server.load("sprites/laser.png"),
        meteor: asset_server.load("sprites/meteor.png"),
    }
}
