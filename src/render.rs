use eframe::egui::{Ui, Color32};
use eframe::egui;

// dot grid
pub fn draw_year_progress_grid(
    ui: &mut Ui,
    total_days: u32,
    passed_days: u32,
    cols: u32,
    dot_radius: f32,
    dot_spacing: f32,
) {
    let rows = (total_days + cols - 1) / cols;
    let today = passed_days.saturating_sub(1);
    let step = dot_radius * 2.0 + dot_spacing;
    
    let grid_width  = cols as f32 * step - dot_spacing;
    let grid_height = rows as f32 * step - dot_spacing;
    
    let padding = (dot_radius + 1.0).ceil();
    
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(
            grid_width + padding * 2.0,
            grid_height + padding * 2.0,
        ),
        egui::Sense::hover(),
    );
    
    let painter = ui.painter_at(rect.expand(dot_radius + 1.0));
    let rect = rect.shrink(padding);
    
    for i in 0..total_days {
        let col = i % cols;
        let row = i / cols;
    
        let x = rect.left() + dot_radius + col as f32 * step;
        let y = rect.top()  + dot_radius + row as f32 * step;
    
        let pos = egui::pos2(x + 0.5, y + 0.5);
    
        let color = if i < today {
            Color32::from_rgb(82, 81, 82)
        } else if i == today {
            Color32::from_rgba_premultiplied(254, 254, 254, 150)
        } else {
            Color32::from_rgba_premultiplied(212, 212, 212, 255)
        };
    
        if i == today {
            painter.circle_filled(
                pos,
                dot_radius,
                Color32::from_rgba_premultiplied(255, 255, 255, 150),
            );
        }
    
        painter.circle_filled(pos, dot_radius, color);
    }
}
