use rand::random;
use serde::{Deserialize, Serialize};
use crate::{Widgets};
use crate::widgets::number::NumberXML;

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


impl From<Img> for ImgXML {
    fn from(value: Img) -> Self {
        ImgXML {
            shape: "30".to_string(),
            name: random::<u32>().to_string(),
            bitmap: value.name.to_string(),
            x: value.x.to_string(),
            y: value.y.to_string(),
            width: value.width.to_string(),
            height: value.height.to_string(),
            alpha: value.alpha.to_string(),
            visible_src: "0".to_string(),
        }

    }
}


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ImgSvg {
//     pub x: u32,
//     pub y: u32,
//     pub height: u32,
//     pub width: u32,
//     pub alpha: u8,
//     pub name: u32,
//     pub dpi: u32,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Img {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
    pub alpha: u8,
    pub name: String,
}


impl From<Img> for Widgets {
    fn from(value: Img) -> Self {
        Widgets::Img(value)
    }
}

impl Default for Img {
    fn default() -> Self {
        Self{
            x: 0,
            y: 0,
            height: 0,
            width: 0,
            alpha: 255,
            name: "unnamed".to_string(),
        }
    }
}

impl Img {
    pub fn new(name: impl Into<String>) -> Self{
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}
