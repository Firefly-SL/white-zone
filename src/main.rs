mod app;
mod time;
mod fonts;
mod render;

use app::App;
use eframe::egui;

fn main() {
    // window magics happen in here, for me.
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([450.00, 500.00]),
        ..Default::default()
    };
    
    // the actual part that runs the app
    let _ = eframe::run_native(
        "White Zone",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::new(&_cc.egui_ctx)))),
    );
}
