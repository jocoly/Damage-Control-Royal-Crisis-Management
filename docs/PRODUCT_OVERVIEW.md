# Damage Control: Royal Crisis Management

## Product Overview

Damage Control is a lightweight desktop companion RPG that lives in a small corner of the user's screen while they work, browse, code, write, create, chat, or play.

The player's real computer activity powers the royal influence office of the Aethernet Kingdom. Every meaningful action at the computer can become progress for the royal court's marketing machine, but the game should never demand constant direct interaction.

Unlike traditional idle games, Damage Control does not generate progress passively. Progress comes from real keyboard and mouse activity while the app is running.

The central prototype question remains:

Does earning progress from real-world keyboard and mouse activity feel satisfying?

---

## Core Loop

1. The player types, clicks, scrolls, and uses their computer normally.
2. Inputs generate Influence.
3. Influence can be spent on:
   - Power upgrades that improve Influence generation.
   - Cosmetics that customize the scene.
4. The same earned Influence is also counted as XP.
5. XP increases Level.
6. Level unlocks:
   - Random cosmetic rewards.
   - New shop categories.
   - New scene expansions.

This separates spendable balance from lifetime progression:

- Influence: current spendable balance after purchases.
- XP: lifetime earned Influence in the current save.
- Level: unlock gate and reward cadence.
- Cosmetics: visible status and personal expression.
- Scene: the living visual record of the player's progress.

---

## Core Design Principles

## Real Activity Drives Progress

The game rewards normal computer use.

Examples:

- Typing code.
- Writing documents.
- Browsing the web.
- Playing games.
- Using creative software.
- Chatting with friends.

The player should never need to actively click the game window to grind progress.

## No Passive Idle Economy

Damage Control must not become a timer farm.

Avoid:

- Passive currency generation.
- Login rewards.
- Energy systems.
- Time gates.
- Fear-of-missing-out mechanics.
- Exponential click inflation.
- Scientific notation currency.

## Progression Through Systems and Expression

The game can become deeper and more rewarding over time, but it should not rely on making every click worth absurd amounts.

Progression should come from:

- Better input-driven rewards.
- New shop categories.
- Cosmetic collections.
- Scene expansions.
- Character upgrades.
- Visual transformation.
- Unlocking more things to customize.

---

## Setting

Damage Control is set in the Aethernet Kingdom, a fantasy medieval kingdom where modern corporate technology exists alongside magic.

The world should feel absurd, comedic, and workplace-adjacent:

- Knights carry smartphones.
- Wizards process spreadsheets.
- Dragons deliver packages.
- Castles become data centers.
- Royal offices become corporate departments.
- Arcane infrastructure powers the internet-equivalent Aethernet.

The player is not a chosen hero.

The player starts as a low-level fantasy-corporate nobody contracted by the royal court, then gradually builds influence until the court's tiny marketing desk becomes an absurd castle-campus media empire.

---

## Player Role

The player begins as a shabby royal marketing hire with almost nothing:

- Dented armor.
- Rusty sword.
- Tiny patch of grass.
- Broken wooden sign.
- Empty background.

The starting state should feel intentionally pathetic.

The long-term fantasy is not becoming a traditional hero. It is building enough cultural, social, and organizational influence to transform a sad little royal marketing desk into the Aethernet Kingdom's most powerful influence operation.

---

## Progression Model

## Influence

Influence is the spendable currency.

Influence is earned from real input activity and spent on:

- Power upgrades.
- Cosmetics.
- Scene customization.
- Future systems.

Current foundation:

- 1 keypress = 1 Influence earned = 1 XP.
- 1 mouse click = 1 Influence earned = 1 XP.

Future systems may add bonuses, bundles, procs, or upgrade-driven gains, but all generation must trace back to real user activity.

Current Influence is the player's spendable balance:

Current Influence = lifetime earned Influence - spent Influence.

## XP and Level

Influence is XP.

XP is the lifetime total of earned Influence in the current save.

In the current foundation, XP is equivalent to the total number of counted inputs:

XP = lifetime keypresses + lifetime mouse clicks.

Spending Influence does not reduce XP.

XP increases account Level.

Initial level curve:

- Level 1 starts at 0 XP.
- Level 2 starts at 500 XP.
- Level 3 starts at 1,500 XP.
- Level 4 starts at 6,000 XP.
- Level 5 starts at 30,000 XP.
- Later levels should use nonlinear increasing requirements tuned around the expected multi-week progression timeline.

