use tauri::image::Image;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::path::BaseDirectory;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Position};
use tauri_plugin_store::StoreExt;

use crate::panel::{position_panel_at_tray_icon, show_panel, hide_panel, is_panel_visible};

const LOG_LEVEL_STORE_KEY: &str = "logLevel";

fn get_stored_log_level(app_handle: &AppHandle) -> log::LevelFilter {
    let store = match app_handle.store("settings.json") {
        Ok(s) => s,
        Err(_) => return log::LevelFilter::Error,
    };
    let value = store.get(LOG_LEVEL_STORE_KEY);
    let level_str = value.and_then(|v| v.as_str().map(|s| s.to_string()));
    match level_str.as_deref() {
        Some("error") => log::LevelFilter::Error,
        Some("warn") => log::LevelFilter::Warn,
        Some("info") => log::LevelFilter::Info,
        Some("debug") => log::LevelFilter::Debug,
        Some("trace") => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error, // Default: least verbose
    }
}

fn set_stored_log_level(app_handle: &AppHandle, level: log::LevelFilter) {
    let level_str = match level {
        log::LevelFilter::Error => "error",
        log::LevelFilter::Warn => "warn",
        log::LevelFilter::Info => "info",
        log::LevelFilter::Debug => "debug",
        log::LevelFilter::Trace => "trace",
        log::LevelFilter::Off => "off",
    };
    log::info!("Log level changing to {:?}", level);
    if let Ok(store) = app_handle.store("settings.json") {
        store.set(LOG_LEVEL_STORE_KEY, serde_json::json!(level_str));
        let _ = store.save();
    }
    log::set_max_level(level);
}


fn ensure_panel_initialized(app_handle: &AppHandle) {
    if let Err(err) = crate::panel::init(app_handle) {
        log::error!("Failed to init panel: {}", err);
    }
}

fn show_panel_with_init(app_handle: &AppHandle) {
    ensure_panel_initialized(app_handle);
    show_panel(app_handle);
}

fn should_toggle_panel(button: MouseButton, button_state: MouseButtonState) -> bool {
    button == MouseButton::Left && button_state == MouseButtonState::Up
}

fn tray_click_icon_position(event_position: PhysicalPosition<f64>, rect_position: Position) -> Position {
    #[cfg(target_os = "windows")]
    {
        let _ = rect_position;
        return Position::Physical(PhysicalPosition::new(
            event_position.x.round() as i32,
            event_position.y.round() as i32,
        ));
    }

    #[cfg(not(target_os = "windows"))]
    {
        rect_position
    }
}

fn load_tray_icon(app_handle: &AppHandle) -> tauri::Result<Image<'static>> {
    #[cfg(target_os = "windows")]
    {
        let icon_png_path = app_handle
            .path()
            .resolve("icons/icon.png", BaseDirectory::Resource)?;
        return Image::from_path(icon_png_path);
    }

    #[cfg(not(target_os = "windows"))]
    {
    let tray_icon_path = app_handle
        .path()
        .resolve("icons/tray-icon.png", BaseDirectory::Resource)?;
    Image::from_path(tray_icon_path)
    }
}

