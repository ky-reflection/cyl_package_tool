use crate::cylheim_tools::CylheimChart;
use eframe::egui::{self, CentralPanel, RichText};
use rfd::FileDialog;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

// 子应用 C2to1Tool
#[derive(Default)]
pub struct C2to1Tool {
    selected_file: Option<PathBuf>,
    message: String,
}

impl eframe::App for C2to1Tool {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading("Cytus Chart 2 to 1 Converter");
                ui.add_space(15.0);
                if ui.button("Select File").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("text", &["txt", "json"])
                        .pick_file()
                    {
                        if let Ok(f) = fs::read_to_string(&path) {
                            let result: Result<CylheimChart, _> = serde_json::from_str(&f);
                            if let Ok(cylchart) = result {
                                match cylchart.to_cytus1_chart_with_pageshift(false) {
                                    Ok(chart) => {
                                        self.selected_file = Some(path.clone());
                                        let new_file_path = save_new_file(
                                            &path,
                                            &chart.to_string(),
                                            "converted",
                                            "txt",
                                        )
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
                            } else {
                                self.selected_file = Some(path.clone());
                                self.message =
                                    format!("File you selected is not a valid Cytus2 chart.");
                            }
                        } else {
                            self.selected_file = Some(path.clone());
                            self.message = format!("File you selected is not a text file.");
                        }
                    }
                }
                ui.add_space(15.0);
                ui.label(RichText::new(&self.message).size(18.0));
            });
        });
    }
}

pub fn save_new_file(
    original_path: &Path,
    content: &str,
    msg: &str,
    ext: &str,
) -> io::Result<PathBuf> {
    let mut new_file_path = original_path.to_path_buf();
    if let Some(stem) = new_file_path.file_stem() {
        let new_file_name = format!("{}_{}.{}", stem.to_string_lossy(), msg.to_string(), ext);
        new_file_path.set_file_name(new_file_name);
    }
    let mut file = File::create(&new_file_path)?;
    file.write(content.as_bytes())?;
    Ok(new_file_path)
}
