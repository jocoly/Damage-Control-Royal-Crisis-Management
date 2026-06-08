import { levelDefinition } from "$lib/levelCatalog";

export type ProcVisualFamily =
  | "spark"
  | "explosion"
  | "lightning"
  | "fire"
  | "water"
  | "bubble"
  | "confetti"
  | "stars"
  | "shockwave"
  | "smoke"
  | "snow"
  | "leaves"
  | "glitch"
  | "prism"
  | "void"
  | "runes";

export type ShopItem = {
  id: string;
  name: string;
  cost: number;
  requiredLevel: number;
  category: "org_chart" | "power_upgrade";
  effect: string;
  inventoryDescription?: string;
  procChance?: string;
  procEffect?: string;
  procValue?: number;
  expectedValuePerThousandInputs?: number;
  visualFamily?: ProcVisualFamily;
  visualHue?: number;
  visualIntensity?: number;
};

export type PowerUpgradeDetails = {
  procChance: string;
  effect: string;
  value: number;
};

const powerUpgradeContent: Array<[string, string, string]> = [
  ["crumpled_court_onboarding_manual", "Royal Reminder Kit", "Post a royal reminder to Ye Olde X"],
  ["official_wax_seal", "Official Wax Seal", "Approve a minor court notice"],
  ["pamphlet_satchel", "Pamphlet Satchel", "Distribute fresh royal pamphlets"],
  ["bottomless_court_coffee_mug", "Bottomless Court Coffee Mug", "Fuel a late-night court campaign"],
  ["court_slogan_slate", "Court Slogan Slate", "Reveal a usable campaign slogan"],
  ["audience_tally_board", "Audience Tally Board", "Find support in the latest tally"],
  ["royal_suggestion_box_of_destiny", "Royal Suggestion Box of Destiny", "Discover a court-approved idea"],
  ["polished_herald_horn", "Polished Herald Horn", "Broadcast a crisp royal announcement"],
  ["town_square_permit", "Town Square Permit", "Secure a crowded public address"],
  ["royal_crier_app_subscription", "Royal Crier App Subscription", "Trigger a royal bulletin"],
  ["court_rumor_ledger", "Court Rumor Ledger", "Turn a rumor into favorable coverage"],
  ["festival_banner_kit", "Festival Banner Kit", "Sponsor a popular kingdom festival"],
  ["ledger_of_courtly_accounting", "Ledger of Courtly Accounting", "Reclassify a campaign win"],
  ["royal_speech_timer", "Royal Speech Timer", "Keep a proclamation mercifully brief"],
  ["enchanted_applause_meter", "Enchanted Applause Meter", "Capture an unexpected burst of approval"],
  ["crystal_audience_analytics_orb", "Crystal Audience Analytics Orb", "Discover an audience trend"],
  ["focus_group_biscuit_tin", "Focus Group Biscuit Tin", "Improve a difficult focus group"],
  ["sealed_decree_folder", "Sealed Decree Folder", "Release a well-timed royal decree"],
  ["enchanted_press_release_quill", "Enchanted Press Release Quill", "Draft a viral royal post"],
  ["court_headline_stencil", "Court Headline Stencil", "Print a persuasive front-page headline"],
  ["royal_courier_whistle", "Royal Courier Whistle", "Dispatch a persuasive court messenger"],
  ["royal_mimic_stamp", "Royal Mimic Stamp", "Stamp a royal duplicate"],
  ["portable_scandal_screen", "Portable Scandal Screen", "Hide an inconvenient court detail"],
  ["illuminated_audience_map", "Illuminated Audience Map", "Locate an overlooked audience"],
  ["royal_messaging_handbook_revised_edition", "Royal Messaging Handbook, Revised Edition", "Approve revised messaging"],
  ["portable_proclamation_press", "Portable Proclamation Press", "Publish an emergency proclamation"],
  ["public_sentiment_gauge", "Public Sentiment Gauge", "Detect a favorable shift in opinion"],
  ["court_newsletter_press", "Court Newsletter Press", "Release a court newsletter"],
  ["village_survey_kit", "Village Survey Kit", "Collect encouraging village feedback"],
  ["royal_festival_sponsorship", "Royal Festival Sponsorship", "Attach the crown to a successful festival"],
  ["goblin_outreach_playbook", "Goblin Outreach Playbook", "Launch a royal outreach campaign"],
  ["ambassador_briefing_notes", "Ambassador Briefing Notes", "Land a diplomatic talking point"],
  ["licensed_rumor_mill", "Licensed Rumor Mill", "Spread an officially harmless rumor"],
  ["royal_quest_board", "Royal Quest Board", "Complete a royal contract"],
  ["town_crier_roster", "Town Crier Roster", "Coordinate a chorus of royal criers"],
  ["public_trust_seal", "Public Trust Seal", "Certify a trustworthy royal message"],
  ["arcane_audience_survey_scrolls", "Arcane Audience Survey Scrolls", "Uncover audience sentiment"],
  ["court_slogan_forge", "Court Slogan Forge", "Hammer out a memorable royal phrase"],
  ["campaign_archive_key", "Campaign Archive Key", "Recover a proven campaign tactic"],
  ["court_recruitment_poster_set", "Court Recruitment Poster Set", "Recruit royal advocates"],
  ["royal_event_planner", "Royal Event Planner", "Stage a successful court appearance"],
  ["enchanted_messenger_relay", "Enchanted Messenger Relay", "Relay a message across the realm"],
  ["royal_courier_satchel", "Royal Courier Satchel", "Receive a royal decree"],
  ["embassy_bulletin_board", "Embassy Bulletin Board", "Win favorable foreign attention"],
  ["applause_amplifier", "Applause Amplifier", "Magnify a modest public cheer"],
  ["lute_of_royal_ballads", "Lute of Royal Ballads", "Start a royal hype train"],
  ["royal_endorsement_desk", "Royal Endorsement Desk", "Secure a noble endorsement"],
  ["public_ceremony_kit", "Public Ceremony Kit", "Stage a flawless royal ceremony"],
  ["royal_sponsorship_contract", "Royal Sponsorship Contract", "Secure a royal sponsorship"],
  ["court_patron_ledger", "Court Patron Ledger", "Activate a generous court patron"],
  ["trade_fair_banner", "Trade Fair Banner", "Dominate a busy kingdom trade fair"],
  ["runic_royal_printing_press", "Runic Royal Printing Press", "Print a runic campaign"],
  ["reserve_of_runic_ink", "Reserve of Runic Ink", "Illuminate a powerful campaign message"],
  ["enchanted_typesetter", "Enchanted Typesetter", "Produce a perfect proclamation run"],
  ["dragon_egg_aethernet_cluster", "Dragon Egg Signal Cluster", "Hatch a signal surge"],
  ["aether_signal_beacon", "Aether Signal Beacon", "Reach a distant royal audience"],
  ["viral_chant_codex", "Viral Chant Codex", "Launch an irresistible court chant"],
  ["tome_of_royal_memes", "Tome of Royal Memes", "Create a court meme"],
  ["crisis_response_bell", "Crisis Response Bell", "Rally the office around a public crisis"],
  ["reputation_ward", "Reputation Ward", "Deflect a damaging public rumor"],
  ["royal_public_relations_handbook", "Royal Public Relations Handbook", "Avert a royal PR crisis"],
  ["rumor_containment_kit", "Rumor Containment Kit", "Contain a fast-moving court rumor"],
  ["ceremonial_apology_seal", "Ceremonial Apology Seal", "Issue a convincing royal apology"],
  ["patent_pending_campaign_spellbook", "Patent Pending Campaign Spellbook", "Invent a royal trend"],
  ["trend_divining_rod", "Trend Divining Rod", "Locate the next public obsession"],
  ["campaign_experiment_chamber", "Campaign Experiment Chamber", "Prove an unusual campaign theory"],
  ["arcane_audience_research_journal", "Arcane Audience Research Journal", "Discover an audience breakthrough"],
  ["crystal_audience_model", "Crystal Audience Model", "Predict a major shift in public taste"],
  ["royal_focus_network", "Royal Focus Network", "Align every audience segment"],
  ["royal_aetherwave_broadcast_tower", "Royal Aetherwave Broadcast Tower", "Hit an Aetherwave broadcast"],
  ["aetherwave_resonance_tuner", "Aetherwave Resonance Tuner", "Find a perfect broadcast frequency"],
  ["kingdom_broadcast_schedule", "Kingdom Broadcast Schedule", "Capture the realm's busiest hour"],
  ["royal_expedition_contract_ledger", "Royal Expedition Contract Ledger", "Receive a royal expedition report"],
  ["frontier_dispatch_case", "Frontier Dispatch Case", "Publish news from the kingdom frontier"],
  ["map_room_signal_desk", "Map Room Signal Desk", "Coordinate reports across the realm"],
  ["dragon_endorsed_royal_campaign", "Dragon-Endorsed Royal Campaign", "Receive a dragon endorsement"],
  ["skyfire_campaign_banner", "Skyfire Campaign Banner", "Light the sky with a royal message"],
  ["crown_endorsement_archive", "Crown Endorsement Archive", "Revive a legendary royal endorsement"],
  ["royal_census_crystal", "Royal Census Crystal", "Receive a kingdom audience report"],
  ["population_pulse_map", "Population Pulse Map", "Detect a realm-wide movement"],
  ["realm_sentiment_engine", "Realm Sentiment Engine", "Measure the kingdom's collective mood"],
  ["royal_prophecy_engine", "Royal Prophecy Engine", "Trigger a royal prophecy"],
  ["omen_press_kit", "Omen Press Kit", "Turn an omen into favorable coverage"],
  ["future_messaging_bureau", "Future Messaging Bureau", "Publish tomorrow's winning message"],
  ["grand_royal_campaign_blueprint", "Grand Royal Campaign Blueprint", "Launch a legendary campaign"],
  ["grand_campaign_war_room", "Grand Campaign War Room", "Coordinate every royal channel"],
  ["influence_command_table", "Influence Command Table", "Direct a kingdom-wide response"],
  ["aethernet_kingdom_news_license", "Kingdom News License", "Become front-page news"],
  ["front_page_royal_seal", "Front Page Royal Seal", "Command the kingdom's leading headline"],
  ["kingdom_wire_service", "Kingdom Wire Service", "Syndicate a royal story everywhere"],
  ["aethernet_data_core", "Royal Data Core", "Trigger a royal data surge"],
  ["court_algorithm_ledger", "Court Algorithm Ledger", "Optimize a kingdom-wide campaign"],
  ["predictive_audience_engine", "Predictive Audience Engine", "Anticipate the realm's next reaction"],
  ["royal_influence_exchange_charter", "Royal Influence Exchange Charter", "Move the royal markets"],
  ["market_whisper_network", "Market Whisper Network", "Turn a whisper into a market signal"],
  ["influence_futures_desk", "Influence Futures Desk", "Trade on tomorrow's public opinion"],
  ["crown_of_the_aethernet_algorithm_dragon", "Crown of the Algorithm Dragon", "Start a royal trend cascade"],
  ["throne_of_public_opinion", "Throne of Public Opinion", "Command the kingdom's shared belief"],
];
const visualFamilies: ProcVisualFamily[] = [
  "spark",
  "explosion",
  "lightning",
  "fire",
  "water",
  "bubble",
  "confetti",
  "stars",
  "shockwave",
  "smoke",
  "snow",
  "leaves",
  "glitch",
  "prism",
  "void",
  "runes",
];

