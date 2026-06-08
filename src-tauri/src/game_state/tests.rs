use super::*;
use crate::{
    persistence::{load_or_create_save, load_save_file, reset_counts_to_path, save_counts_to_path},
    shop_catalog::{
        find_shop_item, shop_items, PowerUpgradeEffect, CHANCE_SCALE, ROYAL_CONTRACT_ID,
    },
};
use rdev::{Button, EventType, Key};
use std::fs;

fn named_counts() -> InputCounts {
    let counts = InputCounts::default();
    counts
        .set_kingdom_name("New Avalon")
        .expect("test kingdom name should be valid");
    counts
}

#[test]
fn input_counts_track_only_key_presses_and_mouse_presses() {
    let counts = named_counts();

    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::ButtonPress(Button::Left));
    counts.record_event(EventType::MouseMove { x: 10.0, y: 10.0 });
    counts.record_event(EventType::KeyRelease(Key::KeyA));

    let snapshot = counts.snapshot();

    assert_eq!(snapshot.keys, 1);
    assert_eq!(snapshot.clicks, 1);
    assert_eq!(snapshot.influence, 2);
    assert_eq!(snapshot.xp, 2);
    assert_eq!(snapshot.level, 1);
    assert_eq!(snapshot.xp_for_current_level, 0);
    assert_eq!(snapshot.xp_for_next_level, 500);
    assert!(snapshot.last_input_at_millis > 0);
}

#[test]
fn focused_keypresses_are_counted_when_global_hook_misses_them() {
    let counts = named_counts();

    counts.record_focused_keypress(1_000);

    let snapshot = counts.snapshot();

    assert_eq!(snapshot.keys, 1);
    assert_eq!(snapshot.clicks, 0);
    assert_eq!(snapshot.influence, 1);
    assert_eq!(snapshot.xp, 1);
    assert_eq!(snapshot.level, 1);
    assert_eq!(snapshot.xp_for_current_level, 0);
    assert_eq!(snapshot.xp_for_next_level, 500);
    assert_eq!(snapshot.last_input_at_millis, 1_000);
}

#[test]
fn focused_keypresses_are_not_double_counted_when_global_hook_reports_them() {
    let counts = named_counts();

    counts.record_event(EventType::KeyPress(Key::KeyA));
    let global_key_at_millis = counts.last_global_key_at_millis.load(Ordering::Relaxed);
    counts.record_focused_keypress(global_key_at_millis);

    let snapshot = counts.snapshot();

    assert_eq!(snapshot.keys, 1);
    assert_eq!(snapshot.influence, 1);
}

#[test]
fn held_keys_are_counted_once_until_each_key_is_released() {
    let counts = named_counts();

    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::KeyPress(Key::KeyB));
    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::KeyPress(Key::KeyB));

    assert_eq!(counts.snapshot().keys, 2);

    counts.record_event(EventType::KeyRelease(Key::KeyA));
    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::KeyPress(Key::KeyB));

    let snapshot = counts.snapshot();

    assert_eq!(snapshot.keys, 3);
    assert_eq!(snapshot.influence, 3);
}

#[test]
fn save_file_round_trips_key_and_click_totals() {
    let path = std::env::temp_dir().join(format!(
        "damage-control-save-test-{}.json",
        current_time_millis()
    ));
    let counts = named_counts();

    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::ButtonPress(Button::Left));
    counts.mark_story_event_seen("welcome_to_aethernet_kingdom".to_string());
    counts.mark_shop_item_seen(ROYAL_CONTRACT_ID.to_string());
    save_counts_to_path(&counts, &path).expect("save should write");

    let loaded = load_save_file(&path).expect("save should load");

    assert_eq!(
        loaded,
        SaveData {
            version: SAVE_VERSION,
            keys: 1,
            clicks: 1,
            bonus_influence: 0,
            spent_influence: 0,
            crumpled_court_onboarding_manual_input_baseline: 0,
            crumpled_court_onboarding_manual_trigger_count: 0,
            pending_power_proc_inputs: 0,
            last_power_proc_roll_at_millis: 0,
            seen_story_event_ids: vec!["welcome_to_aethernet_kingdom".to_string()],
            seen_shop_item_ids: vec![ROYAL_CONTRACT_ID.to_string()],
            kingdom_name: Some("New Avalon".to_string()),
            inventory: Inventory::default(),
        }
    );

    let _ = fs::remove_file(path);
}

