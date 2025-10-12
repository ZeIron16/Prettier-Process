use crate::cli;
use eframe::egui;
use std::sync::mpsc::Receiver;

pub struct MyApp {
    pub input: String,
    pub output: String,
    pub cli_hist: Vec<String>,
    pub hist_index: Option<usize>,
    pub downloading: bool,
    pub download_progress: f32,
    pub download_rx: Option<Receiver<f32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            cli_hist: Vec::new(),
            hist_index: None,
            downloading: false,
            download_progress: 0.,
            download_rx: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.cli_hist.last() != None {
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .max_height(ui.available_height() - 40.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.output.as_str())
                                .font(egui::TextStyle::Monospace)
                                .interactive(true)
                                .desired_width(f32::INFINITY),
                        );
                    });
                ui.add_space(-35.0);
                if let Some(rx) = &self.download_rx {
                    while let Ok(p) = rx.try_recv() {
                        self.download_progress = p;
                        if p >= 100.0 {
                            self.downloading = false;
                            self.download_rx = None;
                            break;
                        }
                    }
                    ctx.request_repaint();
                }

                if self.downloading {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Download:");
                        ui.add(
                            egui::ProgressBar::new((self.download_progress / 100.0) as f32)
                                .text(format!("{:.1}%", self.download_progress)),
                        );
                    });
                }
                ui.separator();
                ui.add_space(3.0);
            }

            if !self.downloading {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(">")
                            .monospace()
                            .color(egui::Color32::WHITE),
                    );

                    let response = ui.add_sized(
                        [ui.available_width(), ui.available_height()],
                        egui::TextEdit::singleline(&mut self.input)
                            .font(egui::TextStyle::Monospace)
                            .frame(false),
                    );

                    response.request_focus();

                    if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                        if !self.cli_hist.is_empty() {
                            match self.hist_index {
                                None => {
                                    self.hist_index = Some(self.cli_hist.len() - 1);
                                    self.input = self.cli_hist[self.hist_index.unwrap()].clone();
                                }
                                Some(idx) if idx > 0 => {
                                    self.hist_index = Some(idx - 1);
                                    self.input = self.cli_hist[self.hist_index.unwrap()].clone();
                                }
                                _ => {}
                            }
                        }
                    }

                    if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                        match self.hist_index {
                            Some(idx) if idx < self.cli_hist.len() - 1 => {
                                self.hist_index = Some(idx + 1);
                                self.input = self.cli_hist[self.hist_index.unwrap()].clone();
                            }
                            Some(_) => {
                                self.hist_index = None;
                                self.input.clear();
                            }
                            None => {}
                        }
                    }

                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.input.trim().is_empty() {
                            self.execute_command();
                        }
                        response.request_focus();
                    }
                });
            }
        });
    }
}

impl MyApp {
    fn execute_command(&mut self) {
        if self.downloading {
            self.output
                .push_str("Download in progress, please wait.\n\n");
            self.input.clear();
            return;
        }

        let input = self.input.trim().to_string();

        self.output.push_str(&format!("> {}\n", input));

        if self.cli_hist.last() != Some(&input) {
            self.cli_hist.push(input.clone());
        }
        self.hist_index = None;

        let args: Vec<&str> = input.split_whitespace().collect();

        match cli::build_cli().try_get_matches_from(args) {
            Ok(matches) => match cli::handle_command(matches, self) {
                Ok(result) => {
                    if !result.is_empty() {
                        self.output.push_str(&format!("{}\n", result));
                    }
                }
                Err(e) => {
                    self.output.push_str(&format!("Erreur: {}\n", e));
                }
            },
            Err(e) => {
                self.output.push_str(&format!("{}\n", e));
            }
        }

        self.output.push('\n');
        self.input.clear();
    }
}

fn logo() -> std::sync::Arc<egui::IconData> {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/Chuka_logo.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    std::sync::Arc::new(egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}

pub fn shell() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Chuka")
            .with_icon(logo()),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Chuka",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)),
    );
}