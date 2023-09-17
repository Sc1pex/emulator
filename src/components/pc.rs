use super::*;

#[derive(Default, Clone)]
pub struct PC {
    pub val: u16,

    pub load: bool,
    pub load_low: bool,
    pub load_high: bool,
    pub oe: bool,
    pub oe_low: bool,
    pub oe_high: bool,
    pub inc: bool,
}

impl Component for PC {
    fn reset_controls(&mut self) {
        self.load = false;
        self.load_low = false;
        self.load_high = false;
        self.oe = false;
        self.oe_low = false;
        self.oe_high = false;
        self.inc = false;
    }

    fn run_command(&mut self, command: &Command, data_bus: &mut u8, addr_bus: &mut u16) {
        match command {
            Command::Load(s) if s == "PC" => {
                self.val = *addr_bus;
            }
            Command::Load(s) if s == "PCLow" => {
                self.val = self.val & 0xFF00 | *data_bus as u16;
            }
            Command::Load(s) if s == "PCHigh" => {
                self.val = self.val & 0x00FF | (*data_bus as u16) << 8;
            }
            Command::OE(s) if s == "PC" => {
                *addr_bus = self.val;
            }
            Command::OE(s) if s == "PCLow" => {
                *data_bus = self.val as u8;
            }
            Command::OE(s) if s == "PCHigh" => {
                *data_bus = (self.val >> 8) as u8;
            }
            Command::IncPC => {
                self.val = self.val.wrapping_add(1);
            }
            _ => {}
        };
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        if self.load {
            commands.push(Command::Load("PC".into()));
        }
        if self.load_low {
            commands.push(Command::Load("PCLow".into()));
        }
        if self.load_high {
            commands.push(Command::Load("PCHigh".into()));
        }
        if self.oe {
            commands.push(Command::OE("PC".into()));
        }
        if self.oe_low {
            commands.push(Command::OE("PCLow".into()));
        }
        if self.oe_high {
            commands.push(Command::OE("PCHigh".into()));
        }
        if self.inc {
            commands.push(Command::IncPC);
        }

        println!("Commands from PC: {:?}", commands);
        commands
    }

    fn draw(&mut self, ctx: &Context) {
        Window::new("PC").show(ctx, |ui| {
            ui.label(format!("Value: {:X}", self.val));

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.load, "Load");
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.load_low, "Load Low");
                        ui.checkbox(&mut self.load_high, "Load High");
                    })
                });
                ui.add_space(20.0);
                ui.vertical(|ui| {
                    ui.checkbox(&mut self.oe, "OE");
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.oe_low, "OE Low");
                        ui.checkbox(&mut self.oe_high, "OE High");
                    })
                });
            })
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
