use eframe::egui::{self, Vec2};
use oorandom::Rand32;

use crate::core::{Error, State};

const APPHEADING: &str = "wuerfel app";

pub fn run_gui<'a>(state: State<'a>, random: Rand32) -> Result<(), Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let range = 0..(state.get_dice().len() - 1);
    let app = App {
        state,
        random,
        current_index: None,
        current_range: range,
        current_die_roll: None,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([280.0, 120.0]),
        ..Default::default()
    };
    eframe::run_native(
        APPHEADING,
        options,
        Box::new(|_| {
            // This gives us image support:
            //egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(app))
        }),
    )
    .map_err(|error| error.into())
}

struct App<'a> {
    state: State<'a>,
    current_index: Option<usize>,
    current_range: std::ops::Range<usize>,
    current_die_roll: Option<u32>,
    random: Rand32,
}

impl<'a> eframe::App for App<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let has_dice = !self.current_range.is_empty();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(APPHEADING);
            });
            ui.vertical(|ui| {
                ui.label("Dice");
                ui.horizontal(|hui| {
                    if hui.button("Previous").clicked() && has_dice {
                        if self.current_index.is_none() && self.current_range.len() != 0 {
                            self.current_index = Some(self.current_range.end);
                        } else {
                            self.current_index = self.current_index.map(|index| {
                                if index == 0 {
                                    self.current_range.end
                                } else {
                                    index - 1
                                }
                            });
                        }
                    }
                    if let Some(index) = self.current_index {
                        if let Some(die) = self.state.get_dice().get(index) {
                            hui.label(format!("Die: {}", die.get_name()));
                        }
                    } else {
                        hui.label("No die selected");
                    }
                    if hui.button("Next").clicked() && has_dice {
                        if self.current_index.is_none() && self.current_range.len() != 0 {
                            self.current_index = Some(0);
                        } else {
                            let max_index = self.current_range.end;
                            self.current_index = self.current_index.map(|index| {
                                if index + 1 > max_index {
                                    0
                                } else {
                                    index + 1
                                }
                            });
                        }
                    }
                });
                if let Some(index) = self.current_index {
                    if ui.button("Roll die").clicked() {
                        if let Some(die) = self.state.get_dice().get(index) {
                            let die_range = die.get_range();
                            self.current_die_roll = Some(self.random.rand_range(die_range));
                        }
                    }
                }
                if let Some(roll) = self.current_die_roll {
                    ui.label(format!("Current roll: {}", roll.to_string()));
                }
            });
        });
    }
}
