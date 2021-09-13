use std::{error::Error, fs, io};

use gltf_json;


fn main() -> Result<(), Box<dyn Error>>{
    let file = fs::File::open("examples/Box.gltf")?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;

    /*
    let accessors = gltf.accessors().map(|elem| elem.json.clone()).collect();
    let buffers = gltf.buffers().map(|elem| elem.json.clone()).collect();
    let buffer_views = gltf.views().map(|elem| elem.json.clone()).collect();
    let meshes = gltf.meshes().map(|elem| elem.json.clone()).collect();
    let nodes = gltf.nodes().map(|elem| elem.json.clone()).collect();
    let scenes = gltf.scenes().map(|elem| elem.json.clone()).collect();
    let images = gltf.images().map(|elem| elem.json.clone()).collect();
    let extensions_used = gltf.extensions_used().map(|elem| elem.to_string()).collect();
    let extensions_required = gltf.extensions_required().map(|elem| elem.to_string()).collect();
    let textures = gltf.textures().map(|elem| elem.json.clone()).collect();
    let materials = gltf.materials().map(|elem| elem.json.clone()).collect();
    let cameras = gltf.cameras().map(|elem| elem.json.clone()).collect();
    let animations = gltf.animations().map(|elem| elem.json.clone()).collect();
    let skins = gltf.skins().map(|elem| elem.json.clone()).collect();
    */

    let mut new_accessors = Vec::<gltf_json::Accessor>::new();
    for a in gltf.accessors() {
        let mut tmp = a.json.clone();
        tmp.byte_offset = 42;
        new_accessors.push(tmp);
    }
    
    let mut rooty = gltf.document.into_json();
    rooty.accessors = new_accessors;

    let writer = fs::File::create("Box_modified.gltf").expect("I/O error");
    gltf_json::serialize::to_writer_pretty(writer, &rooty).expect("Serialization error");
    
    /*let test: Option<&gltf_json::Accessor> = rooty.get(gltf_json::Index::new(0));
    dbg!(test);
    */
    
    /* 
    let root = gltf_json::Root {
        accessors,
        buffers,
        buffer_views,
        meshes,
        nodes,
        scenes,
        images,
        extensions_used,
        extensions_required,
        textures,
        materials,
        cameras,
        scene: Some(gltf_json::Index::<gltf_json::Scene>::new(0)),
        animations,
        skins,
    };
    */

    Ok(())
}