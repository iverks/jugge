use egui::{
    epaint::CubicBezierShape, Align2, Color32, FontId, Id, Pos2, Rect, Sense, Stroke, Ui, Vec2,
};

use super::util::{bez_at_t, get_screen_coords, screen_d_to_frac};

pub type Point = Pos2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub ids: [Id; 4],
    pub pts: [Point; 4],
    pub label: String,
    pub active: bool,
}

impl Person {
    pub fn new(id: i32, pts: [Point; 4], label: &str) -> Self {
        let ids = [
            Id::new(format!("{id} {label} 1")),
            Id::new(format!("{id} {label} 2")),
            Id::new(format!("{id} {label} 3")),
            Id::new(format!("{id} {label} 4")),
        ];
        Self {
            ids,
            pts,
            label: label.to_string(),
            active: false,
        }
    }

    pub fn from_prev(id: i32, prev: [Point; 4], label: &str) -> Self {
        let prev_mvmnt = prev[3] - prev[0];
        let prev_speed = prev[3] - prev[2];
        let prev_last_speed = prev[3] - prev[1];
        let pts = [
            prev[3],
            prev[3] + prev_speed,
            prev[3] + prev_last_speed,
            prev[3] + prev_mvmnt,
        ];
        Self::new(id, pts, label)
    }

    pub fn display(&mut self, ui: &mut Ui, rect: Rect) -> bool {
        let radius: f32 = 10.0;
        let screen_pt = get_screen_coords(self.pts[0], rect);
        let mut activated = false;

        // Check for clicks
        let bounding_rect = Rect::from_center_size(screen_pt, Vec2::splat(radius * 1.7));
        let i = if self.active {
            ui.interact(bounding_rect, self.ids[0], Sense::drag())
        } else {
            ui.interact(bounding_rect, self.ids[0], Sense::click())
        };

        let draw_radius = if i.dragged() {
            ui.ctx().animate_value_with_time(i.id, 13.0, 0.1)
        } else if i.hovered() {
            ui.ctx().animate_value_with_time(i.id, 11.0, 0.1)
        } else {
            ui.ctx().animate_value_with_time(i.id, radius, 0.1)
        };

        if i.dragged() {
            let d = i.drag_delta();
            for idx in 0..4 {
                self.pts[idx] += screen_d_to_frac(d, rect);
            }
        }

        if i.clicked() {
            activated = true;
        }

        // Edit ui if this is the active dot
        if self.active {
            self.draw_lines(ui, rect);
            for dot_idx in 1..4 {
                self.draw_dot(ui, rect, dot_idx);
            }
        }

        // Draw main dot
        ui.painter()
            .circle(screen_pt, draw_radius, Color32::RED, Stroke::NONE);
        ui.painter().text(
            screen_pt,
            Align2::CENTER_CENTER,
            &self.label,
            FontId::default(),
            Color32::WHITE,
        );

        activated
    }

    fn draw_dot(&mut self, ui: &mut Ui, rect: Rect, dot_idx: usize) {
        let radius: f32 = 5.0;
        let screen_pt = get_screen_coords(self.pts[dot_idx], rect);

        // Check for clicks
        let bounding_rect = Rect::from_center_size(screen_pt, Vec2::splat(radius * 1.7));
        let i = ui.interact(bounding_rect, self.ids[dot_idx], Sense::click_and_drag());

        // Nice ui
        let draw_radius = if i.dragged() {
            ui.ctx().animate_value_with_time(i.id, 7.0, 0.1)
        } else if i.hovered() {
            ui.ctx().animate_value_with_time(i.id, 6.0, 0.1)
        } else {
            ui.ctx().animate_value_with_time(i.id, radius, 0.1)
        };

        // Move dot
        if i.dragged() {
            let d = i.drag_delta();
            self.pts[dot_idx] += screen_d_to_frac(d, rect);
        }

        // Draw
        ui.painter()
            .circle(screen_pt, draw_radius, Color32::RED, Stroke::NONE);
    }

    fn draw_lines(&mut self, ui: &mut Ui, rect: Rect) {
        let stroke = Stroke {
            width: 1.0,
            color: Color32::GRAY,
        };

        let mut points = [Point::ZERO; 4];
        for idx in 0..self.pts.len() {
            points[idx] = get_screen_coords(self.pts[idx], rect);
        }

        let bez = CubicBezierShape {
            closed: false,
            points,
            fill: Color32::TRANSPARENT,
            stroke,
        };

        ui.painter()
            .line_segment(points[0..=1].try_into().unwrap(), stroke);
        ui.painter()
            .line_segment(points[2..=3].try_into().unwrap(), stroke);

        ui.painter().add(bez);
    }

    pub fn animate(&self, ui: &mut Ui, rect: Rect, t: f32) {
        let radius: f32 = 10.0;

        let pt = bez_at_t(self.pts, t);

        let screen_pt = get_screen_coords(pt, rect);

        // Draw main dot
        ui.painter()
            .circle(screen_pt, radius, Color32::RED, Stroke::NONE);
        ui.painter().text(
            screen_pt,
            Align2::CENTER_CENTER,
            &self.label,
            FontId::default(),
            Color32::WHITE,
        );
    }
}
