use self::{alu::Alu, i::I, mb::MB, pc::PC, reg8::Reg8, sp::SP};
use downcast_rs::{impl_downcast, Downcast};
use eframe::egui::{self, Context, Window};

pub mod alu;
pub mod i;
pub mod mb;
pub mod pc;
pub mod reg8;
pub mod sp;

pub struct Components {
    pub data_bus: u8,
    pub addr_bus: u16,

    pub data_custom_input: bool,
    pub data_custom_val: u8,
    pub addr_custom_input: bool,
    pub addr_custom_val: u16,

    pub v: Vec<Box<dyn Component>>,
}

impl Components {
    pub fn new() -> Self {
        let pc: Box<PC> = Box::default();
        let alu: Box<Alu> = Box::default();
        let sp: Box<SP> = Box::default();
        let mb: Box<MB> = Box::default();
        let i: Box<I> = Box::default();

        let a_reg = Box::new(Reg8::new("A"));
        let x_reg = Box::new(Reg8::new("X"));
        let y_reg = Box::new(Reg8::new("Y"));

        Components {
            data_bus: 0,
            addr_bus: 0,

            data_custom_input: false,
            data_custom_val: 0,
            addr_custom_input: false,
            addr_custom_val: 0,

            v: vec![pc, a_reg, x_reg, y_reg, alu, sp, mb, i],
        }
    }

    pub fn commands(&self) -> Vec<Command> {
        let mut commands: Vec<Command> = self.v.iter().flat_map(|c| c.commands()).collect();
        commands.sort();
        commands
    }

    pub fn reset_controls(&mut self) {
        self.data_custom_input = false;
        self.data_custom_val = 0;
        self.addr_custom_input = false;
        self.addr_custom_val = 0;

        for component in self.v.iter_mut() {
            component.reset_controls();
        }
    }

    pub fn draw(&mut self, ctx: &Context) {
        Window::new("Buses").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(format!("Data bus: {:X}", self.data_bus));
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.data_custom_val)
                                .speed(1.0)
                                .hexadecimal(2, false, true),
                        );
                        ui.checkbox(&mut self.data_custom_input, "Set data bus");
                    });
                });
                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.label(format!("Address bus: {:X}", self.addr_bus));
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.addr_custom_val)
                                .speed(1.0)
                                .hexadecimal(4, false, true),
                        );
                        ui.checkbox(&mut self.addr_custom_input, "Set address bus");
                    });
                })
            });
        });
        for component in self.v.iter_mut() {
            component.draw(ctx);
        }
    }

    pub fn run_command(&mut self, command: &Command) {
        if self.addr_custom_input {
            self.addr_bus = self.addr_custom_val;
            self.addr_custom_input = false;
        }
        if self.data_custom_input {
            self.data_bus = self.data_custom_val;
            self.data_custom_input = false;
        }
        self.alu().a = self.a_reg().val;
        self.alu().b = self.data_bus;

        for component in self.v.iter_mut() {
            component.run_command(command, &mut self.data_bus, &mut self.addr_bus);
        }
        println!();
    }

    fn a_reg(&mut self) -> &mut Reg8 {
        self.v[1].downcast_mut::<Reg8>().unwrap()
    }
    fn alu(&mut self) -> &mut Alu {
        self.v[4].downcast_mut::<Alu>().unwrap()
    }
}

pub trait Component: Downcast {
    fn reset_controls(&mut self);
    fn run_command(&mut self, command: &Command, data_bus: &mut u8, addr_bus: &mut u16);
    fn commands(&self) -> Vec<Command>;

    fn draw(&mut self, ctx: &Context);

    fn clone_box(&self) -> Box<dyn Component>;
}
impl_downcast!(Component);

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Command {
    OE(String),
    AddAlu,
    SubAlu,

    IncSP,
    DecSP,
    IncPC,

    Load(String),
}
