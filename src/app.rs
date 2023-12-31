use egui::Id;

use crate::anim::person::{Person, PlayerType, Point};
use crate::anim::Animation;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JuggeApp {
    #[serde(skip)] // opted out for debug reasons
    animation: Animation,

    #[serde(skip)] // opted out on purpose
    is_animating: bool,
}

impl Default for JuggeApp {
    fn default() -> Self {
        Self {
            animation: Animation::new(vec![
                Person::still(Point::new(0.02, 0.02), "LW", PlayerType::Attacking),
                Person::still(Point::new(0.11, 0.56), "LB", PlayerType::Attacking),
                Person::still(Point::new(0.50, 0.62), "CB", PlayerType::Attacking),
                Person::still(Point::new(0.50, 0.32), "PV", PlayerType::Attacking),
                Person::still(Point::new(0.89, 0.56), "RB", PlayerType::Attacking),
                Person::still(Point::new(0.98, 0.02), "RW", PlayerType::Attacking),
                // Defense
                Person::still(Point::new(0.16, 0.15), "LW", PlayerType::Defending),
                Person::still(Point::new(0.28, 0.28), "LB", PlayerType::Defending),
                Person::still(Point::new(0.43, 0.32), "CB", PlayerType::Defending),
                Person::still(Point::new(0.57, 0.32), "PV", PlayerType::Defending),
                Person::still(Point::new(0.72, 0.28), "RB", PlayerType::Defending),
                Person::still(Point::new(0.84, 0.15), "RW", PlayerType::Defending),
                // Ball
                Person::still(Point::new(0.50, 0.65), "", PlayerType::Ball),
            ]),
            is_animating: false,
        }
    }
}

impl JuggeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for JuggeApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Handball move editor");

            let numsteps = self.animation.frames.len() as f32;
            let mut animation_time = ui.ctx().animate_value_with_time(
                Id::new("main animation"),
                numsteps - f32::EPSILON,
                numsteps,
            );

            ui.horizontal(|ui| {
                if ui.button("Animate").clicked() {
                    // Move time to start
                    self.is_animating = true;
                    animation_time =
                        ui.ctx()
                            .animate_value_with_time(Id::new("main animation"), 0.0, 0.0);
                }
                if ui.button("Reset").clicked() {
                    // Move time to end
                    self.is_animating = false;
                }
            });

            ui.separator();

            let num_steps = self.animation.frames.len();
            ui.horizontal(|ui| {
                for i in 1..=num_steps {
                    let step_name = i.to_string();
                    if ui.button(step_name).clicked() {
                        self.animation.cur_frame = i - 1;
                        self.is_animating = false;
                    }
                }
                if ui.button("Add step").clicked() {
                    self.animation.add_frame();
                    self.is_animating = false;
                }
            });

            ui.separator();

            // Display animation or display editing
            if self.is_animating {
                self.animation.display(ui, Some(animation_time));
            } else {
                self.animation.display(ui, None);
            }

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
        });
    }
}

// Default positions

// Attacking
// LW: 0.02 0.02
// LB: 0.11 0.56
// CB: 0.5 0.62
// PV: 0.5 0.32
// RB: 0.89 0.56
// RW: 0.98 0.02

// Defending
// LW: 0.16 0.15
// LB: 0.28 0.28
// CB: 0.43 0.32
// PV: 0.57 0.32
// RB: 0.72 0.28
// RW: 0.16 0.15
