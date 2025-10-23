use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, KeyboardEvent};
use crate::mind_map::MindMap;

#[wasm_bindgen]
pub struct MindMapApp {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    mind_map: MindMap,
    // Viewport offset for panning
    viewport_offset_x: f64,
    viewport_offset_y: f64,
    // Panning state
    is_panning: bool,
    pan_start_x: f64,
    pan_start_y: f64,
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
            viewport_offset_x: 0.0,
            viewport_offset_y: 0.0,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
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

        // Draw connections with viewport offset
        self.context.set_stroke_style_str("#999");
        self.context.set_line_width(2.0);

        for node in &self.mind_map.nodes {
            for child_id in &node.children {
                if let Some(child) = self.mind_map.get_node(*child_id) {
                    self.context.begin_path();
                    self.context.move_to(
                        node.x + self.viewport_offset_x,
                        node.y + self.viewport_offset_y
                    );
                    self.context.line_to(
                        child.x + self.viewport_offset_x,
                        child.y + self.viewport_offset_y
                    );
                    self.context.stroke();
                }
            }
        }

        // Draw nodes with viewport offset
        for node in &self.mind_map.nodes {
            let is_selected = self.mind_map.selected_node == Some(node.id);

            self.context.set_fill_style_str(
                if is_selected { "#4CAF50" } else { "#2196F3" }
            );

            let x = node.x + self.viewport_offset_x - node_width / 2.0;
            let y = node.y + self.viewport_offset_y - node_height / 2.0;

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
            let _ = self.context.fill_text(
                text,
                node.x + self.viewport_offset_x,
                node.y + self.viewport_offset_y
            );
        }
    }

    // 내부 함수: 좌표 기반 마우스 다운 처리
    fn handle_down_internal(&mut self, x: f64, y: f64) {
        let canvas_width = self.canvas.width() as f64;

        // Convert screen coordinates to virtual canvas coordinates
        let virtual_x = x - self.viewport_offset_x;
        let virtual_y = y - self.viewport_offset_y;

        if let Some(node_id) = self.mind_map.find_node_at(virtual_x, virtual_y, canvas_width) {
            // Node found - start dragging node
            self.mind_map.selected_node = Some(node_id);
            self.mind_map.dragging_node = Some(node_id);

            if let Some(node) = self.mind_map.get_node(node_id) {
                let node_x = node.x;
                let node_y = node.y;
                self.mind_map.drag_offset_x = virtual_x - node_x;
                self.mind_map.drag_offset_y = virtual_y - node_y;
            }

            self.render();
        } else {
            // No node found - start panning canvas
            self.is_panning = true;
            self.pan_start_x = x;
            self.pan_start_y = y;
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
        if self.is_panning {
            // Panning canvas
            let dx = x - self.pan_start_x;
            let dy = y - self.pan_start_y;

            self.viewport_offset_x += dx;
            self.viewport_offset_y += dy;

            self.pan_start_x = x;
            self.pan_start_y = y;

            self.render();
        } else if let Some(node_id) = self.mind_map.dragging_node {
            // Dragging node
            let virtual_x = x - self.viewport_offset_x;
            let virtual_y = y - self.viewport_offset_y;

            let offset_x = self.mind_map.drag_offset_x;
            let offset_y = self.mind_map.drag_offset_y;

            if let Some(node) = self.mind_map.get_node_mut(node_id) {
                node.x = virtual_x - offset_x;
                node.y = virtual_y - offset_y;
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
        self.is_panning = false;
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
