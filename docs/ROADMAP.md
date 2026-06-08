# Damage Control: Royal Crisis Management Roadmap

## Guiding Goal

Build the core loop before building content volume.

The first playable version should answer:

Does earning progress from real-world keyboard and mouse activity feel satisfying?

---

## Current Prototype Foundation

Implemented foundation:

- Global keypress counting.
- Global mouse click counting.
- Focused-window keypress bridge.
- Influence as spendable balance.
- XP as lifetime earned Influence.
- Level derived from XP.
- Level progress display.
- Level title brackets for Levels 1-100.
- Level-up notification foundation.
- JSON persistence.
- Autosave.
- Shop purchase confirmation.
- Invisible inventory category for org chart items.
- Temporary reset control for testing.
- Dev-only toolkit for save-state testing.
- Compact transparent desktop companion window.
- Assetless GUI-mode presentation.

Current limitations:

- No scene.
- No cosmetic rewards.
- No visible inventory.
- Only one shop item.
- Royal Contract unlocks access to the rest of the shop but has no generation effect yet.

---

## Phase 1: Core Progression Model

Goal:

Maintain and tune the minimum progression structure needed for the real game loop.

Tasks:

- Tune the initial XP curve.
- Keep level-up detection reliable across save loads and dev tools.
- Keep Level and XP progress compact and readable.
- Keep title brackets visible and aligned with level progression.
- Keep Influence as spendable currency.
- Preserve lifetime key/click stats separately from spendable Influence.

Design rules:

- XP is lifetime earned Influence.
- Spending Influence must not reduce XP or Level.
- Level should be a long-term account progression signal.
- Titles should make status legible before the full scene exists.
- Early anchors are Level 2 at 500 XP, Level 3 at 1,500 XP, Level 4 at 6,000 XP, and Level 5 at 30,000 XP.
- XP requirements are lifetime totals; the progress bar normalizes progress between the current level's total XP threshold and the next level's total XP threshold.
- Post-Level-5 progression should grow nonlinearly rather than adding a flat amount per level.
- Avoid passive XP or timer-based progression.

---

## Phase 1.5: GUI Mode as Source of Truth

Goal:

Keep the assetless game fully playable while systems are built, then port those systems 1:1 into the art-driven scene.

Tasks:

- Treat the current compact GUI as the canonical functional interface.
- Build new systems in the current interface before art dependencies exist.
- Keep all controls, confirmations, notifications, and state readouts usable without scene art.
- Document any control that needs an art-scene equivalent.

Design rules:

- The current interface is not throwaway.
- Damage Control has one build and one feature set.
- The art scene should express the same systems, not redefine them.
- Final art should not be required to test economy, progression, purchases, rewards, or notifications.
- Every art-driven interaction should have a clear behavioral match to the existing interaction.

---

## Phase 2: First Upgrade Effect

Goal:

Make the first functional power upgrade matter without undermining real activity.

Tasks:

- Give Tiny Royal Reminder Kit an activity-triggered effect.
- Define effect timing and probability.
- Persist owned upgrade state.
- Add tests proving the effect only triggers from real input-driven activity.

Candidate effect:

- Every real input, small random chance for a bonus Influence bundle.

Constraints:

- No passive generation.
- No timer income.
- No exponential multiplier.
- Bonus must trace back to player input.
- Proc rolls should be batched on a short cooldown window rather than calculated directly inside every raw input handler.

---

## Phase 3: First Scene

Goal:

Add the first visible diorama while preserving the GUI-mode systems 1:1.

Level 1 scene:

- Shabby knight.
- Dented armor.
- Rusty sword.
- Tiny patch of grass.
- Broken wooden sign.
- Empty background.

Tasks:

- Add scene rendering surface.
- Decide initial renderer: Canvas or PixiJS.
- Keep the scene inside the compact window.
- Port existing GUI-mode readouts and controls into the scene layout without changing behavior.
- Keep the assetless GUI path available during scene development.
- Ensure visual updates use the existing batched visual update loop.

Design intent:

- The scene should feel intentionally pathetic.
- The player is not a hero yet.
- The scene is the long-term status symbol.
- Art should add expression and context, not remove functional clarity.

