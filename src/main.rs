use eframe::egui;


// Define App
struct App {
    name: String,
    age: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

// Run the app
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
        create_file_tree(ctx);
        create_file_tab_bar(ctx);
        create_terminal(ctx);
        create_file_editor(ctx);
    }
}

fn create_file_tree(ctx: &egui::Context) {
    egui::SidePanel::left("file_tree").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("File Tree");
        });
        ui.label("TODO: add collapsible file tree");
    });
}

fn create_file_tab_bar(ctx: &egui::Context) {
    egui::TopBottomPanel::top("file_tab_bar").show(ctx, |ui| {
        ui.label("TODO: add tabs based on open files");
    });
}

fn create_terminal(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("terminal").show(ctx, |ui| {
        ui.heading("Terminal");
        ui.label("TODO: make this run commands");
    });
}

fn create_file_editor(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut "Thank you downloading VBS-S ♡").code_editor()
        );
    });
}
