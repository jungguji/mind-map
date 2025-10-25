# Mind Map Documentation

**Version:** 0.1.0
**Last Updated:** 2025-10-25

## Overview

An interactive mind map application built with Rust (WebAssembly) and HTML5 Canvas. Features include multi-node selection, drag-and-drop, area selection, and responsive canvas panning for both desktop and mobile devices.

**Technology Stack:**
- **Language:** Rust
- **Build Target:** WebAssembly (wasm32-unknown-unknown)
- **Rendering:** HTML5 Canvas 2D API
- **Bindings:** wasm-bindgen
- **Frontend:** Vanilla JavaScript (ES6 modules)

---

## Architecture

### Module Structure

```
mind-map/
├── src/
│   ├── lib.rs          # Entry point, module exports
│   ├── app.rs          # Application controller & event handling
│   ├── mind_map.rs     # Data structure & state management
│   └── node.rs         # Node definition & hit detection
├── index.html          # UI, JavaScript integration
├── pkg/                # Generated WebAssembly output
└── docs/               # Documentation (you are here)
```

### Component Relationships

```
┌─────────────────────────────────────────┐
│          JavaScript (index.html)        │
│  - Event listeners                      │
│  - Touch gesture detection              │
│  - UI controls                          │
└────────────┬────────────────────────────┘
             │ wasm-bindgen
             v
┌─────────────────────────────────────────┐
│         MindMapApp (app.rs)             │
│  - Event handlers                       │
│  - Viewport management                  │
│  - Rendering                            │
└────────────┬────────────────────────────┘
             │ owns
             v
┌─────────────────────────────────────────┐
│         MindMap (mind_map.rs)           │
│  - Node storage (Vec<Node>)             │
│  - Selection state (HashSet)            │
│  - Spatial queries                      │
└────────────┬────────────────────────────┘
             │ contains
             v
┌─────────────────────────────────────────┐
│           Node (node.rs)                │
│  - Data (id, text, position)            │
│  - Children references                  │
│  - Hit detection                        │
└─────────────────────────────────────────┘
```

---

## Documentation Index

### Core Modules

| Document | File | Description |
|----------|------|-------------|
| **[App Reference](./app.md)** | `src/app.rs` | Application controller, event handling, rendering |
| **[MindMap Reference](./mind_map.md)** | `src/mind_map.rs` | Data structure, node management, selections |
| **[Node Reference](./node.md)** | `src/node.rs` | Node structure, hit detection, properties |

---

## Quick Start

### Building the Project

```bash
# Install wasm-pack (first time only)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WebAssembly
wasm-pack build --target web

# Serve locally
python3 -m http.server 8000

# Open in browser
open http://localhost:8000
```

### Basic Usage

```javascript
import init, { MindMapApp } from './pkg/mind_map.js';

async function run() {
    // Initialize WASM module
    await init();

    // Create app instance
    const canvas = document.getElementById('canvas');
    const app = new MindMapApp(canvas);

    // Render initial state
    app.render();

    // Attach event handlers
    canvas.addEventListener('mousedown', (e) => app.handle_mouse_down(e));
    canvas.addEventListener('mousemove', (e) => app.handle_mouse_move(e));
    canvas.addEventListener('mouseup', (e) => app.handle_mouse_up(e));

    // Add keyboard support
    document.addEventListener('keydown', (e) => {
        if (e.code === 'Space') app.set_space_pressed(true);
        app.handle_key_down(e);
    });
    document.addEventListener('keyup', (e) => {
        if (e.code === 'Space') app.set_space_pressed(false);
    });

    // Add a child node
    app.add_child_to_selected("New Node");
}

run();
```

---

## Key Features

### 1. Multi-Node Selection

**Desktop:**
- Drag in empty space to create selection rectangle
- All nodes within rectangle become selected

**Mobile:**
- Same drag-to-select functionality with touch

**Implementation:** `MindMap.selected_nodes: HashSet<usize>`

