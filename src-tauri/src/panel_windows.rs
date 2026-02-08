use tauri::{Manager, Position, Size};

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if max < min {
        return min;
    }
    value.max(min).min(max)
}

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

    // Get window size in physical pixels.
    let window_size = window.outer_size().unwrap_or(tauri::PhysicalSize {
        width: 400,
        height: 500,
    });
    let window_width = window_size.width as f64;
    let window_height = window_size.height as f64;

    // Find the monitor containing the tray icon.
    let monitors = window.available_monitors().unwrap_or_default();
    let target_monitor = monitors
        .iter()
        .find(|m| {
            let pos = m.position();
            let size = m.size();
            match &icon_position {
                Position::Physical(icon_pos) => {
                    icon_pos.x >= pos.x
                        && icon_pos.x < pos.x + size.width as i32
                        && icon_pos.y >= pos.y
                        && icon_pos.y < pos.y + size.height as i32
                }
                Position::Logical(icon_pos) => {
                    let monitor_scale = m.scale_factor();
                    let logical_left = pos.x as f64 / monitor_scale;
                    let logical_top = pos.y as f64 / monitor_scale;
                    let logical_right = (pos.x as f64 + size.width as f64) / monitor_scale;
                    let logical_bottom = (pos.y as f64 + size.height as f64) / monitor_scale;
                    icon_pos.x >= logical_left
                        && icon_pos.x < logical_right
                        && icon_pos.y >= logical_top
                        && icon_pos.y < logical_bottom
                }
            }
        })
        .cloned()
        .or_else(|| window.current_monitor().ok().flatten())
        .or_else(|| monitors.into_iter().next());

    let (scale_factor, monitor_left, monitor_top, monitor_right, monitor_bottom) =
        if let Some(monitor) = target_monitor {
            let pos = monitor.position();
            let size = monitor.size();
            (
                monitor.scale_factor(),
                pos.x as f64,
                pos.y as f64,
                pos.x as f64 + size.width as f64,
                pos.y as f64 + size.height as f64,
            )
        } else {
            // Fallback: allow computed position without monitor clamping.
            (
                1.0,
                f64::NEG_INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::INFINITY,
            )
        };

    let (icon_x, icon_y) = match &icon_position {
        Position::Physical(pos) => (pos.x as f64, pos.y as f64),
        Position::Logical(pos) => (pos.x * scale_factor, pos.y * scale_factor),
    };

    let (icon_width, icon_height) = match &icon_size {
        Size::Physical(size) => (size.width as f64, size.height as f64),
        Size::Logical(size) => (size.width * scale_factor, size.height * scale_factor),
    };

    // Position window centered relative to tray icon.
    let icon_center_x = icon_x + (icon_width / 2.0);
    let preferred_x = icon_center_x - (window_width / 2.0);

    // Prefer above the tray icon, fallback below if needed.
    let padding = 8.0;
    let preferred_above_y = icon_y - window_height - padding;
    let preferred_below_y = icon_y + icon_height + padding;
    let preferred_y = if preferred_above_y < monitor_top {
        preferred_below_y
    } else {
        preferred_above_y
    };

    let max_x = monitor_right - window_width;
    let max_y = monitor_bottom - window_height;
    let clamped_x = clamp(preferred_x, monitor_left, max_x);
    let clamped_y = clamp(preferred_y, monitor_top, max_y);

    let final_pos = tauri::PhysicalPosition::new(clamped_x.round() as i32, clamped_y.round() as i32);
    let _ = window.set_position(final_pos);
}

/// Show the window
pub fn show_panel(app_handle: &tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.set_always_on_top(true);
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
