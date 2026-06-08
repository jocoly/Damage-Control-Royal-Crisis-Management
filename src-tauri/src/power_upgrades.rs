#[cfg(test)]
use crate::shop_catalog::PowerUpgradeEffect;
use crate::{
    game_state::{current_time_millis, next_random_state, InputCounts},
    shop_catalog::{find_shop_item, CHANCE_SCALE, CRUMPLED_COURT_ONBOARDING_MANUAL_ID},
};
use std::sync::atomic::Ordering;

const POWER_PROC_ROLL_INTERVAL_MILLIS: u64 = 1_000;

impl InputCounts {
    pub(crate) fn queue_power_upgrade_inputs(&self, input_at_millis: u64, input_count: u64) {
        let has_power_upgrade = !self
            .inventory
            .lock()
            .expect("inventory lock poisoned")
            .power_upgrades
            .is_empty();

        if !has_power_upgrade {
            return;
        }

        self.pending_power_proc_inputs
            .fetch_add(input_count, Ordering::Relaxed);
        self.last_power_proc_roll_at_millis
            .compare_exchange(0, input_at_millis, Ordering::Relaxed, Ordering::Relaxed)
            .ok();
    }

    pub(crate) fn flush_due_power_upgrades(&self, now_millis: u64) {
        let last_roll_at_millis = self.last_power_proc_roll_at_millis.load(Ordering::Relaxed);

        if last_roll_at_millis == 0
            || now_millis.saturating_sub(last_roll_at_millis) < POWER_PROC_ROLL_INTERVAL_MILLIS
        {
            return;
        }

        let input_count = self.pending_power_proc_inputs.swap(0, Ordering::Relaxed);

        if input_count == 0 {
            self.last_power_proc_roll_at_millis
                .store(now_millis, Ordering::Relaxed);
            return;
        }

        self.last_power_proc_roll_at_millis
            .store(now_millis, Ordering::Relaxed);
        self.process_power_upgrade_rolls(now_millis, input_count);
    }

    fn process_power_upgrade_rolls(&self, input_at_millis: u64, input_count: u64) {
        let owned_power_upgrade_ids = self
            .inventory
            .lock()
            .expect("inventory lock poisoned")
            .power_upgrades
            .clone();

        let mut total_trigger_count = 0;
        let mut total_reward = 0;
        let mut highest_triggered_level = 0;

        for item_id in owned_power_upgrade_ids {
            let Some(item) = find_shop_item(&item_id) else {
                continue;
            };
            let Some(effect) = item.power_upgrade_effect else {
                continue;
            };

            let trigger_count =
                self.roll_power_upgrade_triggers(input_count, effect.chance_per_million_inputs);

            if trigger_count == 0 {
                continue;
            }

            if item.id.as_str() == CRUMPLED_COURT_ONBOARDING_MANUAL_ID {
                self.crumpled_court_onboarding_manual_trigger_count
                    .fetch_add(trigger_count, Ordering::Relaxed);
            }

            total_trigger_count += trigger_count;
            total_reward += trigger_count * effect.reward;
            highest_triggered_level = highest_triggered_level.max(item.required_level);
        }

        if total_trigger_count == 0 {
            return;
        }

        self.bonus_influence
            .fetch_add(total_reward, Ordering::Relaxed);
        self.power_event_sequence
            .fetch_add(total_trigger_count, Ordering::Relaxed);
        self.last_power_event_at_millis
            .store(input_at_millis, Ordering::Relaxed);
        self.last_power_event_amount
            .store(total_reward, Ordering::Relaxed);
        self.last_power_event_item_level
            .store(highest_triggered_level, Ordering::Relaxed);
        self.mark_dirty();
    }

    #[cfg(test)]
    pub(crate) fn process_power_upgrade_rolls_with_effect(
        &self,
        input_at_millis: u64,
        input_count: u64,
        effect: PowerUpgradeEffect,
    ) {
        let trigger_count =
            self.roll_power_upgrade_triggers(input_count, effect.chance_per_million_inputs);

        if trigger_count == 0 {
            return;
        }

        let reward = trigger_count * effect.reward;

        self.bonus_influence.fetch_add(reward, Ordering::Relaxed);
        self.power_event_sequence
            .fetch_add(trigger_count, Ordering::Relaxed);
        self.last_power_event_at_millis
            .store(input_at_millis, Ordering::Relaxed);
        self.last_power_event_amount
            .store(reward, Ordering::Relaxed);
        self.last_power_event_item_level.store(0, Ordering::Relaxed);
        self.mark_dirty();
    }

    fn roll_power_upgrade_triggers(&self, input_count: u64, chance_per_million_inputs: u64) -> u64 {
        let mut trigger_count = 0;

        for _ in 0..input_count {
            if self.next_random_roll() < chance_per_million_inputs {
                trigger_count += 1;
            }
        }

        trigger_count
    }

    fn next_random_roll(&self) -> u64 {
        loop {
            let current_state = self.random_state.load(Ordering::Relaxed);
            let seeded_state = if current_state == 0 {
                current_time_millis() | 1
            } else {
                current_state
            };
            let next_state = next_random_state(seeded_state);

            if self
                .random_state
                .compare_exchange(
                    current_state,
                    next_state,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return next_state % CHANCE_SCALE;
            }
        }
    }
}
