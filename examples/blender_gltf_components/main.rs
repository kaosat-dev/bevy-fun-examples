use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher};

mod models;
use models::*;

mod camera;
use camera::*;

fn main(){
    App::new()
    .add_plugins((
        DefaultPlugins.set(
            AssetPlugin {
                // This tells the AssetServer to watch for changes to assets.
                // It enables our scenes to automatically reload in game when we modify their files.
                // practical in our case to be able to edit the shaders without needing to recompile
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(50)),
                ..default()
            }
        ),
        ModelsPlugin,
        CameraPlugin
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, (
        stuff
    ))
    .run();
}


fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/level1.glb#Scene0"),
        ..default()
    });
}


fn stuff(){
    
}