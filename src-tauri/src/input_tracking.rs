use crate::game_state::{current_time_millis, InputCounts};
use rdev::EventType;
use std::sync::atomic::Ordering;

impl InputCounts {
    pub(crate) fn record_event(&self, event_type: EventType) {
        if !self.can_record_input() {
            if let EventType::KeyRelease(key) = event_type {
                self.pressed_keys
                    .lock()
                    .expect("pressed keys lock poisoned")
                    .remove(&key);
            }
            return;
        }

        match event_type {
            EventType::KeyPress(key) => {
                let mut pressed_keys = self
                    .pressed_keys
                    .lock()
                    .expect("pressed keys lock poisoned");

                if !pressed_keys.insert(key) {
                    return;
                }

                drop(pressed_keys);
                let input_at_millis = current_time_millis();
                self.record_keypress(input_at_millis);
                self.last_global_key_at_millis
                    .store(input_at_millis, Ordering::Relaxed);
            }
            EventType::KeyRelease(key) => {
                self.pressed_keys
                    .lock()
                    .expect("pressed keys lock poisoned")
                    .remove(&key);
            }
            EventType::ButtonPress(_) => {
                let input_at_millis = current_time_millis();
                self.clicks.fetch_add(1, Ordering::Relaxed);
                self.record_input_time_at(input_at_millis);
                self.queue_power_upgrade_inputs(input_at_millis, 1);
                self.mark_dirty();
            }
            _ => {}
        }
    }

    pub(crate) fn record_focused_keypress(&self, event_at_millis: u64) {
        if !self.can_record_input() {
            return;
        }

        let last_global_key_at_millis = self.last_global_key_at_millis.load(Ordering::Relaxed);

        if event_at_millis.abs_diff(last_global_key_at_millis) <= 100 {
            return;
        }

        self.record_keypress(event_at_millis);
    }

    fn record_keypress(&self, input_at_millis: u64) {
        self.keys.fetch_add(1, Ordering::Relaxed);
        self.record_input_time_at(input_at_millis);
        self.queue_power_upgrade_inputs(input_at_millis, 1);
        self.mark_dirty();
    }
}
