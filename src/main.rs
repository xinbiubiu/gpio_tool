use eframe::egui;

struct GpioToolApp {
    bank_list: [usize; 5],
    group_list: [usize; 4],
    pin_list: [usize; 8],
    bank: usize,
    group: usize,
    pin: usize,
    pin_num: usize,
}

impl GpioToolApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            bank_list: [0, 1, 2, 3, 4],
            group_list: [0, 1, 2, 3],
            pin_list: [0, 1, 2, 3, 4, 5, 6, 7],
            bank: 0,
            group: 0,
            pin: 0,
            pin_num: 0,
        }
    }
}

impl eframe::App for GpioToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let combo_bank = egui::containers::ComboBox::from_label("Bank")
             .show_index(ui, &mut self.bank, self.bank_list.len(), |i| format!("{}", self.bank_list[i]));
            let combo_group = egui::containers::ComboBox::from_label("Group")
             .show_index(ui, &mut self.group, self.group_list.len(), |i| format!("{}", ["A", "B", "C", "D"][i]));
            let combo_pin = egui::containers::ComboBox::from_label("Pin")
             .show_index(ui, &mut self.pin, self.pin_list.len(), |i| format!("{}", self.pin_list[i]));

            ui.label(format!("GPIO{}_{}{}", self.bank, ["A", "B", "C", "D"][self.group], self.pin));
            ui.label(format!("gpio{}", self.pin_num));

            if combo_bank.changed() || combo_group.changed() || combo_pin.changed() {
                self.pin_num = self.bank * 32 + self.group * 8 + self.pin;
            }
        });
    }
}

fn main() {
    println!("GPIO Tool start up.");
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("GPIO Tool", native_options, Box::new(|cc| Ok(Box::new(GpioToolApp::new(cc)))));
}
