use o9::value_src::xiaomi_band_9::*;
use o9::Ui;



fn main() {
    let mut ui = Ui::new("overview", "~/watchface/overview");
    gui(&mut ui);
    ui.build(false);
}

const BIG: u32 = 100;

fn gui(ui: &mut Ui) {
    ui.spacer(100);

    ui.add_number_row(BIG, [Hour, Minute]);
    ui.add_number_row(BIG, [Day, Month]);

    // day of week
    ui.number_custom(Week, BIG, 1, "week");

    // battery indicator
    ui.number_custom(BatteryPercent, BIG, 2, "bat");
}
