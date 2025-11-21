use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, KeyboardEvent};
use crate::mind_map::MindMap;

// 영역 선택 박스
struct SelectionBox {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
}

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
    // Space key state
    is_space_pressed: bool,
    // Selection box for area selection
    selection_box: Option<SelectionBox>,
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
            is_space_pressed: false,
            selection_box: None,
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
            let is_selected = self.mind_map.selected_nodes.contains(&node.id);

            self.context.set_fill_style_str(
                if is_selected { "#4CAF50" } else { "#2196F3" }
            );

            let x = node.x + self.viewport_offset_x - node_width / 2.0;
            let y = node.y + self.viewport_offset_y - node_height / 2.0;

            self.context.fill_rect(x, y, node_width, node_height);

            // 선택된 노드는 테두리 강조
            if is_selected {
                self.context.set_stroke_style_str("#fff");
                self.context.set_line_width(3.0);
            } else {
                self.context.set_stroke_style_str("#fff");
                self.context.set_line_width(2.0);
            }
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

        // 영역 선택 박스 렌더링
        if let Some(ref box_) = self.selection_box {
            self.context.set_stroke_style_str("#4CAF50");
            self.context.set_line_width(2.0);
            self.context.set_fill_style_str("rgba(76, 175, 80, 0.1)");

            let x = box_.start_x.min(box_.end_x);
            let y = box_.start_y.min(box_.end_y);
            let w = (box_.end_x - box_.start_x).abs();
            let h = (box_.end_y - box_.start_y).abs();

            self.context.fill_rect(x, y, w, h);
            self.context.stroke_rect(x, y, w, h);
        }
    }

    // 내부 함수: 좌표 기반 마우스 다운 처리
    fn handle_down_internal(&mut self, x: f64, y: f64) {
        let canvas_width = self.canvas.width() as f64;

        // Space 키가 눌려있으면 무조건 패닝
        if self.is_space_pressed {
            self.is_panning = true;
            self.pan_start_x = x;
            self.pan_start_y = y;
            return;
        }

        // Convert screen coordinates to virtual canvas coordinates
        let virtual_x = x - self.viewport_offset_x;
        let virtual_y = y - self.viewport_offset_y;

        if let Some(node_id) = self.mind_map.find_node_at(virtual_x, virtual_y, canvas_width) {
            // Node found
            let was_selected = self.mind_map.selected_nodes.contains(&node_id);

            if !was_selected {
                // 선택되지 않은 노드 클릭 → 기존 선택 해제 후 새로 선택
                self.mind_map.clear_selection();
                self.mind_map.selected_nodes.insert(node_id);
            }

            // 선택된 모든 노드의 드래그 오프셋 계산
            self.mind_map.drag_offsets.clear();
            for &selected_id in &self.mind_map.selected_nodes {
                if let Some(node) = self.mind_map.get_node(selected_id) {
                    let offset_x = virtual_x - node.x;
                    let offset_y = virtual_y - node.y;
                    self.mind_map.drag_offsets.insert(selected_id, (offset_x, offset_y));
                    self.mind_map.dragging_nodes.insert(selected_id);
                }
            }

            self.render();
        } else {
            // No node found - start selection box
            self.selection_box = Some(SelectionBox {
                start_x: x,
                start_y: y,
                end_x: x,
                end_y: y,
            });
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
        if self.is_panning {
            // Panning canvas
            let dx = x - self.pan_start_x;
            let dy = y - self.pan_start_y;

            self.viewport_offset_x += dx;
            self.viewport_offset_y += dy;

            self.pan_start_x = x;
            self.pan_start_y = y;

            self.render();
        } else if let Some(ref mut box_) = self.selection_box {
            // 영역 선택 박스 크기 업데이트
            box_.end_x = x;
            box_.end_y = y;
            self.render();
        } else if !self.mind_map.dragging_nodes.is_empty() {
            // 멀티 노드 드래그
            let virtual_x = x - self.viewport_offset_x;
            let virtual_y = y - self.viewport_offset_y;

            let dragging_ids: Vec<usize> = self.mind_map.dragging_nodes.iter().copied().collect();

            for node_id in dragging_ids {
                if let Some(&(offset_x, offset_y)) = self.mind_map.drag_offsets.get(&node_id) {
                    if let Some(node) = self.mind_map.get_node_mut(node_id) {
                        node.x = virtual_x - offset_x;
                        node.y = virtual_y - offset_y;
                    }
                }
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
        // 영역 선택 완료
        if let Some(box_) = self.selection_box.take() {
            let canvas_width = self.canvas.width() as f64;

            // 화면 좌표를 가상 좌표로 변환
            let virtual_x1 = box_.start_x - self.viewport_offset_x;
            let virtual_y1 = box_.start_y - self.viewport_offset_y;
            let virtual_x2 = box_.end_x - self.viewport_offset_x;
            let virtual_y2 = box_.end_y - self.viewport_offset_y;

            // 사각형 안의 노드들 찾기
            let nodes_in_rect = self.mind_map.find_nodes_in_rect(
                virtual_x1, virtual_y1, virtual_x2, virtual_y2, canvas_width
            );

            // 선택 업데이트
            self.mind_map.clear_selection();
            for node_id in nodes_in_rect {
                self.mind_map.selected_nodes.insert(node_id);
            }

            self.render();
        }

        self.mind_map.dragging_nodes.clear();
        self.mind_map.drag_offsets.clear();
        self.is_panning = false;
    }

    pub fn handle_mouse_up(&mut self, _event: MouseEvent) {
        self.handle_up_internal();
    }

    // 터치 이벤트를 위한 좌표 기반 함수
    pub fn handle_pointer_up(&mut self) {
        self.handle_up_internal();
    }

    // Space 키 상태 설정
    pub fn set_space_pressed(&mut self, pressed: bool) {
        self.is_space_pressed = pressed;
    }

    pub fn add_child_to_selected(&mut self, text: String) {
        // 선택된 노드가 하나일 때만 자식 추가
        if self.mind_map.selected_nodes.len() == 1 {
            if let Some(&selected_id) = self.mind_map.selected_nodes.iter().next() {
                let canvas_width = self.canvas.width() as f64;
                self.mind_map.add_child(selected_id, text, canvas_width);
                self.render();
            }
        }
    }

    pub fn update_selected_text(&mut self, text: String) {
        // 선택된 노드가 하나일 때만 텍스트 업데이트
        if self.mind_map.selected_nodes.len() == 1 {
            if let Some(&selected_id) = self.mind_map.selected_nodes.iter().next() {
                self.mind_map.update_node_text(selected_id, text);
                self.render();
            }
        }
    }

    pub fn get_selected_text(&self) -> Option<String> {
        // 선택된 노드가 하나일 때만 텍스트 반환
        if self.mind_map.selected_nodes.len() == 1 {
            self.mind_map.selected_nodes.iter().next()
                .and_then(|&id| self.mind_map.get_node(id))
                .map(|node| node.text.clone())
        } else {
            None
        }
    }

    pub fn delete_selected_node(&mut self) -> bool {
        // 선택된 모든 노드 삭제
        let selected_ids: Vec<usize> = self.mind_map.selected_nodes.iter().copied().collect();
        let mut any_deleted = false;

        for selected_id in selected_ids {
            if self.mind_map.delete_node(selected_id) {
                any_deleted = true;
            }
        }

        if any_deleted {
            self.render();
        }

        any_deleted
    }

    pub fn handle_key_down(&mut self, event: KeyboardEvent) {
        let key = event.key();
        if key == "Delete" {
            self.delete_selected_node();
        }
    }

    // 내부 함수: 좌표 기반 더블 클릭 처리
    fn handle_double_click_internal(&mut self, x: f64, y: f64) {
        let canvas_width = self.canvas.width() as f64;

        let virtual_x = x - self.viewport_offset_x;
        let virtual_y = y - self.viewport_offset_y;

        if let Some(node_id) = self.mind_map.find_node_at(virtual_x, virtual_y, canvas_width) {
            self.mind_map.clear_selection();
            self.mind_map.selected_nodes.insert(node_id);
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
