## Creating watch faces

### Setup
Create a new rust project with o9 as a dependancy, import `value_src` depending on the type of watch you're designing for (most are supported)
```rust
use o9::{value_src::xiaomi_band_9::*, Ui};

fn main() {
    let mut ui = Ui::new("overview", "../overview");
    gui(&mut ui);

    ui.build(true);
}

const BIG: u32 = 100;
const SMALL: u32 = 40;
```

### Designing 
All UI items in a `horizontal` are automatically spaced out from left to right, in the order that they're added
```rust
fn gui(ui: &mut Ui) {
    ui.spacer(100);
    
    ui.horizontal(|ui| {
        ui.number(Hour, BIG, 2);
        ui.number(Minute, BIG, 2);
    });
}
```
### Preview
![img.png](img.png)

### Todo
#### Integration
Context: This program utilizes a read-only install of https://github.com/ooflet/Mi-Create for the WYSIWYG and for compiling the watchfaces. 
- Automate Mi-Create installation
- Integrate Mi-Crate watchface compilation
- project creation tool

#### Widgets
- Images
- Vector Images
- Arcs


### Wontfix
#### Windows support
This tool utilizes BASH (not necessarily POSIX) scripts and wine. We suggest looking elsewhere for Windows compatible alternatives
https://toastxc.xyz/policies/#windows