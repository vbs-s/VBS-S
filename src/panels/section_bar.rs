use eframe::egui;

pub fn create(ctx: &egui::Context) {
    egui::SidePanel::left("section_bar")
        .max_width(32.)
        .frame(egui::Frame {
            inner_margin: egui::Margin::same(4.),
            fill: egui::Color32::from_rgb(30, 28, 26),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.image(egui::include_image!("../../assets/folder.svg"));
        });
}
