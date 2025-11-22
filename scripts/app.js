/**
 * Main application module for Mind Map
 * Initializes and coordinates all modules
 */

import init, { MindMapApp } from '../pkg/mind_map.js';
import { resizeCanvas } from './utils.js';
import { setupMouseEvents, setupTouchEvents, setupKeyboardEvents } from './event-handlers.js';
import { setupInlineEditing, setupButtons, setupInfoSection } from './ui-controller.js';

/**
 * Initialize and run the Mind Map application
 */
async function run() {
    // Initialize WebAssembly module
    await init();

    const canvas = document.getElementById('canvas');

    // Create and setup canvas size
    const app = new MindMapApp(canvas);
    window.app = app; // Expose globally for debugging

    // Setup canvas resize handler
    function handleResize() {
        resizeCanvas(canvas, app);
    }

    handleResize();
    window.addEventListener('resize', handleResize);

    // Initial render
    app.render();

    // Setup UI components
    const editingController = setupInlineEditing(canvas, app);
    setupButtons(app);
    setupInfoSection();

    // Setup event handlers
    setupMouseEvents(canvas, app);
    setupTouchEvents(canvas, app);
    setupKeyboardEvents(app, editingController.isEditing);

    console.log('Mind Map application initialized successfully');
}

// Run the application
run();
