mod app;
mod time;
mod fonts;

use app::MyEguiApp;
//use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions {
        // viewport: egui::ViewportBuilder::default()
        // .with_inner_size([800.00, 600.00]),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::new(&_cc.egui_ctx)))),
    );
}
