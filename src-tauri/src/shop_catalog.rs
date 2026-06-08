use crate::level_catalog::xp_required_for_level;
use std::sync::OnceLock;

pub const ROYAL_CONTRACT_ID: &str = "royal_contract";
pub const CRUMPLED_COURT_ONBOARDING_MANUAL_ID: &str = "crumpled_court_onboarding_manual";
pub const CHANCE_SCALE: u64 = 1_000_000;

static SHOP_ITEMS: OnceLock<Vec<ShopItem>> = OnceLock::new();

pub fn shop_items() -> &'static [ShopItem] {
    SHOP_ITEMS.get_or_init(|| {
        let mut items = Vec::with_capacity(99);
        items.push(ShopItem::new(
            ROYAL_CONTRACT_ID.to_string(),
            cost_for_level(2),
            2,
            ShopItemCategory::OrgChart,
            None,
        ));

        for level in 3..=100 {
            items.push(ShopItem::new(
                item_id_for_level(level),
                cost_for_level(level),
                level,
                ShopItemCategory::PowerUpgrade,
                Some(PowerUpgradeEffect::new(
                    chance_per_million_for_level(level),
                    proc_value_for_level(level),
                )),
            ));
        }

        items
    })
}

pub struct ShopItem {
    pub id: String,
    pub cost: u64,
    pub required_level: u64,
    pub category: ShopItemCategory,
    pub power_upgrade_effect: Option<PowerUpgradeEffect>,
}

impl ShopItem {
    fn new(
        id: String,
        cost: u64,
        required_level: u64,
        category: ShopItemCategory,
        power_upgrade_effect: Option<PowerUpgradeEffect>,
    ) -> Self {
        Self {
            id,
            cost,
            required_level,
            category,
            power_upgrade_effect,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PowerUpgradeEffect {
    pub chance_per_million_inputs: u64,
    pub reward: u64,
}

impl PowerUpgradeEffect {
    pub const fn new(chance_per_million_inputs: u64, reward: u64) -> Self {
        Self {
            chance_per_million_inputs,
            reward,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ShopItemCategory {
    OrgChart,
    PowerUpgrade,
}

pub fn find_shop_item(item_id: &str) -> Option<&'static ShopItem> {
    shop_items().iter().find(|item| item.id == item_id)
}

fn item_id_for_level(level: u64) -> String {
    const ITEM_IDS: [&str; 98] = [
        CRUMPLED_COURT_ONBOARDING_MANUAL_ID,
        "official_wax_seal",
        "pamphlet_satchel",
        "bottomless_court_coffee_mug",
        "court_slogan_slate",
        "audience_tally_board",
        "royal_suggestion_box_of_destiny",
        "polished_herald_horn",
        "town_square_permit",
        "royal_crier_app_subscription",
        "court_rumor_ledger",
        "festival_banner_kit",
        "ledger_of_courtly_accounting",
        "royal_speech_timer",
        "enchanted_applause_meter",
        "crystal_audience_analytics_orb",
        "focus_group_biscuit_tin",
        "sealed_decree_folder",
        "enchanted_press_release_quill",
        "court_headline_stencil",
        "royal_courier_whistle",
        "royal_mimic_stamp",
        "portable_scandal_screen",
        "illuminated_audience_map",
        "royal_messaging_handbook_revised_edition",
        "portable_proclamation_press",
        "public_sentiment_gauge",
        "court_newsletter_press",
        "village_survey_kit",
        "royal_festival_sponsorship",
        "goblin_outreach_playbook",
        "ambassador_briefing_notes",
        "licensed_rumor_mill",
        "royal_quest_board",
        "town_crier_roster",
        "public_trust_seal",
        "arcane_audience_survey_scrolls",
        "court_slogan_forge",
        "campaign_archive_key",
        "court_recruitment_poster_set",
        "royal_event_planner",
        "enchanted_messenger_relay",
        "royal_courier_satchel",
        "embassy_bulletin_board",
        "applause_amplifier",
        "lute_of_royal_ballads",
        "royal_endorsement_desk",
        "public_ceremony_kit",
        "royal_sponsorship_contract",
        "court_patron_ledger",
        "trade_fair_banner",
        "runic_royal_printing_press",
        "reserve_of_runic_ink",
        "enchanted_typesetter",
        "dragon_egg_aethernet_cluster",
        "aether_signal_beacon",
        "viral_chant_codex",
        "tome_of_royal_memes",
        "crisis_response_bell",
        "reputation_ward",
        "royal_public_relations_handbook",
        "rumor_containment_kit",
        "ceremonial_apology_seal",
        "patent_pending_campaign_spellbook",
        "trend_divining_rod",
        "campaign_experiment_chamber",
        "arcane_audience_research_journal",
        "crystal_audience_model",
        "royal_focus_network",
        "royal_aetherwave_broadcast_tower",
        "aetherwave_resonance_tuner",
        "kingdom_broadcast_schedule",
        "royal_expedition_contract_ledger",
        "frontier_dispatch_case",
        "map_room_signal_desk",
        "dragon_endorsed_royal_campaign",
        "skyfire_campaign_banner",
        "crown_endorsement_archive",
        "royal_census_crystal",
        "population_pulse_map",
        "realm_sentiment_engine",
        "royal_prophecy_engine",
        "omen_press_kit",
        "future_messaging_bureau",
        "grand_royal_campaign_blueprint",
        "grand_campaign_war_room",
        "influence_command_table",
        "aethernet_kingdom_news_license",
        "front_page_royal_seal",
        "kingdom_wire_service",
        "aethernet_data_core",
        "court_algorithm_ledger",
        "predictive_audience_engine",
        "royal_influence_exchange_charter",
        "market_whisper_network",
        "influence_futures_desk",
        "crown_of_the_aethernet_algorithm_dragon",
        "throne_of_public_opinion",
    ];

    ITEM_IDS[level.saturating_sub(3) as usize].to_string()
}

fn cost_for_level(level: u64) -> u64 {
    let unlock_interval =
        xp_required_for_level(level).saturating_sub(xp_required_for_level(level - 1));
    let scaled_cost = if level % 4 == 0 {
        unlock_interval.saturating_mul(65) / 100
    } else if level % 7 == 0 {
        unlock_interval.saturating_mul(85) / 100
    } else {
        unlock_interval.saturating_mul(110) / 100
    };

    round_up_for_display(scaled_cost)
}

fn chance_per_million_for_level(level: u64) -> u64 {
    let chance = 7_500_f64 * 0.94_f64.powi(level.saturating_sub(3) as i32);
    (chance.floor() as u64).max(15)
}

fn proc_value_for_level(level: u64) -> u64 {
    let value = 25_f64 * 1.13_f64.powi(level.saturating_sub(3) as i32);
    round_up_for_display(value.ceil() as u64).max(25)
}

fn round_up_for_display(value: u64) -> u64 {
    let unit = if value >= 10_000_000 {
        100_000
    } else if value >= 1_000_000 {
        10_000
    } else if value >= 100_000 {
        1_000
    } else if value >= 10_000 {
        100
    } else if value >= 1_000 {
        50
    } else {
        10
    };

    value.div_ceil(unit) * unit
}
