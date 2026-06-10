use crate::{
    level_catalog::{level_for_xp, xp_required_for_level},
    shop_catalog::{ShopItem, ShopItemCategory, CRUMPLED_COURT_ONBOARDING_MANUAL_ID},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

pub const SAVE_VERSION: u32 = 4;
pub struct InputCounts {
    pub(crate) keys: AtomicU64,
    pub(crate) clicks: AtomicU64,
    pub(crate) bonus_influence: AtomicU64,
    pub(crate) spent_influence: AtomicU64,
    pub(crate) crumpled_court_onboarding_manual_input_baseline: AtomicU64,
    pub(crate) crumpled_court_onboarding_manual_trigger_count: AtomicU64,
    pub(crate) pending_power_proc_inputs: AtomicU64,
    pub(crate) last_power_proc_roll_at_millis: AtomicU64,
    pub(crate) random_state: AtomicU64,
    pub(crate) power_event_sequence: AtomicU64,
    pub(crate) last_power_event_at_millis: AtomicU64,
    pub(crate) last_power_event_amount: AtomicU64,
    pub(crate) last_power_event_item_level: AtomicU64,
    pub(crate) last_input_at_millis: AtomicU64,
    pub(crate) last_global_key_at_millis: AtomicU64,
    pub(crate) seen_story_event_ids: Mutex<HashSet<String>>,
    pub(crate) seen_shop_item_ids: Mutex<HashSet<String>>,
    pub(crate) kingdom_name: Mutex<Option<String>>,
    pub(crate) pressed_keys: Mutex<HashSet<rdev::Key>>,
    pub(crate) inventory: Mutex<Inventory>,
    pub(crate) input_enabled: AtomicBool,
    pub(crate) dirty: AtomicBool,
}

impl Default for InputCounts {
    fn default() -> Self {
        Self {
            keys: AtomicU64::default(),
            clicks: AtomicU64::default(),
            bonus_influence: AtomicU64::default(),
            spent_influence: AtomicU64::default(),
            crumpled_court_onboarding_manual_input_baseline: AtomicU64::default(),
            crumpled_court_onboarding_manual_trigger_count: AtomicU64::default(),
            pending_power_proc_inputs: AtomicU64::default(),
            last_power_proc_roll_at_millis: AtomicU64::default(),
            random_state: AtomicU64::default(),
            power_event_sequence: AtomicU64::default(),
            last_power_event_at_millis: AtomicU64::default(),
            last_power_event_amount: AtomicU64::default(),
            last_power_event_item_level: AtomicU64::default(),
            last_input_at_millis: AtomicU64::default(),
            last_global_key_at_millis: AtomicU64::default(),
            seen_story_event_ids: Mutex::default(),
            seen_shop_item_ids: Mutex::default(),
            kingdom_name: Mutex::default(),
            pressed_keys: Mutex::default(),
            inventory: Mutex::default(),
            input_enabled: AtomicBool::new(true),
            dirty: AtomicBool::default(),
        }
    }
}

#[derive(Serialize)]
pub struct InputSnapshot {
    pub(crate) influence: u64,
    pub(crate) xp: u64,
    pub(crate) level: u64,
    pub(crate) xp_for_current_level: u64,
    pub(crate) xp_for_next_level: u64,
    pub(crate) keys: u64,
    pub(crate) clicks: u64,
    pub(crate) bonus_influence: u64,
    pub(crate) spent_influence: u64,
    pub(crate) power_event_sequence: u64,
    pub(crate) last_power_event_at_millis: u64,
    pub(crate) last_power_event_amount: u64,
    pub(crate) last_power_event_item_level: u64,
    pub(crate) inventory_item_ids: Vec<String>,
    pub(crate) seen_story_event_ids: Vec<String>,
    pub(crate) seen_shop_item_ids: Vec<String>,
    pub(crate) kingdom_name: Option<String>,
    pub(crate) last_input_at_millis: u64,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SaveData {
    pub(crate) version: u32,
    keys: u64,
    clicks: u64,
    #[serde(default)]
    bonus_influence: u64,
    #[serde(default)]
    spent_influence: u64,
    #[serde(default)]
    crumpled_court_onboarding_manual_input_baseline: u64,
    #[serde(default)]
    crumpled_court_onboarding_manual_trigger_count: u64,
    #[serde(default)]
    pending_power_proc_inputs: u64,
    #[serde(default)]
    last_power_proc_roll_at_millis: u64,
    #[serde(default)]
    seen_story_event_ids: Vec<String>,
    #[serde(default)]
    seen_shop_item_ids: Vec<String>,
    #[serde(default)]
    kingdom_name: Option<String>,
    #[serde(default)]
    inventory: Inventory,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Inventory {
    #[serde(default)]
    pub(crate) org_chart: Vec<String>,
    #[serde(default)]
    pub(crate) power_upgrades: Vec<String>,
}

impl Inventory {
    pub(crate) fn item_ids(&self) -> Vec<String> {
        self.org_chart
            .iter()
            .chain(self.power_upgrades.iter())
            .cloned()
            .collect()
    }

    pub(crate) fn has_item(&self, item_id: &str) -> bool {
        self.org_chart
            .iter()
            .chain(self.power_upgrades.iter())
            .any(|owned_id| owned_id == item_id)
    }

    pub(crate) fn add_item(&mut self, item: &ShopItem) {
        match item.category {
            ShopItemCategory::OrgChart => self.org_chart.push(item.id.clone()),
            ShopItemCategory::PowerUpgrade => self.power_upgrades.push(item.id.clone()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PurchaseStatus {
    Purchased,
    AlreadyOwned,
    NotEnoughInfluence,
    Locked,
    UnknownItem,
}

#[derive(Serialize)]
pub struct PurchaseResult {
    pub(crate) status: PurchaseStatus,
    pub(crate) snapshot: InputSnapshot,
}

impl InputCounts {
    pub(crate) fn set_input_enabled(&self, enabled: bool) {
        self.input_enabled.store(enabled, Ordering::Relaxed);

        if !enabled {
            self.pressed_keys
                .lock()
                .expect("pressed keys lock poisoned")
                .clear();
        }
    }

    pub(crate) fn can_record_input(&self) -> bool {
        self.input_enabled.load(Ordering::Relaxed) && self.has_kingdom_name()
    }

    pub(crate) fn snapshot(&self) -> InputSnapshot {
        self.flush_due_power_upgrades(current_time_millis());

        let keys = self.keys.load(Ordering::Relaxed);
        let clicks = self.clicks.load(Ordering::Relaxed);
        let bonus_influence = self.bonus_influence.load(Ordering::Relaxed);
        let spent_influence = self.spent_influence.load(Ordering::Relaxed);
        let xp = keys + clicks + bonus_influence;
        let level = level_for_xp(xp);
        let inventory = self.inventory.lock().expect("inventory lock poisoned");

        InputSnapshot {
            influence: xp.saturating_sub(spent_influence),
            xp,
            level,
            xp_for_current_level: xp_required_for_level(level),
            xp_for_next_level: xp_required_for_level(level + 1),
            keys,
            clicks,
            bonus_influence,
            spent_influence,
            power_event_sequence: self.power_event_sequence.load(Ordering::Relaxed),
            last_power_event_at_millis: self.last_power_event_at_millis.load(Ordering::Relaxed),
            last_power_event_amount: self.last_power_event_amount.load(Ordering::Relaxed),
            last_power_event_item_level: self.last_power_event_item_level.load(Ordering::Relaxed),
            inventory_item_ids: inventory.item_ids(),
            seen_story_event_ids: self.story_event_ids(),
            seen_shop_item_ids: self.shop_item_ids(),
            kingdom_name: self.kingdom_name(),
            last_input_at_millis: self.last_input_at_millis.load(Ordering::Relaxed),
        }
    }

    pub(crate) fn snapshot_save(&self) -> SaveData {
        self.flush_due_power_upgrades(current_time_millis());

        SaveData {
            version: SAVE_VERSION,
            keys: self.keys.load(Ordering::Relaxed),
            clicks: self.clicks.load(Ordering::Relaxed),
            bonus_influence: self.bonus_influence.load(Ordering::Relaxed),
            spent_influence: self.spent_influence.load(Ordering::Relaxed),
            crumpled_court_onboarding_manual_input_baseline: self
                .crumpled_court_onboarding_manual_input_baseline
                .load(Ordering::Relaxed),
            crumpled_court_onboarding_manual_trigger_count: self
                .crumpled_court_onboarding_manual_trigger_count
                .load(Ordering::Relaxed),
            pending_power_proc_inputs: self.pending_power_proc_inputs.load(Ordering::Relaxed),
            last_power_proc_roll_at_millis: self
                .last_power_proc_roll_at_millis
                .load(Ordering::Relaxed),
            seen_story_event_ids: self.story_event_ids(),
            seen_shop_item_ids: self.shop_item_ids(),
            kingdom_name: self.kingdom_name(),
            inventory: self
                .inventory
                .lock()
                .expect("inventory lock poisoned")
                .clone(),
        }
    }

    pub(crate) fn load_save(&self, save_data: SaveData) {
        let crumpled_court_onboarding_manual_input_baseline = if save_data
            .inventory
            .has_item(CRUMPLED_COURT_ONBOARDING_MANUAL_ID)
            && save_data.crumpled_court_onboarding_manual_input_baseline == 0
            && save_data.crumpled_court_onboarding_manual_trigger_count == 0
        {
            save_data.keys + save_data.clicks
        } else {
            save_data.crumpled_court_onboarding_manual_input_baseline
        };

        self.keys.store(save_data.keys, Ordering::Relaxed);
        self.clicks.store(save_data.clicks, Ordering::Relaxed);
        self.bonus_influence
            .store(save_data.bonus_influence, Ordering::Relaxed);
        self.spent_influence
            .store(save_data.spent_influence, Ordering::Relaxed);
        self.crumpled_court_onboarding_manual_input_baseline.store(
            crumpled_court_onboarding_manual_input_baseline,
            Ordering::Relaxed,
        );
        self.crumpled_court_onboarding_manual_trigger_count.store(
            save_data.crumpled_court_onboarding_manual_trigger_count,
            Ordering::Relaxed,
        );
        self.pending_power_proc_inputs
            .store(save_data.pending_power_proc_inputs, Ordering::Relaxed);
        self.last_power_proc_roll_at_millis
            .store(save_data.last_power_proc_roll_at_millis, Ordering::Relaxed);
        *self
            .seen_story_event_ids
            .lock()
            .expect("story events lock poisoned") =
            save_data.seen_story_event_ids.into_iter().collect();
        *self
            .seen_shop_item_ids
            .lock()
            .expect("shop items lock poisoned") =
            save_data.seen_shop_item_ids.into_iter().collect();
        *self
            .kingdom_name
            .lock()
            .expect("kingdom name lock poisoned") = save_data.kingdom_name;
        *self.inventory.lock().expect("inventory lock poisoned") = save_data.inventory;
        self.dirty.store(false, Ordering::Relaxed);
    }

    pub(crate) fn reset_progress(&self) -> InputSnapshot {
        self.keys.store(0, Ordering::Relaxed);
        self.clicks.store(0, Ordering::Relaxed);
        self.bonus_influence.store(0, Ordering::Relaxed);
        self.spent_influence.store(0, Ordering::Relaxed);
        self.crumpled_court_onboarding_manual_input_baseline
            .store(0, Ordering::Relaxed);
        self.crumpled_court_onboarding_manual_trigger_count
            .store(0, Ordering::Relaxed);
        self.pending_power_proc_inputs.store(0, Ordering::Relaxed);
        self.last_power_proc_roll_at_millis
            .store(0, Ordering::Relaxed);
        self.random_state.store(0, Ordering::Relaxed);
        self.power_event_sequence.store(0, Ordering::Relaxed);
        self.last_power_event_at_millis.store(0, Ordering::Relaxed);
        self.last_power_event_amount.store(0, Ordering::Relaxed);
        self.last_power_event_item_level.store(0, Ordering::Relaxed);
        self.last_input_at_millis.store(0, Ordering::Relaxed);
        self.last_global_key_at_millis.store(0, Ordering::Relaxed);
        self.seen_story_event_ids
            .lock()
            .expect("story events lock poisoned")
            .clear();
        self.seen_shop_item_ids
            .lock()
            .expect("shop items lock poisoned")
            .clear();
        *self
            .kingdom_name
            .lock()
            .expect("kingdom name lock poisoned") = None;
        self.pressed_keys
            .lock()
            .expect("pressed keys lock poisoned")
            .clear();
        *self.inventory.lock().expect("inventory lock poisoned") = Inventory::default();
        self.mark_dirty();

        self.snapshot()
    }

    pub(crate) fn dev_add_influence(&self, amount: u64) -> InputSnapshot {
        self.keys.fetch_add(amount, Ordering::Relaxed);
        let input_at_millis = current_time_millis();
        self.record_input_time_at(input_at_millis);
        self.queue_power_upgrade_inputs(input_at_millis, amount);
        self.mark_dirty();

        self.snapshot()
    }

    pub(crate) fn mark_story_event_seen(&self, event_id: String) {
        self.seen_story_event_ids
            .lock()
            .expect("story events lock poisoned")
            .insert(event_id);
        self.mark_dirty();
    }

    pub(crate) fn mark_shop_item_seen(&self, item_id: String) {
        self.seen_shop_item_ids
            .lock()
            .expect("shop items lock poisoned")
            .insert(item_id);
        self.mark_dirty();
    }

    pub(crate) fn story_event_ids(&self) -> Vec<String> {
        let mut event_ids = self
            .seen_story_event_ids
            .lock()
            .expect("story events lock poisoned")
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        event_ids.sort();
        event_ids
    }

    pub(crate) fn shop_item_ids(&self) -> Vec<String> {
        let mut item_ids = self
            .seen_shop_item_ids
            .lock()
            .expect("shop items lock poisoned")
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        item_ids.sort();
        item_ids
    }

    pub(crate) fn kingdom_name(&self) -> Option<String> {
        self.kingdom_name
            .lock()
            .expect("kingdom name lock poisoned")
            .clone()
    }

    pub(crate) fn has_kingdom_name(&self) -> bool {
        self.kingdom_name
            .lock()
            .expect("kingdom name lock poisoned")
            .is_some()
    }

    pub(crate) fn record_input_time_at(&self, input_at_millis: u64) {
        self.last_input_at_millis
            .store(input_at_millis, Ordering::Relaxed);
    }

    pub(crate) fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub(crate) fn take_dirty(&self) -> bool {
        self.dirty.swap(false, Ordering::Relaxed)
    }
}

pub(crate) fn next_random_state(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

pub(crate) fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
