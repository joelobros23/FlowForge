// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, CreationContext};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    id: Uuid,
    name: String,
    x: f32,
    y: f32,
    node_type: NodeType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum NodeType {
    Input,
    Add,
    Output,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Edge {
    source: Uuid,
    target: Uuid,
}

struct FlowForgeApp {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    graph: Graph<Uuid, ()>,
    run_result: String,
}

impl FlowForgeApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        let mut app = FlowForgeApp {
            nodes: vec![
                Node {
                    id: Uuid::new_v4(),
                    name: "Input 1".to_string(),
                    x: 50.0,
                    y: 50.0,
                    node_type: NodeType::Input,
                },
                Node {
                    id: Uuid::new_v4(),
                    name: "Input 2".to_string(),
                    x: 50.0,
                    y: 150.0,
                    node_type: NodeType::Input,
                },
                Node {
                    id: Uuid::new_v4(),
                    name: "Add".to_string(),
                    x: 200.0,
                    y: 100.0,
                    node_type: NodeType::Add,
                },
                Node {
                    id: Uuid::new_v4(),
                    name: "Output".to_string(),
                    x: 400.0,
                    y: 100.0,
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: app.nodes[0].id,
                    target: app.nodes[2].id,
                },
                Edge {
                    source: app.nodes[1].id,
                    target: app.nodes[2].id,
                },
                Edge {
                    source: app.nodes[2].id,
                    target: app.nodes[3].id,
                },
            ],
            graph: Graph::new(),
            run_result: "".to_string(),
        };

        // Build the petgraph graph from the node and edge data
        let mut graph = Graph::<Uuid, ()>::new();
        let mut node_indices = std::collections::HashMap::new();

        for node in &app.nodes {
            let node_index = graph.add_node(node.id);
            node_indices.insert(node.id, node_index);
        }

        for edge in &app.edges {
            if let (Some(&source_index), Some(&target_index)) = (
                node_indices.get(&edge.source),
                node_indices.get(&edge.target),
            ) {
                graph.add_edge(source_index, target_index, ());
            }
        }

        app.graph = graph;

        app
    }

    fn execute_flow(&mut self) {
        // Simple execution logic for the example: Add two input nodes and output the result.
        let mut input_values: Vec<i32> = Vec::new();
        for node in &self.nodes {
            if node.node_type == NodeType::Input {
                // Dummy input values
                if input_values.len() < 2 {
                    input_values.push(5); // Example input value
                }
            }
        }

        if input_values.len() != 2 {
            self.run_result = "Error: Not enough inputs".to_string();
            return;
        }

        let sum = input_values[0] + input_values[1];
        self.run_result = format!("Result: {}", sum);
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");

            for node in &mut self.nodes {
                ui.interact(egui::Rect::from_min_size(egui::pos2(node.x, node.y), egui::Vec2::new(50.0, 30.0)), egui::Id::new(node.id), egui::Sense::drag()).hovered();
            }

            if ui.button("Run").clicked() {
                self.execute_flow();
            }

            ui.label(&self.run_result);
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
