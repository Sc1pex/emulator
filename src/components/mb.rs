use super::*;

#[derive(Default, Clone)]
pub struct MB {
    pub val: u16,

    pub load_low: bool,
    pub load_high: bool,
    pub oe: bool,
}

impl Component for MB {
    fn reset_controls(&mut self) {
        self.load_low = false;
        self.load_high = false;
        self.oe = false;
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        if self.load_low {
            commands.push(Command::Load("MBLow".to_string()));
        }
        if self.load_high {
            commands.push(Command::Load("MBHigh".to_string()));
        }
        if self.oe {
            commands.push(Command::OE("MB".to_string()));
        }
        commands
    }

    fn run_command(&mut self, command: &Command, data_bus: &mut u8, addr_bus: &mut u16) {
        match command {
            Command::Load(s) if s == "MBLow" => {
                self.val = self.val & 0xFF00 | *data_bus as u16;
            }
            Command::Load(s) if s == "MBHigh" => {
                self.val |= self.val & 0x00FF | (*data_bus as u16) << 8;
            }
            Command::OE(s) if s == "MB" => {
                *addr_bus = self.val;
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new("MB").show(ctx, |ui| {
            ui.label(format!("Value: {:X}", self.val));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.load_low, "Load Low");
                ui.checkbox(&mut self.load_high, "Load High");
                ui.add_space(20.0);
                ui.checkbox(&mut self.oe, "OE");
            })
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