XP requirements should be displayed as lifetime totals. For example, Level 2 begins at 500 XP, and the Level 2 progress span runs from 500 total XP to 1,500 total XP.

Early tuning intent:

- Level 1 to Level 2 should take roughly 5 minutes of passive real computer activity.
- Level 2 to Level 3 currently feels acceptable and should remain close to the current pacing.
- Level 4 should feel like the first longer-session milestone.
- Level 5 should feel like the first multi-hour milestone.

Level unlocks:

- Random cosmetic rewards.
- New shop categories.
- Scene expansions.
- Milestone rewards.

Level should make long-term progress legible even when Influence is spent.

## Title Progression

Levels also grant a fantasy-corporate title. Titles are visible progression labels that make account status readable before the full scene exists.

Initial title progression for Levels 1-100:

- Level 1: Some Noob.
  - New to the Aethernet Kingdom
- Levels 2-5: Court Marketing Intern.
  - Fresh recruit in the royal influence office
  - Changes to Court Marketing Manager after purchasing Royal Contract
- Levels 6-10: Assistant Royal Page.
  - Trusted with minor court messaging tasks
- Levels 11-15: Junior Campaign Coordinator.
  - Allowed to schedule goblin focus groups
- Levels 16-20: Court Communications Clerk.
  - Officially part of the royal influence office
- Levels 21-25: Senior Court Communications Clerk.
  - Knows which announcements should never be archived
- Levels 26-30: Royal Marketing Associate.
  - First taste of managing the crown's reputation
- Levels 31-35: Campaign Supervisor.
  - Managing interns and minor royal campaigns
- Levels 36-40: Regional Influence Captain.
  - Responsible for royal reputation in distant provinces
- Levels 41-45: Royal Influence Officer.
  - Focused on growing the royal court's reputation
- Levels 46-50: Court Marketing Director.
  - A respected leader in the royal influence machine
- Levels 51-55: Chief Campaign Strategist.
  - Plans kingdom-wide royal messaging campaigns
- Levels 56-60: Master of Royal Operations.
  - Runs the court's day-to-day influence machine
- Levels 61-65: High Chancellor of Messaging.
  - Now shaping the crown's official voice
- Levels 66-70: Grand Marshal of Campaigns.
  - Commands the royal court's largest influence initiatives
- Levels 71-75: Royal Brand Advisor.
  - The crown starts taking your positioning seriously
- Levels 76-80: Archchancellor of Aethernet Reach.
  - One of the most influential messengers in the kingdom
- Levels 81-85: Supreme Court Marketer.
  - The royal court's influence office is now legendary
- Levels 86-90: Lord of Royal Influence.
  - Your recommendations shape kingdom culture
- Levels 91-95: Aetherlord of Messaging.
  - A title granted to only a handful of royal operators in history
- Levels 96-99: Legend of the Royal Court.
  - People know your campaigns everywhere
- Level 100: Kingmaker.
  - Not the monarch. The person who decides what the kingdom believes about the monarch

Title hover flavor:

- Some Noob: "New to the Aethernet Kingdom"
- Court Marketing Intern: "Still learning which end of the slogan is business-facing"
- Court Marketing Manager: "The contract is mostly legitimate, depending on who asks"
- Court Communications Clerk: "Your signature can now delay a proclamation"
- Senior Court Communications Clerk: "You know which forms can make scandals disappear"
- Royal Marketing Associate: "You have acquired a clipboard and the confidence to brief a duke"
- Campaign Supervisor: "The interns fear your campaign calendars"
- Regional Influence Captain: "Your jurisdiction now includes villages that spell Aethernet differently"
- Royal Influence Officer: "Reputation is just logistics with better heraldry"
- Court Marketing Director: "Your calendar is now more dangerous than most coup attempts"
- Chief Campaign Strategist: "You can turn a vague prophecy into a royal rollout plan"
- Master of Royal Operations: "If something moves, you have already scheduled it"
- High Chancellor of Messaging: "Royalty has discovered your inbox and the kingdom will hear about it"
- Grand Marshal of Campaigns: "Your campaign map requires a cartographer and a legal review"
- Royal Brand Advisor: "The King has started forwarding your taglines"
- Archchancellor of Aethernet Reach: "Your memos travel faster than most spells and trend harder than dragons"
- Supreme Court Marketer: "Bards have started quoting your campaign briefs"
- Lord of Royal Influence: "A casual suggestion from you becomes policy by lunch"
- Aetherlord of Messaging: "Entire markets panic when the court account posts"
- Kingmaker: "Influence has surpassed authority"

