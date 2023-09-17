use super::*;

#[derive(Default, Clone)]
pub struct Reg8 {
    pub val: u8,

    pub load: bool,
    pub oe: bool,

    name: String,
}

impl Reg8 {
    pub fn new(name: &str) -> Self {
        Reg8 {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl Component for Reg8 {
    fn reset_controls(&mut self) {
        self.load = false;
        self.oe = false;
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        if self.load {
            commands.push(Command::Load(self.name.clone()));
        }
        if self.oe {
            commands.push(Command::OE(self.name.clone()));
        }

        println!("Commands for {}: {:?}", self.name, commands);

        commands
    }

    fn run_command(&mut self, command: &Command, data_bus: &mut u8, _addr_bus: &mut u16) {
        match command {
            Command::Load(s) if *s == self.name => {
                self.val = *data_bus;
            }
            Command::OE(s) if *s == self.name => {
                *data_bus = self.val;
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new(self.name.clone()).show(ctx, |ui| {
            ui.label(format!("Value: {:X}", self.val));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.load, "Load");
                ui.add_space(20.0);
                ui.checkbox(&mut self.oe, "OE");
            })
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
