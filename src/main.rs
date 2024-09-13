#![cfg_attr(not(test), windows_subsystem = "windows")]

mod fonts;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    native_options.viewport.decorations = Some(true);
    native_options.vsync = false;
    eframe::run_native(
        "Epub Toc Inspector",
        native_options,
        Box::new(|cc| Ok(Box::new(Inspector::new(cc)))),
    )
}

#[derive(Default)]
struct Inspector {
    selected_file: String,
    error_message: String,
    toc_list: Vec<String>,
}

impl Inspector {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 폰트 설정
        cc.egui_ctx.set_fonts(fonts::get_fonts());

        Default::default()
    }
}

impl eframe::App for Inspector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.selected_file.is_empty() && self.error_message.is_empty() {
                ui.heading("Drag & Drop epub file");
            } else if !self.error_message.is_empty() {
                ui.label(self.error_message.as_str());
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(&self.selected_file);
                    ui.separator();
                    for toc in &self.toc_list {
                        ui.label(toc);
                    }
                });
            }
        });
    }
}

impl Inspector {
    fn read_epub(&mut self, path: &str) {
        let Ok(file) = epub::doc::EpubDoc::new(path) else {
            self.error_message = "Invalid epub file".to_string();
            return;
        };
        self.toc_list = file.toc.into_iter().map(|x| x.label).collect();
    }
}
