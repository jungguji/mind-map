#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub children: Vec<usize>,
}

impl Node {
    pub fn new(id: usize, text: String, x: f64, y: f64) -> Self {
        Node {
            id,
            text,
            x,
            y,
            children: Vec::new(),
        }
    }

    pub fn contains_point(&self, px: f64, py: f64, canvas_width: f64) -> bool {
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