pub fn create(app_handle: &AppHandle) -> tauri::Result<()> {
    let icon = load_tray_icon(app_handle)?;

    // Load persisted log level
    let current_level = get_stored_log_level(app_handle);
    log::set_max_level(current_level);

    let show_stats = MenuItem::with_id(app_handle, "show_stats", "Show Stats", true, None::<&str>)?;
    let go_to_settings = MenuItem::with_id(app_handle, "go_to_settings", "Go to Settings", true, None::<&str>)?;

    // Log level submenu - clone items for use in event handler
    let log_error = CheckMenuItem::with_id(app_handle, "log_error", "Error", true, current_level == log::LevelFilter::Error, None::<&str>)?;
    let log_warn = CheckMenuItem::with_id(app_handle, "log_warn", "Warn", true, current_level == log::LevelFilter::Warn, None::<&str>)?;
    let log_info = CheckMenuItem::with_id(app_handle, "log_info", "Info", true, current_level == log::LevelFilter::Info, None::<&str>)?;
    let log_debug = CheckMenuItem::with_id(app_handle, "log_debug", "Debug", true, current_level == log::LevelFilter::Debug, None::<&str>)?;
    let log_trace = CheckMenuItem::with_id(app_handle, "log_trace", "Trace", true, current_level == log::LevelFilter::Trace, None::<&str>)?;
    let log_level_submenu = Submenu::with_items(
        app_handle,
        "Debug Level",
        true,
        &[&log_error, &log_warn, &log_info, &log_debug, &log_trace],
    )?;

    // Clone for capture in event handler
    let log_items = [
        (log_error.clone(), log::LevelFilter::Error),
        (log_warn.clone(), log::LevelFilter::Warn),
        (log_info.clone(), log::LevelFilter::Info),
        (log_debug.clone(), log::LevelFilter::Debug),
        (log_trace.clone(), log::LevelFilter::Trace),
    ];

    let separator = PredefinedMenuItem::separator(app_handle)?;
    let about = MenuItem::with_id(app_handle, "about", "About OpenUsage", true, None::<&str>)?;
    let quit = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app_handle, &[&show_stats, &go_to_settings, &log_level_submenu, &separator, &about, &quit])?;

    TrayIconBuilder::with_id("tray")
        .icon(icon)
        .icon_as_template(cfg!(target_os = "macos"))
        .tooltip("OpenUsage")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app_handle, event| {
            log::debug!("tray menu: {}", event.id.as_ref());
            match event.id.as_ref() {
                "show_stats" => {
                    show_panel_with_init(app_handle);
                    let _ = app_handle.emit("tray:navigate", "home");
                }
                "go_to_settings" => {
                    show_panel_with_init(app_handle);
                    let _ = app_handle.emit("tray:navigate", "settings");
                }
                "about" => {
                    show_panel_with_init(app_handle);
                    let _ = app_handle.emit("tray:show-about", ());
                }
                "quit" => {
                    log::info!("quit requested via tray");
                    app_handle.exit(0);
                }
                "log_error" | "log_warn" | "log_info" | "log_debug" | "log_trace" => {
                    let selected_level = match event.id.as_ref() {
                        "log_error" => log::LevelFilter::Error,
                        "log_warn" => log::LevelFilter::Warn,
                        "log_info" => log::LevelFilter::Info,
                        "log_debug" => log::LevelFilter::Debug,
                        "log_trace" => log::LevelFilter::Trace,
                        _ => unreachable!(),
                    };
                    set_stored_log_level(app_handle, selected_level);
                    // Update all checkmarks - only the selected level should be checked
                    for (item, level) in &log_items {
                        let _ = item.set_checked(*level == selected_level);
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            let app_handle = tray.app_handle();

            if let TrayIconEvent::Click {
                button,
                button_state,
                position,
                rect,
                ..
            } = event
            {
                if should_toggle_panel(button, button_state) {
                    ensure_panel_initialized(app_handle);

                    if is_panel_visible(app_handle) {
                        log::debug!("tray click: hiding panel");
                        hide_panel(app_handle);
                        return;
                    }
                    log::debug!("tray click: showing panel");

                    // Must show window before positioning to another monitor
                    show_panel(app_handle);
                    let icon_position = tray_click_icon_position(position, rect.position);
                    position_panel_at_tray_icon(app_handle, icon_position, rect.size);
                }
            }
        })
        .build(app_handle)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{should_toggle_panel, tray_click_icon_position};
    use tauri::{PhysicalPosition, Position};
    use tauri::tray::{MouseButton, MouseButtonState};

    #[test]
    fn toggles_panel_on_left_button_up() {
        assert!(should_toggle_panel(MouseButton::Left, MouseButtonState::Up));
    }

    #[test]
    fn does_not_toggle_panel_on_right_button_up() {
        assert!(!should_toggle_panel(MouseButton::Right, MouseButtonState::Up));
    }

    #[test]
    fn does_not_toggle_panel_on_left_button_down() {
        assert!(!should_toggle_panel(MouseButton::Left, MouseButtonState::Down));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn uses_physical_click_position_on_windows() {
        let resolved = tray_click_icon_position(
            PhysicalPosition::new(1200.6, 800.4),
            Position::Physical(PhysicalPosition::new(10, 20)),
        );
        match resolved {
            Position::Physical(pos) => {
                assert_eq!(pos.x, 1201);
                assert_eq!(pos.y, 800);
            }
            _ => panic!("expected physical position"),
        }
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn keeps_rect_position_on_non_windows() {
        let rect_position = Position::Physical(PhysicalPosition::new(10, 20));
        let resolved = tray_click_icon_position(PhysicalPosition::new(1200.6, 800.4), rect_position);
        match resolved {
            Position::Physical(pos) => {
                assert_eq!(pos.x, 10);
                assert_eq!(pos.y, 20);
            }
            _ => panic!("expected physical position"),
        }
    }
}
