use tauri::{Manager, Position, Size};

/// Initialize the panel (no-op on Windows, window is already created)
pub fn init(_app_handle: &tauri::AppHandle) -> tauri::Result<()> {
    Ok(())
}

/// Position the window near the tray icon
pub fn position_panel_at_tray_icon(
    app_handle: &tauri::AppHandle,
    icon_position: Position,
    icon_size: Size,
) {
    let Some(window) = app_handle.get_webview_window("main") else {
        return;
    };

    // Extract icon position
    let (icon_x, icon_y) = match &icon_position {
        Position::Physical(pos) => (pos.x as f64, pos.y as f64),
        Position::Logical(pos) => (pos.x, pos.y),
    };

    let (icon_width, _icon_height) = match &icon_size {
        Size::Physical(size) => (size.width as f64, size.height as f64),
        Size::Logical(size) => (size.width, size.height),
    };

    // Get window size
    let window_size = window.outer_size().unwrap_or(tauri::PhysicalSize {
        width: 400,
        height: 500,
    });

    // Find the monitor containing the tray icon
    let monitors = window.available_monitors().unwrap_or_default();
    let scale_factor = monitors
        .iter()
        .find(|m| {
            let pos = m.position();
            let size = m.size();
            icon_x >= pos.x as f64
                && icon_x < (pos.x + size.width as i32) as f64
                && icon_y >= pos.y as f64
                && icon_y < (pos.y + size.height as i32) as f64
        })
        .map(|m| m.scale_factor())
        .unwrap_or(1.0);

    let window_width = window_size.width as f64 / scale_factor;
    let window_height = window_size.height as f64 / scale_factor;

    // On Windows, tray is typically at bottom-right
    // Position window above the tray icon, centered horizontally
    let icon_center_x = icon_x + (icon_width / 2.0);
    let panel_x = icon_center_x - (window_width / 2.0);

    // Position above the tray icon with some padding
    let padding = 8.0;
    let panel_y = icon_y - window_height - padding;

    let final_pos = tauri::LogicalPosition::new(panel_x, panel_y);
    let _ = window.set_position(final_pos);
}

/// Show the window
pub fn show_panel(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

/// Hide the window
pub fn hide_panel(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
}

/// Check if the window is visible
pub fn is_panel_visible(app_handle: &tauri::AppHandle) -> bool {
    app_handle
        .get_webview_window("main")
        .map(|w| w.is_visible().unwrap_or(false))
        .unwrap_or(false)
}
