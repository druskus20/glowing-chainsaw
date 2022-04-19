use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

// Consider using
// https://github.com/NiklasEi/bevy_asset_loader#:~:text=README.md-,Bevy%20asset%20loader,The%20trait%20can%20be%20derived.

use lib::camera::CameraPlugin;
use lib::game_state::GameStatePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game".to_string(),
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GameStatePlugin::new())
        .add_plugin(CameraPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn_bundle(SpriteBundle {
    //    transform: Transform::from_xyz(100., 0., 0.).with_scale(Vec3::new(3., 3., 3.)),
    //    texture: asset_server.load("oak.png"),
    //    ..default()
    //});
}
