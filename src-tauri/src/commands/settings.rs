use crate::db::{self, AppState};
use crate::domain::settings::{AppSettings, AppSettingsInput};
use std::fs;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub fn app_settings_get(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let conn = state
        .conn
        .lock()
        .map_err(|_| "database lock poisoned".to_string())?;
    db::get_app_settings(&conn)
}

#[tauri::command]
pub fn app_settings_update(
    state: State<'_, AppState>,
    input: AppSettingsInput,
) -> Result<AppSettings, String> {
    if let Some(path) = &input.export_dir {
        fs::create_dir_all(Path::new(path))
            .map_err(|err| format!("cannot create export dir: {err}"))?;
    }
    let conn = state
        .conn
        .lock()
        .map_err(|_| "database lock poisoned".to_string())?;
    db::update_app_settings(&conn, input)
}