#[test]
fn incompatible_save_versions_are_overwritten_with_a_fresh_save() {
    let path = std::env::temp_dir().join(format!(
        "damage-control-version-reset-test-{}.json",
        current_time_millis()
    ));
    let old_save = SaveData {
        version: SAVE_VERSION - 1,
        keys: 48_000,
        clicks: 12_000,
        bonus_influence: 5_000,
        spent_influence: 1_100,
        crumpled_court_onboarding_manual_input_baseline: 0,
        crumpled_court_onboarding_manual_trigger_count: 0,
        pending_power_proc_inputs: 0,
        last_power_proc_roll_at_millis: 0,
        seen_story_event_ids: vec!["you_are_hired".to_string()],
        seen_shop_item_ids: vec![ROYAL_CONTRACT_ID.to_string()],
        kingdom_name: Some("Old Save".to_string()),
        inventory: Inventory {
            org_chart: vec![ROYAL_CONTRACT_ID.to_string()],
            power_upgrades: Vec::new(),
        },
    };
    fs::write(
        &path,
        serde_json::to_string_pretty(&old_save).expect("old save should serialize"),
    )
    .expect("old save should write");

    let counts = InputCounts::default();
    load_or_create_save(&counts, &path).expect("old save should be replaced");
    let loaded = load_save_file(&path).expect("replacement save should load");

    assert_eq!(loaded.version, SAVE_VERSION);
    assert_eq!(loaded.keys, 0);
    assert_eq!(loaded.clicks, 0);
    assert_eq!(loaded.bonus_influence, 0);
    assert_eq!(loaded.spent_influence, 0);
    assert_eq!(loaded.kingdom_name, None);
    assert_eq!(loaded.inventory, Inventory::default());
    assert!(loaded.seen_story_event_ids.is_empty());
    assert!(loaded.seen_shop_item_ids.is_empty());

    let _ = fs::remove_file(path);
}

#[test]
fn purchasing_royal_contract_spends_influence_and_adds_org_chart_inventory() {
    let counts = named_counts();

    for index in 0..2_500 {
        counts.record_focused_keypress(1_000 + index);
    }

    let result = counts.purchase_shop_item(ROYAL_CONTRACT_ID);

    assert!(matches!(result.status, PurchaseStatus::Purchased));
    assert_eq!(result.snapshot.influence, 1_950);
    assert_eq!(result.snapshot.xp, 2_500);
    assert_eq!(result.snapshot.level, 3);
    assert_eq!(result.snapshot.keys, 2_500);
    assert_eq!(result.snapshot.spent_influence, 550);
    assert_eq!(
        result.snapshot.inventory_item_ids,
        vec![ROYAL_CONTRACT_ID.to_string()]
    );

    let save_data = counts.snapshot_save();

    assert_eq!(
        save_data.spent_influence,
        find_shop_item(ROYAL_CONTRACT_ID)
            .expect("royal contract exists")
            .cost
    );
    assert_eq!(save_data.inventory.org_chart, vec![ROYAL_CONTRACT_ID]);
}

