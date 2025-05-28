use o9::value_src::xiaomi_band_9::*;

use o9::{Number, Ui};


fn main() {
    let mut ui = Ui::new("overview", "/home/kaiaxc/projects/watchface/overview");
    gui(&mut ui);

    ui.build(true);
}

const BIG: u32 = 100;
const SMALL: u32 = 40;

fn gui(ui: &mut Ui) {
    ui.spacer(100);

    // example
    // ui.add_number_row(BIG, 10, [Hour, Minute]);
    //
    // ui.add_number_row(BIG, 10, [Day, Month]);



    ui.horizontal_spaced(Some(100), |ui|{
       ui.number(Hour, BIG, 2);
        let mut slash = o9::Img::new("slashj.png");
        slash.width = 20;
        slash.y += 33/2;
        ui.add(slash);
        ui.number(Minute, BIG, 2);
    });

    // let mut seal = o9::Img::new("seal.png");
    // seal.x  = 70;
    //
    //
    //
    // ui.horizontal_spaced(Some(10),|ui|{
    //     ui.number(BatteryPercent, BIG, 2);
    //     ui.add(seal);
    // });
    //
    // ui.add(o9::Img::new("slashj.png"));



}
