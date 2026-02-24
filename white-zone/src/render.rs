use eframe::egui::Ui;
use eframe::egui;

use crate::config::DotGrid;

// dot grid
pub fn draw_year_progress_grid(
    ui: &mut Ui,
    total_days: i32,
    passed_days: i32,
    cols: i32,
    dot_radius: f32,
    dot_spacing: f32,
    grid: &DotGrid,
) {
    let rows = (total_days + cols - 1) / cols;
    let today = passed_days.saturating_sub(1);
    let step = dot_radius * 2.0 + dot_spacing;
    
    let grid_width  = cols as f32 * step - dot_spacing;
    let grid_height = rows as f32 * step - dot_spacing;
    
    let padding = (dot_radius + 1.0).ceil();
    
    // desired size for it
    let desired_size = egui::vec2(
        grid_width + padding * 2.0,
        grid_height + padding * 2.0,
    );

    // allocate horizontal space
    let (outer_rect, _) = ui.allocate_at_least(
        egui::vec2(ui.available_width(), desired_size.y),
        egui::Sense::hover(),
    );


    // center grid horizontally inside available width
    let centered_rect = outer_rect.shrink2(egui::vec2(
        (outer_rect.width() - desired_size.x).max(0.0) * 0.5,
        0.0,
    ));

    // apply inner padding
    let grid_rect = centered_rect.shrink(padding);

    let painter = ui.painter_at(grid_rect.expand(dot_radius + 1.0));
    
    for i in 0..total_days {
        let col = i % cols;
        let row = i / cols;
    
        let x = grid_rect.left() + dot_radius + col as f32 * step;
        let y = grid_rect.top()  + dot_radius + row as f32 * step;
    
        let pos = egui::pos2(x + 0.5, y + 0.5);
    
        if i == today {
            for (r, a) in [
                (dot_radius * 2.6, 6),
                (dot_radius * 1.9, 12),
                (dot_radius * 1.4, 20),
            ] {
                painter.circle_filled(
                    pos,
                    r,
                    grid.color_today_glow.with_alpha(a),
                );
            }
        }
    
        let color = if i < today {
            grid.color_past.to_color32()
        } else if i == today {
            grid.color_today.to_color32()
        } else {
            grid.color_future.to_color32()
        };
        
        painter.circle_filled(pos, dot_radius, color);
    }
}