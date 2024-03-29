
# Bevy fun examples

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![MIT](https://img.shields.io/badge/license-Mit-blue.svg)](./LICENSE)

A few different examples for bevy (not the official examples :)

Tested with Bevy 0.11.x


## List of examples

### [Vertex Animation](./examples/vertex_animation/README.md)

- vertex shader animation (specifically for animation fishes etc) 


    ![demo](./examples/vertex_animation/vertex_animation_fishy.gif)


    run it with 
    
    ```cargo run --features bevy/dynamic_linking --features bevy/file_watcher --example vertex_animation```


### [Blender Gltf Components](https://github.com/kaosat-dev/Blender_bevy_components_workflow)

- this is now a full fledged workflow with Crates for Bevy , Blender addon etc, see the link above

    ![demo](./examples/blender_gltf_components/_docs/blender_gltf_components.png)





## Notes

- feedback is welcome, although I do not know how much time I will have to update this repo
- my Rust & Bevy knowledge is limited for now, so suggestions to make things more idiomatic are welcome !

## License

These example are licensed under MIT.