#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod fonts;

use app::App;
use eframe::egui;

fn main() {
    
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
        .with_inner_size(egui::Vec2::new(300.0, 200.0))
        .with_has_shadow(true)
        .with_app_id("whitezone_installer")
        .with_decorations(false)
        .with_taskbar(false)
        .with_resizable(false)
        .with_has_shadow(true)
        .with_transparent(true),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "White Zone Installer",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::new(&_cc.egui_ctx)))),
    );
}
