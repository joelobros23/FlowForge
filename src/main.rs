// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::plot::{Line, Plot, Value, Values};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Dot, Config};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn main() -> Result<(), eframe::Error {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "FlowForge",
        options,
        Box::new(|cc| {
            // This gives us image support: egui_extras::install_image_loaders(&cc.image_loaders);
            Box::new(FlowForgeApp::new(cc))
        }),
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeData {
    pub id: Uuid,
    pub node_type: String, // e.g., "Input", "Processing", "Output"
    pub data: serde_json::Value, // Generic data associated with the node
    pub position: [f32; 2], // x, y coordinates for display
}

impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            id: Uuid::new_v4(),
            node_type: "Default".to_string(),
            data: serde_json::json!({}),
            position: [0.0, 0.0],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub graph: DiGraph<NodeData, ()>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Workflow {
            name,
            graph: DiGraph::new(),
        }
    }

    pub fn add_node(&mut self, node_data: NodeData) -> NodeIndex {
        self.graph.add_node(node_data)
    }
}

struct FlowForgeApp {
    workflow: Workflow,
    new_node_type: String,
    new_node_data: String,
}

impl FlowForgeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Alternatively, you can load custom fonts with:
        //     cc.egui_ctx.set_fonts(FontDefinitions::default());

        let mut workflow = Workflow::new("My Workflow".to_string());
        let node1 = NodeData {
            id: Uuid::new_v4(),
            node_type: "Input".to_string(),
            data: serde_json::json!({ "value": 10 }),
            position: [10.0, 10.0],
        };
        let node2 = NodeData {
            id: Uuid::new_v4(),
            node_type: "Processing".to_string(),
            data: serde_json::json!({ "operation": "multiply", "factor": 2 }),
            position: [50.0, 50.0],
        };
        let node3 = NodeData {
            id: Uuid::new_v4(),
            node_type: "Output".to_string(),
            data: serde_json::json!({}),
            position: [90.0, 10.0],
        };

        let node1_index = workflow.add_node(node1);
        let node2_index = workflow.add_node(node2);
        let node3_index = workflow.add_node(node3);

        workflow.graph.add_edge(node1_index, node2_index, ());
        workflow.graph.add_edge(node2_index, node3_index, ());

        FlowForgeApp {
            workflow,
            new_node_type: "New Node Type".to_string(),
            new_node_data: "{}".to_string(),
        }
    }
}

impl eframe::App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FlowForge");

            ui.label(format!("Workflow Name: {}", self.workflow.name));

            // Display the graph as a DOT string
            let dot = Dot::with_config(&self.workflow.graph, &[Config::NodeIndexLabel, Config::EdgeNoLabel]);
            ui.label(format!("Graph (DOT format):\n{}", dot.to_string()));

            ui.separator();

            ui.label("Add New Node:");
            ui.horizontal(|ui| {
                ui.label("Type:");
                ui.text_edit_singleline(&mut self.new_node_type);
            });
            ui.horizontal(|ui| {
                ui.label("Data (JSON):");
                ui.text_edit_singleline(&mut self.new_node_data);
            });

            if ui.button("Add Node").clicked() {
                let new_node_data_result = serde_json::from_str(&self.new_node_data);

                match new_node_data_result {
                    Ok(json_value) => {
                        let new_node = NodeData {
                            id: Uuid::new_v4(),
                            node_type: self.new_node_type.clone(),
                            data: json_value,
                            position: [0.0, 0.0],
                        };
                        self.workflow.add_node(new_node);
                        self.new_node_data = "{}".to_string(); // Clear the input
                    }
                    Err(e) => {
                        println!("Error parsing JSON: {}", e);
                    }
                }

            }

            ui.separator();

            // Example Plot (just to show egui is working)
            let sin: Values = (0..1000).map(|i| {
                let x = i as f64 / 100.0;
                Value::new(x, x.sin())
            }).collect();
            let line = Line::new(sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));


            if ui.button("Serialize Workflow").clicked() {
                let serialized = serde_json::to_string(&self.workflow).unwrap();
                println!("Serialized Workflow:\n{}", serialized);
            }

            if ui.button("Deserialize Workflow").clicked() {
                let serialized = serde_json::to_string(&self.workflow).unwrap(); // just reusing the serialized data
                let deserialized: Workflow = serde_json::from_str(&serialized).unwrap();  // Deserialize from the same data we serialized for testing

                // Update the current workflow with the deserialized one
                self.workflow = deserialized;
                println!("Workflow Deserialized successfully!");

            }
        });
    }
}
