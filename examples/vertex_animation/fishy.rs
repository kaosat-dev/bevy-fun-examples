

use bevy::{
    prelude::*, 

    reflect::{TypeUuid, TypePath},
    render::{
        primitives::Aabb,
        render_resource::{
            AsBindGroup, ShaderRef,
        },
    }, math::Vec3A, utils::{HashMap, hashbrown::hash_map::Entry}
};


// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(2)]
    color: Color,
    #[uniform(3)]
    speed: f32,
    #[uniform(4)]
    amplitude: f32,
    #[uniform(5)]
    min_bounds: Vec3,
    #[uniform(6)]
    max_bounds: Vec3,
    #[uniform(7)] // TODO: regroup these into a vector ?
    side_to_side_intensity: f32,
    #[uniform(8)]
    pivot_intensity: f32,
    #[uniform(9)]
    wave_intensity: f32,
    #[uniform(10)]
    twist_intensity: f32,
}

/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fish-wave.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/fish-wave.wgsl".into()
    }
}

impl Default for CustomMaterial {
    fn default() -> Self {
        CustomMaterial { 
            color: Color::RED,
            speed: 1.0,
            amplitude: 1.0,
            min_bounds: Vec3::new(-1., -1., -1.),
            max_bounds: Vec3::new(1., 1., 1.),
            side_to_side_intensity: 0.3,
            pivot_intensity: 0.2,
            wave_intensity: 2.5,
            twist_intensity: 0.7
        }
    }
}


