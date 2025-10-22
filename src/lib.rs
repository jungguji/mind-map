use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, KeyboardEvent};

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

    fn contains_point(&self, px: f64, py: f64, canvas_width: f64) -> bool {
        // 화면 크기에 따른 노드 크기 (render와 동일)
        let width = if canvas_width < 600.0 { 100.0 } else { 120.0 };
        let height = if canvas_width < 600.0 { 35.0 } else { 40.0 };

        // 터치하기 쉽도록 히트박스를 약간 확대 (모바일)
        let padding = if canvas_width < 600.0 { 5.0 } else { 0.0 };

        px >= self.x - width / 2.0 - padding
            && px <= self.x + width / 2.0 + padding
            && py >= self.y - height / 2.0 - padding
            && py <= self.y + height / 2.0 + padding
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

    fn add_child(&mut self, parent_id: usize, text: String, canvas_width: f64) -> Option<usize> {
        let parent_idx = self.nodes.iter().position(|n| n.id == parent_id)?;
        let parent = &self.nodes[parent_idx];

        let child_count = parent.children.len();

        // 화면 크기에 따라 간격 조정 (작은 화면에서는 간격 축소)
        let horizontal_spacing = if canvas_width < 600.0 { 120.0 } else { 150.0 };
        let vertical_spacing = if canvas_width < 600.0 { 50.0 } else { 60.0 };

        let x = parent.x + horizontal_spacing;
        let y = parent.y + (child_count as f64 * vertical_spacing) - (child_count as f64 * vertical_spacing / 2.0);

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

    fn find_node_at(&self, x: f64, y: f64, canvas_width: f64) -> Option<usize> {
        for node in self.nodes.iter().rev() {
            if node.contains_point(x, y, canvas_width) {
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

    fn delete_node(&mut self, id: usize) -> bool {
        // Root 노드는 삭제 불가
        if self.root_id == Some(id) {
            return false;
        }

        // 자식 노드들을 재귀적으로 삭제
        if let Some(node) = self.get_node(id) {
            let children_ids: Vec<usize> = node.children.clone();
            for child_id in children_ids {
                self.delete_node(child_id);
            }
        }

        // 부모 노드의 children 벡터에서 제거
        for node in &mut self.nodes {
            node.children.retain(|&child_id| child_id != id);
        }

        // nodes 벡터에서 제거
        self.nodes.retain(|n| n.id != id);

        // 선택된 노드가 삭제된 경우 선택 해제
        if self.selected_node == Some(id) {
            self.selected_node = None;
        }

        true
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

        // 동적 Canvas 크기에 맞춰 Root 노드 위치 조정
        let canvas_width = canvas.width() as f64;
        let canvas_height = canvas.height() as f64;
        let root_x = canvas_width / 2.0;
        let root_y = canvas_height / 2.0;

        mind_map.create_root("Root".to_string(), root_x, root_y);

        Ok(MindMapApp {
            canvas,
            context,
            mind_map,
        })
    }

    pub fn render(&self) {
        let canvas_width = self.canvas.width() as f64;
        let canvas_height = self.canvas.height() as f64;

        self.context.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        // 화면 크기에 따른 노드 크기 조정
        let node_width = if canvas_width < 600.0 { 100.0 } else { 120.0 };
        let node_height = if canvas_width < 600.0 { 35.0 } else { 40.0 };
        let font_size = if canvas_width < 600.0 { 12.0 } else { 14.0 };

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

            let x = node.x - node_width / 2.0;
            let y = node.y - node_height / 2.0;

            self.context.fill_rect(x, y, node_width, node_height);

            self.context.set_stroke_style_str("#fff");
            self.context.set_line_width(2.0);
            self.context.stroke_rect(x, y, node_width, node_height);

            self.context.set_fill_style_str("#fff");
            self.context.set_font(&format!("{}px Arial", font_size));
            self.context.set_text_align("center");
            self.context.set_text_baseline("middle");

            // 텍스트 렌더링
            let text = &node.text;
            let _ = self.context.fill_text(text, node.x, node.y);
        }
    }

    // 내부 함수: 좌표 기반 마우스 다운 처리
    fn handle_down_internal(&mut self, x: f64, y: f64) {
        let canvas_width = self.canvas.width() as f64;

        if let Some(node_id) = self.mind_map.find_node_at(x, y, canvas_width) {
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

    pub fn handle_mouse_down(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        self.handle_down_internal(x, y);
    }

    // 터치 이벤트를 위한 좌표 기반 함수
    pub fn handle_pointer_down(&mut self, x: f64, y: f64) {
        self.handle_down_internal(x, y);
    }

    // 내부 함수: 좌표 기반 마우스 이동 처리
    fn handle_move_internal(&mut self, x: f64, y: f64) {
        if let Some(node_id) = self.mind_map.dragging_node {
            let offset_x = self.mind_map.drag_offset_x;
            let offset_y = self.mind_map.drag_offset_y;

            if let Some(node) = self.mind_map.get_node_mut(node_id) {
                node.x = x - offset_x;
                node.y = y - offset_y;
            }

            self.render();
        }
    }

    pub fn handle_mouse_move(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        self.handle_move_internal(x, y);
    }

    // 터치 이벤트를 위한 좌표 기반 함수
    pub fn handle_pointer_move(&mut self, x: f64, y: f64) {
        self.handle_move_internal(x, y);
    }

    // 내부 함수: 마우스 업 처리
    fn handle_up_internal(&mut self) {
        self.mind_map.dragging_node = None;
    }

    pub fn handle_mouse_up(&mut self, _event: MouseEvent) {
        self.handle_up_internal();
    }

    // 터치 이벤트를 위한 좌표 기반 함수
    pub fn handle_pointer_up(&mut self) {
        self.handle_up_internal();
    }

    pub fn add_child_to_selected(&mut self, text: String) {
        if let Some(selected_id) = self.mind_map.selected_node {
            let canvas_width = self.canvas.width() as f64;
            self.mind_map.add_child(selected_id, text, canvas_width);
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

    pub fn delete_selected_node(&mut self) -> bool {
        if let Some(selected_id) = self.mind_map.selected_node {
            let deleted = self.mind_map.delete_node(selected_id);
            if deleted {
                self.render();
            }
            deleted
        } else {
            false
        }
    }

    pub fn handle_key_down(&mut self, event: KeyboardEvent) {
        let key = event.key();
        if key == "Delete" || key == "Backspace" {
            self.delete_selected_node();
        }
    }

    // 내부 함수: 좌표 기반 더블 클릭 처리
    fn handle_double_click_internal(&mut self, x: f64, y: f64) {
        let canvas_width = self.canvas.width() as f64;

        if let Some(node_id) = self.mind_map.find_node_at(x, y, canvas_width) {
            self.mind_map.selected_node = Some(node_id);
            self.render();
        }
    }

    pub fn handle_double_click(&mut self, event: MouseEvent) {
        let x = event.offset_x() as f64;
        let y = event.offset_y() as f64;
        self.handle_double_click_internal(x, y);
    }

    // 터치 이벤트를 위한 좌표 기반 함수
    pub fn handle_pointer_double_click(&mut self, x: f64, y: f64) {
        self.handle_double_click_internal(x, y);
    }
}