## Power Upgrades

Power upgrades affect Influence generation or reward opportunities.

They should be carefully designed so they enhance real activity rather than replacing it.

Examples:

- Bonus Influence bundles triggered by random activity procs.
- Improved reward chances.
- More event variety.
- Better activity-based procs.

Power upgrades should not create passive idle income.

Power upgrade procs should use random percent chances per real input rather than fixed input intervals. Early items should fire fairly often for small rewards; later items should become rarer jackpot-style events with much larger rewards.

Proc rolls should be batched. Raw input handlers should only enqueue eligible inputs, and the game should drain those queued rolls on a short cooldown window, currently about once per second. Every eligible input still receives its chance, but random calculations should not run directly inside each keyboard or mouse event handler.

## Functional Shop Item Ladder

The following functional shop items are planned progression unlocks. Items can be purchased and stored before their effects are implemented.

Status-only shop item:

- Level 2: Royal Contract.
  - Cost: 100 Influence.
  - The player pays 100 Influence to bribe a court official for a royal contract.
  - This makes the player the marketing manager for the Aethernet royal court.
  - This item has no mechanical effect.
  - If the player's current title is Court Marketing Intern, it changes to Court Marketing Manager.
  - Required before any other shop items appear.

| Req. Level | Item | Planned Effect | Cost | EV / 1,000 Inputs |
| ---: | --- | --- | ---: | ---: |
| 3 | Tiny Royal Reminder Kit | 0.75% chance per input to publish a tiny royal reminder worth 25 Influence | 100 | 188 |
| 6 | Bottomless Court Coffee Mug | 0.60% chance per input to fuel a late-night court campaign worth 50 Influence | 250 | 300 |
| 9 | Royal Suggestion Box of Destiny | 0.50% chance per input to discover a court-approved idea worth 100 Influence | 500 | 500 |
| 12 | Royal Crier App Subscription | 0.40% chance per input to trigger a royal bulletin worth 150 Influence | 900 | 600 |
| 15 | Ledger of Courtly Accounting | 0.32% chance per input to reclassify a campaign win worth 250 Influence | 1,500 | 800 |
| 18 | Crystal Audience Analytics Orb | 0.26% chance per input to discover an audience trend worth 400 Influence | 2,500 | 1,040 |
| 21 | Enchanted Press Release Quill | 0.22% chance per input to draft a viral royal post worth 600 Influence | 4,000 | 1,320 |
| 24 | Royal Mimic Stamp | 0.18% chance per input to stamp a royal duplicate worth 900 Influence | 6,000 | 1,620 |
| 27 | Royal Messaging Handbook, Revised Edition | 0.15% chance per input to approve revised messaging worth 1,200 Influence | 8,500 | 1,800 |
| 30 | Court Newsletter Press | 0.12% chance per input to release a court newsletter worth 1,800 Influence | 12,000 | 2,160 |
| 33 | Goblin Outreach Playbook | 0.10% chance per input to launch a royal outreach campaign worth 2,500 Influence | 17,000 | 2,500 |
| 36 | Royal Quest Board | 0.08% chance per input to complete a royal contract worth 3,500 Influence | 23,000 | 2,800 |
| 39 | Arcane Audience Survey Scrolls | 0.07% chance per input to uncover audience sentiment worth 4,500 Influence | 30,000 | 3,150 |
| 42 | Court Recruitment Poster Set | 0.06% chance per input to recruit royal advocates worth 6,000 Influence | 40,000 | 3,600 |
| 45 | Royal Courier Satchel | 0.05% chance per input to receive a royal decree worth 8,000 Influence | 55,000 | 4,000 |
| 48 | Lute of Royal Ballads | 0.04% chance per input to start a royal hype train worth 12,000 Influence | 75,000 | 4,800 |
| 51 | Royal Sponsorship Contract | 0.035% chance per input to secure a royal sponsorship worth 16,000 Influence | 100,000 | 5,600 |
| 54 | Runic Royal Printing Press | 0.03% chance per input to print a runic campaign worth 22,000 Influence | 130,000 | 6,600 |
| 57 | Dragon Egg Aethernet Cluster | 0.026% chance per input to hatch an Aethernet surge worth 30,000 Influence | 170,000 | 7,800 |
| 60 | Tome of Royal Memes | 0.022% chance per input to create a court meme worth 40,000 Influence | 220,000 | 8,800 |
| 63 | Royal Public Relations Handbook | 0.019% chance per input to avert a royal PR crisis worth 55,000 Influence | 280,000 | 10,450 |
| 66 | Patent Pending Campaign Spellbook | 0.016% chance per input to invent a royal trend worth 75,000 Influence | 350,000 | 12,000 |
| 69 | Arcane Audience Research Journal | 0.014% chance per input to discover an audience breakthrough worth 95,000 Influence | 450,000 | 13,300 |
| 72 | Royal Aetherwave Broadcast Tower | 0.012% chance per input to hit an Aetherwave broadcast worth 125,000 Influence | 575,000 | 15,000 |
| 75 | Royal Expedition Contract Ledger | 0.010% chance per input to receive a royal expedition report worth 170,000 Influence | 725,000 | 17,000 |
| 78 | Dragon-Endorsed Royal Campaign | 0.009% chance per input to receive a dragon endorsement for the crown worth 220,000 Influence | 900,000 | 19,800 |
| 81 | Royal Census Crystal | 0.008% chance per input to receive a kingdom audience report worth 280,000 Influence | 1,100,000 | 22,400 |
| 84 | Royal Prophecy Engine | 0.007% chance per input to trigger a royal prophecy worth 360,000 Influence | 1,350,000 | 25,200 |
| 87 | Grand Royal Campaign Blueprint | 0.006% chance per input to launch a legendary campaign worth 475,000 Influence | 1,650,000 | 28,500 |
| 90 | Aethernet Kingdom News License | 0.005% chance per input to become Front Page News worth 650,000 Influence | 2,000,000 | 32,500 |
| 93 | Aethernet Data Core | 0.004% chance per input to trigger an Aethernet Surge worth 900,000 Influence | 2,500,000 | 36,000 |
| 96 | Royal Influence Exchange Charter | 0.0035% chance per input to move the royal markets worth 1,200,000 Influence | 3,100,000 | 42,000 |
| 99 | Crown of the Aethernet Algorithm Dragon | 0.003% chance per input to start an Aethernet trend cascade worth 1,750,000 Influence | 4,000,000 | 52,500 |

