use eframe::egui;

pub fn create(ctx: &egui::Context, current_file: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let content: String = current_file.to_owned();
        ui
            .add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(current_file).frame(false).code_editor()
            )
    });
}
