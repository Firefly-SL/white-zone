use eframe::egui::{self, CentralPanel, RichText};

use crate::config::Config;
use crate::config;
use crate::time;
use crate::fonts;
use crate::render;

pub struct App {
    year: i32,
    days_passed: i32,
    days_passed_percentage: i32,
    days_in_year: i32,
    last_height: f32,
    config: Config,
}

impl App {
    pub fn new(ctx: &eframe::egui::Context) -> Self {
        let config = config::load_or_create_config().unwrap();
        fonts::configure_fonts(ctx);
        
        // days_passed (it returns actual days passed in a year scope)
        let (days_passed, days_passed_percentage) = time::days_passed();
        
        Self {
            year: time::current_year_string(),
            days_passed,
            days_passed_percentage,
            days_in_year: time::days_in_year() - days_passed,
            last_height: 0.0,
            config: config,
        }
    }
    
    fn reload_config(&mut self, ctx: &egui::Context) {
        if let Ok(_) = config::load_or_create_config() {
            // update fonts if changed
            fonts::configure_fonts(ctx);

            // repaint
            ctx.request_repaint();
        }
    }
    
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
        
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::R)) {
            self.reload_config(ctx);
        }
    
        if ctx.input(|i| i.modifiers.shift && i.key_pressed(egui::Key::R)) {
            self.relaunch();
        }
        
        if ctx.input( |i| i.modifiers.ctrl && i.key_pressed(egui::Key::Q)) {
            std::process::exit(0);
        }

        // thought it would work, but it is just a thought
        // if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::P)) {
        //     println!("Key pressed");
        //     if let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
        //         let pixels_per_point = ctx.input(|i| i.viewport().native_pixels_per_point.unwrap_or(1.0));
        //         println!("{}", rect.min.x);
        //         self.config.window.position = [
        //         rect.min.x / pixels_per_point,
        //         rect.min.y / pixels_per_point ];
        //     }
        // }
        
        // if no events, sleep (i don't think this works)
        if ctx.input(|i| 
            i.modifiers.ctrl && i.key_pressed(egui::Key::R) || 
            i.modifiers.shift && i.key_pressed(egui::Key::R) ||
            i.modifiers.ctrl && i.key_pressed(egui::Key::Q)
        ) {
            ctx.request_repaint_after(std::time::Duration::from_secs(1800));
            return;
        }
        
        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // this gets the current window width and height
                let width = ctx.viewport_rect().width();
                let height = ctx.viewport_rect().height();
                
                if (height - self.last_height).abs() > 0.5 {
                    let mut style = (*ctx.style()).clone();

                    style.text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(
                            height * 0.032,
                            egui::FontFamily::Proportional
                        ),);
                
                    style.text_styles.insert(
                        egui::TextStyle::Heading,
                        egui::FontId::new(
                            height * 0.05,
                            egui::FontFamily::Name("Bold".into())),
                        );
                ctx.set_style(style);
                self.last_height = height;
            };
            
            let radius = self.config.window.corner_radius;
            let _shadow = egui::Shadow {
                offset: self.config.window.drop_shadow.offset,
                blur: self.config.window.drop_shadow.blur,
                spread: self.config.window.drop_shadow.spread,
                color: self.config.window.drop_shadow.color.to_color32(),
            };

            egui::Frame::default()
            .outer_margin(5)
            .corner_radius(radius)
            .shadow(_shadow)
            .show(ui, |ui| {
                
                egui::Frame::default()
                .inner_margin(eframe::egui::Margin { 
                    left: self.config.window.padding.left,
                    right: self.config.window.padding.right,
                    top: self.config.window.padding.top,
                    bottom: self.config.window.padding.bottom }) //padding
                .corner_radius(radius)
                .fill(self.config.theme.background.to_color32()) // this is frame bg color
                .show(ui, |ui|{
                    
                    ui.spacing_mut().item_spacing.y = height * -0.012;
                    ui.horizontal(|ui|{ // first row
                        ui.heading(RichText::new(&self.year.to_string())
                                .color(self.config.theme.heading.to_color32())
                            );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.heading(RichText::new(&self.days_in_year.to_string())
                                .color(self.config.theme.heading.to_color32())
                            );
                        });
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label(format!("Day {} • {}%", (&self.days_passed).to_string(), (&self.days_passed_percentage).to_string()));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                            ui.label(format!("{} left", if self.days_in_year == 1 {"day".to_string()} else {"days".to_string()}));
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