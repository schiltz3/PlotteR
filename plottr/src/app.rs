use crate::csv_util::{parse_columns, Column, PivotIter};
use core::result::Iter;
use eframe::{egui, epi};
use itertools::izip;
use native_dialog::FileDialog;
use std::path::PathBuf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    path: Option<PathBuf>,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    table: Option<Vec<Column>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            path: None,
            table: None,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            label,
            value,
            path,
            table,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // if ui.button("Quit").clicked() {
                    //     frame.quit();
                    // }
                    if ui.button("Open").clicked() {
                        *path = FileDialog::new()
                            .add_filter("CSV", &["csv"])
                            .add_filter("TXT", &["txt"])
                            .add_filter("All", &["*"])
                            .show_open_single_file()
                            .unwrap();
                    }
                    match path {
                        Some(path) => {
                            *table = Some(parse_columns(path.to_path_buf()).unwrap());
                        }
                        None => return,
                    };
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            match path {
                Some(path) => {
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    ui.heading(filename);
                }
                None => {
                    ui.heading("");
                }
            };

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("File Contents");

            let text_style = egui::TextStyle::Body;
            let row_height = ui.fonts()[text_style].row_height();
            let num_rows = match table {
                Some(table) => table.len(),
                None => 0,
            };
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_right()
                .show(ui, |ui| {
                    let mut text = String::new();
                    match table {
                        Some(table) => {
                            for column in table.iter() {
                                text.push_str(&column.header);
                                text.push(',');
                            }
                            text.pop();
                            ui.label(text);
                            //let mut table_local
                            let column_iter =
                                PivotIter(table.iter().map(|x| x.column.iter()).collect());
                            for row in column_iter {
                                let mut txt = String::new();
                                for column in row {
                                    txt.push_str(&column.to_string());
                                    txt.push_str(",");
                                }
                                txt.pop();
                                ui.label(txt);
                            }
                        }

                        None => return,
                    }

                    //let zipper: Vec<_> = (0..).zip(table).collect();
                    //println!("{:?}", zipper);

                    //for column in table.iter() {
                    //    rows = column.zip()
                    //}
                });

            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
