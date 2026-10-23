#![windows_subsystem = "windows"]

use cyl_package_tool::cylheim_tools::CylheimChart;
use eframe::egui::{self, CentralPanel, RichText};
use rfd::FileDialog;
use serde_json::Value;
use std::{fs, path::PathBuf};

const REPOSITORY_URL: &str = "https://github.com/ky-reflection/cyl_package_tool";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 450.0])
            .with_transparent(false),
        renderer: eframe::Renderer::Wgpu,
        wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
            // Avoid the OpenGL text flicker seen during interactive repaints on Windows.
            supported_backends: if cfg!(target_os = "windows") {
                eframe::wgpu::Backends::DX12
            } else {
                eframe::wgpu::Backends::PRIMARY
            },
            present_mode: eframe::wgpu::PresentMode::Fifo,
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Cytus Chart 2 to 1 Converter",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    selected_file: Option<PathBuf>,
    message: String,
    use_music_bpm: bool,
    github_mark: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_file: None,
            message: "No file selected".to_owned(),
            use_music_bpm: false,
            github_mark: None,
        }
    }
}

impl MyApp {
    fn conversion_help(ui: &mut egui::Ui) {
        ui.set_width(390.0);
        egui::Frame::none().inner_margin(6.0).show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.label(RichText::new("Conversion rules").size(16.0).strong());
            ui.weak("Cytus II JSON (.json / .txt)  /  Cytus I text chart");
            ui.add_space(6.0);

            ui.strong("Supported charts");
            egui::Grid::new("help_chart_requirements")
                .num_columns(2)
                .spacing([14.0, 4.0])
                .show(ui, |ui| {
                    ui.weak("Notes");
                    ui.label("Tap, hold and drag (types 0, 1, 3, 4)");
                    ui.end_row();
                    ui.weak("Pages");
                    ui.label("Equal length; actual ticks / time_base, not fixed 960");
                    ui.end_row();
                    ui.weak("Tempo");
                    ui.label("Constant effective tempo");
                    ui.end_row();
                });
            ui.label(RichText::new("Initial zero-tempo segment: first page must scan upward; effective tempo must start before two pages elapse.").size(12.0));
            ui.add_space(4.0);
            ui.separator();

            ui.strong("Timing & flashing BPM");
            egui::Grid::new("help_timing")
                .num_columns(2)
                .spacing([14.0, 5.0])
                .show(ui, |ui| {
                    ui.weak("PAGE_SHIFT");
                    ui.label("From the effective tempo's starting tick");
                    ui.end_row();
                    ui.label("BPM unchecked");
                    ui.monospace("240 / PAGE_SIZE");
                    ui.end_row();
                    ui.label("BPM checked");
                    ui.monospace("120,000,000 / tempo");
                    ui.end_row();
                });
            ui.label(RichText::new("Checked = musical BPM × 2. Page and note timing stay unchanged.").size(12.0));
            ui.label(RichText::new("start_offset_time / end_offset_time are not applied.").size(12.0));
            ui.add_space(4.0);
            ui.separator();

            ui.strong("Output");
            ui.monospace("<name>_converted.txt");
            ui.label(RichText::new("Saved beside the source file. An existing output with the same name is overwritten.").size(12.0));
        });
    }

    fn ui(&mut self, ctx: &egui::Context) {
        let github_mark = self.github_mark.get_or_insert_with(|| {
            ctx.load_texture(
                "github-mark",
                egui::ColorImage {
                    size: [32, 32],
                    pixels: include_bytes!("../../assets/github-mark-32.alpha")
                        .iter()
                        .map(|&alpha| egui::Color32::from_white_alpha(alpha))
                        .collect(),
                },
                egui::TextureOptions::LINEAR,
            )
        });
        egui::TopBottomPanel::bottom("footer")
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.weak("© 2024 ky_reflection");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let icon = egui::Image::new((github_mark.id(), egui::vec2(20.0, 20.0)))
                            .tint(ui.visuals().text_color());
                        if ui
                            .add(egui::ImageButton::new(icon).frame(false))
                            .on_hover_text("View source on GitHub")
                            .clicked()
                        {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(REPOSITORY_URL));
                        }
                    });
                });
                ui.add_space(4.0);
            });
        CentralPanel::default().show(ctx, |ui| {
            let help_rect = egui::Rect::from_min_size(
                egui::pos2(ui.max_rect().right() - 20.0, ui.max_rect().top()),
                egui::vec2(20.0, 20.0),
            );
            let help = ui.put(
                help_rect,
                egui::Button::new(RichText::new("?").size(14.0))
                    .small()
                    .rounding(10.0),
            );
            let help_popup = ui.make_persistent_id("conversion_help_popup");
            if help.clicked() {
                ui.memory_mut(|memory| memory.toggle_popup(help_popup));
            }
            if ui.memory(|memory| memory.is_popup_open(help_popup)) {
                egui::popup::popup_below_widget(
                    ui,
                    help_popup,
                    &help,
                    egui::popup::PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::Min),
                            Self::conversion_help,
                        );
                    },
                );
            } else if !help.clicked() {
                if help.is_pointer_button_down_on() {
                    // Keep the preview visible until release opens the persistent popup.
                    help.show_tooltip_ui(Self::conversion_help);
                } else {
                    help.on_hover_ui(Self::conversion_help);
                }
            }
            ui.vertical_centered(|ui| {
                ui.add_space(15.0);
                ui.heading("Cytus Chart 2 to 1 Converter");
                ui.add_space(15.0);
                if ui.button("Select File").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("text", &["txt", "json"])
                        .pick_file()
                    {
                        self.selected_file = Some(path);
                        self.message = "Ready to convert".to_owned();
                    }
                }
                if let Some(path) = &self.selected_file {
                    ui.label(path.file_name().unwrap_or_default().to_string_lossy())
                        .on_hover_text(path.display().to_string());
                }
                ui.add_space(15.0);
                let changed = ui.checkbox(&mut self.use_music_bpm, "Use tempo-based flashing BPM")
                    .on_hover_text("Checked: BPM = 120,000,000 / tempo.\nUnchecked: BPM = 240 / PAGE_SIZE.\nPage and note timing are unchanged.")
                    .changed();
                if changed && self.selected_file.is_some() {
                    self.message = "Options changed. Click Convert to apply.".to_owned();
                }
                ui.add_space(15.0);
                if ui.add_enabled(self.selected_file.is_some(), egui::Button::new("Convert")).clicked() {
                    self.message = match self.convert() {
                        Ok(path) => format!("File processed successfully: {}", path.file_name().unwrap_or_default().to_string_lossy()),
                        Err(error) => error,
                    };
                }
                ui.add_space(15.0);
                ui.label(RichText::new(&self.message).size(18.0));
            });
        });
    }

    fn convert(&self) -> Result<PathBuf, String> {
        let path = self.selected_file.as_ref().ok_or("No file selected")?;
        let text =
            fs::read_to_string(path).map_err(|error| format!("Cannot read file: {error}"))?;
        let value: Value = serde_json::from_str(text.trim_start_matches('\u{feff}'))
            .map_err(|error| format!("Invalid Cytus II JSON: {error}"))?;
        // Guard inputs that the original converter cannot safely process.
        if value["page_list"].as_array().map_or(true, Vec::is_empty) {
            return Err("The chart must contain at least one page.".to_owned());
        }
        let tempo_tick = value["tempo_list"]
            .as_array()
            .and_then(|list| list.last())
            .and_then(|tempo| tempo["tick"].as_u64())
            .unwrap_or(0);
        if value["note_list"].as_array().is_some_and(|notes| {
            notes
                .iter()
                .any(|note| note["tick"].as_u64().is_some_and(|tick| tick < tempo_tick))
        }) {
            return Err("Notes cannot precede the effective tempo.".to_owned());
        }
        let source: CylheimChart = serde_json::from_value(value)
            .map_err(|error| format!("Invalid Cytus II chart: {error}"))?;
        let chart = if self.use_music_bpm {
            source.to_cytus1_chart_with_music_bpm(false)
        } else {
            source.to_cytus1_chart_with_pageshift(false)
        }
        .map_err(|error| format!("Error processing file: {error}"))?;
        let output = path.with_file_name(format!(
            "{}_converted.txt",
            path.file_stem().unwrap_or_default().to_string_lossy()
        ));
        fs::write(&output, chart.to_string())
            .map_err(|error| format!("Cannot save file: {error}"))?;
        Ok(output)
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        // Match CentralPanel's opaque background instead of eframe's translucent dark clear.
        visuals.panel_fill.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
