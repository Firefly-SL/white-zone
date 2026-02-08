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
    
    // window magics happen in here, for me.
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([config.window_width, config.window_height])
        .with_app_id("whitezone")
        .with_decorations(false)
        .with_title_shown(false)
        .with_titlebar_buttons_shown(false)
        .with_taskbar(false)
        .with_resizable(false)
        .with_has_shadow(false)
        .with_window_level(egui::WindowLevel::AlwaysOnBottom)
        .with_transparent(true),
        ..Default::default()
    };
    
    // the actual part that runs the app
    let _ = eframe::run_native(
        "White Zone",
        native_options,
        Box::new(|_cc| Ok(Box::new(App::new(&_cc.egui_ctx)))),
    );
}