use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Widget")]
pub struct ImgXML {
    #[serde(rename = "@Shape")]
    pub shape: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Bitmap")]
    pub bitmap: String,
    #[serde(rename = "@X")]
    pub x: String,
    #[serde(rename = "@Y")]
    pub y: String,
    #[serde(rename = "@Width")]
    pub width: String,
    #[serde(rename = "@Height")]
    pub height: String,
    #[serde(rename = "@Alpha")]
    pub alpha: String,
    #[serde(rename = "@Visible_Src")]
    pub visible_src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgVec {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
    pub alpha: u8,
    pub name: u32,
    pub dpi: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Img {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
    pub alpha: u8,
    pub name: u32,
}
