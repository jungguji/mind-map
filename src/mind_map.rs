use crate::node::Node;
use std::collections::{HashSet, HashMap};

pub struct MindMap {
    pub nodes: Vec<Node>,
    pub next_id: usize,
    pub root_id: Option<usize>,
    pub selected_nodes: HashSet<usize>,  // 멀티 선택 지원
    pub dragging_nodes: HashSet<usize>,  // 멀티 드래그 지원
    pub drag_offsets: HashMap<usize, (f64, f64)>,  // 각 노드의 드래그 오프셋
}

impl MindMap {
    pub fn new() -> Self {
        MindMap {
            nodes: Vec::new(),
            next_id: 0,
            root_id: None,
            selected_nodes: HashSet::new(),
            dragging_nodes: HashSet::new(),
            drag_offsets: HashMap::new(),
        }
    }

    pub fn create_root(&mut self, text: String, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let node = Node::new(id, text, x, y);
        self.nodes.push(node);
        self.root_id = Some(id);
        self.selected_nodes.insert(id);
        id
    }

    pub fn add_child(&mut self, parent_id: usize, text: String, canvas_width: f64) -> Option<usize> {
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

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn find_node_at(&self, x: f64, y: f64, canvas_width: f64) -> Option<usize> {
        for node in self.nodes.iter().rev() {
            if node.contains_point(x, y, canvas_width) {
                return Some(node.id);
            }
        }
        None
    }

    // 사각형 영역 내의 모든 노드를 찾는 함수
    pub fn find_nodes_in_rect(&self, x1: f64, y1: f64, x2: f64, y2: f64, _canvas_width: f64) -> Vec<usize> {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        self.nodes.iter()
            .filter(|node| {
                node.x >= min_x && node.x <= max_x &&
                node.y >= min_y && node.y <= max_y
            })
            .map(|node| node.id)
            .collect()
    }

    // 모든 선택 해제
    pub fn clear_selection(&mut self) {
        self.selected_nodes.clear();
    }

    pub fn update_node_text(&mut self, id: usize, text: String) {
        if let Some(node) = self.get_node_mut(id) {
            node.text = text;
        }
    }

    pub fn delete_node(&mut self, id: usize) -> bool {
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
        self.selected_nodes.remove(&id);
        self.dragging_nodes.remove(&id);
        self.drag_offsets.remove(&id);

        true
    }
}
