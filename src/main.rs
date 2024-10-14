use eframe::egui::{self, Color32};


#[derive(Default)]
struct Pretty {
    output: String,
    written: bool,
    failed: bool,
}

impl eframe::App for Pretty {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .show(ctx, |ui| {

                let mut user_requested_change = false;
                let mut user_has_output = false;

                ui.hyperlink_to("@dutchaen on git <3", "https://www.github.com/dutchaen");

                let pick_from_file_button = ui.button("Read from JavaScript file");
                let paste_from_clipboard_button = ui.button("Read JavaScript from clipboard");

                if pick_from_file_button.clicked() {
                    user_requested_change = true;
                    self.failed = true;

                    let file = rfd::FileDialog::new()
                        .add_filter("JavaScript files", &["js"])
                        .set_directory("/")
                        .pick_file();

                    if let Some(file) = file {

                        let content = std::fs::read_to_string(file);
                        if let Ok(content) = content {
                            let (contents, _) = prettify_js::prettyprint(&content);
                            self.output = contents;
                            user_has_output = true;
                            self.failed = false;
                        }

                    } 
                }

                if paste_from_clipboard_button.clicked() {
                    user_requested_change = true;
                    self.failed = true;

                    let mut clipboard = clippers::Clipboard::get();
                    if let Some(clipboard_content) = clipboard.read() {
                        if let Some(content) = clipboard_content.as_text() {
                            let (contents, _) = prettify_js::prettyprint(&content);
                            self.output = contents;
                            user_has_output = true;
                            self.failed = false;
                        }
                    }
                }


                if user_requested_change && user_has_output {
                    if let Ok(_) = std::fs::write("clean.js", &self.output) {
                        self.written = true;
                        self.failed = false;
                    }
                }

                if self.failed {
                    ui.colored_label(Color32::DARK_RED, "Failed to pretty JavaScript :(");
                    self.written = false;
                }

                if self.written {
                    ui.colored_label(Color32::GREEN, "Written to \"clean.js\"!");
                    self.failed = false;
                }

            }
        );
    }
}


fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 100.0])
            .with_window_type(egui::X11WindowType::Combo)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "pretty my js pls",
        options,
        Box::new(|cc| {

            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(Pretty::default()))
        }),
    )
}
