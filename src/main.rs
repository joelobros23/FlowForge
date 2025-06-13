use eframe::{egui, App, CreationContext, Frame};

#[derive(Default)]
struct FlowForgeApp {
    // Add any state you need here
}

impl FlowForgeApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.
        // Load custom icons etc here.
        FlowForgeApp::default()
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        // Handle New action
                        println!("New clicked");
                    }

                    if ui.button("Open").clicked() {
                        // Handle Open action
                        println!("Open clicked");
                    }

                    if ui.button("Save").clicked() {
                        // Handle Save action
                        println!("Save clicked");
                    }

                    if ui.separator().above_auto_layout() {

                    }
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "FlowForge",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(FlowForgeApp::new(cc))
        }),
    )
}
