# MindMap Reference Guide

**File:** `src/mind_map.rs`
**Last Updated:** 2025-10-25

## Overview

`MindMap` is the core data structure that manages the mind map's nodes, hierarchy, and selection state. It provides methods for creating, querying, updating, and deleting nodes while maintaining parent-child relationships.

**Key Responsibilities:**
- Node storage and lifecycle management
- Multi-node selection state tracking
- Drag-and-drop state management
- Spatial queries (find nodes at coordinates or within areas)
- Hierarchical relationship maintenance

---

## Structure

### `MindMap`

**Fields:**
```rust
nodes: Vec<Node>                          // All nodes in the mind map
next_id: usize                            // Auto-incrementing ID counter
root_id: Option<usize>                    // ID of the root node
selected_nodes: HashSet<usize>            // Currently selected node IDs
dragging_nodes: HashSet<usize>            // Nodes being dragged
drag_offsets: HashMap<usize, (f64, f64)> // Drag offset for each node
```

**Selection State:**
- `selected_nodes`: Supports multi-selection (can contain 0-N node IDs)
- `dragging_nodes`: Temporary set during active drag operations
- `drag_offsets`: Stores cursor-to-node offset for smooth dragging

**Node Storage:**
- All nodes stored in flat `Vec<Node>` for efficient iteration
- Parent-child relationships maintained via `Node.children` field
- Root node tracked separately in `root_id`

---

## Public API Reference

### Constructor

#### `new() -> Self`

Creates an empty MindMap.

**Returns:**
- `MindMap` - New instance with empty node list

**Initial State:**
- No nodes
- No root
- Empty selections

---

### Node Creation

#### `create_root(text: String, x: f64, y: f64) -> usize`

Creates the root node of the mind map.

**Parameters:**
- `text` - Initial text for root node
- `x` - X coordinate in canvas space
- `y` - Y coordinate in canvas space

**Returns:**
- `usize` - ID of the created root node

**Behavior:**
- Assigns next available ID
- Creates node at specified position
- Sets as `root_id`
- Automatically selects the root node

**Example:**
```rust
let mut mind_map = MindMap::new();
let root_id = mind_map.create_root("My Mind Map".to_string(), 400.0, 300.0);
```

---

#### `add_child(parent_id: usize, text: String, canvas_width: f64) -> Option<usize>`

Adds a child node to a parent node.

**Parameters:**
- `parent_id` - ID of the parent node
- `text` - Text for the new child node
- `canvas_width` - Canvas width (for responsive spacing)

**Returns:**
- `Some(usize)` - ID of created child node
- `None` - If parent node not found

**Behavior:**
- Automatically positions child to the right of parent
- Vertical spacing based on number of existing children
- Responsive spacing: smaller gaps on mobile (<600px width)
- Updates parent's `children` list

**Spacing:**
- Horizontal: 150px (desktop) / 120px (mobile)
- Vertical: 60px (desktop) / 50px (mobile)

**Example:**
```rust
let child_id = mind_map.add_child(root_id, "Child 1".to_string(), 800.0)?;
```

---

### Node Access

#### `get_node(id: usize) -> Option<&Node>`

Retrieves immutable reference to a node.

**Parameters:**
- `id` - Node ID

**Returns:**
- `Some(&Node)` - Reference to node
- `None` - If node not found

---

#### `get_node_mut(id: usize) -> Option<&mut Node>`

Retrieves mutable reference to a node.

**Parameters:**
- `id` - Node ID

**Returns:**
- `Some(&mut Node)` - Mutable reference to node
- `None` - If node not found

**Use Cases:**
- Updating node position during drag
- Modifying node text

---

### Spatial Queries

#### `find_node_at(x: f64, y: f64, canvas_width: f64) -> Option<usize>`

Finds the topmost node at given coordinates.

**Parameters:**
- `x` - X coordinate in virtual canvas space
- `y` - Y coordinate in virtual canvas space
- `canvas_width` - Canvas width (for responsive hit detection)

**Returns:**
- `Some(usize)` - ID of topmost node at coordinates
- `None` - If no node at that position

**Behavior:**
- Searches in **reverse order** (top-to-bottom Z-order)
- Uses `Node.contains_point()` for hit testing
- Returns first match (topmost node)

**Example:**
```rust
if let Some(node_id) = mind_map.find_node_at(250.0, 180.0, 800.0) {
    println!("Clicked node: {}", node_id);
}
```

---

#### `find_nodes_in_rect(x1: f64, y1: f64, x2: f64, y2: f64, _canvas_width: f64) -> Vec<usize>`

Finds all nodes within a rectangular area.

**Parameters:**
- `x1, y1` - First corner of rectangle (virtual coordinates)
- `x2, y2` - Opposite corner of rectangle
- `_canvas_width` - (unused, kept for API consistency)

