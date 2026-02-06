use eframe::egui;
use egui::CentralPanel;

use crate::time;
use crate::fonts;
use crate::render;

// strcut for app
// it is like saying this varible will be this type
pub struct App {
    year: String,
    days_passed: u32,
    days_passed_percentage: u32,
    days_left: u32,
}

// assinging value to those variables
// i think i should continue with the tutorials
impl App {
    pub fn new(ctx: &eframe::egui::Context) -> Self {
        fonts::configure_fonts(ctx);
        
        // days_passed (it returns actual days passed in a year scope)
        let (days_passed, days_passed_percentage) = time::days_passed();
        
        Self {
            year: time::current_year_string(),
            days_passed,
            days_passed_percentage,
            days_left: time::days_left() - days_passed,
        }
    }
}

// the place where all the magic happen
impl eframe::App for App {
    // the actual place where all the magic happen
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // this suppose to make the things sleep if nothing happens, not sure what it does.
        ctx.request_repaint_after(std::time::Duration::from_secs(3600));
        
        // to set window colors, i played with a lot never figured out how it works
        ctx.set_visuals(egui::Visuals {
            panel_fill: egui::Color32::from_rgba_premultiplied(26, 26, 26, 0),
            window_fill: egui::Color32::from_rgba_premultiplied(26, 26, 26, 0),
            ..Default::default()
        });
        
        // CentralPanel more like control panel, all ui stuff is in here
        CentralPanel::default().show(ctx, |ui| {
            // idk what it does and didn't wanna remove it
            // let width = ui.ctx().content_rect().size().x;
            // let height = ui.ctx().content_rect().size().y;
            
            // A frame is a fancy stuff that let you do fancy stuff
            egui::Frame::new()
            .inner_margin(eframe::egui::Margin { left: 32, right: 32, top: 32, bottom: 32}) //padding
            .corner_radius(egui::CornerRadius::same(24)) // corner radius
            .fill(egui::Color32::from_rgba_premultiplied(26, 26, 26, 255)) // i am not sure if it inside color or outside color
            .show(ui, |ui|{
                ui.spacing_mut().item_spacing.y = -6.0; // negative spacing for horizontal text, dynamic needed here
                ui.horizontal(|ui|{ // first row
                    ui.heading(&self.year);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.heading(&self.days_left.to_string());
                    });
                });
                ui.horizontal(|ui| { // second row
                    ui.label(format!("Day {} - {}%", (&self.days_passed).to_string(), (&self.days_passed_percentage).to_string()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.label(format!("{} left", if self.days_left == 1 {"Day".to_string()} else {"Days".to_string()}));
                    });
                });
                ui.add_space(20.0); // space between text and grid
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    render::draw_year_progress_grid(
                        ui,
                        365,
                        self.days_passed,
                        21,
                        5.2,
                        5.0,
                    );                    
                });
            })
        });
    }
}
