## Creating watch faces

```rust
use o9::{value_src::xiaomi_band_9::*, Ui};

fn main() {
    let mut ui = Ui::new("overview", "/home/kaiaxc/projects/watchface/overview");
    gui(&mut ui);

    ui.build(true);
}

const BIG: u32 = 100;
const SMALL: u32 = 40;

fn gui(ui: &mut Ui) {
    ui.spacer(100);

    ui.horizontal(|ui| {
        ui.number(Hour, BIG, 2);
        ui.number(Minute, BIG, 2);
    });
}
```

