use crate::game_state::InputCounts;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    sync::{Arc, Mutex},
};
use tauri::Manager;
#[cfg(windows)]
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_SET_VALUE},
    RegKey,
};

pub const SETTINGS_FILE_NAME: &str = "settings.json";

const STARTUP_REGISTRY_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const STARTUP_REGISTRY_VALUE_NAME: &str = "Kingdom";
const LEGACY_STARTUP_REGISTRY_VALUE_NAME: &str = "Damage Control";

#[derive(Default)]
pub struct SettingsState {
    settings: Mutex<AppSettings>,
}

impl SettingsState {
    pub fn load(&self, settings: AppSettings) {
        *self.settings.lock().expect("settings lock poisoned") = settings;
    }

    fn current(&self) -> AppSettings {
        self.settings
            .lock()
            .expect("settings lock poisoned")
            .clone()
    }

    fn replace(&self, settings: AppSettings) {
        *self.settings.lock().expect("settings lock poisoned") = settings;
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AppSettings {
    #[serde(default)]
    pub run_on_startup: bool,
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
    #[serde(default = "default_show_taskbar_icon")]
    pub show_taskbar_icon: bool,
    #[serde(default)]
    pub dev_mode: bool,
    #[serde(default = "default_selected_outfit_id")]
    pub selected_outfit_id: String,
    #[serde(default)]
    pub character_created: bool,
    #[serde(default = "default_character_eyes")]
    pub character_eyes: String,
    #[serde(default = "default_character_nose")]
    pub character_nose: String,
    #[serde(default = "default_character_mouth")]
    pub character_mouth: String,
    #[serde(default = "default_character_hair")]
    pub character_hair: String,
    #[serde(default = "default_character_hair_color")]
    pub character_hair_color: String,
    #[serde(default = "default_character_skin_tone")]
    pub character_skin_tone: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            run_on_startup: false,
            always_on_top: true,
            show_taskbar_icon: true,
            dev_mode: false,
            selected_outfit_id: default_selected_outfit_id(),
            character_created: false,
            character_eyes: default_character_eyes(),
            character_nose: default_character_nose(),
            character_mouth: default_character_mouth(),
            character_hair: default_character_hair(),
            character_hair_color: default_character_hair_color(),
            character_skin_tone: default_character_skin_tone(),
        }
    }
}

fn default_show_taskbar_icon() -> bool {
    true
}

fn default_always_on_top() -> bool {
    true
}

fn default_selected_outfit_id() -> String {
    "humble_rags".to_string()
}

fn default_character_eyes() -> String {
    "round".to_string()
}

fn default_character_nose() -> String {
    "button".to_string()
}

fn default_character_mouth() -> String {
    "smile".to_string()
}

fn default_character_hair() -> String {
    "bald".to_string()
}

fn default_character_hair_color() -> String {
    "brown".to_string()
}

fn default_character_skin_tone() -> String {
    "warm".to_string()
}

#[tauri::command]
pub fn get_app_settings(settings: tauri::State<'_, std::sync::Arc<SettingsState>>) -> AppSettings {
    settings.current()
}

#[tauri::command]
pub fn update_app_settings(
    app: tauri::AppHandle,
    settings_state: tauri::State<'_, std::sync::Arc<SettingsState>>,
    counts: tauri::State<'_, Arc<InputCounts>>,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    let previous_settings = settings_state.current();

    let settings = persist_app_settings(&app, &settings_state, previous_settings, settings)?;
    counts.set_input_enabled(settings.character_created);
    Ok(settings)
}

#[tauri::command]
pub fn reset_app_settings(
    app: tauri::AppHandle,
    settings_state: tauri::State<'_, std::sync::Arc<SettingsState>>,
    counts: tauri::State<'_, Arc<InputCounts>>,
) -> Result<AppSettings, String> {
    let previous_settings = settings_state.current();
    let settings = AppSettings::default();

    let settings = persist_app_settings(&app, &settings_state, previous_settings, settings)?;
    counts.set_input_enabled(false);
    Ok(settings)
}

fn persist_app_settings(
    app: &tauri::AppHandle,
    settings_state: &SettingsState,
    previous_settings: AppSettings,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    apply_window_settings(&app, &settings)?;

    if previous_settings.run_on_startup != settings.run_on_startup {
        set_run_on_startup(settings.run_on_startup).map_err(|error| error.to_string())?;
    }

    let settings_path = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join(SETTINGS_FILE_NAME);

    save_settings_to_path(&settings, &settings_path).map_err(|error| error.to_string())?;
    settings_state.replace(settings.clone());

    Ok(settings)
}

pub fn load_settings_file(path: &Path) -> AppSettings {
    fs::read_to_string(path)
        .ok()
        .and_then(|settings_json| serde_json::from_str::<AppSettings>(&settings_json).ok())
        .unwrap_or_default()
}

fn save_settings_to_path(
    settings: &AppSettings,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let settings_json = serde_json::to_string_pretty(settings)?;
    let temp_path = path.with_extension("json.tmp");

    fs::write(&temp_path, settings_json)?;

    if path.exists() {
        fs::remove_file(path)?;
    }

    fs::rename(temp_path, path)?;

    Ok(())
}

pub fn apply_app_settings(app: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    apply_window_settings(app, settings)?;
    set_run_on_startup(settings.run_on_startup).map_err(|error| error.to_string())
}

fn apply_window_settings(app: &tauri::AppHandle, settings: &AppSettings) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(settings.always_on_top)
            .map_err(|error| error.to_string())?;
        window
            .set_skip_taskbar(!settings.show_taskbar_icon)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

#[cfg(windows)]
fn set_run_on_startup(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);

    if enabled {
        let executable_path = std::env::current_exe()?;
        let executable_value = format!("\"{}\"", executable_path.display());
        let (startup_key, _) = current_user.create_subkey(STARTUP_REGISTRY_KEY)?;

        startup_key.set_value(STARTUP_REGISTRY_VALUE_NAME, &executable_value)?;
        delete_registry_value_if_present(&startup_key, LEGACY_STARTUP_REGISTRY_VALUE_NAME)?;
    } else {
        let startup_key =
            match current_user.open_subkey_with_flags(STARTUP_REGISTRY_KEY, KEY_SET_VALUE) {
                Ok(key) => key,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
                Err(error) => return Err(error.into()),
            };

        delete_registry_value_if_present(&startup_key, STARTUP_REGISTRY_VALUE_NAME)?;
        delete_registry_value_if_present(&startup_key, LEGACY_STARTUP_REGISTRY_VALUE_NAME)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_enable_always_on_top() {
        assert!(AppSettings::default().always_on_top);
    }

    #[test]
    fn default_settings_disable_dev_mode() {
        assert!(!AppSettings::default().dev_mode);
    }

    #[test]
    fn settings_without_always_on_top_use_the_enabled_default() {
        let settings: AppSettings =
            serde_json::from_str(r#"{"run_on_startup":false,"show_taskbar_icon":true}"#)
                .expect("legacy settings should deserialize");

        assert!(settings.always_on_top);
        assert!(!settings.dev_mode);
        assert_eq!(settings.selected_outfit_id, "humble_rags");
        assert!(!settings.character_created);
        assert_eq!(settings.character_eyes, "round");
        assert_eq!(settings.character_skin_tone, "warm");
    }
}

#[cfg(windows)]
fn delete_registry_value_if_present(
    key: &RegKey,
    value_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match key.delete_value(value_name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

#[cfg(not(windows))]
fn set_run_on_startup(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    if enabled {
        return Err("run on startup is only supported on Windows".into());
    }

    Ok(())
}
