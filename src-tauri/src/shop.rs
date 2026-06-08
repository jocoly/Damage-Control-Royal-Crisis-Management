use crate::{
    game_state::{InputCounts, InputSnapshot, Inventory, PurchaseResult, PurchaseStatus},
    level_catalog::{level_for_xp, xp_required_for_level},
    shop_catalog::{find_shop_item, CRUMPLED_COURT_ONBOARDING_MANUAL_ID, ROYAL_CONTRACT_ID},
};
use std::sync::atomic::Ordering;

impl InputCounts {
    pub(crate) fn purchase_shop_item(&self, item_id: &str) -> PurchaseResult {
        self.flush_due_power_upgrades(crate::game_state::current_time_millis());

        let Some(item) = find_shop_item(item_id) else {
            return self.purchase_result(PurchaseStatus::UnknownItem);
        };

        if level_for_xp(self.lifetime_xp()) < item.required_level {
            return self.purchase_result(PurchaseStatus::Locked);
        }

        let mut inventory = self.inventory.lock().expect("inventory lock poisoned");

        if item.id.as_str() != ROYAL_CONTRACT_ID && !inventory.has_item(ROYAL_CONTRACT_ID) {
            return self.purchase_result_with_inventory(PurchaseStatus::Locked, inventory);
        }

        if inventory.has_item(item_id) {
            return self.purchase_result_with_inventory(PurchaseStatus::AlreadyOwned, inventory);
        }

        if self.available_influence() < item.cost {
            return self
                .purchase_result_with_inventory(PurchaseStatus::NotEnoughInfluence, inventory);
        }

        inventory.add_item(item);
        if item.id.as_str() == CRUMPLED_COURT_ONBOARDING_MANUAL_ID {
            self.crumpled_court_onboarding_manual_input_baseline
                .store(self.raw_input_total(), Ordering::Relaxed);
            self.crumpled_court_onboarding_manual_trigger_count
                .store(0, Ordering::Relaxed);
            self.pending_power_proc_inputs.store(0, Ordering::Relaxed);
            self.last_power_proc_roll_at_millis
                .store(0, Ordering::Relaxed);
        }
        self.spent_influence.fetch_add(item.cost, Ordering::Relaxed);
        self.mark_dirty();

        self.purchase_result_with_inventory(PurchaseStatus::Purchased, inventory)
    }

    fn purchase_result(&self, status: PurchaseStatus) -> PurchaseResult {
        PurchaseResult {
            status,
            snapshot: self.snapshot(),
        }
    }

    fn available_influence(&self) -> u64 {
        let keys = self.keys.load(Ordering::Relaxed);
        let clicks = self.clicks.load(Ordering::Relaxed);
        let bonus_influence = self.bonus_influence.load(Ordering::Relaxed);

        (keys + clicks + bonus_influence)
            .saturating_sub(self.spent_influence.load(Ordering::Relaxed))
    }

    fn lifetime_xp(&self) -> u64 {
        self.keys.load(Ordering::Relaxed)
            + self.clicks.load(Ordering::Relaxed)
            + self.bonus_influence.load(Ordering::Relaxed)
    }

    fn raw_input_total(&self) -> u64 {
        self.keys.load(Ordering::Relaxed) + self.clicks.load(Ordering::Relaxed)
    }

    fn purchase_result_with_inventory(
        &self,
        status: PurchaseStatus,
        inventory: std::sync::MutexGuard<'_, Inventory>,
    ) -> PurchaseResult {
        let keys = self.keys.load(Ordering::Relaxed);
        let clicks = self.clicks.load(Ordering::Relaxed);
        let bonus_influence = self.bonus_influence.load(Ordering::Relaxed);
        let spent_influence = self.spent_influence.load(Ordering::Relaxed);
        let xp = keys + clicks + bonus_influence;
        let level = level_for_xp(xp);

        PurchaseResult {
            status,
            snapshot: InputSnapshot {
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
                last_power_event_item_level: self
                    .last_power_event_item_level
                    .load(Ordering::Relaxed),
                inventory_item_ids: inventory.item_ids(),
                seen_story_event_ids: self.story_event_ids(),
                seen_shop_item_ids: self.shop_item_ids(),
                kingdom_name: self.kingdom_name(),
                last_input_at_millis: self.last_input_at_millis.load(Ordering::Relaxed),
            },
        }
    }
}
