use eframe::egui;

pub fn create(ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("terminal")
    .frame(egui::Frame {
        inner_margin: egui::Margin::same(4.),
        fill: egui::Color32::from_rgb(30, 28, 26),
        ..Default::default()
    })
    .show(ctx, |ui| {
        ui.heading("Terminal");
        ui.label("TODO: make this run commands");
    });
}