#[test]
fn power_upgrade_purchase_is_level_gated_and_stored_separately() {
    let counts = named_counts();

    let locked_result = counts.purchase_shop_item("crumpled_court_onboarding_manual");

    assert!(matches!(locked_result.status, PurchaseStatus::Locked));
    assert!(locked_result.snapshot.inventory_item_ids.is_empty());

    counts.dev_add_influence(3_000);
    let contract_result = counts.purchase_shop_item(ROYAL_CONTRACT_ID);
    let purchased_result = counts.purchase_shop_item("crumpled_court_onboarding_manual");
    let save_data = counts.snapshot_save();

    assert!(matches!(contract_result.status, PurchaseStatus::Purchased));
    assert!(matches!(purchased_result.status, PurchaseStatus::Purchased));
    assert_eq!(purchased_result.snapshot.influence, 250);
    assert_eq!(purchased_result.snapshot.spent_influence, 2_750);
    assert_eq!(
        purchased_result.snapshot.inventory_item_ids,
        vec![
            ROYAL_CONTRACT_ID.to_string(),
            "crumpled_court_onboarding_manual".to_string()
        ]
    );
    assert_eq!(save_data.inventory.org_chart, vec![ROYAL_CONTRACT_ID]);
    assert_eq!(
        save_data.inventory.power_upgrades,
        vec!["crumpled_court_onboarding_manual"]
    );
}

#[test]
fn purchase_fails_without_enough_influence() {
    let counts = named_counts();

    let result = counts.purchase_shop_item(ROYAL_CONTRACT_ID);

    assert!(matches!(result.status, PurchaseStatus::Locked));
    assert_eq!(result.snapshot.influence, 0);
    assert_eq!(result.snapshot.xp, 0);
    assert_eq!(result.snapshot.level, 1);
    assert_eq!(result.snapshot.xp_for_current_level, 0);
    assert_eq!(result.snapshot.xp_for_next_level, 500);
    assert!(result.snapshot.inventory_item_ids.is_empty());
}

#[test]
fn dev_add_influence_changes_earned_state_without_spending() {
    let counts = named_counts();

    let snapshot = counts.dev_add_influence(500);
    let save_data = counts.snapshot_save();

    assert_eq!(snapshot.influence, 500);
    assert_eq!(snapshot.xp, 500);
    assert_eq!(snapshot.level, 2);
    assert_eq!(snapshot.keys, 500);
    assert_eq!(snapshot.clicks, 0);
    assert_eq!(snapshot.bonus_influence, 0);
    assert_eq!(snapshot.spent_influence, 0);
    assert_eq!(save_data.keys, 500);
    assert_eq!(save_data.bonus_influence, 0);
    assert_eq!(save_data.spent_influence, 0);
}

#[test]
fn crumpled_court_onboarding_manual_tracks_random_proc_state_after_purchase() {
    let counts = named_counts();

    counts.dev_add_influence(3_000);
    let contract_result = counts.purchase_shop_item(ROYAL_CONTRACT_ID);
    let purchased_result = counts.purchase_shop_item(CRUMPLED_COURT_ONBOARDING_MANUAL_ID);

    assert!(matches!(contract_result.status, PurchaseStatus::Purchased));
    assert!(matches!(purchased_result.status, PurchaseStatus::Purchased));
    assert_eq!(purchased_result.snapshot.influence, 250);
    assert_eq!(purchased_result.snapshot.bonus_influence, 0);
    assert_eq!(purchased_result.snapshot.spent_influence, 2_750);
    assert_eq!(purchased_result.snapshot.power_event_sequence, 0);

    let save_data = counts.snapshot_save();

    assert_eq!(save_data.bonus_influence, 0);
    assert_eq!(save_data.crumpled_court_onboarding_manual_trigger_count, 0);
    assert_eq!(
        save_data.crumpled_court_onboarding_manual_input_baseline,
        3_000
    );
}