export const shopItems: ShopItem[] = [
  {
    id: "royal_contract",
    name: "Royal Contract",
    cost: costForLevel(2),
    requiredLevel: 2,
    category: "org_chart",
    effect:
      "Bribe a court official for a royal contract and become a marketing manager for the royal court.",
    inventoryDescription:
      "Your official contract confirms that you work for the Royal Influence Office.",
  },
  ...Array.from({ length: 98 }, (_, index) => createPowerUpgrade(index + 3)),
];

export function powerUpgradeDetails(item: ShopItem): PowerUpgradeDetails | null {
  if (
    item.category !== "power_upgrade" ||
    item.procChance === undefined ||
    item.procEffect === undefined ||
    item.procValue === undefined
  ) {
    return null;
  }

  return {
    procChance: item.procChance,
    effect: item.procEffect,
    value: item.procValue,
  };
}

export function shopItemForLevel(level: number) {
  return shopItems.find((item) => item.requiredLevel === level);
}

function createPowerUpgrade(level: number): ShopItem {
  const [id, name, procEffect] = powerUpgradeContent[level - 3];
  const chancePerMillion = chancePerMillionForLevel(level);
  const procValue = procValueForLevel(level);
  const procChance = `${formatChance(chancePerMillion)}% chance`;
  const effect = `${procChance} per input to ${procEffect.toLowerCase()} worth ${procValue.toLocaleString("en-US")} Influence.`;

  return {
    id,
    name,
    cost: costForLevel(level),
    requiredLevel: level,
    category: "power_upgrade",
    effect,
    procChance,
    procEffect,
    procValue,
    expectedValuePerThousandInputs: Math.round((chancePerMillion * procValue) / 1_000),
    visualFamily: visualFamilies[(level - 3) % visualFamilies.length],
    visualHue: (level * 47) % 360,
    visualIntensity: Math.min(1, 0.32 + level / 130),
  };
}