**Returns:**
- `Vec<usize>` - IDs of all nodes within rectangle

**Behavior:**
- Normalizes rectangle (min/max coordinates)
- Checks node center position (not bounding box)
- Returns all matching nodes

**Use Case:**
- Area drag selection (drag-to-select multiple nodes)

**Example:**
```rust
// Select all nodes in a rectangle
let nodes_in_area = mind_map.find_nodes_in_rect(100.0, 100.0, 400.0, 300.0, 800.0);
mind_map.selected_nodes.extend(nodes_in_area);
```

---

### Selection Management

#### `clear_selection()`

Clears all selected nodes.

**Behavior:**
- Empties `selected_nodes` set
- Does not affect dragging state

**Use Cases:**
- Clicking on empty canvas
- Starting new selection

---

### Node Modification

#### `update_node_text(id: usize, text: String)`

Updates the text of a node.

**Parameters:**
- `id` - Node ID to update
- `text` - New text content

**Behavior:**
- No-op if node not found
- Updates node's `text` field

---

#### `delete_node(id: usize) -> bool`

Recursively deletes a node and all its descendants.

**Parameters:**
- `id` - Node ID to delete

**Returns:**
- `true` - Node was deleted
- `false` - Node was not deleted (is root or not found)

**Behavior:**
1. **Root Protection**: Root node cannot be deleted
2. **Recursive Deletion**: All child nodes deleted recursively
3. **Parent Cleanup**: Removes node from parent's `children` list
4. **State Cleanup**: Removes from selections and drag state

**Example:**
```rust
if mind_map.delete_node(node_id) {
    println!("Node deleted successfully");
} else {
    println!("Cannot delete root node");
}
```

---

## Node Lifecycle

```
[Creation]
    |
    v
create_root() / add_child()
    |
    v
[Active] <---> [Selected] <---> [Dragging]
    |             |                 |
    |             v                 |
    |       (multi-selection)       |
    |                               |
    v                               v
update_node_text()        Update position via get_node_mut()
    |
    v
delete_node()
    |
    v
[Deleted]
```

---

## Multi-Selection Support

The MindMap supports multi-node selection for advanced interactions:

**Selection Operations:**
```rust
// Single selection (replace existing)
mind_map.clear_selection();
mind_map.selected_nodes.insert(node_id);

// Add to selection
mind_map.selected_nodes.insert(node_id);

// Remove from selection
mind_map.selected_nodes.remove(&node_id);

// Check selection
if mind_map.selected_nodes.contains(&node_id) {
    // Node is selected
}

// Get count
let count = mind_map.selected_nodes.len();
```

---

## Usage Patterns

### Creating a Mind Map
```rust
let mut mind_map = MindMap::new();

// Create root
let root = mind_map.create_root("Project".to_string(), 400.0, 300.0);

// Add children
let task1 = mind_map.add_child(root, "Task 1".to_string(), 800.0).unwrap();
let task2 = mind_map.add_child(root, "Task 2".to_string(), 800.0).unwrap();

// Add sub-tasks
mind_map.add_child(task1, "Subtask 1.1".to_string(), 800.0);
```

### Area Selection
```rust
// User drags from (100, 100) to (400, 300)
let nodes = mind_map.find_nodes_in_rect(100.0, 100.0, 400.0, 300.0, 800.0);

mind_map.clear_selection();
for node_id in nodes {
    mind_map.selected_nodes.insert(node_id);
}
```

### Multi-Node Dragging
```rust
// Store drag offsets for all selected nodes
mind_map.drag_offsets.clear();
for &node_id in &mind_map.selected_nodes {
    if let Some(node) = mind_map.get_node(node_id) {
        let offset_x = cursor_x - node.x;
        let offset_y = cursor_y - node.y;
        mind_map.drag_offsets.insert(node_id, (offset_x, offset_y));
        mind_map.dragging_nodes.insert(node_id);
    }
}

// During drag: update all positions
for &node_id in &mind_map.dragging_nodes {
    if let Some(&(ox, oy)) = mind_map.drag_offsets.get(&node_id) {
        if let Some(node) = mind_map.get_node_mut(node_id) {
            node.x = cursor_x - ox;
            node.y = cursor_y - oy;
        }
    }
}
```

---

## Design Notes

**Why Flat Vec?**
- Efficient iteration for rendering
- Simple memory layout
- Easy serialization

**Why HashSet for Selection?**
- O(1) lookup for selection checks
- Prevents duplicate selections
- Easy set operations (union, difference)

**Why HashMap for Drag Offsets?**
- Associates each node with its specific offset
- Enables smooth multi-node dragging
- Automatically cleaned up after drag

---

## See Also

- `src/node.rs` - Node structure definition
- `src/app.rs` - App controller (uses MindMap)
- `docs/node.md` - Node reference guide

---

**Generated for Mind Map v0.1.0**
