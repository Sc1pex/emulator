use super::*;

#[derive(Default, Clone)]
pub struct I {
    pub val: u8,

    pub load: bool,
}

impl Component for I {
    fn reset_controls(&mut self) {
        self.load = false;
    }

    fn commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();

        if self.load {
            commands.push(Command::Load("I".into()));
        }

        commands
    }

    fn run_command(&mut self, command: &Command, data_bus: &mut u8, _addr_bus: &mut u16) {
        match command {
            Command::Load(s) if *s == "I" => {
                self.val = *data_bus;
            }
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new("I Reg").show(ctx, |ui| {
            ui.label(format!("Value: {:X}", self.val));
            ui.checkbox(&mut self.load, "Load");
        });
    }

    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