---

## Phase 4: Cosmetics Foundation

Goal:

Add cosmetic inventory and visible cosmetic application.

Tasks:

- Define cosmetic item schema.
- Add cosmetic categories:
  - Character.
  - Environment.
  - Building.
- Persist cosmetic inventory.
- Track unlocked cosmetics separately from currently visible scene state.
- Add first visible cosmetic rewards.

Initial cosmetic examples:

- Slightly shinier helmet.
- Wooden shield.
- Campfire.
- Barrel.
- Flower patch.
- Roadside camp sign.

Rules:

- Cosmetics are visible progression.
- Cosmetics should not be buried behind deep menus.
- The scene is the inventory expression surface.

---

## Phase 5: Level-Up Rewards

Goal:

Make every level feel rewarding.

Tasks:

- Add level-up reward table.
- Grant a reward on every level.
- Add milestone rewards.
- Extend the existing notification foundation for reward delivery.
- Persist claimed rewards.
- Implement and verify reward flow in the current interface before scene-specific presentation.

Most level rewards may include:

- Common cosmetic.
- Small Influence bundle.
- Tiny shop discount token.

Milestone examples:

- Level 5: Wooden training dummy.
- Level 10: Unlock Housing and reward Small Cottage.
- Level 25: Unlock Town Decorations and reward Fountain.
- Level 50: Unlock Prestige Decorations and reward Royal Media Hall.

Rules:

- Random rewards should be exciting but not required for basic function.
- Milestones should be predictable and meaningful.
- Reward delivery should not interrupt normal computer use.

---

## Phase 6: Shop Categories

Goal:

Turn the shop into a level-gated discovery system.

Unlock structure:

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

Tasks:

- Add shop category schema.
- Gate shop categories by level.
- Show locked categories compactly.
- Add category tabs or segmented controls.
- Keep purchase logic backend-authoritative.

---

## Phase 7: Scene Expansion Milestones

Goal:

Make progression visible from screenshots.

Building milestones:

- Level 1: Roadside camp.
- Level 10: Small cottage.
- Level 25: Royal marketing office.
- Level 50: Town hall.
- Level 100: Castle.
- Level 250: Corporate citadel.
- Level 500+: Floating techno-magical metropolis.

Tasks:

- Define scene expansion state.
- Persist unlocked expansion tier.
- Render tier-appropriate scene.
- Add milestone reward presentation.

Design goal:

The player should be able to look at the window and immediately feel how far they have come.

---

## Phase 8: Long-Term Scene Life

Goal:

Make the scene feel increasingly alive without hurting performance.

Tasks:

- Add small NPC idle animations.
- Add activity-triggered visual reactions.
- Add ambient scene details.
- Add more fantasy-corporate jokes.
- Keep updates batched and lightweight.

Endgame vision:

- Castle-campus hybrid.
- Arcane office workers.
- Knights carrying servers.
- Crystal network infrastructure.
- Floating techno-magical city elements.

Performance rule:

Visuals must never process directly in response to every raw input event. Use the visual batching system.

---

## Phase 9: Testing and Tuning

Goal:

Tune progression so the app feels rewarding over days and weeks.

Questions to answer:

- Does Influence accumulation feel satisfying?
- Does spending Influence feel meaningful?
- Does Level progression feel too fast or too slow?
- Are early rewards frequent enough?
- Does the scene change soon enough?
- Does the app remain lightweight while running all day?

Testing tools:

- Temporary reset button.
- Dev-only GUI toolkit.
- Save-state modifiers such as +100 Influence.
- Debug save inspection.
- Adjustable XP curve during development.
- Synthetic input tests for backend logic only.

Remove, hide, or keep dev-gated test-only controls before a public release.

---

## Near-Term Next Steps

Recommended next implementation order:

1. Tune the XP curve using the current interface and dev tools.
2. Add first level-up reward plumbing in the current interface.
3. Add first simple scene placeholder without replacing existing behavior.
4. Make Tiny Royal Reminder Kit produce an input-triggered bonus.
5. Port reward and shop presentation into the scene once art direction exists.
