use std::sync::atomic::{AtomicUsize, Ordering};

use egui::{
    epaint::CubicBezierShape, Align2, Color32, FontId, Id, Pos2, Rect, Sense, Stroke, Ui, Vec2,
};

use super::util::{bez_at_t, get_screen_coords, screen_d_to_frac};

pub type Point = Pos2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerType {
    Attacking,
    Defending,
    Ball,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Movement {
    None(Point),
    Bezier([Point; 4]),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub ids: [Id; 4],
    pub movement: Movement,
    pub label: String,
    pub active: bool,
    pub attacking: bool,
}

impl Person {
    pub fn new(movement: Movement, label: &str, attacking: bool) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let ids = [
            Id::new(format!("{id} {label} 1")),
            Id::new(format!("{id} {label} 2")),
            Id::new(format!("{id} {label} 3")),
            Id::new(format!("{id} {label} 4")),
        ];
        Self {
            ids,
            movement,
            label: label.to_string(),
            active: false,
            attacking,
        }
    }

    pub fn still(pt: Point, label: &str, attacking: bool) -> Self {
        let movement = Movement::None(pt);
        Self::new(movement, label, attacking)
    }

    #[allow(dead_code)]
    pub fn moving(pts: [Point; 4], label: &str, attacking: bool) -> Self {
        let movement = Movement::Bezier(pts);
        Self::new(movement, label, attacking)
    }

    #[allow(dead_code)]
    pub fn from_prev(prev: Self) -> Self {
        match prev.movement {
            Movement::Bezier(pts) => {
                let prev_mvmnt = pts[3] - pts[0];
                let prev_speed = pts[3] - pts[2];
                let prev_last_speed = pts[3] - pts[1];
                let pts = [
                    pts[3],
                    pts[3] + prev_speed,
                    pts[3] + prev_last_speed,
                    pts[3] + prev_mvmnt,
                ];
                Self::moving(pts, &prev.label, prev.attacking)
            }
            Movement::None(pt) => Self::still(pt, &prev.label, prev.attacking),
        }
    }

    fn get_color(&self) -> Color32 {
        if self.attacking {
            if self.active {
                Color32::DARK_RED
            } else {
                Color32::RED
            }
        } else {
            if self.active {
                Color32::DARK_BLUE
            } else {
                Color32::BLUE
            }
        }
    }

    pub fn display(&mut self, ui: &mut Ui, rect: Rect) -> bool {
        let radius: f32 = 10.0;
        let screen_pt = match self.movement {
            Movement::Bezier(pts) => get_screen_coords(pts[0], rect),
            Movement::None(pt) => get_screen_coords(pt, rect),
        };
        let mut activated = false;

        // Check for clicks
        let bounding_rect = Rect::from_center_size(screen_pt, Vec2::splat(radius * 1.7));
        let i = ui.interact(bounding_rect, self.ids[0], Sense::click_and_drag());

        let draw_radius = if i.dragged() {
            ui.ctx().animate_value_with_time(i.id, 13.0, 0.1)
        } else if i.hovered() {
            ui.ctx().animate_value_with_time(i.id, 11.0, 0.1)
        } else {
            ui.ctx().animate_value_with_time(i.id, radius, 0.1)
        };

        if i.dragged() && self.active {
            let d = i.drag_delta();
            match &mut self.movement {
                Movement::Bezier(pts) => {
                    for pt in pts.iter_mut() {
                        *pt += screen_d_to_frac(d, rect);
                    }
                }
                Movement::None(pt) => {
                    *pt += screen_d_to_frac(d, rect);
                }
            }
        }

        if i.clicked() {
            activated = true;
        }

        if i.double_clicked() {
            activated = true;
            let root = match &self.movement {
                Movement::Bezier(pts) => pts[0],
                Movement::None(pt) => *pt,
            };
            match self.movement {
                Movement::Bezier(_) => self.movement = Movement::None(root),
                Movement::None(_) => {
                    self.movement = Movement::Bezier([
                        root,
                        root + Vec2 { x: 0.05, y: 0.0 },
                        root + Vec2 { x: 0.1, y: 0.05 },
                        root + Vec2 { x: 0.1, y: 0.1 },
                    ])
                }
            }
        }

        // Edit ui if we have movement
        if let Movement::Bezier(_) = self.movement {
            self.draw_lines(ui, rect);
            for dot_idx in 1..4 {
                self.draw_dot(ui, rect, dot_idx);
            }
        }

        let col = self.get_color();

        // Draw main dot
        ui.painter()
            .circle(screen_pt, draw_radius, col, Stroke::NONE);
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
        let screen_pt = match self.movement {
            Movement::Bezier(pts) => get_screen_coords(pts[dot_idx], rect),
            Movement::None(_) => panic!("Dots should not be drawn if we dont have bezier"),
        };

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
        if i.dragged() && self.active {
            let d = i.drag_delta();
            if let Movement::Bezier(pts) = &mut self.movement {
                pts[dot_idx] += screen_d_to_frac(d, rect);
            }
        }

        let col = self.get_color();

        // Draw
        ui.painter()
            .circle(screen_pt, draw_radius, col, Stroke::NONE);
    }

    fn draw_lines(&mut self, ui: &mut Ui, rect: Rect) {
        let stroke = Stroke {
            width: 1.0,
            color: Color32::GRAY,
        };

        let mut points = [Point::ZERO; 4];
        if let Movement::Bezier(pts) = &mut self.movement {
            for (screen_pt, frac_pt) in points.iter_mut().zip(pts.iter()) {
                *screen_pt = get_screen_coords(*frac_pt, rect);
            }
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

        let screen_pt = match &self.movement {
            Movement::Bezier(pts) => {
                let pt = bez_at_t(*pts, t);

                get_screen_coords(pt, rect)
            }
            Movement::None(pt) => *pt,
        };
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
