use crate::widgets::Widgets;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct Number {
    pub bitmap_name: Option<String>,
    pub dpi: u32,
    pub x: u32,
    pub y: u32,
    // ONLY FOR INTERNAL USE
    pub height: u32,
    pub width: u32,
    pub value_src: u32,
    pub digits: u8,
    pub spacing: u32,
    pub alpha: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Widget")]
pub struct NumberXML {
    #[serde(rename = "@Shape")]
    pub shape: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@BitmapList")]
    pub bitmap_list: String,
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
    #[serde(rename = "@Digits")]
    pub digits: String,
    #[serde(rename = "@Alignment")]
    pub alignment: String,
    #[serde(rename = "@Value_Src")]
    pub value_src: String,
    #[serde(rename = "@Spacing")]
    pub spacing: String,
    #[serde(rename = "@Blanking")]
    pub blanking: String,
}

pub fn number_bmp(name: Option<impl Into<String>>, dpi: u32) -> String {
    // 0_small.png|1_small.png
    // digit_dpi_name.png|
    let name = match name {
        None => String::new(),
        Some(a) => a.into(),
    };

    let mut str: String = (0..10).map(|a| format!("{name}{a}_{dpi}.png|",)).collect();
    str.pop();
    str
}

impl From<Number> for NumberXML {
    fn from(value: Number) -> Self {
        NumberXML {
            bitmap_list: number_bmp(value.bitmap_name, value.dpi),
            name: random::<u32>().to_string(),
            x: value.x.to_string(),
            y: value.y.to_string(),
            digits: value.digits.to_string(),
            value_src: value.value_src.to_string(),
            ..Default::default()
        }
    }
}

impl Default for NumberXML {
    fn default() -> Self {
        Self {
            shape: 32.to_string(),
            name: String::from("UNNAMED"),
            bitmap_list: String::new(),
            x: 0.to_string(),
            y: 0.to_string(),
            width: 48.to_string(),
            height: 48.to_string(),
            alpha: 255.to_string(),
            visible_src: 0.to_string(),
            digits: 1.to_string(),
            alignment: 0.to_string(),
            value_src: 0.to_string(),
            spacing: 0.to_string(),
            blanking: 0.to_string(),
        }
    }
}

/*
<Widget Shape="32"
 Name="widget-0"
  BitmapList=""
  X="39"
   Y="33"
    Width="48"
     Height="48"
      Alpha="255"
       Visible_Src="0"
        Digits="1"
         Alignment="0"
          Value_Src="0"
           Spacing="0"
Blanking="0"/>
 */

impl Number {
    pub fn new(value_src: impl Into<u32>, dpi: u32, digits: u8) -> Self {
        Self {
            width: (dpi as f32 / 3.2) as u32 * digits as u32,
            // ghost
            height: 0,
            value_src: value_src.into(),
            dpi,
            digits,
            ..Default::default()
        }
    }
} //

impl From<Number> for Widgets {
    fn from(value: Number) -> Self {
        Widgets::Number(value)
    }
}
