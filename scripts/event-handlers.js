/**
 * Event handlers for Mind Map application
 */

import { getEventCoordinates, getTwoFingerDistance } from './utils.js';

/**
 * Setup mouse event handlers
 * @param {HTMLCanvasElement} canvas - The canvas element
 * @param {MindMapApp} app - The MindMapApp instance
 */
export function setupMouseEvents(canvas, app) {
    canvas.addEventListener('mousedown', (e) => {
        app.handle_mouse_down(e);
    });

    canvas.addEventListener('mousemove', (e) => {
        app.handle_mouse_move(e);
    });

    canvas.addEventListener('mouseup', (e) => {
        app.handle_mouse_up(e);
    });

    canvas.addEventListener('dblclick', (e) => {
        app.handle_double_click(e);
        const selectedText = app.get_selected_text();

        if (selectedText) {
            // Dispatch custom event for UI controller
            const event = new CustomEvent('nodeDoubleClick', {
                detail: {
                    offsetX: e.offsetX,
                    offsetY: e.offsetY,
                    selectedText
                }
            });
            canvas.dispatchEvent(event);
        }
    });
}

/**
 * Setup touch event handlers for mobile support
 * @param {HTMLCanvasElement} canvas - The canvas element
 * @param {MindMapApp} app - The MindMapApp instance
 */
export function setupTouchEvents(canvas, app) {
    let lastTouchTime = 0;
    let touchStartCoords = null;
    let longPressTimer = null;
    let isTwoFingerTouch = false;
    let twoFingerStartDistance = 0;

    canvas.addEventListener('touchstart', (e) => {
        e.preventDefault();

        // 2-finger touch detection → panning mode
        if (e.touches.length === 2) {
            isTwoFingerTouch = true;
            twoFingerStartDistance = getTwoFingerDistance(e);

            // Calculate center point of two fingers
            const rect = canvas.getBoundingClientRect();
            const centerX = ((e.touches[0].clientX + e.touches[1].clientX) / 2) - rect.left;
            const centerY = ((e.touches[0].clientY + e.touches[1].clientY) / 2) - rect.top;

            // Start panning mode (like holding Space key)
            app.set_space_pressed(true);
            app.handle_pointer_down(centerX, centerY);
            return;
        }

        const coords = getEventCoordinates(e, canvas);
        touchStartCoords = coords;

        // Start long press timer (500ms)
        longPressTimer = setTimeout(() => {
            // Vibration feedback (if supported)
            if (navigator.vibrate) {
                navigator.vibrate(50);
            }
            longPressTimer = null;
        }, 500);

        // Double tap detection
        const currentTime = new Date().getTime();
        const tapLength = currentTime - lastTouchTime;
        if (tapLength < 300 && tapLength > 0) {
            // Double tap
            clearTimeout(longPressTimer);
            longPressTimer = null;
            app.handle_pointer_double_click(coords.offsetX, coords.offsetY);
            const selectedText = app.get_selected_text();
            if (selectedText) {
                // Dispatch custom event for UI controller
                const event = new CustomEvent('nodeDoubleClick', {
                    detail: {
                        offsetX: coords.offsetX,
                        offsetY: coords.offsetY,
                        selectedText
                    }
                });
                canvas.dispatchEvent(event);
            }
            lastTouchTime = 0;
        } else {
            // Single tap
            app.handle_pointer_down(coords.offsetX, coords.offsetY);
            lastTouchTime = currentTime;
        }
    }, { passive: false });

    canvas.addEventListener('touchmove', (e) => {
        e.preventDefault();

        // Cancel long press timer (movement detected)
        if (longPressTimer) {
            clearTimeout(longPressTimer);
            longPressTimer = null;
        }

        // 2-finger touch: move based on center point
        if (isTwoFingerTouch && e.touches.length === 2) {
            const rect = canvas.getBoundingClientRect();
            const centerX = ((e.touches[0].clientX + e.touches[1].clientX) / 2) - rect.left;
            const centerY = ((e.touches[0].clientY + e.touches[1].clientY) / 2) - rect.top;
            app.handle_pointer_move(centerX, centerY);
            return;
        }

        const coords = getEventCoordinates(e, canvas);
        app.handle_pointer_move(coords.offsetX, coords.offsetY);
    }, { passive: false });

    canvas.addEventListener('touchend', (e) => {
        e.preventDefault();

        // Cancel long press timer
        if (longPressTimer) {
            clearTimeout(longPressTimer);
            longPressTimer = null;
        }

        // End 2-finger touch
        if (isTwoFingerTouch) {
            isTwoFingerTouch = false;
            app.set_space_pressed(false);
        }

        app.handle_pointer_up();
        touchStartCoords = null;
    }, { passive: false });
}

/**
 * Setup keyboard event handlers
 * @param {MindMapApp} app - The MindMapApp instance
 * @param {function} isEditingCallback - Function that returns whether editing is active
 */
export function setupKeyboardEvents(app, isEditingCallback) {
    document.addEventListener('keydown', (e) => {
        const isEditing = isEditingCallback();

        // Space key detection (ignore when editing or in nodeText input)
        if (e.code === 'Space' && !isEditing && document.activeElement.id !== 'nodeText') {
            e.preventDefault(); // Prevent page scroll
            app.set_space_pressed(true);
        }

        // Ignore node deletion key when editing or in nodeText input
        if (!isEditing && document.activeElement.id !== 'nodeText') {
            app.handle_key_down(e);
        }
    });

    document.addEventListener('keyup', (e) => {
        // Space key release
        if (e.code === 'Space') {
            app.set_space_pressed(false);
        }
    });
}
