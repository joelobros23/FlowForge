// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App, Frame, NativeOptions};
use egui::{Color32, Pos2, Rect, Vec2, Sense};
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = NativeOptions {
        initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "FlowForge",
        native_options,
        Box::new(|cc| {
            // This gives us image support: egui_extras::install_image_loaders(&cc);
            Box::new(FlowForgeApp::new(cc))
        }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    id: Uuid,
    name: String,
    position: Pos2,
}

impl Node {
    fn new(name: String, position: Pos2) -> Self {
        Node {
            id: Uuid::new_v4(),
            name,
            position,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Edge {
    source: Uuid,
    target: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
struct Workflow {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Workflow {
    fn new() -> Self {
        Workflow {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
}


struct FlowForgeApp {
    workflow: Workflow,
    node_palette: Vec<String>,
    dragged_node: Option<String>,
}

impl FlowForgeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // cc.egui_ctx.set_visuals(egui::Visuals::dark());

        FlowForgeApp {
            workflow: Workflow::new(),
            node_palette: vec!["Input".to_string(), "Transform".to_string(), "Output".to_string()],
            dragged_node: None,
        }
    }
}

impl App for FlowForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("FlowForge");
        });

        egui::SidePanel::left("node_palette")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Node Palette");

                for node_type in &self.node_palette {
                    let response = ui.add(egui::Label::new(node_type).sense(Sense::drag()));

                    if response.drag_started() {
                        self.dragged_node = Some(node_type.clone());
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Workflow Editor");

            // Handle dropping nodes onto the canvas
            let drop_position = ui.input(|i| i.pointer.hover_pos());

            if let Some(pos) = drop_position {
                if ui.input(|i| i.pointer.any_released()) {
                    if let Some(node_type) = &self.dragged_node {
                        self.workflow.add_node(Node::new(node_type.clone(), pos));
                        self.dragged_node = None; // Reset dragged node
                    }
                }
            }

            // Draw the nodes
            for node in &self.workflow.nodes {
                let node_rect = Rect::from_center_size(node.position, Vec2::new(80.0, 30.0));
                let node_id = egui::Id::new(node.id);

                let response = ui.allocate_rect(node_rect, Sense::click_and_drag());

                if response.hovered() {
                    ui.output_mut().cursor_icon = egui::CursorIcon::PointingHand;
                }

                if response.dragged() {
                    let delta = response.drag_delta();
                    // Update the node position directly.  A proper implementation would update
                    // the node's position in the data model (self.workflow).
                    let mut mutable_node = node.clone();
                    mutable_node.position.x += delta.x;
                    mutable_node.position.y += delta.y;
                    //Find the node in the workflow and update the position
                    if let Some(node_index) = self.workflow.nodes.iter().position(|n| n.id == node.id) {
                        self.workflow.nodes[node_index].position = mutable_node.position;
                    }


                    response.request_repaint();
                }


                ui.painter().rect_filled(node_rect, egui::Rounding::same(5.0), Color32::LIGHT_BLUE);
                ui.painter().text(
                    node_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &node.name,
                    egui::FontId::default(),
                    Color32::BLACK,
                );
            }
        });
    }
}