use eframe::NativeOptions;

mod app;
mod components;

fn main() {
    let native_options = NativeOptions {
        // maximized: true,
        ..Default::default()
    };

    eframe::run_native(
        "Emulator",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
    .unwrap();
}
