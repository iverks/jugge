use egui::{Color32, Rect, Rounding, Stroke, Ui};

use super::{
    person::Point,
    util::{create_bezier, get_screen_coords},
};

pub fn draw_field(ui: &mut Ui, rect: Rect) {
    let field_width = 20.0;
    let goal_width = 3.0;
    let goal_width_fraction = goal_width / field_width;
    let until_goal = (field_width - goal_width) / 2.0;
    let until_goal_fraction = until_goal / field_width;
    let six_m = 6.0;
    let six_m_fraction = six_m / field_width;
    let nine_m = 9.0;
    let _nine_m_fraction = nine_m / field_width;
    let shoulder_width = (field_width - goal_width - 2.0 * six_m) / 2.0;
    let shoulder_width_fraction = shoulder_width / field_width;

    ui.painter()
        .rect_stroke(rect, Rounding::ZERO, Stroke::new(1.0, Color32::YELLOW));

    draw_goal(ui, rect, until_goal_fraction);
    draw_six_m(
        ui,
        rect,
        shoulder_width_fraction,
        six_m_fraction,
        goal_width_fraction,
    );

    // draw_nine_m(
    //     ui,
    //     rect,
    //     shoulder_width_fraction,
    //     six_m_fraction,
    //     _nine_m_fraction,
    //     goal_width_fraction,
    // );
}

fn draw_goal(ui: &mut Ui, rect: Rect, until_goal_fraction: f32) {
    let goal_start_pt = get_screen_coords(
        Point {
            x: until_goal_fraction,
            y: 0.0,
        },
        rect,
    );
    let goal_end_pt = get_screen_coords(
        Point {
            x: 1.0 - until_goal_fraction,
            y: 0.01,
        },
        rect,
    );
    let goal = Rect::from_min_max(goal_start_pt, goal_end_pt);
    ui.painter()
        .rect(goal, Rounding::ZERO, Color32::RED, Stroke::NONE);
}

fn draw_six_m(
    ui: &mut Ui,
    rect: Rect,
    shoulder_width_fraction: f32,
    six_m_fraction: f32,
    goal_width_fraction: f32,
) {
    let stroke = Stroke {
        width: 1.0,
        color: Color32::YELLOW,
    };

    let center = Point {
        x: shoulder_width_fraction + six_m_fraction,
        y: 0.0,
    };

    let bez = create_bezier(center, six_m_fraction, 3, rect, stroke);

    ui.painter().add(bez);

    let center = Point {
        x: shoulder_width_fraction + six_m_fraction + goal_width_fraction,
        y: 0.0,
    };

    let bez = create_bezier(center, six_m_fraction, 0, rect, stroke);

    ui.painter().add(bez);

    let straight_start = Point {
        x: shoulder_width_fraction + six_m_fraction,
        y: six_m_fraction,
    };

    let straight_end = Point {
        x: shoulder_width_fraction + six_m_fraction + goal_width_fraction,
        y: six_m_fraction,
    };

    ui.painter().line_segment(
        [
            get_screen_coords(straight_start, rect),
            get_screen_coords(straight_end, rect),
        ],
        stroke,
    );
}

#[allow(dead_code)]
fn draw_nine_m(
    ui: &mut Ui,
    rect: Rect,
    shoulder_width_fraction: f32,
    six_m_fraction: f32,
    nine_m_fraction: f32,
    goal_width_fraction: f32,
) {
    let stroke = Stroke {
        width: 1.0,
        color: Color32::LIGHT_YELLOW,
    };

    let center = Point {
        x: shoulder_width_fraction + six_m_fraction,
        y: 0.0,
    };

    let bez = create_bezier(center, nine_m_fraction, 3, rect, stroke);

    ui.painter().add(bez);

    let center = Point {
        x: shoulder_width_fraction + six_m_fraction + goal_width_fraction,
        y: 0.0,
    };

    let bez = create_bezier(center, nine_m_fraction, 0, rect, stroke);

    ui.painter().add(bez);

    let straight_start = Point {
        x: shoulder_width_fraction + six_m_fraction,
        y: nine_m_fraction,
    };

    let straight_end = Point {
        x: shoulder_width_fraction + six_m_fraction + goal_width_fraction,
        y: nine_m_fraction,
    };

    ui.painter().line_segment(
        [
            get_screen_coords(straight_start, rect),
            get_screen_coords(straight_end, rect),
        ],
        stroke,
    );
}
