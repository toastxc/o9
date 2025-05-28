use crate::svg;
use crate::svg::raster_custom;
use crate::widgets::{Number, Widgets};
use std::collections::HashSet;
use std::process::Command;

#[derive(Debug)]
pub struct Ui {
    project_path: String,
    title: String,
    widgets: Vec<Widgets>,
    y_spacers: Vec<(usize, u32)>,
    x_spacers: Option<u32>,
}

// fn dependency_check() {
//     let check = ["~/.o9/mi_create", "python3", "git"];
//
//     Command::new("bash")
//         .arg("-c")
//         .arg(&format!(
//             "python3 ~/Downloads/Mi-Create/src/main.py {}",
//             self.conf_path()
//         ))
//         .output()
//         .expect("failed to execute process");
//
//
// }

impl Ui {
    pub fn new(title: impl Into<String>, project_path: impl Into<String>) -> Self {

        Self {
            project_path: project_path.into(),
            title: title.into(),
            widgets: vec![],
            y_spacers: vec![],
            x_spacers: None,
        }
    }
    pub fn x_spacer(&mut self, pixels: u32) {
        self.x_spacers = Some(pixels);
    }
    pub fn y_spacer_tracking(&self) -> Vec<(usize, u32)> {
        self.y_spacers.clone()
    }
    pub fn x_spacer_tracking(&self) -> Option<u32> {
        self.x_spacers.clone()
    }
    pub fn widgets(&self) -> Vec<Widgets> {
        self.widgets.clone()
    }
    pub fn conf_path(&self) -> String {
        format!("{}/{}.fprj", self.project_path, self.title)
    }
    pub fn build(mut self, attach: bool) {


        for (start, amount) in self.y_spacers.clone() {
            (start..self.widgets.len()).for_each(|x| {
                *self.widgets[x].y() += amount;
            });
        }
        let mut buf = format!(
            r#"<?xml version="1.0" ?>
<FaceProject DeviceType="366" Id="">
<Screen Title="{}" Bitmap="../preview_default.png">"#,
            self.title
        );

        buf += &self
            .widgets()
            .into_iter()
            .map(|a| format!("\n{}", a.export()))
            .collect::<String>();

        buf += r#"</Screen>
</FaceProject>"#;

        println!("{}", self.conf_path());
        std::fs::write(self.conf_path(), buf).unwrap();

        svg::raster(
            self.widgets
                .iter()
                .filter_map(|a| a.dpi())
                .collect::<HashSet<u32>>(),
        );
//
        if attach {
            let arg = format!(
                "~/.o9/mi-create_bin {}",
                self.conf_path()
            );
            println!("{}", arg);
            Command::new("bash")
                .arg("-c")
                .arg(&arg)
                .output()
                .expect("failed to execute process");
        }
    }
    pub fn spacer(&mut self, height: u32) {
        let len = self.widgets.len();
        self.y_spacers.push((len, height));
    }
    pub fn horizontal_spaced(&mut self, space: Option<u32>, add_contents: impl FnOnce(&mut Ui)) {
        self.horizontal(add_contents);
        self.spacer(space.unwrap_or(80));
    }
    pub fn horizontal(&mut self, add_contents: impl FnOnce(&mut Ui)) {
        let start = self.widgets.len();
        add_contents(self);

        let mut previous = 0;

        (start..self.widgets.len()).for_each(|a| {
            println!("{}", self.widgets[a].width());
            if previous == 0 {
                previous = self.widgets[a].width();
                return;
            };

            *self.widgets[a].x() = previous;
            previous += self.widgets[a].width();
        });
    }
    pub fn number(&mut self, value_src: u32, dpi: u32, digits: u8) {
        self.add(Number::new(value_src, dpi, digits))
    }
    pub fn numbers(&mut self, value_src: impl Into<Box<[u32]>>, dpi: u32, digits: u8) {
       for value in value_src.into() {
           self.add(Number::new(value, dpi, digits))
       }
    }
    pub fn add(&mut self, i: impl Into<Widgets>) {
        if let Some(x_spacer) = self.x_spacers {
            *self.widgets.last_mut().unwrap().width_mut() += x_spacer;
            self.x_spacers = None;
        }
        self.widgets.push(i.into());
    }
    pub fn number_custom(&mut self, value_src: u32, dpi: u32, digits: u8, name: impl Into<String>) {
        let mut number = Number::new(value_src, dpi, digits);
        let name = name.into();
        number.bitmap_name = Some(name.clone());
        self.add(number);
        raster_custom(dpi, Some(name));
    }

    pub fn add_number_row(&mut self, dpi: u32, spacing: u32, src: impl Into<Box<[u32]>>) {
        self.horizontal_spaced(None, |ui| {
            for x in src.into() {
                ui.number(x, dpi, 2);
                ui.x_spacer(spacing);
            }
        });
    }
}