## Cosmetics

Cosmetics are visible status and progression.

They should become one of the main reasons to keep playing.

Cosmetics do not need to be equipped through deep menus. The player is decorating a living diorama.

---

## The Scene

The scene is the player's kingdom, office, home, and status symbol in one compact desktop companion window.

It is always visible in the small game window.

The player should be able to look at a screenshot and roughly estimate progression.

## Level 1 Scene

Very humble:

- One shabby knight.
- Dented armor.
- Rusty sword.
- Tiny patch of grass.
- Broken wooden sign.
- Empty background.

It should look intentionally low-status.

## Long-Term Scene Transformation

The scene should gradually evolve from:

1. Roadside camp.
2. Small cottage.
3. Royal marketing office.
4. Town hall.
5. Castle.
6. Corporate citadel.
7. Floating techno-magical metropolis.

By endgame, the scene should look absurd:

- A ruler at a glowing workstation.
- Wizards processing spreadsheets.
- Knights carrying servers.
- Floating crystal Wi-Fi towers.
- Arcane IT department.
- Massive castle-datacenter hybrid.

The joke is that the player started with a rusty sword and eventually built the fantasy equivalent of a multinational tech corporation.

---

## Cosmetic Categories

## Character Cosmetics

Character cosmetics change the knight.

Examples:

Common:

- Slightly shinier helmet.
- Wooden shield.
- New boots.
- Colored tabard.

Rare:

- Plumed helmet.
- Enchanted sword.
- Fancy cape.
- Tiny familiar.

Epic:

- Dragonbone armor.
- Floating spellbook.
- Glowing runes.
- Mechanical gauntlet.

Legendary:

- Living armor.
- Phoenix companion.
- Crown of Influence.
- Hovering crystal throne.

## Environment Cosmetics

Environment cosmetics decorate the world around the player.

Early examples:

- Campfire.
- Barrel.
- Crates.
- Flower patch.
- Mailbox.

Mid-game examples:

- Blacksmith.
- Mage kiosk.
- Merchant wagon.
- Fountain.

Late examples:

- Clockwork factory.
- Floating islands.
- Portal gate.
- Arcane datacenter.

## Building Cosmetics

Building cosmetics are the biggest visible progression pieces.

They dramatically transform the scene.

Milestone examples:

- Level 1: Roadside camp.
- Level 10: Small cottage.
- Level 25: Royal marketing office.
- Level 50: Town hall.
- Level 100: Castle.
- Level 250: Corporate citadel.
- Level 500+: Floating techno-magical metropolis.

