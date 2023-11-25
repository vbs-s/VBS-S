use eframe::egui;
use vbs_s::panels;

// Define App
struct App {
    current_file: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_file: "Thank you downloading VBS-S ♡".to_owned(),
        }
    }
}

// Run the app
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200., 800.]),
        ..Default::default()
    };
    eframe::run_native(
        "VBS-S",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<App>::default()
        }),
    )
}

// App behavior
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        panels::tool_bar::create(ctx);
        panels::section_bar::create(ctx);
        panels::file_tree::create(ctx);
        panels::file_tab_bar::create(ctx);
        panels::terminal::create(ctx);
        panels::file_editor::create(ctx, &mut self.current_file);
    }
}
