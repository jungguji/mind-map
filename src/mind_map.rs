use crate::node::Node;

pub struct MindMap {
    pub nodes: Vec<Node>,
    pub next_id: usize,
    pub root_id: Option<usize>,
    pub selected_node: Option<usize>,
    pub dragging_node: Option<usize>,
    pub drag_offset_x: f64,
    pub drag_offset_y: f64,
}

impl MindMap {
    pub fn new() -> Self {
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

    pub fn create_root(&mut self, text: String, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let node = Node::new(id, text, x, y);
        self.nodes.push(node);
        self.root_id = Some(id);
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
        if self.selected_node == Some(id) {
            self.selected_node = None;
        }

        true
    }
}