---

## Random Level-Up Rewards

Every level should grant something.

Most levels may grant:

- Common cosmetic.
- Small Influence bundle.
- Tiny shop discount token.

Milestone levels should grant:

- Guaranteed rare cosmetic.
- New shop unlock.
- New scene expansion.

Examples:

- Level 5: Wooden training dummy.
- Level 10: Unlock Housing Shop and reward Small Cottage.
- Level 25: Unlock Town Decorations and reward Fountain.
- Level 50: Unlock Prestige Decorations and reward Royal Media Hall.

---

## Shop Unlock Structure

The shop should not show everything immediately.

Level unlocks should create anticipation and make the next category feel meaningful.

Example structure:

- Level 1: Basic Supplies.
  - Armor skins.
  - Weapons.
  - Grass decorations.
- Level 10: Housing.
  - Huts.
  - Cottages.
  - Gardens.
- Level 25: Town Life.
  - Merchants.
  - Citizens.
  - Wagons.
- Level 50: Magic District.
  - Wizards.
  - Portals.
  - Enchantments.
- Level 100: Corporate Kingdom.
  - Cubicles.
  - Arcane servers.
  - Crystal monitors.
- Level 250: Aethernet Division.
  - Data towers.
  - Flying routers.
  - Spell-powered cloud infrastructure.

---

## Expected Progression Feel

## Week 1

The player may have:

- Level 15-25.
- Cottage.
- A few decorations.
- Better armor.
- One companion.

The scene starts to feel alive.

## Week 2

The player may have:

- Level 40-60.
- Small town.
- NPC workers.
- Blacksmith.
- Mage office.

The scene begins animating more.

## Week 3

The player may have:

- Level 75-100.
- Large headquarters.
- Multiple NPCs.
- Emerging fantasy-corporate theme.

## Week 4+

The player may have:

- Castle-campus hybrid.
- Many decorations.
- Complex scene.
- Distinct personal style.

---

## User Interface

Damage Control is not a full-screen game.

The app should remain a tiny desktop companion, roughly 250x250 to 400x400 pixels for normal operation, expanding only when necessary for menus or panels.

The game should sit comfortably:

- In a screen corner.
- Above a taskbar.
- On a secondary monitor.
- As an overlay while gaming.

The scene remains the primary visual surface.

## Assetless Development Baseline

Before the art-driven scene exists, Damage Control should continue development as a complete assetless interface.

The current interface is not a disposable mockup. It is the canonical game build without art assets.

All core systems should be playable and testable in the current interface first:

- Influence, XP, and Level.
- Notifications.
- Shop categories.
- Purchase confirmation.
- Inventory-driven unlocks.
- Level-up rewards.
- Settings and dev tools.
- Future cosmetic ownership and selection.

When scene art and assets are added, they should extend the same build and preserve the existing functionality.

This means:

- Damage Control has one build and one feature set.
- Every existing control should either remain available or receive a clear equivalent in the art-driven scene.
- The scene should decorate and express progression, not hide or replace critical game state.
- Gameplay systems should not depend on final art being ready.
- UI behavior should be validated before being represented through art, animation, or scene interactions.

---

## Technical Requirements

## Performance

Damage Control must remain extremely lightweight.

Goals:

- Minimal CPU usage.
- Minimal memory usage.
- No noticeable impact on gaming performance.
- Low battery consumption.

Visual updates should be batched and should not process directly in response to every input event.

## Privacy

The app must never store typed content.

Input tracking may count:

- Keypress total.
- Mouse click total.
- Future aggregate activity totals.

Do not store:

- Key contents.
- Typed text.
- Screenshots.
- URLs.
- App names.
- Active window titles.

## Proposed Technology Stack

Desktop shell:

- Tauri.

Frontend:

- Svelte.

Game logic:

- TypeScript for frontend display and interaction logic.
- Rust for authoritative input counting, persistence, and privileged desktop behavior.

Persistence:

- JSON save files for prototype.
- SQLite can be considered later if save complexity grows.

Rendering:

- Canvas or PixiJS for the small animated scene.
- PixiJS is preferred when the scene becomes animated and asset-heavy.

---

## Prototype Direction

Current prototype priority:

1. Real input tracking.
2. Influence as spendable currency.
3. Persistence.
4. Shop and purchase flow.
5. Invisible inventory categories.
6. Basic level and XP system.
7. First visible scene.
8. Cosmetic reward loop.

Do not build broad content before the core activity-to-progress loop feels good.
