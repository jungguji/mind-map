# MindMapApp Reference Guide

**File:** `src/app.rs`
**Last Updated:** 2025-10-25

## Overview

`MindMapApp` is the main application controller for the interactive mind map canvas. It handles all user interactions (mouse/touch events), manages the viewport state, and orchestrates rendering of the mind map on an HTML5 canvas via WebAssembly.

**Key Responsibilities:**
- User input event handling (mouse, touch, keyboard)
- Canvas rendering and viewport management
- Multi-node selection and dragging
- Canvas panning with keyboard/touch gestures

---

## Structures

### `MindMapApp`

The primary application struct exposed to JavaScript via `wasm_bindgen`.

**Fields:**
```rust
canvas: HtmlCanvasElement          // HTML canvas element
context: CanvasRenderingContext2d  // 2D rendering context
mind_map: MindMap                  // Mind map data structure
viewport_offset_x: f64             // Horizontal pan offset
viewport_offset_y: f64             // Vertical pan offset
is_panning: bool                   // Active panning state
pan_start_x: f64                   // Pan start X coordinate
pan_start_y: f64                   // Pan start Y coordinate
is_space_pressed: bool             // Space key state (for panning)
selection_box: Option<SelectionBox> // Area selection rectangle
```

### `SelectionBox`

Internal struct for area selection (drag-to-select multiple nodes).

**Fields:**
```rust
start_x: f64   // Selection start X
start_y: f64   // Selection start Y
end_x: f64     // Selection end X (current cursor position)
end_y: f64     // Selection end Y (current cursor position)
```

---

## Public API Reference

### Constructor

#### `new(canvas: HtmlCanvasElement) -> Result<MindMapApp, JsValue>`

Creates a new MindMapApp instance.

**Parameters:**
- `canvas` - HTML canvas element reference

**Returns:**
- `Result<MindMapApp, JsValue>` - App instance or error

**Behavior:**
- Initializes 2D rendering context
- Creates root node at canvas center
- Sets up initial viewport state

---

### Event Handlers

#### Mouse Events

##### `handle_mouse_down(event: MouseEvent)`
Handles mouse button press.

**Behavior:**
- If Space key pressed: Start panning
- If clicking a node: Select node and prepare for dragging
- If clicking empty space: Start area selection box

##### `handle_mouse_move(event: MouseEvent)`
Handles mouse movement.

**Behavior:**
- If panning: Update viewport offset
- If dragging nodes: Move all selected nodes
- If area selecting: Update selection box size

##### `handle_mouse_up(event: MouseEvent)`
Handles mouse button release.

**Behavior:**
- Complete area selection (select nodes in rectangle)
- End node dragging
- End canvas panning

##### `handle_double_click(event: MouseEvent)`
Handles mouse double-click.

**Behavior:**
- Select clicked node (single selection)
- Trigger edit mode in UI

---

#### Touch Events

##### `handle_pointer_down(x: f64, y: f64)`
Touch/pointer down handler (coordinate-based).

##### `handle_pointer_move(x: f64, y: f64)`
Touch/pointer move handler (coordinate-based).

##### `handle_pointer_up()`
Touch/pointer release handler.

##### `handle_pointer_double_click(x: f64, y: f64)`
Touch double-tap handler (coordinate-based).

**Note:** Touch handlers mirror mouse event behavior with coordinate conversion handled in JavaScript.

---

#### Keyboard Events

##### `handle_key_down(event: KeyboardEvent)`
Handles keyboard input.

**Supported Keys:**
- `Delete` / `Backspace`: Delete selected node(s)

##### `set_space_pressed(pressed: bool)`
Sets Space key state for panning mode.

**Parameters:**
- `pressed` - `true` when Space is held, `false` when released

---

### Node Operations

##### `add_child_to_selected(text: String)`
Adds a child node to the selected node.

**Conditions:**
- Requires exactly **one** node selected
- No-op if multiple nodes selected

##### `update_selected_text(text: String)`
Updates the text of the selected node.

**Conditions:**
- Requires exactly **one** node selected

##### `get_selected_text() -> Option<String>`
Retrieves the text of the selected node.

**Returns:**
- `Some(String)` if exactly one node selected
- `None` if zero or multiple nodes selected

##### `delete_selected_node() -> bool`
Deletes all currently selected node(s).

**Returns:**
- `true` if any nodes were deleted
- `false` otherwise

**Note:** Root node cannot be deleted.

---

### Rendering

##### `render()`
Renders the mind map to the canvas.

**Rendering Order:**
1. Clear canvas
2. Draw node connections (lines)
3. Draw nodes (rectangles with text)
4. Draw selection box (if active)

**Visual Styling:**
- **Selected nodes:** Green (#4CAF50), thick border (3px)
- **Unselected nodes:** Blue (#2196F3), normal border (2px)
- **Selection box:** Green stroke with semi-transparent fill

---

## Key Features

### Multi-Node Selection
- **Area Drag:** Drag in empty space to create selection rectangle
- **Visual Feedback:** Semi-transparent green box during selection
- **Result:** All nodes within rectangle become selected

### Multi-Node Dragging
- Click on any selected node to drag all selected nodes together
- Relative positions preserved during drag

### Canvas Panning
- **Desktop:** Hold `Space` key + drag
- **Mobile:** Two-finger drag (handled in JavaScript)
- Updates `viewport_offset_x/y` for smooth pan

### Coordinate Conversion
- **Virtual Coordinates:** Node positions in infinite canvas space
- **Screen Coordinates:** Actual pixel positions on canvas
- Conversion: `screen = virtual + viewport_offset`

---

## Event Flow

```
User Input (Mouse/Touch)
    |
    v
Event Handler (public methods)
    |
    v
Internal Logic (handle_*_internal)
    |
    +---> Update State (selection, dragging, panning)
    |
    +---> Modify MindMap (node positions, selections)
    |
    v
render()
    |
    v
Canvas Update (visual feedback)
```

---

## Usage Example

```javascript
import init, { MindMapApp } from './pkg/mind_map.js';

async function run() {
    await init();

    const canvas = document.getElementById('canvas');
    const app = new MindMapApp(canvas);

    // Initial render
    app.render();

    // Attach event listeners
    canvas.addEventListener('mousedown', (e) => app.handle_mouse_down(e));
    canvas.addEventListener('mousemove', (e) => app.handle_mouse_move(e));
    canvas.addEventListener('mouseup', (e) => app.handle_mouse_up(e));
    canvas.addEventListener('dblclick', (e) => app.handle_double_click(e));

    // Keyboard for panning
    document.addEventListener('keydown', (e) => {
        if (e.code === 'Space') app.set_space_pressed(true);
        app.handle_key_down(e);
    });
    document.addEventListener('keyup', (e) => {
        if (e.code === 'Space') app.set_space_pressed(false);
    });

    // Add child node
    app.add_child_to_selected("New Node");
}

run();
```

---

## See Also

- `src/mind_map.rs` - Mind map data structure
- `src/node.rs` - Node definition
- `index.html` - JavaScript integration and touch handling

---

**Generated for Mind Map v0.1.0**
