use eframe::{egui, App, Frame, NativeOptions};
use egui::{Color32, Id, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "FlowForge",
        native_options,
        Box::new(|cc| Box::new(FlowForgeApp::new(cc))),
    );
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeData {
    pub value: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub position: Pos2,
    pub data: NodeData,
}

impl Node {
    pub fn new(name: String, position: Pos2) -> Self {
        Node {
            id: Uuid::new_v4(),
            name,
            position,
            data: NodeData { value: 0 },
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

    fn show(&mut self, ui: &mut Ui) -> egui::Response {
        let node_id = Id::new(self.node.id);
        let desired_size = Vec2::new(100.0, 50.0);

        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

        let visuals = ui.style().interact(&response);
        let rect = rect.expand(visuals.expansion);

        ui.painter().rect(
            rect,
            5.0,
            visuals.bg_fill,
            Stroke::new(2.0, visuals.fg_stroke.color),
        );

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(&self.node.name);
                ui.label(format!("Value: {}", self.node.data.value));
            });
        });

        if response.dragged() {
            self.node.position += response.drag_delta();
        }

        response
    }
}

#[derive(Serialize, Deserialize)]
pub struct FlowForgeApp {
    nodes: Vec<Node>,
    #[serde(skip)]
    graph: Graph<Uuid, ()>,
}

impl FlowForgeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.

        let mut app = FlowForgeApp {
            nodes: vec![
                Node::new("Node A".to_string(), Pos2::new(100.0, 100.0)),
                Node::new("Node B".to_string(), Pos2::new(300.0, 200.0)),
            ],
            graph: Graph::new(),
        };
        app.build_graph();
        app
    }

    fn build_graph(&mut self) {
        self.graph = Graph::new();
        let mut node_indices = std::collections::HashMap::new();
        for node in &self.nodes {
            let node_index = self.graph.add_node(node.id);
            node_indices.insert(node.id, node_index);
        }

        // Example edge
        if self.nodes.len() >= 2 {
            if let (Some(&node_a_id), Some(&node_b_id)) = (node_indices.get(&self.nodes[0].id), node_indices.get(&self.nodes[1].id)) {
                self.graph.add_edge(node_a_id, node_b_id, ());
            }
        }
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");

            for node in &mut self.nodes {
                let mut widget = NodeWidget::new(node);
                let response = widget.show(ui);
            }
        });
    }
}