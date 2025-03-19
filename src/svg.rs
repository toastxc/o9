use std::collections::HashSet;
use std::fs;
use std::process::Command;
fn bitmapper(size: u32, name: Option<String>) {
    for x in 0..10 {
        let name = name.clone().unwrap_or_default();
        let png_name = format!("{name}{x}_{size}.png");
        let svg_name = format!("{name}{x}.svg");
        if fs::read(format!("../overview/images/{png_name}")).is_ok() {
            continue;
        };
        let arg = format!("flatpak run org.inkscape.Inkscape  svgs/{svg_name} --export-type=png --export-filename=../overview/images/{png_name} --export-dpi={size}");
        println!("Rasterizing... {arg}");
        Command::new("bash")
            .arg("-c")
            .arg(&arg)
            .output()
            .expect("failed to execute process");
    }
}

pub fn raster(dpis: HashSet<u32>) {
    for x in dpis {
        bitmapper(x, None);
    }
}

pub fn raster_custom(dpi: u32, name: Option<String>) {
    bitmapper(dpi, name.into())
}
