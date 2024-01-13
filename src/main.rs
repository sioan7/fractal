#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        maximized: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Fractal Explorer",
        native_options,
        Box::new(|cc| Box::new(fractal_explorer::app::FractalExplorerApp::new(cc))),
    )
}
