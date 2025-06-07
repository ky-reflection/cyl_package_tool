#![windows_subsystem = "windows"]

// 移除未使用的导入
use cyl_package_tool::C2to1Tool;
use eframe::egui::{self, Color32, Frame, Margin, Stroke};
// 创建应用选择枚举
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Anchor {
    C2to1Tool,
}

impl Default for Anchor {
    fn default() -> Self {
        Self::C2to1Tool
    }
}

#[derive(Default)]
struct MainApp {
    c2to1_tool: C2to1Tool,
    selected_anchor: Anchor,
}

impl MainApp {
    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        vec![(
            "Cytus Chart 2 to 1",
            Anchor::C2to1Tool,
            &mut self.c2to1_tool as &mut dyn eframe::App,
        )]
        .into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor {
                app.update(ctx, frame);
            }
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let side_panel_frame = Frame {
            inner_margin: 8.0.into(),                                   // 内边距
            outer_margin: Margin::same(0.0),                            // 外边距
            rounding: 0.0.into(),                                       // 边角圆角
            fill: Color32::from_rgb(230, 230, 230),                     // 修改为淡灰色背景
            stroke: Stroke::new(1.0, Color32::from_rgb(200, 200, 200)), // 修改为淡灰色边框
            shadow: egui::Shadow::NONE,                                 // 设置无阴影
        };

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .frame(side_panel_frame)
            .show(ctx, |ui| {
                ui.heading("Applications");
                let mut selected_anchor = self.selected_anchor;
                for (name, anchor, _app) in self.apps_iter_mut() {
                    if ui
                        .selectable_label(selected_anchor == anchor, name)
                        .clicked()
                    {
                        selected_anchor = anchor;
                    }
                }
                self.selected_anchor = selected_anchor;
            });
        self.show_selected_app(ctx, frame);
    }
}

fn main() -> Result<(), eframe::Error> {
    let font_path = "fonts/simhei.ttf";
    let font_data = match std::fs::read(font_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read font file: {}", e);
            return eframe::run_native(
                "Cytus Toolkit",
                eframe::NativeOptions {
                    viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 450.0]),
                    ..Default::default()
                },
                Box::new(|_| Ok(Box::new(MainApp::default()))),
            );
        }
    };

    let ubuntu_font_path = "fonts/ubuntu.ttf";
    let ubuntu_font_data = match std::fs::read(ubuntu_font_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read ubuntu font file: {}", e);
            // 若读取失败，使用默认配置继续运行
            return eframe::run_native(
                "Cytus Toolkit",
                eframe::NativeOptions {
                    viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 450.0]),
                    ..Default::default()
                },
                Box::new(|_| Ok(Box::new(MainApp::default()))),
            );
        }
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Cytus Toolkit",
        native_options,
        Box::new(|cc| {
            let mut font_definitions = egui::FontDefinitions::default();
            font_definitions.font_data.insert(
                "ubuntu_font".to_owned(),
                egui::FontData::from_owned(ubuntu_font_data),
            );
            font_definitions.font_data.insert(
                "chinese_font".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            if let Some(proportional_fonts) = font_definitions
                .families
                .get_mut(&egui::FontFamily::Proportional)
            {
                proportional_fonts.clear();
                proportional_fonts.push("ubuntu_font".to_owned());
                proportional_fonts.push("chinese_font".to_owned());
            }
            if let Some(monospace_fonts) = font_definitions
                .families
                .get_mut(&egui::FontFamily::Monospace)
            {
                monospace_fonts.clear();
                monospace_fonts.push("ubuntu_font".to_owned());
                monospace_fonts.push("chinese_font".to_owned());
            }
            cc.egui_ctx.set_fonts(font_definitions);
            Ok(Box::new(MainApp::default()))
        }),
    )
}
