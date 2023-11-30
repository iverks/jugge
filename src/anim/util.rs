use egui::{epaint::CubicBezierShape, Color32, Rect, Stroke, Vec2};

use super::person::Point;

/// Get coordinate on screen given bounding rect and pt with x and y [0, 1]
pub fn get_screen_coords(pt: Point, rect: Rect) -> Point {
    let min = rect.min;
    let delta = rect.max - min;
    min + delta * pt.to_vec2()
}

pub fn screen_d_to_frac(dist: Vec2, rect: Rect) -> Vec2 {
    let min = rect.min;
    let delta = rect.max - min;
    dist / delta
}

pub fn get_screen_length(length: f32, rect: Rect) -> f32 {
    let delta = rect.max.x - rect.min.x;
    length * delta
}

pub fn create_bezier(
    center: Point,
    r: f32,
    quadrant: i32,
    rect: Rect,
    stroke: Stroke,
) -> CubicBezierShape {
    let k = 4.0 / 3.0 * (2.0_f32.sqrt() - 1.0);
    let a = Vec2 { x: 0.0, y: 1.0 };
    let a_prime = Vec2 {
        x: k + 0.0009,
        y: 1.0 - 0.00103,
    };
    let b_prime = Vec2 {
        x: 1.0 - 0.00103,
        y: k + 0.0009,
    };
    let b = Vec2 { x: 1.0, y: 0.0 };

    let mut pts = [a, a_prime, b_prime, b];

    for _ in 0..quadrant {
        for p in pts.iter_mut() {
            *p = p.rot90();
        }
    }

    let mut points = [Point::ZERO; 4];
    for idx in 0..pts.len() {
        points[idx] = get_screen_coords(center + r * pts[idx], rect);
    }

    CubicBezierShape {
        closed: false,
        points,
        fill: Color32::TRANSPARENT,
        stroke,
    }
}

pub fn bez_at_t(pts: [Point; 4], t: f32) -> Point {
    (1.0 - t).powi(3) * pts[0]
        + 3.0 * (1.0 - t).powi(2) * t * pts[1].to_vec2()
        + 3.0 * (1.0 - t) * t.powi(2) * pts[2].to_vec2()
        + t.powi(3) * pts[3].to_vec2()
}
