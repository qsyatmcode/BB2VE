use serde_json;
use std::{fs, error, io};
use crate::types::{bb, ve};

pub mod types;
pub mod convertion;

// pub fn run(args: impl Iterator<Item = String>) -> Result<String, Box<dyn error::Error>> {
//     return 0k(String::from("t"))
// }

pub fn create_outobj(primitive_count: &mut i32, block_name: String, input: bb::Obj) -> Result<ve::OutObj, Box<dyn std::error::Error>> {
    let hitbox: (f32, f32, f32, f32, f32, f32) = (0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
    let mut aabbs: Vec<(f32, f32, f32, f32, f32, f32, String, String, String, String, String, String)> = Vec::new();

    for geometry in input.minecraft_geometry {
        if let Some(bones) = geometry.bones {
            for bone in bones {
                for primitive in bone.cubes {
                    if let bb::BbPrimitive::OriginAndSize {origin, size}  = primitive.get_os() {
                        aabb_add(&mut aabbs, origin, size, &block_name);
                        *primitive_count += 1;
                    }
                }
            }
        }
    }

    return Ok(ve::OutObj {
        model: String::from("custom"),
        model_primitives: ve::VeModel {
            aabbs,
        },
        hitbox,
    });

    fn to_ve_origin(value: f32) -> f32 { value / 16.0 }
    fn to_ve_size(value: f32) -> f32 {
        value / 16.0
    }
    fn aabb_add(aabbs: &mut Vec<(f32, f32, f32, f32, f32, f32, String, String, String, String, String, String)>, origin: Vec<f32>, size: Vec<f32>, block_name: &String) {
        //let face_placeholder = String::from("REPLACE_THIS");

        if origin.len() < 3 || size.len() < 3 {
            // TODO error
        }
        aabbs.push((
            // The models in the Blockbench are modeled with an offset of 8 units in X and Z. For the model to be in the center of the block, we need to remove this offset
            to_ve_origin(origin[0]) + 0.5, to_ve_origin(origin[1]), to_ve_origin(origin[2]) + 0.5,
            to_ve_size(size[0]), to_ve_size(size[1]), to_ve_size(size[2]),
            get_formatted_sidename(types::TextureSides::North, block_name), get_formatted_sidename(types::TextureSides::East, block_name), get_formatted_sidename(types::TextureSides::Up, block_name),
            get_formatted_sidename(types::TextureSides::Down, block_name), get_formatted_sidename(types::TextureSides::South, block_name), get_formatted_sidename(types::TextureSides::West, block_name),
        ));
    }
}

// pub async fn pick_file() -> Result<Arc<String>, Error> {
//     let handle = AsyncFileDialog::new()
//         .set_title("Choose a Blockbench model...")
//         .add_filter("BB model", &[".json"])
//         .set_directory("/")
//         .pick_file()
//         .await.ok_or(Error::DialogClosed)?;
//
//     load_file(handle.path()).await
// }
//
// pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
//     tokio::fs::read_to_string(path).await
//         .map(Arc::new)
//         .map_err(|e| e.kind())
//         .map_err(Error::IO)
// }

fn get_formatted_sidename(side: types::TextureSides, prefix: &String) -> String {
    match side {
        types::TextureSides::North => format!("{}_{}", prefix, String::from("north")),
        types::TextureSides::East => format!("{}_{}", prefix, String::from("east")),
        types::TextureSides::Up => format!("{}_{}", prefix, String::from("up")),
        types::TextureSides::Down => format!("{}_{}", prefix, String::from("down")),
        types::TextureSides::South => format!("{}_{}", prefix, String::from("south")),
        types::TextureSides::West => format!("{}_{}", prefix, String::from("west")),
    }
}

// #[derive(Debug, Clone)]
// enum Error {
//     DialogClosed,
//     IO(io::ErrorKind)
// }