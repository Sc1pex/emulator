use super::*;

#[derive(Default, Clone)]
pub struct SP {
    val: u8,

    pub inc: bool,
    pub dec: bool,
    pub oe: bool,
}

impl Component for SP {
    fn reset_controls(&mut self) {
        self.inc = false;
        self.dec = false;
        self.oe = false;
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        if self.inc {
            commands.push(Command::IncSP);
        }
        if self.dec {
            commands.push(Command::DecSP);
        }
        if self.oe {
            commands.push(Command::OE("SP".into()));
        }

        commands
    }

    fn run_command(&mut self, command: &Command, _data_bus: &mut u8, addr_bus: &mut u16) {
        match command {
            Command::IncSP => {
                self.val = self.val.wrapping_add(1);
            }
            Command::DecSP => {
                self.val = self.val.wrapping_sub(1);
            }
            Command::OE(s) if s == "SP" => {
                *addr_bus = 0xFF00 | self.val as u16;
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new("SP").show(ctx, |ui| {
            ui.label(format!("Value: {:X}", self.val));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.inc, "Increment");
                ui.checkbox(&mut self.dec, "Decrement");
                ui.checkbox(&mut self.oe, "OE");
            })
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
