use crate::components::{Command, Components};
use eframe::{
    egui::{self, Context},
    CreationContext,
};

pub struct App {
    components: Components,
    settings: Settings,
    clock_high: bool,
}

impl App {
    pub fn new(_cc: &CreationContext) -> Self {
        App {
            components: Components::new(),
            settings: Settings::default(),
            clock_high: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Clock".to_string()).show(ctx, |ui| {
            if self.clock_high && self.settings.reset_on_clock {
                self.components.reset_controls();
            }

            self.clock_high = false;
            if ui.button("Toggle clock").clicked() {
                self.clock_high = true;
            }
        });

        self.components.draw(ctx);
        self.settings.draw(ctx);

        if self.clock_high {
            let commands = self.components.commands();
            println!("Running commands: {:?}", commands);
            tick(&mut self.components, commands);
        }
        ctx.request_repaint();
    }
}

fn tick(components: &mut Components, commands: Vec<Command>) {
    for command in commands.iter() {
        components.run_command(command);
    }
}

#[derive(Default)]
struct Settings {
    reset_on_clock: bool,
}

impl Settings {
    fn draw(&mut self, ctx: &Context) {
        egui::Window::new("Settings").show(ctx, |ui| {
            ui.checkbox(&mut self.reset_on_clock, "Reset on clock");
        });
    }
}
