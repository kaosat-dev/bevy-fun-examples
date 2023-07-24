use std::collections::HashMap;
use core::ops::Deref;

use serde_json::Value;
use serde::de::DeserializeSeed;
use bevy::prelude::*;
use bevy::reflect::serde::UntypedReflectDeserializer;
use bevy::reflect::TypeRegistryInternal;
use bevy::gltf::{Gltf, GltfExtras};

use super::capitalize_first_letter;

pub fn gltf_extras_to_components(
    gltf: &mut Gltf,
    scenes: &mut ResMut<Assets<Scene>>,
    type_registry: impl Deref<Target = TypeRegistryInternal>,
    gltf_name: &str
){
    let mut added_components = 0;  
    for (_name, scene) in &gltf.named_scenes {
      info!("gltf: {:?} scene name {:?}", gltf_name, _name);
   
      let scene = scenes.get_mut(scene).unwrap();

      let mut query = scene.world.query::<(Entity, &Name, &GltfExtras, &Parent)>();
      let mut entity_components: HashMap<Entity, Vec<Box<dyn Reflect>> > = HashMap::new();
      for (entity, name, extras, parent) in query.iter(&scene.world) {
        debug!("Name: {}, entity {:?}, parent: {:?}", name, entity, parent);
        let reflect_components = jsonstring_to_reflect_component(&extras.value, &type_registry);
        added_components = reflect_components.len();
        debug!("Found components {}", added_components);

        // we assign the components specified in entity_data objects to their parent node
        let mut target_entity = entity;
        if name.as_str().starts_with("entity_data") || name.as_str().ends_with("entity_data") || name.as_str().contains("components") { 
          debug!("adding components to parent");
          target_entity = parent.get();
        }
      
        debug!("adding to {:?}", target_entity);

        // if there where already components set to be added to this entity (for example when entity_data was refering to a parent), update the vec of entity_components accordingly
        // this allows for example blender collection to provide basic ecs data & the instances to override/ define their own values
        if entity_components.contains_key(&target_entity) {
          let mut updated_components: Vec<Box<dyn Reflect>> = Vec::new();
          let current_components = &entity_components[&target_entity];
          
          // first inject the current components
          for component in current_components {
            updated_components.push(component.clone_value());
          }
          // then inject the new components: this also enables overwrite components set in the collection
          for component in reflect_components {
            updated_components.push(component.clone_value());
          }
          entity_components.insert(target_entity, updated_components);
          
        
        }else {
          entity_components.insert(target_entity, reflect_components);
        }
        // shorthand, did not manage to get it working
       /*  entity_components.insert(
          target_entity, 
          if entity_components.contains_key(&target_entity) { 
            entity_components[&target_entity].push(reflect_components) } else { reflect_components }
          );*/

          debug!("-----value {:?}", &extras.value);
      }

      // println!("FOUND ASSET {:?}", foob);
      // GltfNode
      // find a way to link this name to the current entity ?
      debug!("done pre-processing components, now adding them to entities");
      for (entity, components) in entity_components {
        if components.len() > 0 {
          debug!("--entity {:?}, components {}", entity, components.len());
        }
        for component in components {
            let mut entity_mut = scene.world.entity_mut(entity);
            debug!("------adding {} {:?}", component.type_name(), component);

            type_registry
                .get_with_name(component.type_name())
                .unwrap() // Component was successfully deserialized, it has to be in the registry
                .data::<ReflectComponent>()
                .unwrap() // Hopefully, the component deserializer ensures those are components
                .insert(&mut entity_mut, &*component)
                ; 

            // info!("all components {:?}", scene.world.entity(entity).archetype().components());
            // scene.world.components().
                // TODO: how can we insert any additional components "by hand" here ?
        }
        let e_mut = scene.world.entity_mut(entity);
        let archetype = e_mut.archetype().clone();//.components();
        let _all_components = archetype.components();
        // println!("All components {:?}", all_components);

        if added_components > 0 {
          debug!("------done adding {} components", added_components);
        }
      }
    }
    info!("done extracting gltf_extras /n");   
  }
  

  pub fn jsonstring_to_reflect_component(
    json_string: &String,
    type_registry: &TypeRegistryInternal
  ) -> Vec<Box<dyn Reflect>> {
    let lookup: HashMap<String, Value> = serde_json::from_str(json_string.as_str()).unwrap();
    let mut components: Vec<Box<dyn Reflect>> = Vec::new();
    for (key, value) in lookup.into_iter() {
      let mut parsed_value = format!("{}", value);
      parsed_value = serde_json::from_str(parsed_value.as_str()).unwrap_or(parsed_value);
  
      let type_string = key.replace("component: ", "").trim().to_string();
      let capitalized_type_name = capitalize_first_letter(type_string.as_str());
      debug!("type_string {:?}", type_string);
      if let Some(type_registration) = type_registry.get_with_short_name(capitalized_type_name.as_str()) {
        // println!("parsed_value {:?}", parsed_value);
        if parsed_value == "null" {
          parsed_value = "{}".to_string();
        } 
        // UGH , horrible hack
        if !parsed_value.contains("{") && !parsed_value.contains("\"") {
          parsed_value = format!("\"{}\"", parsed_value);
        }
        let json_string = format!("
          {{
              \"{}\":{}
          }}",
          type_registration.type_name(),
          parsed_value
        );
  
        debug!("component data json string {}", json_string);
        let mut deserializer =serde_json::Deserializer::from_str(json_string.as_str());
        let reflect_deserializer = UntypedReflectDeserializer::new(&type_registry);
        let component = reflect_deserializer.deserialize(&mut deserializer).unwrap();
        components.push(component);
        debug!("found type registration for {}", capitalized_type_name);
      } else {
        warn!("no type registration for {}", capitalized_type_name);
      }
    }
    components
  }
  
