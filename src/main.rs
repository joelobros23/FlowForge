// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, CreationContext};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Direction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    id: Uuid,
    name: String,
    node_type: NodeType,
    value1: f64, // Example data: first input value
    value2: f64, // Example data: second input value
    result: f64, // Store the result of node execution
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum NodeType {
    Add,
    Display,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: Uuid::new_v4(),
            name: "New Node".to_string(),
            node_type: NodeType::Add, // default node type
            value1: 0.0,
            value2: 0.0,
            result: 0.0,
        }
    }
}

#[derive(Default)]
struct FlowForgeApp {
    workflow: Workflow,
    run_enabled: bool,
    log: String, // added for logging
}

impl FlowForgeApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        FlowForgeApp {
            workflow: Workflow::new(),
            run_enabled: true, // enable run button by default
            log: String::new(),
        }
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");

            // Workflow editor (simplified for this example)
            ui.label("Workflow Editor (simplified)");
            if ui.button("Add Node").clicked() {
                self.workflow.add_node(Node::default());
            }

            if ui.button("Run").clicked() && self.run_enabled {
                self.execute_workflow();
            }

            // Display workflow graph (very basic)
            ui.label(format!("Nodes: {}", self.workflow.graph.node_count()));
            ui.label(format!("Edges: {}", self.workflow.graph.edge_count()));

            ui.separator();

            // Display log
            ui.label("Execution Log:");
            ui.label(format!("{}", self.log));
        });
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Workflow {
    graph: Graph<Node, ()>,
}

impl Workflow {
    fn new() -> Self {
        Workflow {
            graph: Graph::new(),
        }
    }
    fn add_node(&mut self, node: Node) -> NodeIndex {
        self.graph.add_node(node)
    }
}

impl FlowForgeApp {
    fn execute_workflow(&mut self) {
        self.log.clear(); // Clear previous log
        let mut results = std::collections::HashMap::new();

        for node_index in self.workflow.graph.node_indices() {
            let node = self.workflow.graph.node_weight(node_index).unwrap().clone();
            match node.node_type {
                NodeType::Add => {
                    let result = node.value1 + node.value2;
                    self.log.push_str(&format!("Adding {} + {} = {}\n", node.value1, node.value2, result));

                    // Store the result in the results map for later use
                    results.insert(node.id, result);

                    // Update the node in the graph with the result
                    let mut updated_node = self.workflow.graph.node_weight(node_index).unwrap().clone();
                    updated_node.result = result;
                    self.workflow.graph[node_index] = updated_node;
                }
                NodeType::Display => {
                    // Find the input value (assuming it comes from an edge)
                    let mut input_value = 0.0;
                    for neighbor in self.workflow.graph.neighbors_directed(node_index, Direction::Incoming) {
                        let neighbor_node = self.workflow.graph.node_weight(neighbor).unwrap();
                        if let Some(value) = results.get(&neighbor_node.id) {
                            input_value = *value;
                            break;
                        }
                    }
                    self.log.push_str(&format!("Displaying: {}\n", input_value));
                }
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
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
