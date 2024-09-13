#![cfg_attr(not(test), windows_subsystem = "windows")]

mod fonts;
use eframe::egui::{self, DroppedFile};
use std::{ffi::OsStr, path::Path};

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
    dropped_file: Option<DroppedFile>,
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
            if self.dropped_file.is_none() && self.error_message.is_empty() {
                ui.heading("Drag & Drop epub file");
            } else if !self.error_message.is_empty() {
                ui.label(self.error_message.as_str());
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading(
                        get_name_of_dropped_file(self.dropped_file.as_ref().unwrap())
                            .unwrap_or_default(),
                    );
                    ui.separator();
                    for toc in &self.toc_list {
                        ui.label(toc);
                    }
                });
            }
        });
        self.catch_drop_file(ctx);
    }
}

impl Inspector {
    fn catch_drop_file(&mut self, ctx: &egui::Context) {
        if ctx.input(|x| x.raw.dropped_files.is_empty() || x.raw.dropped_files.len() > 1) {
            return;
        }
        let dropped_file = ctx.input(|x| x.raw.dropped_files.first().cloned().unwrap());
        if dropped_file.path.is_some() {
            self.read_epub(dropped_file.path.as_ref().unwrap());
        }
        self.dropped_file = Some(dropped_file);
    }
    fn read_epub(&mut self, path: impl AsRef<Path>) {
        let Ok(file) = epub::doc::EpubDoc::new(path) else {
            self.error_message = "Invalid epub file".to_string();
            return;
        };
        self.toc_list = file.toc.into_iter().map(|x| x.label).collect();
    }
}

fn get_name_of_dropped_file(dropped_file: &DroppedFile) -> Option<String> {
    dropped_file
        .path
        .as_ref()
        .map(AsRef::as_ref)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(ToOwned::to_owned)
}
