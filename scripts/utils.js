/**
 * Utility functions for Mind Map application
 */

/**
 * Extract coordinates from mouse or touch event
 * @param {MouseEvent|TouchEvent} e - The event object
 * @param {HTMLCanvasElement} canvas - The canvas element
 * @returns {{offsetX: number, offsetY: number}} Coordinates relative to canvas
 */
export function getEventCoordinates(e, canvas) {
    const rect = canvas.getBoundingClientRect();
    let clientX, clientY;

    if (e.touches && e.touches.length > 0) {
        clientX = e.touches[0].clientX;
        clientY = e.touches[0].clientY;
    } else if (e.changedTouches && e.changedTouches.length > 0) {
        clientX = e.changedTouches[0].clientX;
        clientY = e.changedTouches[0].clientY;
    } else {
        clientX = e.clientX;
        clientY = e.clientY;
    }

    return {
        offsetX: clientX - rect.left,
        offsetY: clientY - rect.top
    };
}

/**
 * Calculate distance between two fingers in a touch event
 * @param {TouchEvent} e - The touch event
 * @returns {number} Distance in pixels
 */
export function getTwoFingerDistance(e) {
    if (e.touches.length >= 2) {
        const dx = e.touches[0].clientX - e.touches[1].clientX;
        const dy = e.touches[0].clientY - e.touches[1].clientY;
        return Math.sqrt(dx * dx + dy * dy);
    }
    return 0;
}

/**
 * Resize canvas dynamically based on window size
 * @param {HTMLCanvasElement} canvas - The canvas element
 * @param {MindMapApp} app - The MindMapApp instance
 */
export function resizeCanvas(canvas, app) {
    const maxWidth = Math.min(800, window.innerWidth - 40);
    const maxHeight = Math.min(600, window.innerHeight - 300);
    canvas.width = maxWidth;
    canvas.height = maxHeight;
    if (app) {
        app.render();
    }
}
