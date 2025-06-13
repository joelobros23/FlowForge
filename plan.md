# Project Plan: FlowForge

**Description:** A visual scripting tool for creating and managing data pipelines. Users can drag and drop nodes representing data transformations and connect them to form workflows.


## Development Goals

- [ ] Set up the basic eframe application with a main window and menu bar (File -> New, Open, Save, Exit).
- [ ] Define data structures for nodes (containing data, type, position) and workflows (a directed graph of nodes). Implement serde serialization/deserialization.
- [ ] Create a custom `Node` struct with fields for ID (UUID), name, position, and data. Implement a simple `NodeWidget` to visually represent the node in the UI using `egui` with drag and drop capabilities.
- [ ] Implement the workflow editor using `egui`.  Allow adding new nodes by dragging them from a node palette (side panel).
- [ ] Implement node connection (drawing lines between node outputs and inputs using `egui` shapes and interaction).
- [ ] Implement basic data flow execution: on 'Run' button click, traverse the workflow graph and execute node logic (simple example: add two numbers).
- [ ] Implement saving and loading workflows to/from JSON files.
- [ ] Implement pan and zoom for the workflow canvas.
- [ ] Add a properties panel to inspect and modify node data.
- [ ] Create a module for common node types (e.g., Input, Output, Add, Multiply, Filter) with a basic implementation for each node's functionality.
