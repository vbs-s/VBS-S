use eframe::egui;

pub fn create(ctx: &egui::Context) {
    egui::SidePanel::left("file_tree")
    .frame(egui::Frame {
        inner_margin: egui::Margin::same(4.),
        fill: egui::Color32::from_rgb(30, 28, 26),
        ..Default::default()
    })
    .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("File Tree");
        });
        ui.label("TODO: add collapsible file tree");
    });
}
