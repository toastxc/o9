mod image;
mod number;
pub mod value_src;

use crate::widgets::number::NumberXML;
pub use number::Number;
use serde::Deserialize;

pub use image::Img;
use crate::widgets::image::ImgXML;

#[derive(Debug, Clone)]
pub enum Widgets {
    Number(Number),
    NumberXML(NumberXML),
    Img(Img),
    Arc,
}

// IMPLEMENTATION OF MEMBERS

impl Widgets {
    pub fn x(&mut self) -> &mut u32 {
        match self {
            Widgets::Number(Number { x, .. }) | Widgets::Img(Img{x, ..}) => x,
            _ => unreachable!(),
        }
    }
    pub fn y(&mut self) -> &mut u32 {
        match self {
            Widgets::Number(Number { y, .. }) | Widgets::Img(Img{y,..}, ..) => y,
            _ => unreachable!(),
        }
    }
    pub fn width(&self) -> u32 {
        match self {
            Widgets::Number(Number { width, .. }) | Widgets::Img(Img { width, .. }) => *width,
            _ => unreachable!(),
        }
    }
    pub fn width_mut(&mut self) -> &mut u32 {
        match self {
            Widgets::Number(Number { width, .. }) => width,
            _ => unreachable!(),
        }
    }

    pub fn dpi(&self) -> Option<u32> {
        match self {
            Widgets::Number(Number { dpi, .. }) => Some(*dpi),
            _ => None,
        }
    }
    pub fn export(self) -> String {
        match self {
            Widgets::Number(number) => quick_xml::se::to_string(&NumberXML::from(number)).unwrap(),
            Widgets::Img(img) => quick_xml::se::to_string(&ImgXML::from(img)).unwrap(),
            _ => unreachable!(),
        }
    }
}
