use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Obj { // 48 bytes
format_version: String,

    #[serde(rename = "minecraft:geometry")]
    pub minecraft_geometry: Vec<Geometry>,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub description: Description,

    #[serde(default)]
    pub bones: Option<Vec<Bone>>,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    pub identifier: String,
    pub texture_width: i32,
    pub texture_height: i32,

    #[serde(default)]
    pub visible_bounds_width: i32,

    #[serde(default)]
    pub visible_bounds_height: f32,

    #[serde(default)]
    pub visible_bounds_offset: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Bone {
    pub name: String,
    pub pivot: Vec<i32>,
    pub cubes: Vec<BbPrimitive>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BbPrimitive {
    Textured { origin: Vec<f32>, size: Vec<f32>, uv: BbUv },
    NonTextured { origin: Vec<f32>, size: Vec<f32>, uv: Vec<f32> },
    Rotated { origin: Vec<f32>, size: Vec<f32>, pivot: Vec<f32>, rotation: Vec<f32>, uv: BbUv },

    #[serde(skip_deserializing)]
    OriginAndSize { origin: Vec<f32>, size: Vec<f32> }
}
impl BbPrimitive {
    pub fn get_os(self) -> BbPrimitive {
        match self {
            BbPrimitive::NonTextured { origin, size, uv } => BbPrimitive::OriginAndSize { origin, size },
            BbPrimitive::Textured { origin, size, uv } => BbPrimitive::OriginAndSize { origin, size },
            BbPrimitive::Rotated { origin, size, pivot, rotation, uv } => BbPrimitive::OriginAndSize { origin, size },
            BbPrimitive::OriginAndSize { ref origin, ref size } => self,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BbUv {
    pub north: UvPosition,
    pub east: UvPosition,
    pub south: UvPosition,
    pub west: UvPosition,
    pub up: UvPosition,
    pub down: UvPosition
}

#[derive(Deserialize, Debug)]
pub struct UvPosition {
    pub uv: Vec<f32>,
    pub uv_size: Vec<f32>
}