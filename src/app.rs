use eframe::egui::{self, CentralPanel, RichText};

use crate::config::Config;
use crate::config;
use crate::time;
use crate::fonts;
use crate::render;

pub struct App {
    days_passed: i32,
    days_in_year: i32,
    days_passed_percentage: i32,
    config: Config,
}

impl App {
    pub fn new(ctx: &eframe::egui::Context) -> Self {
        let config = config::load_or_create_config().unwrap();
        fonts::configure_fonts(ctx);
        
        // days_passed (it returns actual days passed in a year scope)
        let (days_passed, days_passed_percentage) = time::days_passed();
        
        Self {
            days_passed,
            days_passed_percentage,
            days_in_year: time::days_in_year() - days_passed,
            config: config,
        }
    }
    
    // fn reload_config(&mut self, ctx: &egui::Context) {
    //     if let Ok(_) = config::load_or_create_config() {
    //         // update fonts if changed
    //         fonts::configure_fonts(ctx);

    //         // repaint
    //         ctx.request_repaint();
    //     }
    // }
    
    fn relaunch(&self) {
        if let Ok(current_exe) = std::env::current_exe() {
            if let Err(_) = std::process::Command::new(current_exe).spawn() {
                return;
            }
            std::process::exit(0);
        }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::R)) {
        //     self.reload_config(ctx);
        // }
    
        if ctx.input(|i| i.modifiers.shift && i.key_pressed(egui::Key::R)) {
            self.relaunch();
        }
        
        if ctx.input( |i| i.modifiers.ctrl && i.key_pressed(egui::Key::Q)) {
            std::process::exit(0);
        }
        
        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // this gets the current window width and height
                let width = ctx.viewport_rect().width();
                let height = ctx.viewport_rect().height();

            egui::Frame::default()
            .outer_margin(5)
            .corner_radius(self.config.window.corner_radius)
            .shadow(egui::Shadow {
                offset: self.config.window.drop_shadow.offset,
                blur: self.config.window.drop_shadow.blur,
                spread: self.config.window.drop_shadow.spread,
                color: self.config.window.drop_shadow.color.to_color32(),
            })
            .show(ui, |ui| {
                
                egui::Frame::default()
                .inner_margin(eframe::egui::Margin { 
                    left: 32,
                    right: 32,
                    top: 32,
                    bottom: 32 }) //padding
                .corner_radius(self.config.window.corner_radius)
                .fill(self.config.theme.background.to_color32()) // this is frame bg color
                .show(ui, |ui|{
                    
                    let bold_text_size = height * 0.053;
                    ui.spacing_mut().item_spacing.y = height * -0.012;
                    ui.horizontal(|ui|{ // first row
                        ui.heading(RichText::new(time::current_year_string().to_string())
                                .color(self.config.theme.heading.to_color32())
                                .size(bold_text_size) //20.0
                            );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.heading(RichText::new(&self.days_in_year.to_string())
                                .color(self.config.theme.heading.to_color32())
                                .size(bold_text_size)
                            );
                        });
                    });
                    
                    let normal_text_size = height * 0.033;
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Day {} • {}%",self.days_passed,self.days_passed_percentage))
                                .size(normal_text_size)
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.label(RichText::new(format!("{} left", if self.days_in_year == 1 { "day" } else { "days" }))
                                .size(normal_text_size)
                            );
                        });
                    });
                    ui.add_space(width * 0.085);
                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                        render::draw_year_progress_grid(
                            ui,
                            self.days_in_year + self.days_passed,
                            self.days_passed,
                            self.config.dot_grid.column_count, // cols
                            width * 0.0114, // dot radius
                            height * 0.011, // dot spacing
                            &self.config.dot_grid
                        );
                    });
                });
            });
        });
    }
}