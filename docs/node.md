# Node Reference Guide

**File:** `src/node.rs`
**Last Updated:** 2025-10-25

## Overview

`Node` represents a single node in the mind map. Each node has a unique ID, text content, position, and maintains references to its child nodes. The Node struct is designed to be simple, cloneable, and efficient.

**Key Responsibilities:**
- Store node data (id, text, position)
- Track child node relationships
- Provide hit detection for user interaction
- Support responsive sizing on different screen sizes

---

## Structure

### `Node`

**Attributes:**
```rust
#[derive(Clone, Debug)]
```

- `Clone`: Enables node duplication (useful for undo/redo systems)
- `Debug`: Provides automatic debug formatting

**Fields:**
```rust
pub id: usize              // Unique identifier
pub text: String           // Display text
pub x: f64                 // X position (center point)
pub y: f64                 // Y position (center point)
pub children: Vec<usize>   // IDs of child nodes
```

**Coordinate System:**
- `(x, y)` represents the **center** of the node
- Infinite virtual canvas (not limited by screen size)
- Viewport offset applied during rendering

**Parent-Child Relationships:**
- Parent nodes store child IDs in `children` vector
- No direct parent reference (navigated via MindMap)
- Hierarchical tree structure

---

## Public API Reference

### Constructor

#### `new(id: usize, text: String, x: f64, y: f64) -> Self`

Creates a new node.

**Parameters:**
- `id` - Unique identifier (typically from `MindMap.next_id`)
- `text` - Initial text content
- `x` - X coordinate (center position)
- `y` - Y coordinate (center position)

**Returns:**
- `Node` - New node instance with empty children list

**Example:**
```rust
let node = Node::new(0, "Root".to_string(), 400.0, 300.0);
```

---

### Hit Detection

#### `contains_point(px: f64, py: f64, canvas_width: f64) -> bool`

Determines if a point is within the node's clickable area.

**Parameters:**
- `px` - Point X coordinate
- `py` - Point Y coordinate
- `canvas_width` - Canvas width (for responsive sizing)

**Returns:**
- `true` - Point is inside node bounds
- `false` - Point is outside node bounds

**Behavior:**

**Responsive Sizing:**
| Screen Width | Node Width | Node Height | Touch Padding |
|--------------|------------|-------------|---------------|
| < 600px (mobile) | 100px | 35px | 5px |
| ≥ 600px (desktop) | 120px | 40px | 0px |

**Hit Box Calculation:**
```
Bounds:
  Left:   x - width/2 - padding
  Right:  x + width/2 + padding
  Top:    y - height/2 - padding
  Bottom: y + height/2 + padding
```

**Touch Padding:**
- Mobile screens: +5px padding on all sides
- Desktop: No padding
- Makes nodes easier to tap on mobile devices

**Example:**
```rust
let node = Node::new(0, "Test".to_string(), 200.0, 150.0);

// Desktop (800px width)
assert!(node.contains_point(200.0, 150.0, 800.0));  // Center
assert!(node.contains_point(260.0, 150.0, 800.0));  // Right edge (200 + 60)
assert!(!node.contains_point(270.0, 150.0, 800.0)); // Outside

// Mobile (500px width)
assert!(node.contains_point(155.0, 150.0, 500.0));  // Left edge with padding
```

---

## Node Dimensions

The node's visual size is responsive and must match the rendering in `app.rs`:

**Desktop (canvas_width ≥ 600px):**
```
Width:  120px
Height: 40px
Font:   14px
```

**Mobile (canvas_width < 600px):**
```
Width:  100px
Height: 35px
Font:   12px
```

**Coordinate Reference:**
- `x, y` = center point of the rectangle
- Top-left corner for rendering = `(x - width/2, y - height/2)`

---

## Usage Patterns

### Creating Nodes

```rust
// Manual creation
let node = Node::new(1, "My Node".to_string(), 300.0, 200.0);

// Via MindMap (recommended)
let root_id = mind_map.create_root("Root".to_string(), 400.0, 300.0);
let child_id = mind_map.add_child(root_id, "Child".to_string(), 800.0).unwrap();
```

### Managing Children

```rust
// Access children
for &child_id in &node.children {
    if let Some(child) = mind_map.get_node(child_id) {
        println!("Child: {}", child.text);
    }
}

// Add child manually (typically done via MindMap.add_child)
parent_node.children.push(new_child_id);
```

### Hit Testing

```rust
// Check if user clicked on node
let mouse_x = 250.0;
let mouse_y = 180.0;
let canvas_width = 800.0;

if node.contains_point(mouse_x, mouse_y, canvas_width) {
    println!("Node clicked!");
}
```

### Position Updates

```rust
// Update node position (during drag)
if let Some(node) = mind_map.get_node_mut(node_id) {
    node.x = new_x;
    node.y = new_y;
}
```

### Text Updates

```rust
// Update node text
if let Some(node) = mind_map.get_node_mut(node_id) {
    node.text = "Updated Text".to_string();
}
```

---

## Rendering Considerations

When rendering a node, the visual bounds must match the hit detection bounds:

```rust
// Rendering (in app.rs)
let node_width = if canvas_width < 600.0 { 100.0 } else { 120.0 };
let node_height = if canvas_width < 600.0 { 35.0 } else { 40.0 };

let x = node.x + viewport_offset_x - node_width / 2.0;
let y = node.y + viewport_offset_y - node_height / 2.0;

context.fill_rect(x, y, node_width, node_height);
```

**Important:** The `contains_point()` calculation must exactly match the rendered bounds for accurate click detection.

---

## Data Flow

```
[User Click]
    |
    v
Convert screen coords to virtual coords
    |
    v
For each node (reverse order):
    node.contains_point(x, y, canvas_width)?
    |
    v
[Return first match = topmost node]
```

```
[Node Creation]
    |
    v
Node::new(id, text, x, y)
    |
    v
Add to parent.children
    |
    v
Add to mind_map.nodes
    |
    v
[Node Ready]
```

---

## Design Notes

**Why Store Center Position?**
- Simplifies calculations (rotation, scaling in future)
- Natural reference point for connections between nodes
- Easy conversion to top-left for rendering

**Why Store Child IDs Instead of References?**
- Avoids Rust ownership/borrowing complexity
- Enables simple flat Vec storage in MindMap
- Easy serialization to JSON

**Why Responsive Sizing?**
- Better UX on mobile (larger touch targets)
- Fits more content on desktop screens
- Consistent hit detection across devices

**Why Touch Padding on Mobile?**
- Average finger tap area: ~44-48px diameter
- Adding 5px padding increases hit area without visual clutter
- Improves accuracy on small touch screens

---

## Performance Notes

**Clone Cost:**
- O(1) for numeric fields
- O(n) for text string (where n = text length)
- O(m) for children vector (where m = number of children)
- Typically very cheap (most nodes have short text and few children)

**Hit Detection:**
- O(1) - Simple arithmetic comparisons
- No allocation
- Extremely fast

---

## Future Extensions

Potential fields that could be added:

```rust
pub struct Node {
    // Existing fields...
    pub id: usize,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub children: Vec<usize>,

    // Possible additions:
    // pub color: String,           // Custom node colors
    // pub icon: Option<String>,    // Icon identifier
    // pub collapsed: bool,         // Hide/show children
    // pub metadata: HashMap<String, String>,  // User-defined data
}
```

---

## See Also

- `src/mind_map.rs` - MindMap container structure
- `src/app.rs` - Rendering and interaction logic
- `docs/mind_map.md` - MindMap reference guide

---

**Generated for Mind Map v0.1.0**
