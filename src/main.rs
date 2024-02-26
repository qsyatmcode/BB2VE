use serde_json;
use log;
use eframe::egui::{Button, CentralPanel, Context, Vec2, ViewportBuilder, Widget};
use eframe::{egui, Frame, NativeOptions, run_native};

use bb2ve::types::{self, bb, ve};

fn main() {
    let app = Bb2veApp {
        loaded_files: Vec::<String>::new(),
        converted: false,
    };
    let win_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2 { x: 710.0, y: 510.0 })
            .with_min_inner_size(Vec2 { x: 700.0, y: 500.0 })
            .with_drag_and_drop(true),
        ..Default::default()
    };

    run_native("BB2VE Converter", win_options, Box::new(|cc| Box::new(Bb2veApp::new(cc))));
}

fn convert(path: &String) {
    use std::{process, fs, time::Instant};

    let json_data = match fs::read_to_string(path) {
        Ok(r) => match serde_json::from_str::<bb::Obj>(&r) {
            Ok(j) => j,
            Err(e) => {
                println!("{}", e.to_string());
                process::exit(1);
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1);
        }
    };
    let mut primitive_count = 0;
    let now = Instant::now();
    
    if let Ok(out) = bb2ve::create_outobj(&mut primitive_count, String::from("TEXTURE_NAME"), json_data) {
        let elapsed_time = now.elapsed();
        println!("{} primitives converted in {:.2?}", primitive_count, elapsed_time);
        fs::write(format!("{}_{}", path, "output.json"), serde_json::to_string_pretty(&out).unwrap()).expect("Error of writing");
    } else {
        process::exit(69);
    }
}

#[derive(Default)]
struct Bb2veApp {
    loaded_files: Vec<String>,
    converted: bool,
}

impl Bb2veApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(2.0);
        Self::default()
    }
}

impl eframe::App for Bb2veApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {
                ui.heading("BB2VE Converter");
            });

            ui.add_space(50.0);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui.button("Open file..").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("BB Model", &["json"]).pick_file() {
                        let path = vec![path.display().to_string()];
                        self.loaded_files = path.clone();
                        self.converted = false;
                    }
                }
                ui.add_space(20.0);
                if self.loaded_files.len() > 0 {
                    //let convert_button = Button::new("text");
                    if ui.button(format!("Convert {}", self.loaded_files[0])).clicked() {
                        convert(&self.loaded_files[0]);
                        self.converted = true;
                    }
                }
                ui.add_space(10.0);
                if self.converted {
                    ui.label("Succefully converted!");
                }
            });
        });

        ctx.input(|is|{
            if !is.raw.dropped_files.is_empty() {
                let clone = is.raw.dropped_files.clone().into_iter().map(|e| e.path.unwrap().display().to_string()).collect();
                self.loaded_files = clone;
            }
        });
    }
}