#[test]
fn random_power_upgrade_rolls_can_trigger_bonus_rewards() {
    let counts = named_counts();

    counts.load_save(SaveData {
        version: SAVE_VERSION,
        keys: 1_500,
        clicks: 0,
        bonus_influence: 0,
        spent_influence: 100,
        crumpled_court_onboarding_manual_input_baseline: 0,
        crumpled_court_onboarding_manual_trigger_count: 0,
        pending_power_proc_inputs: 0,
        last_power_proc_roll_at_millis: 0,
        seen_story_event_ids: Vec::new(),
        seen_shop_item_ids: Vec::new(),
        kingdom_name: None,
        inventory: Inventory {
            org_chart: Vec::new(),
            power_upgrades: vec![CRUMPLED_COURT_ONBOARDING_MANUAL_ID.to_string()],
        },
    });

    counts.process_power_upgrade_rolls_with_effect(
        1_000,
        3,
        PowerUpgradeEffect::new(1_000_000, 25),
    );
    let triggered = counts.snapshot();

    assert_eq!(triggered.bonus_influence, 75);
    assert_eq!(triggered.power_event_sequence, 3);
    assert_eq!(triggered.last_power_event_amount, 75);
    assert_eq!(triggered.last_power_event_at_millis, 1_000);
}

#[test]
fn all_power_upgrade_catalog_items_have_backend_proc_effects() {
    for item in shop_items() {
        match item.category {
            ShopItemCategory::OrgChart => assert!(item.power_upgrade_effect.is_none()),
            ShopItemCategory::PowerUpgrade => {
                let effect = item
                    .power_upgrade_effect
                    .expect("power upgrade should have an effect");

                assert!(effect.chance_per_million_inputs > 0);
                assert!(effect.chance_per_million_inputs <= CHANCE_SCALE);
                assert!(effect.reward > 0);
            }
        }
    }
}

#[test]
fn shop_has_one_item_per_level_with_mixed_affordability() {
    let items = shop_items();
    let mut affordable_unlocks = 0;
    let mut aspirational_unlocks = 0;

    assert_eq!(items.len(), 99);

    for (index, item) in items.iter().enumerate() {
        let expected_level = index as u64 + 2;
        let unlock_interval = crate::level_catalog::xp_required_for_level(expected_level)
            - crate::level_catalog::xp_required_for_level(expected_level - 1);

        assert_eq!(item.required_level, expected_level);

        if item.cost <= unlock_interval {
            affordable_unlocks += 1;
        } else {
            aspirational_unlocks += 1;
        }
    }

    assert!(affordable_unlocks >= 20);
    assert!(aspirational_unlocks >= 20);
}

#[test]
fn proc_rolls_are_batched_until_cooldown_window_elapses() {
    let counts = InputCounts::default();

    counts.load_save(SaveData {
        version: SAVE_VERSION,
        keys: 0,
        clicks: 0,
        bonus_influence: 0,
        spent_influence: 0,
        crumpled_court_onboarding_manual_input_baseline: 0,
        crumpled_court_onboarding_manual_trigger_count: 0,
        pending_power_proc_inputs: 0,
        last_power_proc_roll_at_millis: 0,
        seen_story_event_ids: Vec::new(),
        seen_shop_item_ids: Vec::new(),
        kingdom_name: None,
        inventory: Inventory {
            org_chart: Vec::new(),
            power_upgrades: vec![CRUMPLED_COURT_ONBOARDING_MANUAL_ID.to_string()],
        },
    });

    counts.queue_power_upgrade_inputs(1_000, 3);
    counts.flush_due_power_upgrades(1_999);

    assert_eq!(counts.pending_power_proc_inputs.load(Ordering::Relaxed), 3);
    assert_eq!(counts.power_event_sequence.load(Ordering::Relaxed), 0);

    assert_eq!(counts.bonus_influence.load(Ordering::Relaxed), 0);
}

