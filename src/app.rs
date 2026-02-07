use eframe::egui;
use eframe::egui::RichText;
use egui::CentralPanel;

use crate::time;
use crate::fonts;
use crate::render;
use crate::time::days_in_year;

// strcut for app
// it is like saying this varible will be this type
pub struct App {
    year: i32,
    days_passed: i32,
    days_passed_percentage: i32,
    days_in_year: i32,
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
            days_in_year: time::days_in_year() - days_passed,
        }
    }
}

// the place where all the magic happen
impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    // the actual place where all the magic happen
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // this suppose to make the things sleep if nothing happens, not sure what it does.
        ctx.request_repaint_after(std::time::Duration::from_secs(3600));
        
        // CentralPanel more like control panel, all ui stuff is in here
        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                // this gets the current window width and height
                let width = ctx.viewport_rect().width();
                let height = ctx.viewport_rect().height();
                
                
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
                // A frame is a fancy stuff that let you do fancy stuff

            egui::Frame::new()
            .inner_margin(eframe::egui::Margin { left: 32, right: 32, top: 32, bottom: 32}) //padding
            .corner_radius(24) // corner radius
            .fill(egui::Color32::from_rgba_unmultiplied(22, 22, 22, 255)) // i am not sure if it inside color or outside color
            .show(ui, |ui|{
                
                ui.spacing_mut().item_spacing.y = height * -0.012; // negative spacing for horizontal text, dynamic needed here
                ui.horizontal(|ui|{ // first row
                    ui.heading(RichText::new(&self.year.to_string())
                            .color(egui::Color32::from_rgb(254, 254, 254))
                        );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.heading(RichText::new(&self.days_in_year.to_string())
                            .color(egui::Color32::from_rgb(254, 254, 254))
                        );
                    });
                });
                ui.horizontal(|ui| { // second row
                    ui.label(format!("Day {} - {}%", (&self.days_passed).to_string(), (&self.days_passed_percentage).to_string()));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        ui.label(format!("{} left", if self.days_in_year == 1 {"day".to_string()} else {"days".to_string()}));
                    });
                });
                ui.add_space(width * 0.04); // space between text and grid
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    render::draw_year_progress_grid(
                        ui,
                        days_in_year(),
                        self.days_passed,
                        21, // cols
                        width * 0.0115, // dot radius
                        height * 0.012, // dot spacing
                    );
                });
            })
        });
    }
}
