use eframe::egui;

pub fn create(ctx: &egui::Context) {
    egui::TopBottomPanel::top("file_tab_bar").show(ctx, |ui| {
        ui.label("TODO: add tabs based on open files");
    });
}