#[test]
fn reset_progress_clears_counts_spending_and_inventory() {
    let counts = named_counts();

    for index in 0..2_500 {
        counts.record_focused_keypress(1_000 + index);
    }

    counts.purchase_shop_item(ROYAL_CONTRACT_ID);
    counts.record_event(EventType::ButtonPress(Button::Left));
    counts.mark_story_event_seen("welcome_to_aethernet_kingdom".to_string());
    counts.mark_shop_item_seen(ROYAL_CONTRACT_ID.to_string());
    let snapshot = counts.reset_progress();
    let save_data = counts.snapshot_save();

    assert_eq!(snapshot.influence, 0);
    assert_eq!(snapshot.xp, 0);
    assert_eq!(snapshot.level, 1);
    assert_eq!(snapshot.xp_for_current_level, 0);
    assert_eq!(snapshot.xp_for_next_level, 500);
    assert_eq!(snapshot.keys, 0);
    assert_eq!(snapshot.clicks, 0);
    assert_eq!(snapshot.bonus_influence, 0);
    assert_eq!(snapshot.spent_influence, 0);
    assert_eq!(snapshot.power_event_sequence, 0);
    assert!(snapshot.inventory_item_ids.is_empty());
    assert!(snapshot.seen_story_event_ids.is_empty());
    assert!(snapshot.seen_shop_item_ids.is_empty());
    assert_eq!(snapshot.kingdom_name, None);
    assert_eq!(save_data.spent_influence, 0);
    assert_eq!(save_data.bonus_influence, 0);
    assert_eq!(save_data.crumpled_court_onboarding_manual_input_baseline, 0);
    assert_eq!(save_data.crumpled_court_onboarding_manual_trigger_count, 0);
    assert!(save_data.seen_story_event_ids.is_empty());
    assert!(save_data.seen_shop_item_ids.is_empty());
    assert_eq!(save_data.kingdom_name, None);
    assert_eq!(save_data.inventory, Inventory::default());
}

#[test]
fn reset_progress_replaces_the_existing_save_file_with_a_fresh_save() {
    let path = std::env::temp_dir().join(format!(
        "damage-control-reset-save-test-{}.json",
        current_time_millis()
    ));
    let counts = named_counts();

    for index in 0..2_500 {
        counts.record_focused_keypress(1_000 + index);
    }

    counts.purchase_shop_item(ROYAL_CONTRACT_ID);
    counts.mark_story_event_seen("you_are_hired".to_string());
    counts.mark_shop_item_seen(ROYAL_CONTRACT_ID.to_string());
    save_counts_to_path(&counts, &path).expect("populated save should write");

    let snapshot =
        reset_counts_to_path(&counts, &path).expect("reset should replace the save file");
    let loaded = load_save_file(&path).expect("reset save should load");

    assert_eq!(snapshot.influence, 0);
    assert_eq!(snapshot.level, 1);
    assert_eq!(snapshot.kingdom_name, None);
    assert!(snapshot.inventory_item_ids.is_empty());
    assert!(snapshot.seen_story_event_ids.is_empty());
    assert!(snapshot.seen_shop_item_ids.is_empty());
    assert_eq!(
        loaded,
        SaveData {
            version: SAVE_VERSION,
            keys: 0,
            clicks: 0,
            bonus_influence: 0,
            spent_influence: 0,
            crumpled_court_onboarding_manual_input_baseline: 0,
            crumpled_court_onboarding_manual_trigger_count: 0,
            pending_power_proc_inputs: 0,
            last_power_proc_roll_at_millis: 0,
            seen_story_event_ids: Vec::new(),
            seen_shop_item_ids: Vec::new(),
            kingdom_name: None,
            inventory: Inventory::default(),
        }
    );

    let _ = fs::remove_file(path);
}

#[test]
fn inputs_are_ignored_until_the_kingdom_is_named() {
    let counts = InputCounts::default();

    counts.record_event(EventType::KeyPress(Key::KeyA));
    counts.record_event(EventType::ButtonPress(Button::Left));
    counts.record_focused_keypress(1_000);

    let snapshot = counts.snapshot();

    assert_eq!(snapshot.keys, 0);
    assert_eq!(snapshot.clicks, 0);
}
