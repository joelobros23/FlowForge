use eframe::{egui, App, CreationContext, Frame};
use egui::Color32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use petgraph::Graph;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    id: Uuid,
    name: String,
    #[serde(skip)] // Don't serialize position, as it's UI specific
    position: egui::Pos2,
}

impl Node {
    pub fn new(name: String, position: egui::Pos2) -> Self {
        Node {
            id: Uuid::new_v4(),
            name, position,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct FlowForgeApp {
    nodes: Vec<Node>,
    node_palette: Vec<String>,
    graph: Graph<Uuid, ()>,
    dragged_node: Option<String>,
}

impl FlowForgeApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.node_palette = vec!["Input".to_string(), "Output".to_string(), "Transform".to_string()];
        app
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::SidePanel::left("node_palette").show(ctx, |ui| {
            ui.heading("Node Palette");
            for node_type in &self.node_palette {
                if ui.button(node_type).drag_source(|ui, _| {
                    self.dragged_node = Some(node_type.clone());
                    ui.label(format!("Dragging {}", node_type));
                }).is_dragged() {
                    // This needs to be non-empty to trigger dragging
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Workflow Editor");

            let painter = ui.painter();

            // Draw nodes
            for node in &mut self.nodes {
                let rect = egui::Rect::from_center_size(
                    node.position,
                    egui::Vec2::new(80.0, 40.0),
                );
                painter.rect_filled(rect, egui::Rounding::same(5.0), Color32::WHITE);
                painter.rect_stroke(rect, egui::Rounding::same(5.0), egui::Stroke::new(1.0, Color32::BLACK));
                painter.text(rect.center(), egui::Align2::CENTER_CENTER, &node.name, egui::FontId::default(), Color32::BLACK);

                let response = ui.allocate_rect(rect, egui::Sense::drag());
                if response.dragged() {
                    node.position += response.drag_delta();
                }
            }

            // Handle dropping new nodes
            let drop_area = ui.available_rect_before_wrap();
            let drop_response = ui.allocate_rect(drop_area, egui::Sense::hover());
            if drop_response.hovered() && ui.input(|i| i.pointer.is_dragging()) {
                ui.output_mut().cursor_icon = egui::CursorIcon::Move;
            }

            if drop_response.hovered() && ui.input(|i| i.pointer.any_released()) {
                if let Some(node_type) = &self.dragged_node {
                    let mouse_pos = ui.input(|i| i.pointer.latest_pos()).unwrap_or(egui::Pos2::ZERO);
                    let new_node = Node::new(node_type.clone(), mouse_pos);
                    self.nodes.push(new_node);
                }
                self.dragged_node = None; // Reset dragged node
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "FlowForge",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.env.http);
            Box::new(FlowForgeApp::new(cc))
        }),
    )
}
