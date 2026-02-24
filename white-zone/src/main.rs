#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod time;
mod fonts;
mod render;
mod config;

use app::App;
use eframe::egui;

fn main() {
    let config = config::load_or_create_config().unwrap();
    
    let native_options = eframe::NativeOptions {
        centered: config.window.lock_in_center,
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([config.window.size[0], config.window.size[1]])
        .with_position(egui::Pos2::new(config.window.position[0], config.window.position[1]))
        .with_app_id("whitezone")
        .with_decorations(false)
        .with_taskbar(false)
        .with_resizable(config.window.resizable)
        .with_has_shadow(false)
        .with_window_level(egui::WindowLevel::AlwaysOnBottom)
        .with_transparent(true),
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "White Zone",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::new(&_cc.egui_ctx)))),
    );
}
