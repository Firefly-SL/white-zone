use eframe::egui;
use egui::CentralPanel;

use crate::time;
use crate::fonts;

pub struct MyEguiApp {
    year: String,
}

impl MyEguiApp {
    pub fn new(ctx: &eframe::egui::Context) -> Self {
        fonts::configure_fonts(ctx);
        
        Self {
            year: time::current_year_string(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            egui::Frame::new()
            .inner_margin(eframe::egui::Margin { left: 10, right: 10, top: 15, bottom: 15})
            .show(ui, |ui|{
                ui.horizontal(|ui|{
                    ui.heading(&self.year);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.heading("348");
                    });
                });
                ui.horizontal(|ui| {
                    ui.label("Day 18 - 4%");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.label("Days left");
                    });
                });
            })
        });
    }
}
