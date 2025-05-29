use bevy::prelude::*;
use vrm_viewer::VrmViewerPlugin;
use gltf;

fn print_vrma_info(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the glTF document, buffers, and images from the file.
    let (document, buffers, images) = gltf::import(path)?;
    println!("Loaded VRMA file: {}", path);
    // Document metadata
    // println!("  glTF version: {}.{}", document.version().0, document.version().1);
    // if let Some(generator) = document.generator() {
    //     println!("  Generator: {}", generator);
    // }
    // if let Some(extras) = document.extras() {
    //     println!("  Extras: {:?}", extras);
    // }
    // Scenes
    println!("Scenes ({}):", document.scenes().len());
    for scene in document.scenes() {
        println!("  Scene[{}] name: {:?}", scene.index(), scene.name());
    }
    // Nodes
    println!("Nodes ({}):", document.nodes().count());
    for node in document.nodes() {
        println!("  Node[{}] name: {:?}", node.index(), node.name());
    }
    // Meshes
    println!("Meshes ({}):", document.meshes().count());
    for mesh in document.meshes() {
        println!("  Mesh[{}] name: {:?}", mesh.index(), mesh.name());
    }
    // Materials
    // println!("Materials ({}):", document.materials().count());
    // for material in document.materials() {
    //     println!("  Material[{}] name: {:?}", material.index(), material.name());
    // }
    // Skins
    println!("Skins ({}):", document.skins().count());
    for skin in document.skins() {
        println!("  Skin[{}] name: {:?}", skin.index(), skin.name());
    }
    // Animations
    println!("Animations ({}):", document.animations().count());
    for animation in document.animations() {
        println!("  Animation[{}] name: {:?}", animation.index(), animation.name());
    }
    // Buffers and images info
    println!("Buffers loaded: {}", buffers.len());
    println!("Images loaded: {}", images.len());
    Ok(())
}


fn main() {
    if let Err(e) = print_vrma_info("assets/lmc.vrma") {
        eprintln!("Error reading VRMA file: {}", e);
    }
    App::new().add_plugins(VrmViewerPlugin).run();
}
