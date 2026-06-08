use crate::game_state::{InputCounts, InputSnapshot, SaveData, SAVE_VERSION};
use std::{fs, path::Path};

pub const SAVE_FILE_NAME: &str = "save.json";

pub fn load_save_file(path: &Path) -> Option<SaveData> {
    let save_json = fs::read_to_string(path).ok()?;
    let save_data = serde_json::from_str::<SaveData>(&save_json).ok()?;

    (save_data.version == SAVE_VERSION).then_some(save_data)
}

pub fn load_or_create_save(
    counts: &InputCounts,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(save_data) = load_save_file(path) {
        counts.load_save(save_data);
    } else {
        save_counts_to_path(counts, path)?;
        counts.take_dirty();
    }

    Ok(())
}

pub fn save_counts_to_path(
    counts: &InputCounts,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let save_json = serde_json::to_string_pretty(&counts.snapshot_save())?;
    let temp_path = path.with_extension("json.tmp");

    fs::write(&temp_path, save_json)?;

    if path.exists() {
        fs::remove_file(path)?;
    }

    fs::rename(temp_path, path)?;

    Ok(())
}

pub fn reset_counts_to_path(
    counts: &InputCounts,
    path: &Path,
) -> Result<InputSnapshot, Box<dyn std::error::Error>> {
    let snapshot = counts.reset_progress();
    save_counts_to_path(counts, path)?;
    counts.take_dirty();

    Ok(snapshot)
}
