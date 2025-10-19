use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

#[derive(Clone, Debug)]
pub struct Node {
    id: usize,
    text: String,
    x: f64,
    y: f64,
    children: Vec<usize>,
}

impl Node {
    fn new(id: usize, text: String, x: f64, y: f64) -> Self {
        Node {
            id,
            text,
            x,
            y,
            children: Vec::new(),
        }
    }

    fn contains_point(&self, px: f64, py: f64) -> bool {
        let width = 120.0;
        let height = 40.0;
        px >= self.x - width / 2.0
            && px <= self.x + width / 2.0
            && py >= self.y - height / 2.0
            && py <= self.y + height / 2.0
    }
}

pub struct MindMap {
    nodes: Vec<Node>,
    next_id: usize,
    root_id: Option<usize>,
    selected_node: Option<usize>,
    dragging_node: Option<usize>,
    drag_offset_x: f64,
    drag_offset_y: f64,
}

impl MindMap {
    fn new() -> Self {
        MindMap {
            nodes: Vec::new(),
            next_id: 0,
            root_id: None,
            selected_node: None,
            dragging_node: None,
            drag_offset_x: 0.0,
            drag_offset_y: 0.0,
        }
    }

    fn create_root(&mut self, text: String, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let node = Node::new(id, text, x, y);
        self.nodes.push(node);
        self.root_id = Some(id);
        id
    }

    fn add_child(&mut self, parent_id: usize, text: String) -> Option<usize> {
        let parent_idx = self.nodes.iter().position(|n| n.id == parent_id)?;
        let parent = &self.nodes[parent_idx];

        let child_count = parent.children.len();
        let x = parent.x + 150.0;
        let y = parent.y + (child_count as f64 * 60.0) - (child_count as f64 * 30.0);

        let id = self.next_id;
        self.next_id += 1;
        let node = Node::new(id, text, x, y);
        self.nodes.push(node);

        if let Some(parent) = self.nodes.iter_mut().find(|n| n.id == parent_id) {
            parent.children.push(id);
        }

        Some(id)
    }

    fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    fn find_node_at(&self, x: f64, y: f64) -> Option<usize> {
        for node in self.nodes.iter().rev() {
            if node.contains_point(x, y) {
                return Some(node.id);
            }
        }
        None
    }

    fn update_node_text(&mut self, id: usize, text: String) {
        if let Some(node) = self.get_node_mut(id) {
            node.text = text;
        }
    }
}

#[wasm_bindgen]
pub struct MindMapApp {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    mind_map: MindMap,
}

#[wasm_bindgen]
impl MindMapApp {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<MindMapApp, JsValue> {
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut mind_map = MindMap::new();
        mind_map.create_root("Root".to_string(), 400.0, 300.0);

        Ok(MindMapApp {
            canvas,
            context,
            mind_map,
        })
    }

    pub fn render(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        self.context.clear_rect(0.0, 0.0, width, height);

        // Draw connections
        self.context.set_stroke_style_str("#999");
        self.context.set_line_width(2.0);

        for node in &self.mind_map.nodes {
            for child_id in &node.children {
                if let Some(child) = self.mind_map.get_node(*child_id) {
                    self.context.begin_path();
                    self.context.move_to(node.x, node.y);
                    self.context.line_to(child.x, child.y);
                    self.context.stroke();
                }
            }
        }

        // Draw nodes
        for node in &self.mind_map.nodes {
            let is_selected = self.mind_map.selected_node == Some(node.id);

            self.context.set_fill_style_str(
                if is_selected { "#4CAF50" } else { "#2196F3" }
            );

            let width = 120.0;
            let height = 40.0;
            let x = node.x - width / 2.0;
            let y = node.y - height / 2.0;

            self.context.fill_rect(x, y, width, height);

            self.context.set_stroke_style_str("#fff");
            self.context.set_line_width(2.0);
            self.context.stroke_rect(x, y, width, height);

            self.context.set_fill_style_str("#fff");
            self.context.set_font("14px Arial");
            self.context.set_text_align("center");
            self.context.set_text_baseline("middle");
            let _ = self.context.fill_text(&node.text, node.x, node.y);
        }
    }

    pub fn handle_mouse_down(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;

        if let Some(node_id) = self.mind_map.find_node_at(x, y) {
            self.mind_map.selected_node = Some(node_id);
            self.mind_map.dragging_node = Some(node_id);

            if let Some(node) = self.mind_map.get_node(node_id) {
                let node_x = node.x;
                let node_y = node.y;
                self.mind_map.drag_offset_x = x - node_x;
                self.mind_map.drag_offset_y = y - node_y;
            }

            self.render();
        }
    }

    pub fn handle_mouse_move(&mut self, event: MouseEvent) {
        if let Some(node_id) = self.mind_map.dragging_node {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;

            let offset_x = self.mind_map.drag_offset_x;
            let offset_y = self.mind_map.drag_offset_y;

            if let Some(node) = self.mind_map.get_node_mut(node_id) {
                node.x = x - offset_x;
                node.y = y - offset_y;
            }

            self.render();
        }
    }

    pub fn handle_mouse_up(&mut self, _event: MouseEvent) {
        self.mind_map.dragging_node = None;
    }

    pub fn add_child_to_selected(&mut self, text: String) {
        if let Some(selected_id) = self.mind_map.selected_node {
            self.mind_map.add_child(selected_id, text);
            self.render();
        }
    }

    pub fn update_selected_text(&mut self, text: String) {
        if let Some(selected_id) = self.mind_map.selected_node {
            self.mind_map.update_node_text(selected_id, text);
            self.render();
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        self.mind_map.selected_node
            .and_then(|id| self.mind_map.get_node(id))
            .map(|node| node.text.clone())
    }
}
