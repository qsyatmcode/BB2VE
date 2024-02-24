use serde_json;
use serde::{Deserialize, Serialize};
use std::{fs, process};

fn main() {
    let json_data = match fs::read_to_string("d.geo.json") {
        Ok(r) => match serde_json::from_str::<Obj>(&r) {
            Ok(j) => j,
            Err(e) => {
                println!("{}", e.to_string());
                process::exit(1);
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1);
        }
    };

    println!("Data: {:#?}", json_data);
    println!("Size: {}", std::mem::size_of::<Obj>());

    if let Ok(out) = create_outobj(json_data) {
        println!("out: {:#?}", out);
        println!("size: {}", std::mem::size_of::<OutObj>());
        fs::write("output.json", serde_json::to_string_pretty(&out).unwrap()).expect("Error of writing");
    } else {
        process::exit(69);
    }

    //fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap()).expect("Error of writing");
}

fn create_outobj(input: Obj) -> Result<OutObj, Box<dyn std::error::Error>> {
    let face_placeholder = String::from("texture");
    let hitbox: (f32, f32, f32,  f32, f32, f32) = (0.0, 0.0, 0.0,  1.0, 1.0, 1.0);
    let mut aabbs: Vec<(f32, f32, f32,   f32,   f32,    f32,    String, String, String, String, String, String)> = Vec::new();

    for geometry in input.minecraft_geometry {
        if let Some(bones) = geometry.bones { // bones это папки с кубами
            for bone in bones {
                for primitive in bone.cubes {
                    match primitive {
                        BbPrimitive::NonTextured {origin, size, uv} => { // origin: Vec<i32>, size: Vec<i32>, uv: Vec<i32>
                            if origin.len() < 3 || size.len() < 3 {
                                // TODO error
                            }
                            aabbs.push(( // [x, y, z, width, height, depth, имёна текстур для каждой стороны]
                                         to_ve_origin(origin[0]) + 0.5, to_ve_origin(origin[1]), to_ve_origin(origin[2]) + 0.5, // origin[0] as f32 / 16.0, origin[1] as f32 / 16.0, origin[2] as f32 / 16.0,
                                         to_ve_size(size[0]), to_ve_size(size[1]), to_ve_size(size[2]), //size[0] as f32 / 16.0, size[1] as f32 / 16.0, size[2] as f32 / 16.0,
                                         face_placeholder.clone(), face_placeholder.clone(), face_placeholder.clone(),
                                         face_placeholder.clone(), face_placeholder.clone(), face_placeholder.clone(),
                            ));
                        }
                        BbPrimitive::Textured { origin, size, uv } => { // origin: Vec<i32>, size: Vec<i32>, uv: BbUv
                            if origin.len() < 3 || size.len() < 3 {
                                // TODO error
                            }
                            aabbs.push(( // [x, y, z, width, height, depth, имёна текстур для каждой стороны]
                                         to_ve_origin(origin[0]) + 0.5, to_ve_origin(origin[1]), to_ve_origin(origin[2])  + 0.5, // origin[0] as f32 / 16.0, origin[1] as f32 / 16.0, origin[2] as f32 / 16.0,
                                         to_ve_size(size[0]), to_ve_size(size[1]), to_ve_size(size[2]), //size[0] as f32 / 16.0, size[1] as f32 / 16.0, size[2] as f32 / 16.0,
                                         face_placeholder.clone(), face_placeholder.clone(), face_placeholder.clone(),
                                         face_placeholder.clone(), face_placeholder.clone(), face_placeholder.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(OutObj {
        model: String::from("custom"),
        model_primitives: VeModel {
            aabbs,
        },
        hitbox,
    })
}

fn to_ve_origin(value: i32) -> f32 {
    //let value = value as f32;
    value as f32 / 16.0
}

fn to_ve_size(value: i32) -> f32 {
    value as f32 / 16.0
}

// INPUT DATA (from BlockBench)

#[derive(Deserialize, Debug)]
struct Obj { // 48 bytes
    format_version: String,

    #[serde(rename = "minecraft:geometry")]
    minecraft_geometry: Vec<Geometry>,
}

#[derive(Deserialize, Debug)]
struct Geometry {
    description: Description,

    #[serde(default)]
    bones: Option<Vec<Bone>>,
}

#[derive(Deserialize, Debug)]
struct Description {
    identifier: String,
    texture_width: i32,
    texture_height: i32,

    #[serde(default)]
    visible_bounds_width: i32,

    #[serde(default)]
    visible_bounds_height: f32,

    #[serde(default)]
    visible_bounds_offset: Vec<f32>,
}

#[derive(Deserialize, Debug)]
struct Bone {
    name: String,
    pivot: Vec<i32>,
    cubes: Vec<BbPrimitive>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum BbPrimitive {
    Textured { origin: Vec<i32>, size: Vec<i32>, uv: BbUv },
    NonTextured { origin: Vec<i32>, size: Vec<i32>, uv: Vec<i32> },
}

#[derive(Deserialize, Debug)]
struct BbUv {
    north: UvPosition,
    east: UvPosition,
    south: UvPosition,
    west: UvPosition,
    up: UvPosition,
    down: UvPosition
}

#[derive(Deserialize, Debug)]
struct UvPosition {
    uv: Vec<i32>,
    uv_size: Vec<i32>
}

// OUTPUT DATA (for Voxel Engine)

#[derive(Serialize, Debug)]
struct OutObj {
    model: String,

    #[serde(rename = "model-primitives")]
    model_primitives: VeModel,
    //           [0.0, 0.0, 0.0,  1.0, 1.0, 1.0],
    hitbox:      (f32, f32, f32,  f32, f32, f32),
}

#[derive(Serialize, Debug)]
struct VeModel {
    //         [ x,   y,   z,   width, height, depth,          имёна текстур для каждой стороны]
    aabbs: Vec<(f32, f32, f32,   f32,   f32,    f32,    String, String, String, String, String, String)>,
}