**See:** [mind_map.md - Selection Management](./mind_map.md#selection-management)

---

### 2. Multi-Node Dragging

**Behavior:**
- Click any selected node to drag all selected nodes together
- Relative positions preserved during drag
- Smooth dragging via per-node offset tracking

**Implementation:** `MindMap.drag_offsets: HashMap<usize, (f64, f64)>`

**See:** [mind_map.md - Multi-Node Dragging](./mind_map.md#usage-patterns)

---

### 3. Canvas Panning

**Desktop:**
- Hold `Space` key + drag to pan canvas
- Prevents accidental node selection during panning

**Mobile:**
- Two-finger drag to pan
- Natural gesture for mobile users

**Implementation:** `MindMapApp.viewport_offset_x/y`

**See:** [app.md - Canvas Panning](./app.md#key-features)

---

### 4. Responsive Design

**Adaptive Sizing:**
- Nodes: 120x40px (desktop) / 100x35px (mobile)
- Font: 14px (desktop) / 12px (mobile)
- Touch padding: +5px on mobile for easier tapping

**Breakpoint:** 600px canvas width

**See:** [node.md - Node Dimensions](./node.md#node-dimensions)

---

### 5. Touch Gesture Support

**Implemented:**
- Single tap: Select/drag nodes
- Double tap: Edit node text
- Two-finger drag: Pan canvas
- Long press: Vibration feedback (500ms)

**See:** `index.html` touch event handlers

---

## Data Flow

### User Click → Node Selection

```
User clicks at (250, 180)
    |
    v
[JavaScript] MouseEvent
    |
    v
[app.rs] handle_mouse_down(event)
    |
    v
Convert screen coords → virtual coords
(subtract viewport offset)
    |
    v
[mind_map.rs] find_node_at(x, y, canvas_width)
    |
    v
For each node (reverse order):
    [node.rs] contains_point(x, y, canvas_width)?
    |
    v
Return first match (topmost node)
    |
    v
Update selected_nodes (HashSet)
    |
    v
[app.rs] render()
    |
    v
Canvas updated (selected node turns green)
```

---

### Node Creation Flow

```
User clicks "Add Child"
    |
    v
[JavaScript] Get input text
    |
    v
[app.rs] add_child_to_selected(text)
    |
    v
Get single selected node ID
    |
    v
[mind_map.rs] add_child(parent_id, text, canvas_width)
    |
    v
Calculate child position:
  - Horizontal: parent.x + spacing
  - Vertical: parent.y + (child_index * spacing)
    |
    v
[node.rs] Node::new(id, text, x, y)
    |
    v
Add to parent.children
Add to mind_map.nodes
    |
    v
[app.rs] render()
    |
    v
New node appears on canvas
```

---

## API Reference Summary

### MindMapApp (JavaScript-exposed)

**Constructor:**
- `new(canvas: HtmlCanvasElement)`

**Event Handlers:**
- `handle_mouse_down(event)`, `handle_mouse_move(event)`, `handle_mouse_up(event)`
- `handle_pointer_down(x, y)`, `handle_pointer_move(x, y)`, `handle_pointer_up()`
- `handle_key_down(event)`, `set_space_pressed(pressed)`

**Node Operations:**
- `add_child_to_selected(text)`
- `update_selected_text(text)`
- `get_selected_text()` → `Option<String>`
- `delete_selected_node()` → `bool`

**Rendering:**
- `render()`

**Full details:** [app.md](./app.md)

---

### MindMap (Internal)

**Node Management:**
- `create_root(text, x, y)` → `usize`
- `add_child(parent_id, text, canvas_width)` → `Option<usize>`
- `delete_node(id)` → `bool`

**Queries:**
- `find_node_at(x, y, canvas_width)` → `Option<usize>`
- `find_nodes_in_rect(x1, y1, x2, y2, canvas_width)` → `Vec<usize>`

**Full details:** [mind_map.md](./mind_map.md)

---

### Node (Internal)

**Constructor:**
- `new(id, text, x, y)` → `Node`

**Methods:**
- `contains_point(px, py, canvas_width)` → `bool`

**Full details:** [node.md](./node.md)

---

## Development Guide

### Adding a New Feature

1. **Update Data Model** (`mind_map.rs` / `node.rs`)
   - Add necessary fields
   - Implement data manipulation methods

2. **Update Controller** (`app.rs`)
   - Add event handlers if needed
   - Update rendering logic
   - Expose to JavaScript if public

3. **Update UI** (`index.html`)
   - Add event listeners
   - Add controls if needed
   - Update usage instructions

4. **Test**
   - Build: `wasm-pack build --target web`
   - Serve: `python3 -m http.server 8000`
   - Manual testing in browser

---

### Code Style Guidelines

**Rust:**
- Use descriptive variable names
- Add comments for complex logic
- Keep functions focused and small
- Prefer `Option<T>` over panicking

**JavaScript:**
- Use ES6+ features (async/await, arrow functions)
- Keep event handlers thin (delegate to WASM)
- Add comments for touch gesture logic

---

## Performance Considerations

**Rendering:**
- Full canvas redraw on every change (simple, reliable)
- Future: Dirty rectangle tracking for large maps

**Node Storage:**
- Flat `Vec<Node>` for cache-friendly iteration
- O(n) search, but typically n < 100 nodes

**Selection:**
- `HashSet` for O(1) selection checks
- Critical for drag performance with many nodes

**Hit Detection:**
- Reverse iteration (topmost first)
- Early return on first match
- Negligible overhead for typical mind maps

---

## Browser Compatibility

**Tested:**
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

**Requirements:**
- ES6 module support
- WebAssembly support
- Canvas 2D API
- Touch events (mobile)

---

## Future Enhancements

**Planned:**
- [ ] Node colors and icons
- [ ] Collapse/expand subtrees
- [ ] Save/load to JSON
- [ ] Undo/redo system
- [ ] Keyboard navigation
- [ ] Export to PNG/SVG

**Possible:**
- [ ] Real-time collaboration
- [ ] Auto-layout algorithms
- [ ] Search/filter nodes
- [ ] Custom node shapes

---

## Troubleshooting

### WASM not loading
```
Error: WebAssembly module not found
```
**Solution:** Run `wasm-pack build --target web` first

### Touch events not working
```
Touch not detected on mobile
```
**Solution:** Ensure `touch-action: none` in CSS and `{ passive: false }` in event listeners

### Selection box not visible
```
Drag-to-select not working
```
**Solution:** Check `selection_box` rendering in `app.rs` render method

---

## Contributing

When adding new features or fixing bugs:

1. Update relevant documentation files
2. Add code examples if adding new APIs
3. Update this README if changing architecture
4. Test on both desktop and mobile

---

## License

See project root for license information.

---

**Generated for Mind Map v0.1.0**
