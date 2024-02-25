use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct OutObj {
    pub model: String,

    #[serde(rename = "model-primitives")]
    pub model_primitives: VeModel,
    //           [0.0, 0.0, 0.0,  1.0, 1.0, 1.0],
    pub hitbox:      (f32, f32, f32,  f32, f32, f32),
}

#[derive(Serialize, Debug)]
pub struct VeModel {
    //         [ x,   y,   z,   width, height, depth,          имёна текстур для каждой стороны]
    pub aabbs: Vec<(f32, f32, f32,   f32,   f32,    f32,    String, String, String, String, String, String)>,
}