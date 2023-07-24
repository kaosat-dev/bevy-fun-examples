use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher, gltf::Gltf};
use bevy_editor_pls::prelude::*;

mod models;
use models::*;

mod camera;
use camera::*;



#[derive(Component, Reflect, Default, Debug, )]
#[reflect(Component)]
/// Demo marker component
pub struct Player;


#[derive(Component, Reflect, Default, Debug, )]
#[reflect(Component)]
/// Demo marker component
pub struct LoadedMarker;


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Running,
}


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
        // editor
        EditorPlugin::default(),
        ModelsPlugin,
        CameraPlugin
    ))
    .register_type::<Player>()

    .add_state::<AppState>()
    .add_systems(Startup, setup)
    .add_systems(Update, (
        spawn_level.run_if(in_state(AppState::Loading)),
        player_move_demo.run_if(in_state(AppState::Running))
    ))
    .run();
}



#[derive(Resource)]
struct AssetLoadHack(Handle<Scene>);
// we preload the data here, but this is for DEMO PURPOSES ONLY !! Please use https://github.com/NiklasEi/bevy_asset_loader or a similar logic to seperate loading / pre processing 
// of assets from the spawning
// AssetLoadHack is also just for the same purpose, you do not need it in a real scenario
// the states here are also for demo purposes only, 
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let tmp: Handle<Scene>  = asset_server.load("models/level1.glb#Scene0");
    commands.insert_resource(AssetLoadHack(tmp));
}

fn spawn_level(
    mut commands: Commands,
    scene_markers: Query<&LoadedMarker>,
    preloaded_scene: Res<AssetLoadHack>,

    mut asset_event_reader: EventReader<AssetEvent<Gltf>>,
    mut next_state: ResMut<NextState<AppState>>,
){

    if let Some(asset_event) = asset_event_reader.iter().next() {
        match asset_event {
            AssetEvent::Created { handle: _ } => {
                info!("GLTF loaded");
                if scene_markers.is_empty() {
                    info!("spawning scene");
                    commands.spawn(
                        (
                            SceneBundle {
                                scene: preloaded_scene.0.clone(),
                                ..default()
                            },
                            LoadedMarker
                        )
                    );
                    next_state.set(AppState::Running);
                }
            }
            _ => ()
        }
    }

   
   
}


fn player_move_demo(
    keycode: Res<Input<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
){

    let speed = 0.2;
    if let Ok(mut player) = players.get_single_mut() {
        if keycode.pressed(KeyCode::Left) {
            player.translation.x += speed;
        }
        if keycode.pressed(KeyCode::Right) {
            player.translation.x -= speed;
        }

        if keycode.pressed(KeyCode::Up) {
            player.translation.z += speed;
        }
        if keycode.pressed(KeyCode::Down) {
            player.translation.z -= speed;
        }
    }
}