function costForLevel(level: number) {
  const levelXp = levelDefinition(level).xpRequired;
  const previousLevelXp = levelDefinition(level - 1).xpRequired;
  const unlockInterval = Math.max(1, levelXp - previousLevelXp);
  const multiplier = level % 4 === 0 ? 0.65 : level % 7 === 0 ? 0.85 : 1.1;

  return roundUpForDisplay(unlockInterval * multiplier);
}

function chancePerMillionForLevel(level: number) {
  return Math.max(15, Math.floor(7_500 * Math.pow(0.94, level - 3)));
}

function procValueForLevel(level: number) {
  return Math.max(25, roundUpForDisplay(25 * Math.pow(1.13, level - 3)));
}

function roundUpForDisplay(value: number) {
  const unit =
    value >= 10_000_000
      ? 100_000
      : value >= 1_000_000
        ? 10_000
        : value >= 100_000
          ? 1_000
          : value >= 10_000
            ? 100
            : value >= 1_000
              ? 50
              : 10;

  return Math.ceil(value / unit) * unit;
}

function formatChance(chancePerMillion: number) {
  return (chancePerMillion / 10_000)
    .toFixed(chancePerMillion < 100 ? 4 : chancePerMillion < 1_000 ? 3 : 2)
    .replace(/0+$/, "")
    .replace(/\.$/, "");
}

if (powerUpgradeContent.length !== 98) {
  throw new Error(`Expected 98 power upgrade entries, found ${powerUpgradeContent.length}.`);
}
