use rdev::listen;
use std::{sync::Arc, thread, time::Duration};
use tauri::{Manager, WindowEvent};

mod game_state;
mod input_tracking;
mod kingdom;
mod level_catalog;
mod persistence;
mod power_upgrades;
mod settings;
mod shop;
mod shop_catalog;

use game_state::{InputCounts, InputSnapshot, PurchaseResult};
use persistence::{load_or_create_save, reset_counts_to_path, save_counts_to_path, SAVE_FILE_NAME};
use settings::{
    apply_app_settings, get_app_settings, load_settings_file, reset_app_settings,
    update_app_settings, SettingsState, SETTINGS_FILE_NAME,
};

const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(15);

#[tauri::command]
fn get_input_counts(counts: tauri::State<'_, Arc<InputCounts>>) -> InputSnapshot {
    counts.snapshot()
}

#[tauri::command]
fn record_focused_keypress(counts: tauri::State<'_, Arc<InputCounts>>, event_at_millis: u64) {
    counts.record_focused_keypress(event_at_millis);
}

#[tauri::command]
fn purchase_shop_item(
    counts: tauri::State<'_, Arc<InputCounts>>,
    item_id: String,
) -> PurchaseResult {
    counts.purchase_shop_item(&item_id)
}

#[tauri::command]
fn reset_progress(
    app: tauri::AppHandle,
    counts: tauri::State<'_, Arc<InputCounts>>,
) -> Result<InputSnapshot, String> {
    let save_path = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join(SAVE_FILE_NAME);

    reset_counts_to_path(&counts, &save_path).map_err(|error| error.to_string())
}

#[tauri::command]
fn set_kingdom_name(
    counts: tauri::State<'_, Arc<InputCounts>>,
    name: String,
) -> Result<InputSnapshot, String> {
    counts.set_kingdom_name(&name)
}

#[tauri::command]
fn mark_story_event_seen(counts: tauri::State<'_, Arc<InputCounts>>, event_id: String) {
    counts.mark_story_event_seen(event_id);
}

#[tauri::command]
fn mark_shop_item_seen(counts: tauri::State<'_, Arc<InputCounts>>, item_id: String) {
    counts.mark_shop_item_seen(item_id);
}

#[tauri::command]
fn dev_add_influence(counts: tauri::State<'_, Arc<InputCounts>>, amount: u64) -> InputSnapshot {
    counts.dev_add_influence(amount)
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle, counts: tauri::State<'_, Arc<InputCounts>>) {
    if let Ok(save_path) = app
        .path()
        .app_data_dir()
        .map(|path| path.join(SAVE_FILE_NAME))
    {
        if let Err(error) = save_counts_to_path(&counts, &save_path) {
            eprintln!("exit save failed: {error}");
        }
    }

    app.exit(0);
}

fn start_input_listener(counts: Arc<InputCounts>) {
    thread::spawn(move || {
        let callback = move |event: rdev::Event| {
            counts.record_event(event.event_type);
        };

        if let Err(error) = listen(callback) {
            eprintln!("global input listener stopped: {error:?}");
        }
    });
}

fn start_autosave(counts: Arc<InputCounts>, save_path: std::path::PathBuf) {
    thread::spawn(move || loop {
        thread::sleep(AUTOSAVE_INTERVAL);

        if !counts.take_dirty() {
            continue;
        }

        if let Err(error) = save_counts_to_path(&counts, &save_path) {
            counts.mark_dirty();
            eprintln!("autosave failed: {error}");
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let input_counts = Arc::new(InputCounts::default());
    let app_settings = Arc::new(SettingsState::default());
    let setup_counts = input_counts.clone();
    let setup_settings = app_settings.clone();
    let close_counts = input_counts.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(input_counts)
        .manage(app_settings)
        .setup(move |app| {
            let save_path = app.path().app_data_dir()?.join(SAVE_FILE_NAME);
            let settings_path = app.path().app_data_dir()?.join(SETTINGS_FILE_NAME);
            let settings = load_settings_file(&settings_path);

            if let Err(error) = load_or_create_save(&setup_counts, &save_path) {
                eprintln!("initial save load failed: {error}");
            }

            setup_counts.set_input_enabled(settings.character_created);
            setup_settings.load(settings.clone());
            if let Err(error) = apply_app_settings(app.handle(), &settings) {
                eprintln!("settings apply failed: {error}");
            }

            start_input_listener(setup_counts.clone());
            start_autosave(setup_counts.clone(), save_path);

            Ok(())
        })
        .on_window_event(move |window, event| {
            if matches!(event, WindowEvent::CloseRequested { .. }) {
                if let Ok(save_path) = window
                    .path()
                    .app_data_dir()
                    .map(|path| path.join(SAVE_FILE_NAME))
                {
                    if let Err(error) = save_counts_to_path(&close_counts, &save_path) {
                        eprintln!("final save failed: {error}");
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            dev_add_influence,
            exit_app,
            get_app_settings,
            get_input_counts,
            purchase_shop_item,
            reset_progress,
            reset_app_settings,
            set_kingdom_name,
            mark_story_event_seen,
            mark_shop_item_seen,
            record_focused_keypress,
            update_app_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
