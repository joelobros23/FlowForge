use eframe::{egui, App, CreationContext};
use egui::{Color32, Frame, Margin, Pos2, Rect, Rounding, Stroke, Ui, Vec2};
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    id: Uuid,
    name: String,
    position: Pos2,
    data: String,
}

impl Node {
    pub fn new(name: String, position: Pos2, data: String) -> Self {
        Node {
            id: Uuid::new_v4(),
            name,
            position,
            data,
        }
    }
}

struct NodeWidget<'a> {
    node: &'a mut Node,
}

impl<'a> NodeWidget<'a> {
    fn new(node: &'a mut Node) -> Self {
        NodeWidget { node }
    }

    fn show(&mut self, ui: &mut Ui) {
        let frame = Frame::default()
            .fill(Color32::LIGHT_BLUE)
            .stroke(Stroke::new(1.0, Color32::BLACK))
            .rounding(Rounding::same(5.0))
            .margin(Margin::same(5.0));

        let node_id = self.node.id;
        let node_name = self.node.name.clone();
        let node_pos = self.node.position;

        ui.allocate_ui_at_rect(
            Rect::from_min_size(node_pos, Vec2::new(100.0, 50.0)),
            |ui| {
                frame.show(ui, |ui| {
                    ui.set_width(100.0);
                    ui.label(node_name);
                    ui.label(format!("ID: {}", node_id));
                    ui.label(format!("Data: {}", self.node.data));
                });
            },
        );

        if ui
            .rect_contains_pointer(Rect::from_min_size(node_pos, Vec2::new(100.0, 50.0)))
            && ui.input(|i| i.pointer.is_dragging())
        {
            self.node.position += ui.input(|i| i.pointer.delta());
        }
    }
}

#[derive(Default)]
struct FlowForgeApp {
    nodes: Vec<Node>,
    graph: Graph<Uuid, ()>,
}

impl FlowForgeApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");

            if self.nodes.is_empty() {
                self.nodes.push(Node::new("Node 1".to_string(), Pos2::new(100.0, 100.0), "Some Data".to_string()));
                self.nodes.push(Node::new("Node 2".to_string(), Pos2::new(300.0, 150.0), "More Data".to_string()));
            }

            for node in &mut self.nodes {
                NodeWidget::new(node).show(ui);
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
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(FlowForgeApp::new(cc))
        }),
    )
}