pub fn update_uniforms (
    with_custom_materials: Query<(&Aabb, &mut Handle<CustomMaterial>), With<GlobalTransform>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,

    root_entities: Query<(Entity), (With<RootEntity>, Without<Aabb>)>,
    children: Query<&Children>,
    parents: Query<&Parent>,
    with_aabbs: Query<(Entity, &Aabb)>,
    mut commands: Commands,
) {
    // compute compound aabb
   
    let mut entity_to_children_aabbs: HashMap<Entity, Vec<Aabb>> = HashMap::new();

    for root_entity in root_entities.iter() {
        println!("entity {:?}", root_entity);
        for descendant in children.iter_descendants(root_entity) {

            let parent = parents.get(descendant).expect("we should have a parent available").get();
            println!("child {:?}, parent: {:?}", descendant, parent);
            if let Ok((_, aabb)) = with_aabbs.get(descendant) {
                println!("this one has aabb");
                match entity_to_children_aabbs.entry(parent) {
                    Entry::Vacant(e) => {
                        e.insert(vec![aabb.clone()]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(aabb.clone());
                    }
                } 
            }
        }

        // now build the parent's compound aabb
        // let mut root_aabb:Option<Aabb>;
        for (entity, children_aabbs) in &entity_to_children_aabbs {
            println!("adding aabb to {:?}: {:?}", entity, children_aabbs.len());
            // aabb computation
            let mut min = Vec3A::splat(f32::MAX);
            let mut max = Vec3A::splat(f32::MIN);
            for aabb in children_aabbs.iter(){
                min = min.min(aabb.min());
                max = max.max(aabb.max());
            }
        
            // let size = (max - min).length();
            let compound_aabb = Aabb::from_min_max(Vec3::from(min), Vec3::from(max));
            commands.entity(*entity).insert(compound_aabb);

            if entity.index() == root_entity.index() {
                let root_aabb = compound_aabb.clone();

                // then we assign the root aabb to all children with the custom material
                for (_, handle) in with_custom_materials.iter() {
                    let min = Vec3::from(root_aabb.min());
                    let max = Vec3::from(root_aabb.max());
            
                    let material = custom_materials.get_mut(handle).expect("custom material should have been found");
                    material.min_bounds = min;
                    material.max_bounds = max;
                }

            }
        }

        // then we assign the root aabb to all children with the custom material
        /*if let Some(root_aabb) = root_aabb{
            //  && entity_to_children_aabbs.contains_key(&root_entity) 
            for (aabb, handle) in with_custom_materials.iter() {
                let min = Vec3::from(root_aabb.min());
                let max = Vec3::from(root_aabb.max());
        
                let material = custom_materials.get_mut(handle).expect("custom material should have been found");
                material.min_bounds = min;
                material.max_bounds = max;
            }
        }*/
        
    }


}




#[derive(Component)]
pub struct Inserted;

#[derive(Component)]
pub struct RootEntity;

pub fn replace_standard_material( 
    gltf_entities: Query<
        (Entity, &Handle<Mesh>, &Name, &Handle<StandardMaterial>),
        Without<Inserted>,
    >,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    ){

        for (entity, _, _, old_material) in gltf_entities.iter() {
            let original_material = standard_materials.get(old_material).unwrap();
            let custom_material =
            custom_materials.add(CustomMaterial {
                color: original_material.base_color,
                speed: 1.0,
                ..default()
            });
        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>();
        commands.entity(entity)
            .insert(custom_material)
            .insert(Inserted);
        }
}



pub fn setup (
    asset_server: Res<AssetServer>,
    mut commands: Commands,
)
{

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.7, 3.0, 9.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn((SceneBundle {
        scene: asset_server.load("models/fish1.glb#Scene0"),
        ..default()
    }, RootEntity));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 15.0, 0.0).with_rotation(
            Quat::from_euler(EulerRot::XYZ,
                (10.0_f32).to_radians(),
                (10.0_f32).to_radians(),
                (0.0_f32).to_radians(),
            )
        ),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section(
            "Material settings",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

}

fn update_shader_uniforms(
    keycode: Res<Input<KeyCode>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut text: Query<&mut Text>,


) {
    let increment = 0.05;
    for  (_, material) in custom_materials.iter_mut() {
        
        let mut text = text.single_mut();
        let text = &mut text.sections[0].value;
    
        *text = "Vertex animation settings\n".to_string();
        text.push_str("------------------------------\n");
        text.push_str(&format!("Speed: {}\n", material.speed));
        text.push_str(&format!("Side to side: {}\n", material.side_to_side_intensity));
        text.push_str(&format!("Pivot: {}\n", material.pivot_intensity));
        text.push_str(&format!("Wave: {}\n", material.wave_intensity));
        text.push_str(&format!("Twist: {}\n", material.twist_intensity));

        text.push_str("\n\n");
    
        text.push_str("Controls (-/+)\n");
        text.push_str("---------------\n");
        text.push_str("S/D - Speed\n");
        text.push_str("Z/E - Side to side\n");
        text.push_str("R/T - Pivot\n");
        text.push_str("Y/U - Wave\n");
        text.push_str("I/O - Twist\n");


        if keycode.pressed(KeyCode::S) {
            material.speed -= increment;
        }

        if keycode.pressed(KeyCode::D) {
            material.speed += increment;
        }
        //
        if keycode.pressed(KeyCode::Z) {
            material.side_to_side_intensity -= increment;
        }

        if keycode.pressed(KeyCode::E) {
            material.side_to_side_intensity += increment;
        }

        //
        if keycode.pressed(KeyCode::R) {
            material.pivot_intensity -= increment;
        }

        if keycode.pressed(KeyCode::T) {
            material.pivot_intensity += increment;
        }

        //
        if keycode.pressed(KeyCode::Y) {
            material.wave_intensity -= increment;
        }

        if keycode.pressed(KeyCode::U) {
            material.wave_intensity += increment;
        }

         //
         if keycode.pressed(KeyCode::I) {
            material.twist_intensity -= increment;
        }

        if keycode.pressed(KeyCode::O) {
            material.twist_intensity += increment;
        }
    }
   
}


pub struct VertexAnimationPlugin;
impl Plugin for VertexAnimationPlugin {
  fn build(&self, app: &mut App) {
      app
      .insert_resource(ClearColor(Color::rgb(0.1, 0.6, 0.6)))
      .add_plugins((
        MaterialPlugin::<CustomMaterial>::default(),
      ))
      .add_systems(Startup, setup)
      .add_systems(Update, (
        replace_standard_material,
        update_uniforms,
        update_shader_uniforms
      ))
      ;
  }
}


fn main(){
    App::new()
    .add_plugins((
        DefaultPlugins.set(
            AssetPlugin {
                ..default()
            }
        ),
        VertexAnimationPlugin
    ))
    .run();
}