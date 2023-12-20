pub mod field;
pub mod person;
pub mod util;

use egui::{Ui, Vec2};

use self::{
    field::draw_field,
    person::{Movement, Person},
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Animation {
    pub frames: Vec<Vec<Person>>,
    pub cur_frame: usize,
}

impl Animation {
    pub fn new(frame: Vec<Person>) -> Self {
        Self {
            frames: vec![frame],
            cur_frame: 0,
        }
    }

    pub fn add_frame(&mut self) {
        let mut new_frame = Vec::new();

        for player in self.frames[self.frames.len() - 1].iter() {
            let pos = match player.movement {
                Movement::None(pos) => pos,
                Movement::Bezier(pts) => pts[3],
            };
            new_frame.push(Person::new(
                Movement::None(pos),
                &player.label,
                player.p_type,
            ));
        }

        self.frames.push(new_frame);
        self.cur_frame = self.frames.len() - 1;
    }

    pub fn display(&mut self, ui: &mut Ui, animation_time: Option<f32>) {
        let a_size = ui.available_size();
        let a_width = a_size.x.min(a_size.y - 100.0);
        let desired_size = Vec2 {
            x: a_width,
            y: a_width,
        };

        let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

        ui.label(format!(
            "x {} {}, y {} {}",
            rect.min.x, rect.max.x, rect.min.y, rect.max.y
        ));

        ui.separator();

        // Draw handball field
        if ui.is_rect_visible(rect) {
            draw_field(ui, rect);

            match animation_time {
                None => {
                    for i in 0..self.frames[self.cur_frame].len() {
                        let p = &mut self.frames[self.cur_frame][i];
                        p.display(ui, rect);
                    }
                }
                Some(time) => {
                    let timestep = time.floor();
                    let frac = time - timestep;
                    let frameidx = (timestep as usize).min(self.frames.len() - 1);
                    for p in self.frames[frameidx].iter() {
                        p.animate(ui, rect, frac);
                    }
                }
            }
        }
    }
}
