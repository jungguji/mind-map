/**
 * UI Controller for Mind Map application
 * Handles inline editing, buttons, and info section
 */

/**
 * Setup inline editing functionality
 * @param {HTMLCanvasElement} canvas - The canvas element
 * @param {MindMapApp} app - The MindMapApp instance
 * @returns {{isEditing: function(): boolean}} Object with isEditing status checker
 */
export function setupInlineEditing(canvas, app) {
    const editInput = document.getElementById('editInput');
    let isEditing = false;

    /**
     * Show edit input at specified coordinates
     * @param {{offsetX: number, offsetY: number}} coords - Coordinates to show input
     * @param {string} selectedText - Current text of selected node
     */
    function showEditInput(coords, selectedText) {
        const canvasRect = canvas.getBoundingClientRect();
        const isMobile = window.innerWidth <= 768;

        if (isMobile) {
            // Mobile: position at center bottom of screen
            const inputWidth = Math.min(250, window.innerWidth - 40);
            editInput.style.left = '50%';
            editInput.style.top = (canvasRect.bottom + 10) + 'px';
            editInput.style.transform = 'translateX(-50%)';
            editInput.style.width = inputWidth + 'px';
        } else {
            // Desktop: position at node location
            const nodeX = coords.offsetX;
            const nodeY = coords.offsetY;
            const inputWidth = 120;
            editInput.style.left = (canvasRect.left + nodeX - inputWidth / 2) + 'px';
            editInput.style.top = (canvasRect.top + nodeY - 20) + 'px';
            editInput.style.transform = 'none';
            editInput.style.width = inputWidth + 'px';
        }

        editInput.value = selectedText;
        editInput.style.display = 'block';
        editInput.focus();
        editInput.select();
        isEditing = true;
    }

    /**
     * Finish editing and optionally save changes
     * @param {boolean} save - Whether to save the changes
     */
    function finishEditing(save) {
        if (isEditing) {
            if (save && editInput.value.trim()) {
                app.update_selected_text(editInput.value.trim());
            }
            editInput.style.display = 'none';
            editInput.value = '';
            isEditing = false;
        }
    }

    // Listen for custom nodeDoubleClick event
    canvas.addEventListener('nodeDoubleClick', (e) => {
        showEditInput(
            { offsetX: e.detail.offsetX, offsetY: e.detail.offsetY },
            e.detail.selectedText
        );
    });

    // Edit input event handlers
    editInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            finishEditing(true);
        } else if (e.key === 'Escape') {
            e.preventDefault();
            finishEditing(false);
        }
        e.stopPropagation(); // Prevent conflicts with other keyboard events
    });

    editInput.addEventListener('blur', (e) => {
        finishEditing(true);
    });

    return {
        isEditing: () => isEditing
    };
}

/**
 * Setup button event handlers
 * @param {MindMapApp} app - The MindMapApp instance
 */
export function setupButtons(app) {
    const nodeTextInput = document.getElementById('nodeText');

    // Add child node button
    document.getElementById('addNode').addEventListener('click', () => {
        const text = nodeTextInput.value.trim();
        if (text) {
            app.add_child_to_selected(text);
            nodeTextInput.value = '';
        }
    });

    // Update node button
    document.getElementById('updateNode').addEventListener('click', () => {
        const text = nodeTextInput.value.trim();
        if (text) {
            app.update_selected_text(text);
            nodeTextInput.value = '';
        }
    });

    // Delete node button
    document.getElementById('deleteNode').addEventListener('click', () => {
        app.delete_selected_node();
    });

    // Enter key support for adding child nodes
    nodeTextInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            const text = e.target.value.trim();
            if (text) {
                app.add_child_to_selected(text);
                e.target.value = '';
            }
        }
    });
}

/**
 * Setup info section toggle functionality (mobile)
 */
export function setupInfoSection() {
    const infoSection = document.querySelector('.info');
    const infoTitle = document.querySelector('.info h3');

    // Collapse on mobile by default
    if (window.innerWidth <= 768) {
        infoSection.classList.add('collapsed');
    }

    infoTitle.addEventListener('click', () => {
        infoSection.classList.toggle('collapsed');
    });
}
