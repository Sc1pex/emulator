use super::*;

#[derive(Default, Clone)]
pub struct Alu {
    pub a: u8,
    pub b: u8,

    pub add: bool,
    pub sub: bool,
    // pub and: bool,
    // pub or: bool,
    // pub xor: bool,
    // pub not: bool,
    // pub shl: bool,
    // pub shr: bool,
    // pub rol: bool,
    // pub ror: bool,
}

impl Component for Alu {
    fn reset_controls(&mut self) {
        self.add = false;
        self.sub = false;
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        if self.add {
            commands.push(Command::AddAlu);
        }
        if self.sub {
            commands.push(Command::SubAlu);
        }

        println!("Commands for Alu: {:?}", commands);

        commands
    }

    fn run_command(&mut self, command: &Command, data_bus: &mut u8, _addr_bus: &mut u16) {
        match command {
            Command::AddAlu => {
                *data_bus = self.a.wrapping_add(self.b);
            }
            Command::SubAlu => {
                *data_bus = self.a.wrapping_sub(self.b);
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new("Alu").show(ctx, |ui| {
            ui.label(format!("A: {:X}", self.a));
            ui.label(format!("B: {:X}", self.b));
            ui.label(format!(
                "A + B: {:X} ({})",
                self.a.wrapping_add(self.b),
                self.a.wrapping_add(self.b)
            ));
            ui.label(format!(
                "A - B: {:X} ({})",
                self.a.wrapping_sub(self.b),
                self.a.wrapping_sub(self.b)
            ));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.add, "Output Add");
                ui.checkbox(&mut self.sub, "Output Sub");
            });
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
