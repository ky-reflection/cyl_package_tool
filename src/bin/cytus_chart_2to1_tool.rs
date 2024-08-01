use cyl_package_tool::cylheim_tools::CylheimChart;
use eframe::egui::{self, CentralPanel, Label, Layout, RichText};
use rfd::FileDialog;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cytus Chart 2 to 1 Converter",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    selected_file: Option<PathBuf>,
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            message: "No file selected".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading("Cytus Chart 2 to 1 Converter");
                ui.add_space(15.0);

                if ui.button("Select File").clicked() {
                    if let Some(path) = FileDialog::new().pick_file() {
                        let f = fs::read_to_string(&path).unwrap();
                        let cylchart: CylheimChart = serde_json::from_str(&f).unwrap();
                        match cylchart.to_cytus1_chart_with_pageshift(false) {
                            Ok(chart) => {
                                self.selected_file = Some(path.clone());
                                let new_file_path =
                                    save_new_file(&path, &chart.to_string(), "converted", "txt")
                                        .unwrap();
                                self.message = format!(
                                    "File processed successfully: {:?}",
                                    new_file_path.file_name().unwrap()
                                );
                            }
                            Err(err) => {
                                self.selected_file = Some(path.clone());
                                self.message = format!("Error processing file: {:?}", err);
                            }
                        }
                    }
                }
                ui.add_space(15.0);
                ui.label(RichText::new(&self.message).size(18.0));
                // ui.add_space(20.0);
                // if self.selected_file.is_some() && ui.button("Exit").clicked() {
                //     std::process::exit(0);
                // }
            });
        });
    }
}
fn save_new_file(original_path: &Path, content: &str, msg: &str, ext: &str) -> io::Result<PathBuf> {
    let mut new_file_path = original_path.to_path_buf();
    if let Some(stem) = new_file_path.file_stem() {
        let new_file_name = format!("{}_{}.{}", stem.to_string_lossy(), msg.to_string(), ext);
        new_file_path.set_file_name(new_file_name);
    }
    let mut file = File::create(&new_file_path)?;
    file.write(content.as_bytes())?;
    Ok(new_file_path)
}
