pub mod utils;
pub use utils::*;

pub mod gltf_to_components;
pub use gltf_to_components::*;

pub mod process_gltfs;
pub use process_gltfs::*;

pub mod models_replace_proxies;
pub use models_replace_proxies::*;

use bevy::prelude::*;
use bevy::pbr::{NotShadowCaster, DirectionalLightShadowMap};

// use crate::state::{AppState};

pub struct ModelsPlugin;
impl Plugin for ModelsPlugin {
  fn build(&self, app: &mut App) {
      app
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
         // FIXME: adding these since they are missing
        .register_type::<NotShadowCaster>()
        .insert_resource(GltfLoadingTracker::new())

        .add_systems(PreUpdate, (
          track_new_gltf, 
          process_loaded_scenes,
          models_replace_proxies
        ))

        // .add_systems((models_replace_proxies,).in_set(OnUpdate(AppState::GameRunning)))

        // compute the aabbs of a whole hierarchy
        //.add_systems((compute_compound_aabb,).in_set(OnUpdate(AppState::GameRunning)))


      ;
  }
}