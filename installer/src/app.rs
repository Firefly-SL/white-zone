use eframe::egui::{self, CentralPanel, RichText};

use crate::fonts;

pub struct App {
    primary_text: egui::Color32,
    _secondary_text: egui::Color32,
}

impl App {
    pub fn new(ctx: &eframe::egui::Context) -> Self { 
        fonts::configure_fonts(ctx);
        ctx.style_mut(|style| {
            style.interaction.selectable_labels = false;
        });
        
        Self {
            primary_text: egui::Color32::from_rgb(254, 254, 254),
            _secondary_text: egui::Color32::from_rgb(82, 81, 82),
        }
    }    
}


impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        CentralPanel::default()
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgb(22, 22, 22))
            // .stroke(egui::Stroke::new(3.2, egui::Color32::from_rgba_unmultiplied(245, 245, 245, 30)))
            .inner_margin(6)
            .corner_radius(20.0)
        )
        .show(ctx, |ui| {
            
            egui::Frame::default()
            .inner_margin(12)
            // .fill(egui::Color32::from_rgb(39, 38, 44))
            .corner_radius(24.0)
            .show(ui, |ui| {
                
                ui.vertical_centered_justified(|ui| {
                    ui.add_space(5.0);
                    ui.label(RichText::new("Install 'White Zone' widget.").size(16.0)
                        .color(self.primary_text)
                    );
                    ui.add_space(10.0);
                    ui.label(RichText::new("____________________").size(16.0)
                        .color(self.primary_text)
                    );
                    ui.add_space(ui.available_height() - 38.0);
                });
                
                // the fckin buttons that wasted my entire day and still need work.
                ui.columns(2, |columns| {
                    columns[0].allocate_ui_with_layout(egui::Vec2::ZERO, egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(24.0);
                        if ui.add_sized([100.0, 34.0],
                            egui::Button::new(
                                RichText::new("Cancel").size(16.0)
                            ).fill(egui::Color32::from_rgb(30, 30, 30)).corner_radius(48.0)
                            .stroke(egui::Stroke::new(1.6, egui::Color32::from_rgba_unmultiplied(254, 254, 254, 0)))
                        ).clicked() {
                            std::process::exit(0);
                        };  
                    });
                    columns[1].allocate_ui_with_layout(egui::Vec2::ZERO, egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized([100.0, 34.0],
                            egui::Button::new(
                                RichText::new("Install").size(16.0))
                            .fill(egui::Color32::from_rgb(82, 81, 82)).corner_radius(48.0)
                            .stroke(egui::Stroke::new(1.6, egui::Color32::from_rgba_unmultiplied(254, 254, 254, 0)))
                        ).clicked() {
                            println!("Installing button clicked")
                            // if: check if binary is installed or not
                            //      replace the old binary (a function that can replace and put the binries with arguments)
                            //      add autostart reg entry for windows
                            //      
                            // 
                            // 
                        };
                    });
                });
            });
        });
    }
}