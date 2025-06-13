use eframe::{egui, App, Frame, NativeOptions};
use egui::{Color32, Pos2, Stroke, Vec2, Rect, Rounding};
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
pub struct Node {
    id: Uuid,
    name: String,
    position: Pos2,
    width: f32,
    height: f32,
    inputs: Vec<Uuid>,
    outputs: Vec<Uuid>,
}

impl Node {
    fn new(name: String, position: Pos2) -> Self {
        Node {
            id: Uuid::new_v4(),
            name,
            position,
            width: 100.0,
            height: 50.0,
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn rect(&self) -> Rect {
        Rect::from_min_size(self.position, Vec2::new(self.width, self.height))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connection {
    id: Uuid,
    output_node: Uuid,
    input_node: Uuid,
}

impl Connection {
    fn new(output_node: Uuid, input_node: Uuid) -> Self {
        Connection {
            id: Uuid::new_v4(),
            output_node,
            input_node,
        }
    }
}


#[derive(Serialize, Deserialize, Default)]
pub struct FlowForgeApp {
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    selected_node: Option<Uuid>,
    new_connection_start: Option<Uuid>,
    graph: Graph<Uuid, Uuid> // Graph of Node Ids and Connection Ids
}

impl FlowForgeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let mut app = Self::default();

        // Create some initial nodes
        app.nodes.push(Node::new("Node 1".to_string(), Pos2::new(50.0, 50.0)));
        app.nodes.push(Node::new("Node 2".to_string(), Pos2::new(200.0, 150.0)));

        // Update graph
        app.update_graph();

        app
    }

    fn update_graph(&mut self) {
        self.graph = Graph::new();

        let node_indices: std::collections::HashMap<Uuid, _> = self.nodes.iter().map(|n| (n.id, self.graph.add_node(n.id))).collect();

        for conn in &self.connections {
            let output_index = node_indices.get(&conn.output_node).unwrap();
            let input_index = node_indices.get(&conn.input_node).unwrap();
            self.graph.add_edge(*output_index, *input_index, conn.id);
        }
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Draw connections first, so nodes are on top
            for connection in &self.connections {
                if let (Some(output_node), Some(input_node)) = (self.nodes.iter().find(|n| n.id == connection.output_node), self.nodes.iter().find(|n| n.id == connection.input_node)) {
                    let output_pos = output_node.rect().right_center();
                    let input_pos = input_node.rect().left_center();

                    let stroke = Stroke::new(2.0, Color32::DARK_GRAY);
                    ui.painter().line_segment([output_pos, input_pos], stroke);
                }
            }

            for node in &mut self.nodes {
                let id = egui::Id::new(node.id);
                let node_rect = node.rect();

                let response = ui.allocate_ui_at_rect(node_rect, |ui| {
                    ui.set_width(node.width);
                    ui.set_height(node.height);
                    let frame = egui::Frame::none().fill(Color32::LIGHT_BLUE).rounding(Rounding::same(5.0));
                    frame.show(ui, |ui| {
                        ui.label(&node.name);
                    });
                }).response;

                let is_being_dragged = ui.memory(|mem| mem.is_being_dragged(id));

                if response.hovered() {
                    ui.output_mut().cursor_icon = egui::CursorIcon::Grab;
                }

                if is_being_dragged {
                    ui.output_mut().cursor_icon = egui::CursorIcon::Grabbing;
                    node.position += response.drag_delta();
                }

                if response.dragged() {
                    self.selected_node = Some(node.id);
                }

                if response.drag_released() {
                    self.selected_node = None;
                }

                // Output connector
                let output_connector_rect = Rect::from_center_size(
                    node.rect().right_center(),
                    Vec2::new(10.0, 10.0),
                );

                let output_connector_response = ui.allocate_ui_at_rect(output_connector_rect, |ui|{
                    ui.allocate_space(ui.available_size());
                }).response;

                if output_connector_response.hovered() {
                    ui.output_mut().cursor_icon = egui::CursorIcon::Crosshair;
                }

                if output_connector_response.clicked() {
                    self.new_connection_start = Some(node.id);
                }
                ui.painter().circle_filled(output_connector_rect.center(), 5.0, Color32::DARK_GREEN);

                // Input connector
                let input_connector_rect = Rect::from_center_size(
                    node.rect().left_center(),
                    Vec2::new(10.0, 10.0),
                );
                let input_connector_response = ui.allocate_ui_at_rect(input_connector_rect, |ui|{
                    ui.allocate_space(ui.available_size());
                }).response;

                if input_connector_response.hovered() {
                    ui.output_mut().cursor_icon = egui::CursorIcon::Crosshair;
                }

                if let Some(start_node_id) = self.new_connection_start {
                    if input_connector_response.clicked() {
                        self.connections.push(Connection::new(start_node_id, node.id));
                        self.new_connection_start = None;
                        self.update_graph();
                    }
                }

                ui.painter().circle_filled(input_connector_rect.center(), 5.0, Color32::DARK_RED);
            }

            // Draw a line while creating new connection
            if let Some(start_node_id) = self.new_connection_start {
                if let Some(start_node) = self.nodes.iter().find(|n| n.id == start_node_id) {
                    let start_pos = start_node.rect().right_center();
                    let end_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(start_pos); //default to start pos if no hover
                    let stroke = Stroke::new(2.0, Color32::LIGHT_GREEN);
                    ui.painter().line_segment([start_pos, end_pos], stroke);
                }
            }

            // Example usage: Add a button to create a new node
            if ui.button("Add Node").clicked() {
                self.nodes.push(Node::new("New Node".to_string(), Pos2::new(100.0, 100.0)));
                self.update_graph();
            }
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}