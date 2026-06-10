<script lang="ts">
  import { LogicalSize } from "@tauri-apps/api/dpi";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount, tick } from "svelte";
  import { startFocusedInputBridge } from "$lib/focusedInputBridge";
  import CharacterAvatar from "$lib/CharacterAvatar.svelte";
  import CharacterScene from "$lib/CharacterScene.svelte";
  import {
    defaultCharacterAppearance,
    eyeOptions,
    hairColorOptions,
    hairOptions,
    mouthOptions,
    noseOptions,
    skinToneOptions,
    type CharacterAppearance,
  } from "$lib/characterAppearance";
  import {
    outfitForId,
    outfitUnlockedAtLevel,
    outfitsForProgress,
    sceneUnlockedAtLevel,
    sceneSelectionForProgress,
  } from "$lib/sceneCatalog";
  import {
    isPromotionLevel,
    loyalSubjectTitle,
    titleForLevelAndInventory,
    titleHoverText,
  } from "$lib/titleCatalog";
  import { levelDefinition } from "$lib/levelCatalog";
  import {
    hiredStoryEvent,
    kingdomNamingStoryEvent,
    royalContractStoryEvent,
    storyEventsForLevelRange,
    type MenuTutorialTarget,
    type MenuTutorialStep,
    type MenuTutorialStoryEvent,
    type NotificationStoryEvent,
    type PurchaseStoryEvent,
  } from "$lib/storyEventCatalog";
  import {
    inventoryImageForItem,
    powerUpgradeDetails,
    shopItemForLevel,
    shopItems,
    type ProcVisualFamily,
    type ShopItem,
  } from "$lib/shopCatalog";
  import {
    startVisualUpdateLoop,
    type InputSnapshot,
    type VisualFrame,
  } from "$lib/visualUpdateLoop";

  const startingLevel = levelDefinition(1);

  let counts = $state<VisualFrame>({
    influence: 0,
    xp: 0,
    level: 1,
    xp_for_current_level: startingLevel.xpRequired,
    xp_for_next_level: startingLevel.xpRequiredForNextLevel,
    keys: 0,
    clicks: 0,
    bonus_influence: 0,
    spent_influence: 0,
    power_event_sequence: 0,
    last_power_event_at_millis: 0,
    last_power_event_amount: 0,
    last_power_event_item_level: 0,
    inventory_item_ids: [],
    seen_story_event_ids: [],
    seen_shop_item_ids: [],
    kingdom_name: null,
    last_input_at_millis: 0,
    isIdle: true,
    lastInputAgeMs: null,
  });
  let isDetailsOpen = $state(false);
  let isShopOpen = $state(false);
  let isInventoryOpen = $state(false);
  let isSettingsOpen = $state(false);
  let isDevToolsOpen = $state(false);
  let devMessage = $state("");
  let purchaseMessage = $state("");
  let resetMessage = $state("");
  let settingsMessage = $state("");
  let appSettings = $state<AppSettings>({
    run_on_startup: false,
    always_on_top: true,
    show_taskbar_icon: true,
    dev_mode: false,
    selected_outfit_id: "humble_rags",
    character_created: false,
    character_eyes: defaultCharacterAppearance.eyes,
    character_nose: defaultCharacterAppearance.nose,
    character_mouth: defaultCharacterAppearance.mouth,
    character_hair: defaultCharacterAppearance.hair,
    character_hair_color: defaultCharacterAppearance.hairColor,
    character_skin_tone: defaultCharacterAppearance.skinTone,
  });
  let areSettingsLoaded = $state(false);
  let isSavingCharacter = $state(false);
  let characterCreationError = $state("");
  let characterDraft = $state<CharacterAppearance>({
    ...defaultCharacterAppearance,
  });
  let pendingPurchaseId = $state<string | null>(null);
  let confirmingPurchaseId = $state<string | null>(null);
  let isConfirmingReset = $state(false);
  let isResetting = $state(false);
  let isTitleDescriptionOpen = $state(false);
  let isPowerProcVisible = $state(false);
  let powerProcAmount = $state(0);
  let powerProcVisualFamily = $state<ProcVisualFamily>("spark");
  let powerProcVisualHue = $state(45);
  let powerProcVisualIntensity = $state(0.35);
  let powerProcOriginX = $state(50);
  let powerProcOriginY = $state(50);
  let lastPointerX = 50;
  let lastPointerY = 50;
  let lastPointerAtMillis = 0;
  let hasLoadedInitialSnapshot = false;
  let lastSeenLevel = 1;
  let lastSeenPowerEventSequence = 0;
  let notifications = $state<GameNotification[]>([]);
  let activeNotification = $state<GameNotification | null>(null);
  let suspendedTutorialNotification = $state<GameNotification | null>(null);
  let suspendedTutorialStepIndex = $state(0);
  let isTutorialNotificationReview = $state(false);
  let activeTutorialStepIndex = $state(0);
  let tutorialInfluenceInputCount = $state(0);
  let panelElement: HTMLElement;
  let lastWindowWidth = 0;
  let lastWindowHeight = 0;
  let resizeFrameId: number | undefined;
  let powerProcTimeoutId: number | undefined;
  let highlightedShopItemIds = $state<Set<string>>(new Set());
  let kingdomNameInput = $state("");
  let kingdomNameError = $state("");
  let isSavingKingdomName = $state(false);
  let appliedWindowTitle = "";

  type PurchaseResult = {
    status:
      | "purchased"
      | "already_owned"
      | "not_enough_influence"
      | "locked"
      | "unknown_item";
    snapshot: InputSnapshot;
  };

  type GameNotification = {
    id: string;
    title: string;
    body: string;
    level: number;
    storyEventId?: string;
    tutorialSteps?: MenuTutorialStep[];
    reveal?:
      | {
          kind: "scene";
          name: string;
          image: string;
          filter?: string;
        }
      | {
          kind: "title";
          name: string;
          description: string;
          flavor: string;
        };
  };

  type AppSettings = {
    run_on_startup: boolean;
    always_on_top: boolean;
    show_taskbar_icon: boolean;
    dev_mode: boolean;
    selected_outfit_id: string;
    character_created: boolean;
    character_eyes: string;
    character_nose: string;
    character_mouth: string;
    character_hair: string;
    character_hair_color: string;
    character_skin_tone: string;
  };

  const numberFormatter = new Intl.NumberFormat("en-US");
  const royalContractId = "royal_contract";
  const maxKingdomNameLength = 12;
  const procParticles = Array.from({ length: 18 }, (_, index) => {
    const angle = (index / 18) * Math.PI * 2;
    const distance = 42 + (index % 4) * 13;

    return {
      x: Math.round(Math.cos(angle) * distance),
      y: Math.round(Math.sin(angle) * distance),
      delay: (index % 6) * 22,
      size: 4 + (index % 4) * 2,
      rotation: index * 47,
    };
  });

  function isOwned(itemId: string) {
    return counts.inventory_item_ids.includes(itemId);
  }

  function ownsRoyalContract() {
    return isOwned(royalContractId);
  }

  function applySnapshot(snapshot: InputSnapshot, options = { syncLevelTracker: false }) {
    counts = {
      ...counts,
      ...snapshot,
      spent_influence: Math.max(0, snapshot.xp - snapshot.influence),
      seen_shop_item_ids: snapshot.seen_shop_item_ids ?? counts.seen_shop_item_ids,
    };
    syncWindowTitle(snapshot.kingdom_name);
    syncHighlightedShopItems();

    if (options.syncLevelTracker) {
      hasLoadedInitialSnapshot = true;
      lastSeenLevel = snapshot.level;
      lastSeenPowerEventSequence = snapshot.power_event_sequence;
      isPowerProcVisible = false;
    } else {
      updatePowerProcFeedback(snapshot);
    }
  }

  function applyVisualFrame(frame: VisualFrame) {
    const previousLevel = lastSeenLevel;

    counts = frame;
    syncWindowTitle(frame.kingdom_name);
    syncHighlightedShopItems();

    if (!hasLoadedInitialSnapshot) {
      hasLoadedInitialSnapshot = true;
      enqueueUnseenStoryNotifications(1, frame.level, frame.seen_story_event_ids);
      lastSeenLevel = frame.level;
      lastSeenPowerEventSequence = frame.power_event_sequence;
      return;
    }

    if (frame.level > previousLevel) {
      enqueueLevelNotifications(previousLevel + 1, frame.level);
      enqueueUnseenStoryNotifications(
        previousLevel + 1,
        frame.level,
        frame.seen_story_event_ids,
      );
    }

    updatePowerProcFeedback(frame);
    lastSeenLevel = frame.level;
  }

  function updatePowerProcFeedback(snapshot: InputSnapshot) {
    if (
      hasLoadedInitialSnapshot &&
      snapshot.power_event_sequence > lastSeenPowerEventSequence
    ) {
      showPowerProcFeedback(
        snapshot.last_power_event_amount,
        snapshot.last_power_event_item_level,
      );
    }

    lastSeenPowerEventSequence = snapshot.power_event_sequence;
  }

  function showPowerProcFeedback(amount: number, itemLevel: number) {
    const item = shopItemForLevel(itemLevel);

    powerProcAmount = amount;
    powerProcVisualFamily = item?.visualFamily ?? "spark";
    powerProcVisualHue = item?.visualHue ?? 45;
    powerProcVisualIntensity = item?.visualIntensity ?? 0.35;
    const hasRecentPointer = Date.now() - lastPointerAtMillis <= 2_000;
    powerProcOriginX = hasRecentPointer ? lastPointerX : 50;
    powerProcOriginY = hasRecentPointer ? lastPointerY : 50;
    isPowerProcVisible = true;

    if (powerProcTimeoutId !== undefined) {
      window.clearTimeout(powerProcTimeoutId);
    }

    powerProcTimeoutId = window.setTimeout(() => {
      isPowerProcVisible = false;
    }, 1_600);
  }

  function resetFrontendRunState() {
    notifications = [];
    activeNotification = null;
    suspendedTutorialNotification = null;
    suspendedTutorialStepIndex = 0;
    isTutorialNotificationReview = false;
    activeTutorialStepIndex = 0;
    tutorialInfluenceInputCount = 0;
    hasLoadedInitialSnapshot = true;
    lastSeenLevel = 1;
    lastSeenPowerEventSequence = 0;
    isPowerProcVisible = false;
    powerProcAmount = 0;
    powerProcVisualFamily = "spark";
    powerProcVisualHue = 45;
    powerProcVisualIntensity = 0.35;
    highlightedShopItemIds = new Set();

    if (powerProcTimeoutId !== undefined) {
      window.clearTimeout(powerProcTimeoutId);
      powerProcTimeoutId = undefined;
    }
  }

  function enqueueLevelNotifications(startLevel: number, endLevel: number) {
    const revealNotifications: GameNotification[] = [];
    const nextNotifications: GameNotification[] = [];

    for (let level = startLevel; level <= endLevel; level += 1) {
      const shopUnlock = shopItemForLevel(level);
      const outfitUnlock = outfitUnlockedAtLevel(level);
      const sceneUnlock = sceneUnlockedAtLevel(level);

      if (sceneUnlock !== undefined) {
        revealNotifications.push({
          id: `scene-unlock-${level}-${Date.now()}`,
          title: "New Scene!",
          body: sceneUnlock.name,
          level,
          reveal: {
            kind: "scene",
            name: sceneUnlock.name,
            image: sceneUnlock.image,
            filter: sceneUnlock.filter,
          },
        });
      }

      if (isPromotionLevel(level)) {
        const unlockedTitle = titleForLevelAndInventory(
          level,
          counts.inventory_item_ids,
        );

        revealNotifications.push({
          id: `promotion-${level}-${Date.now()}`,
          title: "New Title!",
          body: unlockedTitle.name,
          level,
          reveal: {
            kind: "title",
            name: unlockedTitle.name,
            description: unlockedTitle.description,
            flavor: unlockedTitle.flavor,
          },
        });
      }

      nextNotifications.push({
        id: `level-${level}-${Date.now()}`,
        title: "Level up!",
        body: `You reached Level ${level}.`,
        level,
      });

      const unlock = shopUnlock
        ? { title: "New item available!", body: shopUnlock.name }
        : outfitUnlock
          ? { title: "Wardrobe unlocked!", body: outfitUnlock.name }
          : null;

      if (unlock !== null) {
        nextNotifications.push({
          id: `unlock-${level}-${Date.now()}`,
          ...unlock,
          level,
        });
      }
    }

    notifications = [
      ...revealNotifications,
      ...notifications,
      ...nextNotifications,
    ];
  }

  function enqueueUnseenStoryNotifications(
    startLevel: number,
    endLevel: number,
    seenStoryEventIds: string[],
  ) {
    const alreadyQueuedStoryEventIds = new Set(
      [
        ...seenStoryEventIds,
        ...notifications.flatMap((notification) =>
          notification.storyEventId === undefined ? [] : [notification.storyEventId],
        ),
        ...(activeNotification?.storyEventId === undefined
          ? []
          : [activeNotification.storyEventId]),
      ],
    );
    const storyNotifications = storyEventsForLevelRange(startLevel, endLevel)
      .filter((storyEvent) => !alreadyQueuedStoryEventIds.has(storyEvent.id))
      .map((storyEvent) => storyNotification(storyEvent));

    if (storyNotifications.length === 0) {
      return;
    }

    notifications = [...notifications, ...storyNotifications];
  }

  function storyNotification(
    storyEvent: NotificationStoryEvent | MenuTutorialStoryEvent | PurchaseStoryEvent,
  ): GameNotification {
    return {
      id: `story-${storyEvent.id}-${Date.now()}`,
      title: storyEvent.title,
      body: storyEvent.body,
      level: storyEvent.minLevel,
      storyEventId: storyEvent.id,
      tutorialSteps: storyEvent.kind === "menu_tutorial" ? storyEvent.steps : undefined,
    };
  }

  function enqueueStoryNotification(
    storyEvent: NotificationStoryEvent | PurchaseStoryEvent,
    priority = false,
  ) {
    const isAlreadySeen = counts.seen_story_event_ids.includes(storyEvent.id);
    const isAlreadyQueued = notifications.some(
      (notification) => notification.storyEventId === storyEvent.id,
    );
    const isAlreadyActive =
      activeNotification?.storyEventId === storyEvent.id ||
      suspendedTutorialNotification?.storyEventId === storyEvent.id;

    if (isAlreadySeen || isAlreadyQueued || isAlreadyActive) {
      return;
    }

    const notification = storyNotification(storyEvent);
    notifications = priority
      ? [notification, ...notifications]
      : [...notifications, notification];
  }

  function markStoryNotificationSeen(notification: GameNotification) {
    if (notification.storyEventId === undefined) {
      return;
    }

    if (!counts.seen_story_event_ids.includes(notification.storyEventId)) {
      counts = {
        ...counts,
        seen_story_event_ids: [
          ...counts.seen_story_event_ids,
          notification.storyEventId,
        ],
      };
    }

    void invoke("mark_story_event_seen", { eventId: notification.storyEventId });
  }

  async function devAddInfluence(amount: number) {
    devMessage = "";

    try {
      const snapshot = await invoke<InputSnapshot>("dev_add_influence", { amount });
      applySnapshot(snapshot);
      devMessage = `Added ${formatNumber(amount)} Influence.`;
    } catch {
      devMessage = "Dev action failed.";
    }
  }

  function openNextNotification() {
    if (activeNotification !== null || notifications.length === 0) {
      return;
    }

    const [nextNotification, ...remainingNotifications] = notifications;
    activeNotification = nextNotification;
    activeTutorialStepIndex = 0;
    notifications = remainingNotifications;
    if (isTutorialNotification(nextNotification)) {
      if (counts.seen_story_event_ids.includes(hiredStoryEvent.id)) {
        activeTutorialStepIndex = 2;
      } else {
        queueStartingNotifications();
      }
    }
    if (!isTutorialNotification(nextNotification)) {
      markStoryNotificationSeen(nextNotification);
    }
  }

  function dismissNotification() {
    if (activeNotification !== null && isTutorialNotification(activeNotification)) {
      return;
    }

    if (notifications.length > 0) {
      const [nextNotification, ...remainingNotifications] = notifications;
      activeNotification = nextNotification;
      activeTutorialStepIndex = 0;
      if (!isTutorialNotification(nextNotification)) {
        markStoryNotificationSeen(nextNotification);
      }
      notifications = remainingNotifications;
      return;
    }

    if (suspendedTutorialNotification !== null) {
      activeNotification = suspendedTutorialNotification;
      activeTutorialStepIndex = suspendedTutorialStepIndex;
      suspendedTutorialNotification = null;
      suspendedTutorialStepIndex = 0;
      isTutorialNotificationReview = false;
      return;
    }

    activeNotification = null;
    isTutorialNotificationReview = false;
  }

  function isTutorialNotification(notification: GameNotification | null) {
    return notification?.tutorialSteps !== undefined;
  }

  function isTutorialActive() {
    return isTutorialNotification(activeNotification) || isTutorialNotificationReview;
  }

  function activeTutorialStep() {
    return activeNotification?.tutorialSteps?.[activeTutorialStepIndex] ?? null;
  }

  function activeTutorialBody() {
    return activeTutorialStep()?.body ?? activeNotification?.body ?? "";
  }

  function isTutorialTarget(target: MenuTutorialTarget) {
    return activeTutorialStep()?.target === target;
  }

  function completeTutorialStep(target: MenuTutorialTarget) {
    if (!isTutorialTarget(target) || activeNotification === null) {
      return;
    }

    const tutorialSteps = activeNotification.tutorialSteps ?? [];

    if (activeTutorialStepIndex < tutorialSteps.length - 1) {
      activeTutorialStepIndex += 1;
      return;
    }

    markStoryNotificationSeen(activeNotification);
    activeTutorialStepIndex = 0;
    isTutorialNotificationReview = false;

    if (notifications.length > 0) {
      const [nextNotification, ...remainingNotifications] = notifications;
      activeNotification = nextNotification;
      notifications = remainingNotifications;
      if (!isTutorialNotification(nextNotification)) {
        markStoryNotificationSeen(nextNotification);
      }
      return;
    }

    activeNotification = null;
  }

  function queueStartingNotifications() {
    const queuedIds = new Set([
      ...notifications.map((notification) => notification.id),
      activeNotification?.id ?? "",
    ]);
    const startingLevelId = "starting-level";

    notifications = [
      ...notifications,
      ...(queuedIds.has(startingLevelId)
        ? []
        : [
            {
              id: startingLevelId,
              title: "Level 1",
              body: "You begin your work in the kingdom at Level 1.",
              level: 1,
            },
          ]),
    ];
  }

  function handleTutorialAnywherePointerDown(event: PointerEvent) {
    if (event.button !== 0) {
      return;
    }

    const target = activeTutorialStep()?.target;

    if (target === "influence") {
      event.preventDefault();
      event.stopPropagation();
      recordTutorialInfluenceInput();
      return;
    }

    if (
      target === "scene" ||
      target === "details" ||
      target === "shop" ||
      target === "inventory" ||
      target === "settings" ||
      target === "anywhere"
    ) {
      event.preventDefault();
      event.stopPropagation();
      completeTutorialStep(target);
    }
  }

  function recordTutorialInfluenceInput() {
    if (!isTutorialTarget("influence")) {
      return;
    }

    tutorialInfluenceInputCount += 1;

    if (tutorialInfluenceInputCount >= 5) {
      tutorialInfluenceInputCount = 0;
      completeTutorialStep("influence");
    }
  }

  function handleTutorialKeydown(event: KeyboardEvent) {
    if (
      event.repeat ||
      !isTutorialTarget("influence") ||
      (event.target instanceof Element &&
        event.target.closest("input, textarea, [contenteditable='true']") !== null)
    ) {
      return;
    }

    recordTutorialInfluenceInput();
  }

  function handleNotificationButtonClick() {
    if (isTutorialTarget("notifications") && activeNotification !== null) {
      activeTutorialStepIndex += 1;
      suspendedTutorialNotification = activeNotification;
      suspendedTutorialStepIndex = activeTutorialStepIndex;
      isTutorialNotificationReview = true;
      activeNotification = null;
    }

    openNextNotification();
  }

  function levelProgressPercent() {
    const levelSpan = counts.xp_for_next_level - counts.xp_for_current_level;

    if (levelSpan <= 0) {
      return 0;
    }

    return Math.min(
      100,
      Math.max(0, ((counts.xp - counts.xp_for_current_level) / levelSpan) * 100),
    );
  }

  function levelProgressText() {
    return `${formatNumber(counts.xp)} / ${formatNumber(counts.xp_for_next_level)} XP`;
  }

  function formatNumber(value: number) {
    return numberFormatter.format(value);
  }

  function hasBeenHired() {
    return counts.level >= hiredStoryEvent.minLevel;
  }

  function unlockedOutfits() {
    return outfitsForProgress(counts.level, hasBeenHired());
  }

  function equippedOutfit() {
    return (
      unlockedOutfits().find((outfit) => outfit.id === appSettings.selected_outfit_id) ??
      outfitForId("humble_rags")
    );
  }

  function currentTitle() {
    if (!hasBeenHired()) {
      return loyalSubjectTitle;
    }

    return titleForLevelAndInventory(counts.level, counts.inventory_item_ids);
  }

  function currentSceneSelection() {
    return sceneSelectionForProgress(
      counts.level,
      hasBeenHired(),
      ownsRoyalContract(),
      appSettings.selected_outfit_id,
    );
  }

  function toggleTitleDescription() {
    isTitleDescriptionOpen = !isTitleDescriptionOpen;
    scheduleWindowResize();
  }

  function kingdomTitle() {
    return counts.kingdom_name === null ? "Kingdom" : `Kingdom of ${counts.kingdom_name}`;
  }

  function syncWindowTitle(kingdomName: string | null) {
    const title = kingdomName === null ? "Kingdom" : `Kingdom of ${kingdomName}`;

    if (title === appliedWindowTitle) {
      return;
    }

    appliedWindowTitle = title;
    document.title = title;
    void getCurrentWindow().setTitle(title);
  }

  function updateKingdomNameInput(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const sanitized = input.value
      .replace(/[^A-Za-z0-9 '\-]/g, "")
      .slice(0, maxKingdomNameLength);

    kingdomNameInput = sanitized;
    kingdomNameError = "";

    if (input.value !== sanitized) {
      input.value = sanitized;
    }
  }

  async function saveKingdomName(event: SubmitEvent) {
    event.preventDefault();

    if (isSavingKingdomName) {
      return;
    }

    const normalizedName = kingdomNameInput.trim().replace(/\s+/g, " ");

    if (normalizedName.length === 0) {
      kingdomNameError = "Enter a kingdom name.";
      return;
    }

    isSavingKingdomName = true;
    kingdomNameError = "";

    try {
      const snapshot = await invoke<InputSnapshot>("set_kingdom_name", {
        name: normalizedName,
      });
      kingdomNameInput = snapshot.kingdom_name ?? normalizedName;
      resetMessage = "";
      closeAllMenus();
      applySnapshot(snapshot, { syncLevelTracker: true });
      scheduleWindowResize();
    } catch (error) {
      kingdomNameError =
        typeof error === "string" ? error : "The kingdom name could not be saved.";
    } finally {
      isSavingKingdomName = false;
    }
  }

  function isUnlocked(item: ShopItem) {
    return counts.level >= item.requiredLevel;
  }

  function closeShopState() {
    purchaseMessage = "";
    confirmingPurchaseId = null;
    highlightedShopItemIds = new Set();
  }

  function closeAllMenus() {
    if (isShopOpen) {
      closeShopState();
    }

    isDetailsOpen = false;
    isShopOpen = false;
    isInventoryOpen = false;
    isSettingsOpen = false;
    isDevToolsOpen = false;
  }

  function toggleDetails() {
    const shouldOpen = !isDetailsOpen;
    closeAllMenus();
    isDetailsOpen = shouldOpen;
    scheduleWindowResize();
  }

  function toggleShop() {
    const shouldOpen = !isShopOpen;
    closeAllMenus();
    isShopOpen = shouldOpen;

    if (isShopOpen) {
      highlightedShopItemIds = new Set(unseenAvailableShopItems().map((item) => item.id));
    } else {
      closeShopState();
    }

    scheduleWindowResize();
  }

  function toggleInventory() {
    const shouldOpen = !isInventoryOpen;
    closeAllMenus();
    isInventoryOpen = shouldOpen;
    scheduleWindowResize();
  }

  function toggleSettings() {
    const shouldOpen = !isSettingsOpen;
    closeAllMenus();
    isSettingsOpen = shouldOpen;
    scheduleWindowResize();
  }

  function toggleDevTools() {
    const shouldOpen = !isDevToolsOpen;
    closeAllMenus();
    isDevToolsOpen = shouldOpen;
    scheduleWindowResize();
  }

  async function saveAppSettings(nextSettings: AppSettings) {
    const previousSettings = appSettings;

    settingsMessage = "";
    appSettings = nextSettings;

    try {
      appSettings = await invoke<AppSettings>("update_app_settings", {
        settings: nextSettings,
      });
    } catch {
      appSettings = previousSettings;
      settingsMessage = "Setting update failed.";
    }
  }

  async function updateSetting(
    key: "run_on_startup" | "always_on_top" | "show_taskbar_icon" | "dev_mode",
    value: boolean,
  ) {
    if (key === "dev_mode" && !value) {
      isDevToolsOpen = false;
      devMessage = "";
    }

    await saveAppSettings({
      ...appSettings,
      [key]: value,
    });
  }

  async function equipOutfit(outfitId: string) {
    if (
      appSettings.selected_outfit_id === outfitId ||
      !unlockedOutfits().some((outfit) => outfit.id === outfitId)
    ) {
      return;
    }

    await saveAppSettings({
      ...appSettings,
      selected_outfit_id: outfitId,
    });
  }

  function currentCharacterAppearance(): CharacterAppearance {
    return {
      eyes: appSettings.character_eyes,
      nose: appSettings.character_nose,
      mouth: appSettings.character_mouth,
      hair: appSettings.character_hair,
      hairColor: appSettings.character_hair_color,
      skinTone: appSettings.character_skin_tone,
    };
  }

  function selectCharacterFeature(
    feature: keyof CharacterAppearance,
    value: string,
  ) {
    characterDraft = {
      ...characterDraft,
      [feature]: value,
    };
    characterCreationError = "";
  }

  async function saveCharacter() {
    if (isSavingCharacter) {
      return;
    }

    isSavingCharacter = true;
    characterCreationError = "";

    const nextSettings: AppSettings = {
      ...appSettings,
      character_created: true,
      character_eyes: characterDraft.eyes,
      character_nose: characterDraft.nose,
      character_mouth: characterDraft.mouth,
      character_hair: characterDraft.hair,
      character_hair_color: characterDraft.hairColor,
      character_skin_tone: characterDraft.skinTone,
    };

    try {
      appSettings = await invoke<AppSettings>("update_app_settings", {
        settings: nextSettings,
      });
      enqueueUnseenStoryNotifications(
        1,
        counts.level,
        counts.seen_story_event_ids,
      );
      openNextNotification();
      scheduleWindowResize();
    } catch {
      characterCreationError = "Your character could not be saved.";
    } finally {
      isSavingCharacter = false;
    }
  }

  function availableShopItems() {
    return shopItems.filter(
      (item) =>
        isUnlocked(item) &&
        !isOwned(item.id) &&
        (item.id === royalContractId || ownsRoyalContract()),
    );
  }

  function unseenAvailableShopItems() {
    return availableShopItems().filter(
      (item) => !counts.seen_shop_item_ids.includes(item.id),
    );
  }

  function hasNewShopItems() {
    return unseenAvailableShopItems().length > 0;
  }

  function isHighlightedShopItem(itemId: string) {
    return highlightedShopItemIds.has(itemId);
  }

  function syncHighlightedShopItems() {
    if (!isShopOpen) {
      return;
    }

    highlightedShopItemIds = new Set([
      ...highlightedShopItemIds,
      ...unseenAvailableShopItems().map((item) => item.id),
    ]);
  }

  function observeShopItem(node: HTMLElement, itemId: string) {
    const observer = new IntersectionObserver(
      (entries) => {
        const entry = entries[0];

        if (entry?.isIntersecting && entry.intersectionRatio >= 1) {
          markShopItemSeen(itemId);
          observer.disconnect();
        }
      },
      {
        root: panelElement,
        threshold: 1,
      },
    );

    observer.observe(node);

    return {
      destroy() {
        observer.disconnect();
      },
    };
  }

  function markShopItemSeen(itemId: string) {
    if (counts.seen_shop_item_ids.includes(itemId)) {
      return;
    }

    counts = {
      ...counts,
      seen_shop_item_ids: [...counts.seen_shop_item_ids, itemId],
    };
    void invoke("mark_shop_item_seen", { itemId });
  }

  function ownedInventoryItems() {
    return shopItems.filter((item) => isOwned(item.id));
  }

  function requestPurchaseConfirmation(item: ShopItem) {
    if (!isUnlocked(item) || isOwned(item.id) || pendingPurchaseId !== null) {
      return;
    }

    purchaseMessage = "";
    resetMessage = "";
    confirmingPurchaseId = item.id;
  }

  async function confirmPurchase(item: ShopItem) {
    if (!isUnlocked(item) || isOwned(item.id) || pendingPurchaseId !== null) {
      return;
    }

    pendingPurchaseId = item.id;
    confirmingPurchaseId = null;
    purchaseMessage = "";

    try {
      const result = await invoke<PurchaseResult>("purchase_shop_item", {
        itemId: item.id,
      });

      applySnapshot(result.snapshot);

      if (
        result.status === "purchased" &&
        item.id === royalContractStoryEvent.itemId
      ) {
        enqueueStoryNotification(royalContractStoryEvent, true);
      }

      purchaseMessage =
        result.status === "purchased"
          ? `${item.name} purchased.`
          : result.status === "not_enough_influence"
            ? "Not enough Influence."
            : result.status === "already_owned"
              ? "Already purchased."
              : result.status === "locked"
                ? "Level too low."
                : "Item unavailable.";
    } catch {
      purchaseMessage = "Purchase failed.";
    } finally {
      pendingPurchaseId = null;
    }
  }

  function cancelPurchase() {
    confirmingPurchaseId = null;
  }

  async function requestResetConfirmation() {
    purchaseMessage = "";
    resetMessage = "";
    isConfirmingReset = true;
    await tick();
    panelElement?.scrollTo({
      top: panelElement.scrollHeight,
      behavior: "smooth",
    });
  }

  async function confirmReset() {
    if (isResetting) {
      return;
    }

    isResetting = true;
    isConfirmingReset = false;
    purchaseMessage = "";
    resetMessage = "";

    try {
      const snapshot = await invoke<InputSnapshot>("reset_progress");
      appSettings = await invoke<AppSettings>("reset_app_settings");
      characterDraft = { ...defaultCharacterAppearance };
      applySnapshot(snapshot, { syncLevelTracker: true });
      confirmingPurchaseId = null;
      resetFrontendRunState();
      closeAllMenus();
      resetMessage = "Progress reset.";
      kingdomNameInput = "";
      kingdomNameError = "";
      scheduleWindowResize();
    } catch {
      resetMessage = "Reset failed.";
    } finally {
      isResetting = false;
    }
  }

  function cancelReset() {
    isConfirmingReset = false;
  }

  async function exitApp() {
    await invoke("exit_app");
  }

  async function startWindowDrag(event: PointerEvent) {
    rememberPointerPosition(event);

    if (event.button !== 0) {
      return;
    }

    if (isInteractiveTarget(event.target) || isScrollbarInteraction(event)) {
      return;
    }

    await getCurrentWindow().startDragging();
  }

  function rememberPointerPosition(event: PointerEvent) {
    if (!panelElement) {
      return;
    }

    const bounds = panelElement.getBoundingClientRect();
    lastPointerX = Math.max(0, Math.min(100, ((event.clientX - bounds.left) / bounds.width) * 100));
    lastPointerY = Math.max(0, Math.min(100, ((event.clientY - bounds.top) / bounds.height) * 100));
    lastPointerAtMillis = Date.now();
  }

  function suppressContextMenu(event: MouseEvent) {
    event.preventDefault();
  }

  function isInteractiveTarget(target: EventTarget | null) {
    return (
      target instanceof Element &&
      target.closest("button, a, input, select, textarea, [role='button']") !== null
    );
  }

  function isScrollbarInteraction(event: PointerEvent) {
    const scrollContainer = event.currentTarget;
    if (!(scrollContainer instanceof HTMLElement)) {
      return false;
    }

    const bounds = scrollContainer.getBoundingClientRect();
    const scrollbarSize = 12;
    const isOnVerticalScrollbar =
      scrollContainer.scrollHeight > scrollContainer.clientHeight &&
      event.clientX >= bounds.right - scrollbarSize;
    const isOnHorizontalScrollbar =
      scrollContainer.scrollWidth > scrollContainer.clientWidth &&
      event.clientY >= bounds.bottom - scrollbarSize;

    return isOnVerticalScrollbar || isOnHorizontalScrollbar;
  }

  function scheduleWindowResize() {
    if (resizeFrameId !== undefined) {
      window.cancelAnimationFrame(resizeFrameId);
    }

    resizeFrameId = window.requestAnimationFrame(() => {
      void resizeWindowToPanel();
    });
  }

  async function resizeWindowToPanel() {
    await tick();

    if (!panelElement) {
      return;
    }

    const isOnboarding =
      counts.kingdom_name === null ||
      (areSettingsLoaded && !appSettings.character_created);
    const width = Math.ceil(panelElement.offsetWidth);
    const maximumHeight = isOnboarding
      ? 560
      : isDetailsOpen
        ? Number.POSITIVE_INFINITY
        : 560;
    const height = Math.min(
      maximumHeight,
      Math.max(120, Math.ceil(panelElement.scrollHeight)),
    );

    if (Math.abs(width - lastWindowWidth) < 2 && Math.abs(height - lastWindowHeight) < 2) {
      return;
    }

    lastWindowWidth = width;
    lastWindowHeight = height;

    await getCurrentWindow().setSize(new LogicalSize(width, height));
  }

  onMount(() => {
    void getCurrentWindow().setResizable(false);

    void invoke<AppSettings>("get_app_settings").then((settings) => {
      appSettings = settings;
      characterDraft = currentCharacterAppearance();
      areSettingsLoaded = true;
      scheduleWindowResize();
    });

    const stopFocusedInputBridge = startFocusedInputBridge();
    const stopVisualUpdateLoop = startVisualUpdateLoop({
      readSnapshot: () => invoke<InputSnapshot>("get_input_counts"),
      onFrame: (frame) => {
        applyVisualFrame(frame);
      },
      onError: () => {},
    });
    const resizeObserver = new ResizeObserver(scheduleWindowResize);
    window.addEventListener("pointerdown", handleTutorialAnywherePointerDown, true);
    window.addEventListener("keydown", handleTutorialKeydown, true);

    if (panelElement) {
      resizeObserver.observe(panelElement);
      scheduleWindowResize();
    }

    return () => {
      stopFocusedInputBridge();
      stopVisualUpdateLoop();
      resizeObserver.disconnect();
      window.removeEventListener("pointerdown", handleTutorialAnywherePointerDown, true);
      window.removeEventListener("keydown", handleTutorialKeydown, true);

      if (resizeFrameId !== undefined) {
        window.cancelAnimationFrame(resizeFrameId);
      }

      if (powerProcTimeoutId !== undefined) {
        window.clearTimeout(powerProcTimeoutId);
      }
    };
  });
</script>

<main>
  <section
    bind:this={panelElement}
    class="panel"
    class:kingdom-onboarding={counts.kingdom_name === null || (areSettingsLoaded && !appSettings.character_created)}
    class:details-open={isDetailsOpen}
    class:settings-open={isSettingsOpen}
    aria-label="Input progress"
    onpointerdown={startWindowDrag}
    oncontextmenu={suppressContextMenu}
  >
    {#if counts.kingdom_name === null}
      <section class="kingdom-intro" aria-label="Name your kingdom">
        <p class="kingdom-intro-eyebrow">{kingdomNamingStoryEvent.eyebrow}</p>
        <h1>{kingdomNamingStoryEvent.title}</h1>
        <p>{kingdomNamingStoryEvent.body}</p>
        <form onsubmit={saveKingdomName}>
          <label for="kingdom-name">{kingdomNamingStoryEvent.inputLabel}</label>
          <input
            id="kingdom-name"
            type="text"
            value={kingdomNameInput}
            maxlength={maxKingdomNameLength}
            pattern="[A-Za-z0-9 '\-]+"
            autocomplete="off"
            spellcheck="false"
            oninput={updateKingdomNameInput}
            aria-describedby="kingdom-name-help"
          />
          <small id="kingdom-name-help">{kingdomNamingStoryEvent.inputHelp}</small>
          {#if kingdomNameError}
            <p class="kingdom-name-error">{kingdomNameError}</p>
          {/if}
          <button type="submit" disabled={isSavingKingdomName}>
            {isSavingKingdomName
              ? kingdomNamingStoryEvent.submittingLabel
              : kingdomNamingStoryEvent.submitLabel}
          </button>
        </form>
      </section>
    {:else if areSettingsLoaded && !appSettings.character_created}
      <section class="character-creator" aria-label="Create your character">
        <div class="character-creator-heading">
          <p class="kingdom-intro-eyebrow">Royal Personnel Record</p>
          <h1>Create Your Character</h1>
          <p>Choose the face that will represent the Kingdom of {counts.kingdom_name}.</p>
        </div>

        <div class="character-creator-layout">
          <div class="character-preview" aria-label="Character preview">
            <CharacterAvatar appearance={characterDraft} />
          </div>

          <div class="character-controls">
            <fieldset>
              <legend>Eyes</legend>
              <div class="choice-row">
                {#each eyeOptions as option}
                  <button
                    type="button"
                    class="feature-choice"
                    class:chosen={characterDraft.eyes === option.id}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("eyes", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
            <fieldset>
              <legend>Nose</legend>
              <div class="choice-row">
                {#each noseOptions as option}
                  <button
                    type="button"
                    class="feature-choice"
                    class:chosen={characterDraft.nose === option.id}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("nose", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
            <fieldset>
              <legend>Mouth</legend>
              <div class="choice-row">
                {#each mouthOptions as option}
                  <button
                    type="button"
                    class="feature-choice"
                    class:chosen={characterDraft.mouth === option.id}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("mouth", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
            <fieldset>
              <legend>Hair</legend>
              <div class="choice-row">
                {#each hairOptions as option}
                  <button
                    type="button"
                    class="feature-choice"
                    class:chosen={characterDraft.hair === option.id}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("hair", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
            <fieldset>
              <legend>Hair Color</legend>
              <div class="swatch-row">
                {#each hairColorOptions as option}
                  <button
                    type="button"
                    class:chosen={characterDraft.hairColor === option.id}
                    style={`--swatch: ${option.color}`}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("hairColor", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
            <fieldset>
              <legend>Skin Tone</legend>
              <div class="swatch-row">
                {#each skinToneOptions as option}
                  <button
                    type="button"
                    class:chosen={characterDraft.skinTone === option.id}
                    style={`--swatch: ${option.color}`}
                    aria-label={option.name}
                    title={option.name}
                    onclick={() => selectCharacterFeature("skinTone", option.id)}
                  ></button>
                {/each}
              </div>
            </fieldset>
          </div>
        </div>

        {#if characterCreationError}
          <p class="kingdom-name-error">{characterCreationError}</p>
        {/if}
        <button class="finish-character" type="button" disabled={isSavingCharacter} onclick={saveCharacter}>
          {isSavingCharacter ? "Saving..." : "Begin Your Rise"}
        </button>
      </section>
    {:else if areSettingsLoaded}
    {#if isTutorialActive()}
      <div class="tutorial-dimmer" aria-hidden="true"></div>
    {/if}
    {#if isPowerProcVisible}
      <div
        class={`power-proc-visual proc-${powerProcVisualFamily}`}
        style={`--proc-hue: ${powerProcVisualHue}; --proc-intensity: ${powerProcVisualIntensity}; --origin-x: ${powerProcOriginX}%; --origin-y: ${powerProcOriginY}%`}
        aria-hidden="true"
      >
        <span></span>
        <span></span>
        <span></span>
        <div
          class="proc-burst"
        >
          {#each procParticles as particle}
            <i
              style={`--burst-x: ${particle.x}px; --burst-y: ${particle.y}px; --burst-delay: ${particle.delay}ms; --burst-size: ${particle.size}px; --burst-rotation: ${particle.rotation}deg`}
            ></i>
          {/each}
        </div>
      </div>
    {/if}
    <div class="titlebar">
      <p>{kingdomTitle()}</p>
      <div class="title-actions">
        {#if notifications.length > 0}
          <button
            class="notification-button"
            class:tutorial-target={isTutorialTarget("notifications")}
            type="button"
            aria-label={`${notifications.length} notification${notifications.length === 1 ? "" : "s"} ready`}
            onclick={handleNotificationButtonClick}
          >
            !
          </button>
        {/if}
        <button class="exit-button" type="button" aria-label="Exit" onclick={exitApp}>Exit</button>
      </div>
    </div>

    <div
      class="title-rank"
      class:title-reveal-active={activeNotification?.reveal?.kind === "title"}
    >
      <div class="title-rank-row">
        <button
          class="title-name"
          type="button"
          aria-label={`${isTitleDescriptionOpen ? "Hide" : "Show"} information about ${currentTitle().name}`}
          aria-expanded={isTitleDescriptionOpen}
          aria-controls="title-description"
          onclick={toggleTitleDescription}
        >
          <p>{currentTitle().name}</p>
          <span class="title-tooltip" role="tooltip">{titleHoverText(currentTitle())}</span>
        </button>
      </div>
      {#if isTitleDescriptionOpen}
        <p id="title-description" class="title-description">
          {currentTitle().description}
        </p>
      {/if}
    </div>

    <div
      class="influence-summary"
      class:tutorial-target={isTutorialTarget("influence")}
    >
      <span class="influence-label-row">
        <span class="label">Influence</span>
        {#if isPowerProcVisible}
          <span
            class="power-proc-dot"
            style={`--proc-hue: ${powerProcVisualHue}`}
            aria-label={`Power upgrade triggered for ${formatNumber(powerProcAmount)} Influence`}
          >
            +{formatNumber(powerProcAmount)}
          </span>
        {/if}
      </span>
      <span class="influence-row">
        <span class="influence" class:compact={counts.influence >= 1_000_000}>
          {formatNumber(counts.influence)}
        </span>
      </span>
    </div>

    <div
      class="counter"
      class:tutorial-target={isTutorialTarget("scene")}
      class:scene-reveal-active={activeNotification?.reveal?.kind === "scene"}
    >
      <CharacterScene
        selection={currentSceneSelection()}
        appearance={currentCharacterAppearance()}
      />
      {#if activeNotification?.reveal}
        <section
          class="notification-popup unlock-reveal-popup scene-unlock-overlay"
          class:scene-unlock-popup={activeNotification.reveal.kind === "scene"}
          class:title-unlock-popup={activeNotification.reveal.kind === "title"}
          aria-label="Unlock notification"
        >
          {#if activeNotification.reveal.kind === "scene"}
            <div class="unlock-reveal-heading">
              <span>New Scene</span>
              <strong>{activeNotification.reveal.name}</strong>
            </div>
            <div class="scene-unlock-preview" aria-hidden="true">
              <img
                src={activeNotification.reveal.image}
                alt=""
                style:filter={activeNotification.reveal.filter}
              />
              <i></i>
            </div>
            <p class="unlock-reveal-copy">Your surroundings have been upgraded.</p>
          {:else}
            <div class="unlock-reveal-heading">
              <span>New Title</span>
              <strong>{activeNotification.reveal.name}</strong>
            </div>
            <p class="title-reveal-description">{activeNotification.reveal.description}</p>
            <p class="title-reveal-flavor">{activeNotification.reveal.flavor}</p>
          {/if}
          <button type="button" onclick={dismissNotification}>
            {notifications.length > 0 ? "Next" : "Close"}
          </button>
        </section>
      {/if}
      <div class="level-progress" aria-label="Level progress">
        <div class="progress-summary">
          <span>Level {counts.level}</span>
        </div>
        <div class="level-meter">
          <span style={`width: ${levelProgressPercent()}%`}></span>
        </div>
        <span class="xp-tooltip" role="tooltip">{levelProgressText()}</span>
      </div>
    </div>

    {#if activeNotification && activeNotification.reveal === undefined}
      <section
        class="notification-popup"
        class:tutorial-popup={isTutorialNotification(activeNotification)}
        class:tutorial-notification-review={isTutorialNotificationReview}
        class:tutorial-popup-below-influence={isTutorialTarget("influence")}
        class:tutorial-popup-scene={isTutorialTarget("scene")}
        aria-label="Game notification"
      >
        <p class="notification-title">{activeNotification.title}</p>
        <p>{activeTutorialBody()}</p>
        {#if isTutorialTarget("influence")}
          <p class="tutorial-input-progress">
            {tutorialInfluenceInputCount} / 5 inputs
          </p>
        {/if}
        {#if isTutorialNotification(activeNotification)}
          <p class="tutorial-progress">
            Step {activeTutorialStepIndex + 1} of {activeNotification.tutorialSteps?.length}
          </p>
        {:else}
          <button type="button" onclick={dismissNotification}>
            {notifications.length > 0 ? "Next" : "Close"}
          </button>
        {/if}
      </section>
    {/if}

    <div class="actions">
      <button
        type="button"
        aria-expanded={isDetailsOpen}
        aria-controls="details"
        class:active-menu={isDetailsOpen}
        class:tutorial-target={isTutorialTarget("details")}
        onclick={toggleDetails}
      >
        Details
      </button>

      <button
        class="menu-shop"
        type="button"
        aria-expanded={isShopOpen}
        aria-controls="shop-list"
        class:active-menu={isShopOpen}
        class:new-items={hasNewShopItems()}
        class:tutorial-target={isTutorialTarget("shop")}
        onclick={toggleShop}
      >
        Shop
      </button>

      <button
        class="menu-inventory"
        type="button"
        aria-expanded={isInventoryOpen}
        aria-controls="inventory"
        class:active-menu={isInventoryOpen}
        class:tutorial-target={isTutorialTarget("inventory")}
        onclick={toggleInventory}
      >
        Inventory
      </button>

      <button
        type="button"
        aria-expanded={isSettingsOpen}
        aria-controls="settings"
        class:active-menu={isSettingsOpen}
        class:tutorial-target={isTutorialTarget("settings")}
        onclick={toggleSettings}
      >
        Settings
      </button>

      {#if appSettings.dev_mode}
        <button
          type="button"
          aria-expanded={isDevToolsOpen}
          aria-controls="dev-tools"
          class:active-menu={isDevToolsOpen}
          onclick={toggleDevTools}
        >
          Dev
        </button>
      {/if}
    </div>

    {#if isDevToolsOpen}
      <section id="dev-tools" class="dev-tools" aria-label="Developer tools">
        <button type="button" onclick={() => devAddInfluence(100)}>+100 Influence</button>
        <button type="button" onclick={() => devAddInfluence(1000)}>+1,000 Influence</button>
        <button type="button" onclick={() => devAddInfluence(10000)}>+10,000 Influence</button>
        <button type="button" onclick={() => devAddInfluence(100000)}>
          +100,000 Influence
        </button>
        <button type="button" onclick={() => devAddInfluence(1000000)}>
          +1,000,000 Influence
        </button>
        {#if devMessage}
          <p>{devMessage}</p>
        {/if}
      </section>
    {/if}

    {#if isDetailsOpen}
      <div id="details" class="details">
        <dl>
          <div>
            <dt>Keys</dt>
            <dd>{formatNumber(counts.keys)}</dd>
          </div>
          <div>
            <dt>Clicks</dt>
            <dd>{formatNumber(counts.clicks)}</dd>
          </div>
          <div>
            <dt>
              <button class="stat-label" type="button">
                Perks
                <span class="stat-tooltip" role="tooltip">
                  Influence you've earned by going above and beyond for the kingdom
                </span>
              </button>
            </dt>
            <dd>{formatNumber(counts.bonus_influence)}</dd>
          </div>
          <div>
            <dt>
              <button class="stat-label" type="button">
                Spent
                <span class="stat-tooltip" role="tooltip">
                  Influence spent in the kingdom shop
                </span>
              </button>
            </dt>
            <dd>{formatNumber(counts.spent_influence)}</dd>
          </div>
        </dl>
      </div>
    {/if}

    {#if isShopOpen}
      <section
        id="shop-list"
        class="shop"
        aria-label="Shop"
      >
        {#if availableShopItems().length > 0}
          <ul>
            {#each availableShopItems() as item (item.id)}
              <li
                class:new-shop-item={isHighlightedShopItem(item.id)}
                use:observeShopItem={item.id}
              >
                <button
                  class="shop-item"
                  type="button"
                  disabled={pendingPurchaseId !== null}
                  onclick={() => requestPurchaseConfirmation(item)}
                >
                  <span>{item.name}</span>
                  <span>
                    {#if pendingPurchaseId === item.id}
                      Purchasing
                    {:else}
                      {formatNumber(item.cost)} Influence
                    {/if}
                  </span>
                  <small>{item.effect}</small>
                </button>
                {#if confirmingPurchaseId === item.id}
                  <div class="confirm-purchase" aria-label="Confirm purchase">
                    <p>Spend {formatNumber(item.cost)} Influence?</p>
                    <div>
                      <button type="button" onclick={() => confirmPurchase(item)}>Confirm</button>
                      <button type="button" onclick={cancelPurchase}>Cancel</button>
                    </div>
                  </div>
                {/if}
              </li>
            {/each}
          </ul>
        {:else}
          <p class="empty-panel-message">No available shop items.</p>
        {/if}
      </section>
    {/if}

    {#if isInventoryOpen}
      <section
        id="inventory"
        class="inventory"
        aria-label="Inventory"
      >
        <section class="wardrobe" aria-label="Wardrobe">
          <div class="wardrobe-heading">
            <p>Wardrobe</p>
            <small>Equipped: {equippedOutfit().name}</small>
          </div>
          <ul class="wardrobe-list">
            {#each unlockedOutfits() as outfit}
              <li class:equipped-outfit={appSettings.selected_outfit_id === outfit.id}>
                <button
                  class="wardrobe-item"
                  type="button"
                  aria-label={`${appSettings.selected_outfit_id === outfit.id ? "Equipped" : "Equip"} ${outfit.name}`}
                  aria-pressed={appSettings.selected_outfit_id === outfit.id}
                  onclick={() => equipOutfit(outfit.id)}
                >
                  <span class="outfit-preview" aria-hidden="true">
                    <img
                      src={outfit.image}
                      style:filter={outfit.filter ?? "none"}
                      alt=""
                    />
                  </span>
                  <span class="inventory-icon-tooltip" role="tooltip">
                    <strong>{outfit.name}</strong>
                    {#if appSettings.selected_outfit_id === outfit.id}
                      <small>Equipped</small>
                    {:else}
                      <small>Click to equip</small>
                    {/if}
                  </span>
                </button>
              </li>
            {/each}
          </ul>
        </section>

        <section class="items-category" aria-label="Items">
          <div class="inventory-category-heading">
            <p>Items</p>
            <small>Purchased upgrades and royal documents.</small>
          </div>
          {#if ownedInventoryItems().length > 0}
            <ul class="item-list">
              {#each ownedInventoryItems() as item}
                {@const upgradeDetails = powerUpgradeDetails(item)}
                <li class="inventory-item-card">
                  <button class="inventory-item-button" type="button" aria-label={item.name}>
                    <span class="item-icon" aria-hidden="true">
                      <img src={inventoryImageForItem(item)} alt="" />
                    </span>
                    <span class="inventory-icon-tooltip item-tooltip" role="tooltip">
                      <strong>{item.name}</strong>
                      {#if upgradeDetails}
                        <span class="inventory-effect-details">
                          <span>
                            <b>Proc Chance</b>
                            <small>{upgradeDetails.procChance}</small>
                          </span>
                          <span>
                            <b>Effect</b>
                            <small>{upgradeDetails.effect}</small>
                          </span>
                          <span>
                            <b>Value</b>
                            <small>{formatNumber(upgradeDetails.value)} Influence</small>
                          </span>
                        </span>
                      {:else}
                        <small>{item.inventoryDescription ?? item.effect}</small>
                      {/if}
                    </span>
                  </button>
                </li>
              {/each}
            </ul>
          {:else}
            <p class="empty-panel-message">No items owned.</p>
          {/if}
        </section>
      </section>
    {/if}

    {#if isSettingsOpen}
      <section
        id="settings"
        class="settings"
        aria-label="Settings"
      >
        <label class="settings-toggle">
          <span>
            <strong>Run on startup</strong>
            <small>Launch the kingdom when you sign in.</small>
          </span>
          <input
            type="checkbox"
            checked={appSettings.run_on_startup}
            onchange={(event) =>
              updateSetting("run_on_startup", event.currentTarget.checked)}
          />
        </label>

        <label class="settings-toggle">
          <span>
            <strong>Always on top</strong>
            <small>Keep the window above other windows.</small>
          </span>
          <input
            type="checkbox"
            checked={appSettings.always_on_top}
            onchange={(event) =>
              updateSetting("always_on_top", event.currentTarget.checked)}
          />
        </label>

        <label class="settings-toggle">
          <span>
            <strong>Show taskbar icon</strong>
            <small>Display the kingdom in the Windows taskbar.</small>
          </span>
          <input
            type="checkbox"
            checked={appSettings.show_taskbar_icon}
            onchange={(event) =>
              updateSetting("show_taskbar_icon", event.currentTarget.checked)}
          />
        </label>

        <label class="settings-toggle">
          <span>
            <strong>Dev mode</strong>
            <small>Show testing tools in the main menu.</small>
          </span>
          <input
            type="checkbox"
            checked={appSettings.dev_mode}
            onchange={(event) => updateSetting("dev_mode", event.currentTarget.checked)}
          />
        </label>

        {#if settingsMessage}
          <p class="settings-message">{settingsMessage}</p>
        {/if}

        <button
          class="reset-toggle"
          type="button"
          disabled={isResetting}
          onclick={requestResetConfirmation}
        >
          {isResetting ? "Resetting" : "Reset Progress"}
        </button>

        {#if isConfirmingReset}
          <section class="confirm-reset" aria-label="Confirm reset">
            <p>Reset all progress and settings?</p>
            <div>
              <button type="button" onclick={confirmReset}>Confirm</button>
              <button type="button" onclick={cancelReset}>Cancel</button>
            </div>
          </section>
        {/if}

        {#if resetMessage}
          <p class="reset-message">{resetMessage}</p>
        {/if}
      </section>
    {/if}

    {#if purchaseMessage}
      <p class="purchase-message">{purchaseMessage}</p>
    {/if}
    {/if}

  </section>
</main>

<style>
  :global(html),
  :global(body) {
    margin: 0;
    width: 100%;
    min-width: 0;
    min-height: 0;
    font-family:
      Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI",
      sans-serif;
    color: #f4f0e8;
    background: transparent;
    user-select: none;
  }

  :global(body) {
    overflow: hidden;
  }

  :global(button) {
    user-select: none;
  }

  main {
    width: fit-content;
    min-height: 0;
    padding: 0;
    box-sizing: border-box;
    background: transparent;
  }

  .panel {
    position: relative;
    width: 260px;
    max-height: 560px;
    overflow: auto;
    border: 1px solid rgba(244, 240, 232, 0.14);
    border-radius: 8px;
    padding: 10px;
    box-sizing: border-box;
    background: rgba(18, 18, 18, 0.72);
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.28);
    -webkit-backdrop-filter: blur(10px);
    backdrop-filter: blur(10px);
    scrollbar-color: rgba(200, 192, 178, 0.42) rgba(18, 18, 18, 0.28);
    scrollbar-width: thin;
  }

  .panel.details-open {
    max-height: none;
    overflow: hidden;
  }

  .panel.kingdom-onboarding {
    display: grid;
    width: 260px;
    max-height: 560px;
    overflow: auto;
    padding: 0;
  }

  .kingdom-intro {
    display: grid;
    align-content: start;
    gap: 10px;
    padding: 20px 16px;
    background:
      radial-gradient(circle at 50% 0%, rgba(143, 96, 190, 0.24), transparent 52%),
      linear-gradient(160deg, rgba(45, 38, 28, 0.96), rgba(18, 18, 18, 0.98));
  }

  .kingdom-intro-eyebrow {
    margin: 0;
    color: #d7c9a1;
    font-size: 0.62rem;
    font-weight: 900;
    letter-spacing: 0.1em;
    line-height: 1.35;
    text-transform: uppercase;
  }

  .kingdom-intro h1 {
    margin: 0;
    color: #f7e7af;
    font-size: 1.35rem;
    line-height: 1.1;
    text-shadow: 0 0 18px rgba(255, 217, 87, 0.2);
  }

  .kingdom-intro > p:not(.kingdom-intro-eyebrow) {
    margin: 0;
    color: #d8d1c6;
    font-size: 0.78rem;
    font-weight: 650;
    line-height: 1.4;
  }

  .kingdom-intro form {
    display: grid;
    gap: 7px;
    margin-top: 2px;
  }

  .kingdom-intro label {
    color: #f4f0e8;
    font-size: 0.8rem;
    font-weight: 900;
  }

  .kingdom-intro input {
    min-width: 0;
    height: 36px;
    border: 1px solid rgba(215, 201, 161, 0.42);
    border-radius: 6px;
    padding: 0 11px;
    color: #f4f0e8;
    background: rgba(12, 12, 12, 0.74);
    font: inherit;
    font-size: 0.88rem;
    font-weight: 800;
    outline: none;
  }

  .kingdom-intro input:focus {
    border-color: rgba(255, 217, 87, 0.78);
    box-shadow: 0 0 0 3px rgba(255, 217, 87, 0.12);
  }

  .kingdom-intro small {
    color: #aaa296;
    font-size: 0.64rem;
    font-weight: 700;
    line-height: 1.3;
  }

  .kingdom-name-error {
    margin: 0;
    color: #e0b2b8;
    font-size: 0.72rem;
    font-weight: 800;
  }

  .kingdom-intro button {
    min-height: 36px;
    margin-top: 3px;
    border: 1px solid rgba(255, 217, 87, 0.72);
    border-radius: 6px;
    color: #211805;
    background: #ffd957;
    font: inherit;
    font-size: 0.82rem;
    font-weight: 900;
    cursor: pointer;
  }

  .kingdom-intro button:hover:not(:disabled) {
    background: #ffe37b;
  }

  .kingdom-intro button:disabled {
    cursor: default;
    opacity: 0.68;
  }

  .panel::-webkit-scrollbar {
    width: 8px;
  }

  .panel::-webkit-scrollbar-track {
    background: rgba(18, 18, 18, 0.28);
    border-radius: 999px;
  }

  .panel::-webkit-scrollbar-thumb {
    border: 2px solid rgba(18, 18, 18, 0.28);
    border-radius: 999px;
    background: rgba(200, 192, 178, 0.42);
  }

  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    min-height: 28px;
  }

  .titlebar p {
    margin: 0;
    color: #f7e7af;
    font-size: 0.86rem;
    font-weight: 950;
    text-shadow: 0 0 12px rgba(255, 217, 87, 0.22);
  }

  .title-actions {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .notification-button,
  .exit-button {
    min-width: 46px;
    min-height: 26px;
    border: 1px solid rgba(244, 240, 232, 0.13);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(33, 24, 25, 0.88);
    font: inherit;
    font-size: 0.72rem;
    font-weight: 800;
    cursor: pointer;
  }

  .notification-button {
    min-width: 30px;
    border-color: rgba(255, 218, 87, 0.78);
    color: #1f1704;
    background: #ffd957;
    box-shadow:
      0 0 0 2px rgba(255, 217, 87, 0.16),
      0 0 16px rgba(255, 217, 87, 0.42);
    animation: notification-pulse 1.4s ease-in-out infinite;
  }

  .notification-button:hover {
    background: #ffe37b;
    box-shadow:
      0 0 0 2px rgba(255, 227, 123, 0.2),
      0 0 20px rgba(255, 227, 123, 0.52);
  }

  @keyframes notification-pulse {
    0%,
    100% {
      transform: translateY(0);
    }

    50% {
      transform: translateY(-1px);
    }
  }

  .exit-button:hover {
    background: rgba(49, 34, 37, 0.92);
  }

  .counter {
    position: relative;
    isolation: isolate;
    display: grid;
    align-content: end;
    min-height: 120px;
    overflow: hidden;
    margin: 8px -4px 8px;
    border: 1px solid rgba(244, 240, 232, 0.18);
    border-radius: 8px;
    padding: 72px 10px 10px;
    box-shadow:
      inset 0 0 0 1px rgba(20, 14, 7, 0.4),
      0 5px 16px rgba(0, 0, 0, 0.2);
  }

  .counter.scene-reveal-active {
    border-color: rgba(255, 217, 87, 0.72);
    animation: scene-stage-reveal 1.15s cubic-bezier(0.2, 0.85, 0.25, 1);
    box-shadow:
      inset 0 0 0 1px rgba(255, 240, 168, 0.28),
      0 0 24px rgba(255, 217, 87, 0.38);
  }

  .influence-summary {
    position: relative;
    z-index: 4;
    display: grid;
    width: 100%;
    margin: 8px 0 0;
    border: 1px solid rgba(244, 240, 232, 0.12);
    border-radius: 6px;
    padding: 8px 9px;
    color: inherit;
    background: rgba(18, 18, 18, 0.44);
    font: inherit;
    text-align: left;
    box-sizing: border-box;
  }

  .label {
    margin: 0 0 4px;
    color: #b8b2a6;
    font-size: 0.68rem;
    font-weight: 700;
    letter-spacing: 0;
    text-transform: uppercase;
  }

  .influence-summary .label {
    color: #ffffff;
    font-size: 0.84rem;
    font-weight: 800;
    letter-spacing: 0.04em;
    text-shadow:
      0 2px 2px rgba(0, 0, 0, 0.85),
      0 0 8px rgba(255, 217, 87, 0.38);
  }

  .influence-label-row {
    display: flex;
    align-items: center;
    gap: 7px;
    min-height: 22px;
  }

  .influence {
    margin: 0;
    color: #fff8c9;
    font-size: 2.35rem;
    line-height: 1;
    font-weight: 800;
    text-shadow:
      0 3px 2px rgba(0, 0, 0, 0.86),
      0 0 10px rgba(255, 217, 87, 0.28);
  }

  .influence.compact {
    font-size: 1.9rem;
  }

  .influence-row {
    display: flex;
    align-items: center;
    min-height: 38px;
  }

  .power-proc-dot {
    display: inline-grid;
    place-items: center;
    min-width: 30px;
    height: 22px;
    border: 1px solid hsl(var(--proc-hue) 88% 68% / 0.68);
    border-radius: 999px;
    padding: 0 7px;
    color: hsl(var(--proc-hue) 100% 94%);
    background:
      radial-gradient(circle at 30% 28%, hsl(var(--proc-hue) 100% 92% / 0.72), transparent 28%),
      hsl(var(--proc-hue) 70% 28% / 0.92);
    box-shadow:
      0 0 0 2px hsl(var(--proc-hue) 90% 62% / 0.14),
      0 0 18px hsl(var(--proc-hue) 90% 58% / 0.48);
    font-size: 0.72rem;
    font-weight: 900;
    animation: power-proc-pop 1.6s ease both;
  }

  .power-proc-visual {
    position: fixed;
    z-index: 20;
    inset: 0;
    overflow: hidden;
    border-radius: inherit;
    pointer-events: none;
    opacity: calc(0.55 + var(--proc-intensity) * 0.35);
    mix-blend-mode: screen;
  }

  .power-proc-visual::before,
  .power-proc-visual::after,
  .power-proc-visual span {
    position: absolute;
    content: "";
  }

  .power-proc-visual > span {
    width: 11px;
    height: 11px;
    border-radius: 50%;
    background: hsl(var(--proc-hue) 95% 68% / 0.9);
    box-shadow: 0 0 14px hsl(var(--proc-hue) 100% 62% / 0.8);
    animation: proc-particle 1.6s ease-out both;
  }

  .power-proc-visual > span:nth-child(1) {
    left: 18%;
    top: 68%;
  }

  .power-proc-visual > span:nth-child(2) {
    left: 52%;
    top: 42%;
    animation-delay: 70ms;
  }

  .power-proc-visual > span:nth-child(3) {
    left: 79%;
    top: 74%;
    animation-delay: 140ms;
  }

  .proc-spark::before {
    inset: 24% 14%;
    background:
      linear-gradient(90deg, transparent 48%, hsl(var(--proc-hue) 100% 75% / 0.9) 50%, transparent 52%),
      linear-gradient(0deg, transparent 48%, hsl(var(--proc-hue) 100% 75% / 0.9) 50%, transparent 52%);
    animation: proc-flash 1.2s ease-out both;
  }

  .proc-burst {
    position: absolute;
    left: var(--origin-x);
    top: var(--origin-y);
    z-index: 2;
    width: 0;
    height: 0;
    pointer-events: none;
  }

  .proc-burst i {
    position: absolute;
    left: 0;
    top: 0;
    width: var(--burst-size);
    height: var(--burst-size);
    border-radius: 50%;
    background: hsl(var(--proc-hue) 100% 70% / 0.98);
    box-shadow: 0 0 12px hsl(var(--proc-hue) 100% 62% / 0.78);
    transform: translate(-50%, -50%);
    animation: proc-burst-particle 850ms cubic-bezier(0.11, 0.79, 0.24, 1) both;
    animation-delay: var(--burst-delay);
  }

  .proc-explosion::before {
    left: var(--origin-x);
    top: var(--origin-y);
    width: 150px;
    aspect-ratio: 1;
    border-radius: 50%;
    background: radial-gradient(circle, hsl(var(--proc-hue) 100% 74% / 0.88), hsl(calc(var(--proc-hue) + 30) 100% 54% / 0.38) 38%, transparent 64%);
    transform: translate(-50%, -50%);
    animation: proc-explosion 900ms ease-out both;
  }

  .proc-explosion .proc-burst i {
    border-radius: 28% 72% 42% 58%;
    background: hsl(calc(var(--proc-hue) + 28) 100% 62% / 0.98);
  }

  .proc-fire .proc-burst i {
    border-radius: 60% 40% 55% 45%;
    background: hsl(calc(var(--proc-hue) + 24) 100% 58% / 0.96);
    animation-name: proc-burst-flame;
  }

  .proc-lightning .proc-burst i {
    width: calc(var(--burst-size) * 0.65);
    height: calc(var(--burst-size) * 2.4);
    border-radius: 2px;
    clip-path: polygon(50% 0, 100% 0, 62% 44%, 100% 44%, 20% 100%, 42% 55%, 0 55%);
    animation-name: proc-burst-lightning;
  }

  .proc-water .proc-burst i,
  .proc-bubble .proc-burst i {
    border: 1px solid hsl(var(--proc-hue) 100% 88% / 0.84);
    background: hsl(var(--proc-hue) 100% 70% / 0.28);
    box-shadow:
      inset 0 0 8px hsl(var(--proc-hue) 100% 88% / 0.44),
      0 0 12px hsl(var(--proc-hue) 100% 62% / 0.45);
    animation-name: proc-burst-bubble;
  }

  .proc-confetti .proc-burst i {
    border-radius: 1px;
    background: hsl(var(--proc-hue) 100% 64%);
    box-shadow: none;
    animation-name: proc-burst-confetti;
  }

  .proc-stars .proc-burst i {
    clip-path: polygon(50% 0, 61% 35%, 98% 35%, 68% 57%, 79% 94%, 50% 72%, 21% 94%, 32% 57%, 2% 35%, 39% 35%);
    background: hsl(var(--proc-hue) 100% 82%);
    animation-name: proc-burst-stars;
  }

  .proc-shockwave::before,
  .proc-shockwave::after {
    left: var(--origin-x);
    top: var(--origin-y);
    width: 32px;
    aspect-ratio: 1;
    border: 3px solid hsl(var(--proc-hue) 100% 72% / 0.8);
    border-radius: 50%;
    transform: translate(-50%, -50%);
    animation: proc-shockwave 1.05s ease-out both;
  }

  .proc-shockwave::after {
    animation-delay: 120ms;
  }

  .proc-smoke .proc-burst i {
    background: hsl(var(--proc-hue) 12% 72% / 0.58);
    box-shadow: 0 0 16px hsl(var(--proc-hue) 16% 62% / 0.32);
    filter: blur(1px);
    animation-name: proc-burst-smoke;
  }

  .proc-snow .proc-burst i {
    border-radius: 0;
    background: transparent;
    box-shadow: none;
    color: hsl(var(--proc-hue) 100% 94%);
    animation-name: proc-burst-snow;
  }

  .proc-snow .proc-burst i::before {
    content: "+";
    position: absolute;
    inset: -4px;
    display: grid;
    place-items: center;
    text-shadow: 0 0 7px hsl(var(--proc-hue) 100% 82%);
  }

  .proc-leaves .proc-burst i {
    border-radius: 100% 0 100% 0;
    background: hsl(calc(var(--proc-hue) + 70) 72% 52% / 0.94);
    box-shadow: 0 0 8px hsl(calc(var(--proc-hue) + 70) 70% 42% / 0.42);
    animation-name: proc-burst-leaves;
  }

  .proc-glitch .proc-burst i {
    border-radius: 0;
    background: hsl(var(--proc-hue) 100% 60% / 0.9);
    animation-name: proc-burst-glitch;
  }

  .proc-prism .proc-burst i {
    border-radius: 3px;
    background: conic-gradient(
      hsl(var(--proc-hue) 100% 70%),
      hsl(calc(var(--proc-hue) + 90) 100% 72%),
      hsl(calc(var(--proc-hue) + 190) 100% 72%),
      hsl(var(--proc-hue) 100% 70%)
    );
    animation-name: proc-burst-prism;
  }

  .proc-void .proc-burst i {
    background: #07010f;
    border: 1px solid hsl(var(--proc-hue) 100% 62% / 0.7);
    animation-name: proc-burst-void;
  }

  .proc-runes .proc-burst i {
    border-radius: 2px;
    background: transparent;
    color: hsl(var(--proc-hue) 100% 78% / 0.96);
    box-shadow: none;
  }

  .proc-runes .proc-burst i::before {
    content: "*";
    position: absolute;
    inset: -5px;
    display: grid;
    place-items: center;
    font-size: 12px;
  }

  .proc-bubble::before {
    left: var(--origin-x);
    top: var(--origin-y);
    width: 118px;
    aspect-ratio: 1;
    border: 1px solid hsl(var(--proc-hue) 100% 82% / 0.48);
    border-radius: 50%;
    box-shadow: inset 0 0 24px hsl(var(--proc-hue) 100% 78% / 0.3);
    transform: translate(-50%, -50%);
    animation: proc-bubble-ring 1.1s ease-out both;
  }

  .proc-lightning::before {
    left: 43%;
    top: -12%;
    width: 18%;
    height: 125%;
    background: hsl(var(--proc-hue) 100% 78% / 0.88);
    clip-path: polygon(55% 0, 100% 0, 62% 43%, 88% 43%, 22% 100%, 43% 55%, 12% 55%);
    filter: drop-shadow(0 0 10px hsl(var(--proc-hue) 100% 65%));
    animation: proc-lightning 1.15s steps(2, end) both;
  }

  .proc-fire::before {
    inset: 35% -10% -20%;
    background:
      radial-gradient(ellipse at 18% 100%, hsl(var(--proc-hue) 100% 58% / 0.85), transparent 46%),
      radial-gradient(ellipse at 52% 100%, hsl(calc(var(--proc-hue) + 28) 100% 62% / 0.9), transparent 50%),
      radial-gradient(ellipse at 84% 100%, hsl(var(--proc-hue) 100% 55% / 0.82), transparent 44%);
    animation: proc-fire 1.6s ease-out both;
  }

  .proc-water::before {
    left: -20%;
    right: -20%;
    bottom: -35%;
    height: 72%;
    border-radius: 45%;
    background: hsl(var(--proc-hue) 88% 58% / 0.5);
    box-shadow: 0 -12px 30px hsl(var(--proc-hue) 100% 72% / 0.55);
    animation: proc-water 1.6s ease-in-out both;
  }

  .proc-glitch::before,
  .proc-glitch::after {
    inset: 0;
    background: repeating-linear-gradient(
      0deg,
      transparent 0 8px,
      hsl(var(--proc-hue) 100% 62% / 0.28) 9px 11px
    );
    animation: proc-glitch 1.35s steps(5, end) both;
  }

  .proc-glitch::after {
    --proc-hue: 185;
    transform: translateX(5px);
    animation-delay: 45ms;
  }

  .proc-prism::before {
    inset: -40%;
    background: conic-gradient(
      from 20deg,
      transparent,
      hsl(var(--proc-hue) 100% 65% / 0.6),
      hsl(calc(var(--proc-hue) + 90) 100% 68% / 0.58),
      hsl(calc(var(--proc-hue) + 190) 100% 68% / 0.55),
      transparent
    );
    animation: proc-prism 1.7s ease-out both;
  }

  .proc-void::before {
    left: 50%;
    top: 50%;
    width: 170%;
    aspect-ratio: 1;
    border-radius: 50%;
    background: radial-gradient(circle, transparent 8%, #05020c 18%, hsl(var(--proc-hue) 90% 48% / 0.7) 24%, transparent 43%);
    transform: translate(-50%, -50%);
    animation: proc-void 1.6s ease-in-out both;
  }

  .proc-runes::before {
    inset: 12%;
    border: 2px dashed hsl(var(--proc-hue) 100% 72% / 0.78);
    border-radius: 50%;
    box-shadow:
      inset 0 0 24px hsl(var(--proc-hue) 100% 58% / 0.42),
      0 0 20px hsl(var(--proc-hue) 100% 58% / 0.48);
    animation: proc-runes 1.6s ease-out both;
  }

  @keyframes proc-particle {
    0% {
      opacity: 0;
      transform: translateY(16px) scale(0.4);
    }
    22% {
      opacity: 1;
    }
    100% {
      opacity: 0;
      transform: translateY(-86px) scale(1.35);
    }
  }

  @keyframes proc-burst-particle {
    0% {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.25) rotate(0deg);
    }
    16% {
      opacity: 1;
    }
    100% {
      opacity: 0;
      transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(0.78) rotate(var(--burst-rotation));
    }
  }

  @keyframes proc-burst-flame {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.3) rotate(0deg); }
    18% { opacity: 1; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 88px)) scale(1.2) rotate(var(--burst-rotation)); }
  }

  @keyframes proc-burst-lightning {
    0%, 100% { opacity: 0; transform: translate(-50%, -50%) scale(0.4) rotate(var(--burst-rotation)); }
    20%, 58% { opacity: 1; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(1.1) rotate(var(--burst-rotation)); }
  }

  @keyframes proc-burst-bubble {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.25); }
    24% { opacity: 0.9; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 95px)) scale(1.55); }
  }

  @keyframes proc-burst-confetti {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.2) rotate(0deg); }
    18% { opacity: 1; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) + 58px)) scale(0.9) rotate(calc(var(--burst-rotation) + 420deg)); }
  }

  @keyframes proc-burst-stars {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0) rotate(0deg); }
    26% { opacity: 1; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(1.25) rotate(80deg); }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(0.45) rotate(var(--burst-rotation)); }
  }

  @keyframes proc-shockwave {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.15); }
    18% { opacity: 1; }
    100% { opacity: 0; transform: translate(-50%, -50%) scale(6.8); }
  }

  @keyframes proc-burst-smoke {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.3); }
    24% { opacity: 0.82; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 82px)) scale(2.4); }
  }

  @keyframes proc-burst-snow {
    0% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 110px)) rotate(0deg); }
    20% { opacity: 1; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) + 74px)) rotate(calc(var(--burst-rotation) + 180deg)); }
  }

  @keyframes proc-burst-leaves {
    0% { opacity: 0; transform: translate(-50%, -50%) rotate(0deg) scale(0.3); }
    22% { opacity: 1; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) + 66px)) rotate(calc(var(--burst-rotation) + 360deg)) scale(1.1); }
  }

  @keyframes proc-burst-glitch {
    0%, 100% { opacity: 0; transform: translate(-50%, -50%) scale(0.8); }
    18% { opacity: 1; transform: translate(calc(var(--burst-x) - 62%), calc(var(--burst-y) - 50%)); }
    38% { transform: translate(calc(var(--burst-x) - 42%), calc(var(--burst-y) - 45%)); }
    62% { transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)); }
  }

  @keyframes proc-burst-prism {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.2) rotate(0deg); }
    22% { opacity: 1; }
    100% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(1.1) rotate(calc(var(--burst-rotation) + 240deg)); }
  }

  @keyframes proc-burst-void {
    0% { opacity: 0; transform: translate(calc(var(--burst-x) - 50%), calc(var(--burst-y) - 50%)) scale(1.2); }
    34% { opacity: 1; }
    100% { opacity: 0; transform: translate(-50%, -50%) scale(0.15); }
  }

  @keyframes proc-explosion {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.08); }
    28% { opacity: 1; transform: translate(-50%, -50%) scale(0.62); }
    100% { opacity: 0; transform: translate(-50%, -50%) scale(1.15); }
  }

  @keyframes proc-bubble-ring {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.12); }
    28% { opacity: 0.95; }
    100% { opacity: 0; transform: translate(-50%, -50%) scale(1.28); }
  }

  @keyframes proc-flash {
    0%, 100% { opacity: 0; transform: scale(0.45) rotate(0); }
    18% { opacity: 1; transform: scale(1.1) rotate(18deg); }
  }

  @keyframes proc-lightning {
    0%, 100% { opacity: 0; transform: translateX(-8px); }
    12%, 42% { opacity: 1; transform: translateX(5px); }
  }

  @keyframes proc-fire {
    0% { opacity: 0; transform: translateY(35%) scaleY(0.4); }
    25% { opacity: 1; }
    100% { opacity: 0; transform: translateY(-18%) scaleY(1.25); }
  }

  @keyframes proc-water {
    0%, 100% { opacity: 0; transform: translateY(45%) rotate(0); }
    32% { opacity: 1; transform: translateY(0) rotate(6deg); }
    70% { opacity: 0.75; transform: translateY(-8%) rotate(-5deg); }
  }

  @keyframes proc-glitch {
    0%, 100% { opacity: 0; clip-path: inset(0); }
    18% { opacity: 1; clip-path: inset(12% 0 55%); transform: translateX(-7px); }
    42% { clip-path: inset(58% 0 18%); transform: translateX(8px); }
    68% { clip-path: inset(35% 0 36%); transform: translateX(-3px); }
  }

  @keyframes proc-prism {
    0% { opacity: 0; transform: scale(0.35) rotate(-90deg); }
    28% { opacity: 1; }
    100% { opacity: 0; transform: scale(1.15) rotate(95deg); }
  }

  @keyframes proc-void {
    0%, 100% { opacity: 0; transform: translate(-50%, -50%) scale(0.15); }
    45% { opacity: 1; transform: translate(-50%, -50%) scale(0.82); }
  }

  @keyframes proc-runes {
    0% { opacity: 0; transform: scale(0.4) rotate(-90deg); }
    30% { opacity: 1; }
    100% { opacity: 0; transform: scale(1.12) rotate(120deg); }
  }

  @keyframes power-proc-pop {
    0% {
      opacity: 0;
      transform: scale(0.72) translateY(3px);
    }

    16% {
      opacity: 1;
      transform: scale(1.06) translateY(0);
    }

    70% {
      opacity: 1;
      transform: scale(1) translateY(0);
    }

    100% {
      opacity: 0;
      transform: scale(0.88) translateY(-2px);
    }
  }

  .title-rank {
    position: relative;
    z-index: 5;
    display: grid;
    gap: 6px;
    margin-top: 2px;
    max-width: 100%;
  }

  .title-rank.title-reveal-active .title-name > p {
    color: #fff0a8;
    text-shadow:
      0 0 8px rgba(255, 217, 87, 0.65),
      0 0 18px rgba(255, 217, 87, 0.32);
    animation: title-rank-reveal 1.1s cubic-bezier(0.2, 0.85, 0.25, 1);
  }

  .title-rank-row {
    position: relative;
    display: flex;
    align-items: center;
    gap: 6px;
    width: fit-content;
    max-width: 100%;
  }

  .title-name {
    position: relative;
    display: block;
    min-width: 0;
    border: 0;
    border-radius: 4px;
    padding: 2px 4px 2px 0;
    color: inherit;
    background: transparent;
    font: inherit;
    text-align: left;
    cursor: pointer;
    transition:
      color 120ms ease,
      background 120ms ease;
  }

  .title-name > p {
    margin: 0;
    color: #f4f0e8;
    font-size: 0.86rem;
    font-weight: 900;
  }

  .character-creator {
    display: grid;
    gap: 12px;
    padding: 18px;
    color: #f4efe5;
  }

  .character-creator-heading {
    text-align: center;
  }

  .character-creator-heading h1 {
    margin: 2px 0 4px;
    color: #ffd957;
    font-size: 1.24rem;
  }

  .character-creator-heading p {
    margin: 0;
    color: #c8c0b4;
    font-size: 0.72rem;
  }

  .character-creator-layout {
    display: grid;
    grid-template-columns: 128px minmax(0, 1fr);
    gap: 14px;
    align-items: center;
  }

  .character-preview {
    width: 128px;
    height: 185px;
    border: 1px solid rgba(255, 217, 87, 0.28);
    border-radius: 12px;
    background:
      radial-gradient(circle at 50% 34%, rgba(255, 217, 87, 0.12), transparent 43%),
      rgba(0, 0, 0, 0.2);
    filter: drop-shadow(0 5px 5px rgba(0, 0, 0, 0.3));
  }

  .character-controls {
    display: grid;
    gap: 7px;
  }

  .character-controls fieldset {
    min-width: 0;
    margin: 0;
    padding: 0;
    border: 0;
  }

  .character-controls legend {
    margin-bottom: 3px;
    color: #aaa296;
    font-size: 0.59rem;
    font-weight: 900;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .choice-row,
  .swatch-row {
    display: flex;
    gap: 4px;
  }

  .choice-row button {
    flex: 1;
    min-width: 0;
    min-height: 32px;
    padding: 3px;
    border: 1px solid rgba(255, 255, 255, 0.14);
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.04);
    cursor: pointer;
  }

  .choice-row button.chosen {
    border-color: #ffd957;
    background: rgba(255, 217, 87, 0.14);
    color: #fff4bf;
  }

  .swatch-row button {
    width: 28px;
    height: 24px;
    padding: 3px;
    border: 2px solid transparent;
    border-radius: 7px;
    background: var(--swatch);
    background-clip: content-box;
    cursor: pointer;
  }

  .swatch-row button.chosen {
    border-color: #ffd957;
  }

  .finish-character {
    min-height: 36px;
    border: 1px solid rgba(255, 217, 87, 0.72);
    border-radius: 8px;
    background: linear-gradient(#5d4917, #30230a);
    color: #fff4bf;
    font-weight: 900;
    cursor: pointer;
  }

  .finish-character:disabled {
    cursor: wait;
    opacity: 0.65;
  }

  .title-name:hover,
  .title-name:focus-visible {
    background: rgba(244, 240, 232, 0.08);
  }

  .title-name:hover > p,
  .title-name:focus-visible > p {
    color: #ffffff;
  }

  .title-name:focus-visible {
    outline: 1px solid rgba(215, 201, 161, 0.46);
    outline-offset: 1px;
  }

  .title-description {
    margin: 0;
    border-left: 2px solid rgba(215, 201, 161, 0.38);
    padding: 5px 7px;
    color: #c8c0b2;
    background: rgba(45, 38, 28, 0.42);
    font-size: 0.72rem;
    font-weight: 700;
    line-height: 1.35;
  }

  .title-tooltip {
    position: absolute;
    left: 0;
    top: calc(100% + 6px);
    z-index: 3;
    width: max-content;
    max-width: 220px;
    border: 1px solid rgba(215, 201, 161, 0.32);
    border-radius: 6px;
    padding: 7px 8px;
    color: #f4f0e8;
    background: rgba(45, 38, 28, 0.96);
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.32);
    font-size: 0.72rem;
    font-weight: 700;
    line-height: 1.25;
    opacity: 0;
    pointer-events: none;
    transform: translateY(-2px);
    transition:
      opacity 120ms ease,
      transform 120ms ease;
  }

  .title-name:hover .title-tooltip,
  .title-name:focus-visible .title-tooltip {
    opacity: 1;
    transform: translateY(0);
  }

  .progress-summary {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 7px;
    color: #d7c9a1;
    font-size: 0.75rem;
    font-weight: 800;
  }

  .level-progress {
    position: relative;
    z-index: 2;
    display: grid;
    width: 55%;
  }

  .level-meter {
    width: 100%;
    height: 5px;
    margin-top: 7px;
    overflow: hidden;
    border-radius: 999px;
    background: rgba(244, 240, 232, 0.13);
  }

  .level-meter span {
    display: block;
    height: 100%;
    border-radius: inherit;
    background: #d7c9a1;
  }

  .xp-tooltip {
    position: absolute;
    left: 0;
    bottom: calc(100% + 2px);
    z-index: 6;
    width: max-content;
    max-width: 220px;
    border: 1px solid rgba(215, 201, 161, 0.32);
    border-radius: 6px;
    padding: 7px 8px;
    color: #f4f0e8;
    background: rgba(45, 38, 28, 0.96);
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.32);
    font-size: 0.72rem;
    font-weight: 700;
    line-height: 1.25;
    opacity: 0;
    pointer-events: none;
    transform: translateY(2px);
    transition:
      opacity 120ms ease,
      transform 120ms ease;
  }

  .level-progress:hover .xp-tooltip {
    opacity: 1;
    transform: translateY(0);
  }

  .xp-tooltip::after {
    position: absolute;
    left: 10px;
    top: 100%;
    border: 4px solid transparent;
    border-top-color: rgba(45, 38, 28, 0.96);
    content: "";
  }

  .notification-popup {
    display: grid;
    gap: 8px;
    margin-bottom: 10px;
    border: 1px solid rgba(215, 201, 161, 0.32);
    border-radius: 6px;
    padding: 9px;
    background: rgba(45, 38, 28, 0.92);
  }

  .notification-popup p {
    margin: 0;
    color: #f4f0e8;
    font-size: 0.78rem;
    font-weight: 700;
  }

  .notification-popup .notification-title {
    color: #d7c9a1;
    font-size: 0.86rem;
    font-weight: 900;
  }

  .notification-popup.tutorial-popup {
    position: absolute;
    z-index: 23;
    top: 46px;
    right: 10px;
    left: 10px;
    margin: 0;
    border-color: rgba(255, 217, 87, 0.48);
    background: rgba(45, 38, 28, 0.98);
    box-shadow:
      0 0 0 2px rgba(255, 217, 87, 0.08),
      0 12px 30px rgba(0, 0, 0, 0.58),
      0 0 18px rgba(255, 217, 87, 0.2);
  }

  .notification-popup.tutorial-notification-review {
    position: relative;
    z-index: 23;
    border-color: rgba(255, 229, 126, 0.72);
    outline: 2px solid #fff0a8;
    outline-offset: 3px;
    box-shadow:
      0 0 0 4px rgba(255, 217, 87, 0.14),
      0 10px 28px rgba(0, 0, 0, 0.56),
      0 0 22px rgba(255, 217, 87, 0.52);
    animation: tutorial-target-pulse 1.1s ease-in-out infinite;
  }

  .notification-popup.unlock-reveal-popup {
    position: relative;
    overflow: hidden;
    gap: 10px;
    border-color: rgba(255, 217, 87, 0.58);
    padding: 12px;
    background:
      radial-gradient(circle at 50% 0%, rgba(255, 217, 87, 0.18), transparent 52%),
      linear-gradient(145deg, rgba(58, 47, 29, 0.98), rgba(28, 25, 21, 0.98));
    box-shadow:
      inset 0 0 0 1px rgba(255, 244, 196, 0.08),
      0 8px 22px rgba(0, 0, 0, 0.36),
      0 0 18px rgba(255, 217, 87, 0.18);
    animation: unlock-card-reveal 520ms cubic-bezier(0.16, 0.9, 0.24, 1);
  }

  .notification-popup.scene-unlock-overlay {
    position: absolute;
    z-index: 8;
    inset: 0;
    align-content: center;
    margin: 0;
    border-radius: inherit;
    background:
      radial-gradient(circle at 50% 0%, rgba(255, 217, 87, 0.2), transparent 52%),
      linear-gradient(145deg, rgba(58, 47, 29, 0.99), rgba(28, 25, 21, 0.99));
  }

  .scene-unlock-overlay .scene-unlock-preview {
    height: 72px;
  }

  .notification-popup.unlock-reveal-popup::before {
    position: absolute;
    top: -80%;
    left: -28%;
    width: 42%;
    height: 250%;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(255, 244, 196, 0.16),
      transparent
    );
    content: "";
    pointer-events: none;
    transform: rotate(20deg);
    animation: unlock-card-shine 1.15s 180ms ease-out both;
  }

  .unlock-reveal-heading {
    position: relative;
    z-index: 1;
    display: grid;
    gap: 3px;
    text-align: center;
  }

  .unlock-reveal-heading span {
    color: #d7c9a1;
    font-size: 0.64rem;
    font-weight: 900;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }

  .unlock-reveal-heading strong {
    color: #fff0a8;
    font-size: 1rem;
    line-height: 1.15;
    text-shadow: 0 0 14px rgba(255, 217, 87, 0.34);
  }

  .scene-unlock-preview {
    position: relative;
    z-index: 1;
    display: grid;
    place-items: center;
    height: 92px;
    overflow: hidden;
    border: 1px solid rgba(255, 240, 168, 0.22);
    border-radius: 7px;
    background: rgba(244, 240, 232, 0.05);
  }

  .scene-unlock-preview img {
    position: relative;
    z-index: 1;
    width: 100%;
    height: 100%;
    object-fit: contain;
    animation: scene-preview-arrive 850ms cubic-bezier(0.16, 0.9, 0.24, 1);
  }

  .scene-unlock-preview i {
    position: absolute;
    width: 130px;
    height: 130px;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(255, 217, 87, 0.25), transparent 68%);
    animation: scene-preview-glow 1.5s ease-in-out infinite;
  }

  .notification-popup .unlock-reveal-copy,
  .notification-popup .title-reveal-description,
  .notification-popup .title-reveal-flavor {
    position: relative;
    z-index: 1;
    text-align: center;
  }

  .notification-popup .unlock-reveal-copy,
  .notification-popup .title-reveal-description {
    color: #f4f0e8;
    font-size: 0.76rem;
  }

  .notification-popup .title-reveal-flavor {
    border-top: 1px solid rgba(215, 201, 161, 0.18);
    padding-top: 8px;
    color: #d7c9a1;
    font-size: 0.72rem;
    font-style: italic;
    font-weight: 700;
  }

  .notification-popup.tutorial-popup.tutorial-popup-below-influence {
    top: 196px;
  }

  .notification-popup.tutorial-popup.tutorial-popup-scene {
    top: 8px;
    gap: 4px;
    padding: 6px 8px;
  }

  .notification-popup .tutorial-progress {
    color: #aaa296;
    font-size: 0.68rem;
    font-weight: 800;
    text-transform: uppercase;
  }

  .notification-popup .tutorial-input-progress {
    color: #fff0a8;
    font-size: 0.7rem;
    font-weight: 900;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .notification-popup button {
    min-height: 30px;
    border: 1px solid rgba(244, 240, 232, 0.14);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(24, 24, 24, 0.84);
    font: inherit;
    font-size: 0.75rem;
    font-weight: 800;
    cursor: pointer;
  }

  .notification-popup button:hover {
    background: rgba(48, 43, 37, 0.94);
  }

  .actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .actions button,
  .reset-toggle {
    min-height: 34px;
    border: 1px solid rgba(244, 240, 232, 0.14);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(36, 33, 29, 0.86);
    font: inherit;
    font-size: 0.78rem;
    font-weight: 800;
    cursor: pointer;
  }

  .actions button:hover,
  .reset-toggle:hover:not(:disabled) {
    background: rgba(48, 43, 37, 0.92);
  }

  .actions .menu-shop {
    border-color: rgba(175, 128, 224, 0.58);
    color: #f4eaff;
    background: rgba(84, 45, 126, 0.9);
    box-shadow: 0 0 0 2px rgba(175, 128, 224, 0.1);
  }

  .actions .menu-shop:hover {
    background: rgba(104, 57, 154, 0.96);
    box-shadow: 0 0 0 2px rgba(194, 147, 238, 0.15);
  }

  .actions .menu-shop.new-items {
    border-color: rgba(255, 217, 87, 0.72);
    color: #231806;
    background: #ffd957;
    box-shadow:
      0 0 0 2px rgba(255, 217, 87, 0.12),
      0 0 14px rgba(255, 217, 87, 0.28);
  }

  .actions .menu-shop.new-items:hover {
    background: #ffe37b;
    box-shadow:
      0 0 0 2px rgba(255, 227, 123, 0.16),
      0 0 18px rgba(255, 227, 123, 0.36);
  }

  .actions .menu-inventory {
    border-color: rgba(111, 198, 181, 0.46);
    color: #dffaf4;
    background: rgba(27, 77, 70, 0.86);
    box-shadow: 0 0 0 2px rgba(111, 198, 181, 0.08);
  }

  .actions .menu-inventory:hover {
    background: rgba(34, 93, 85, 0.94);
  }

  .actions button.active-menu {
    border-color: rgba(215, 201, 161, 0.46);
    color: #1f1a12;
    background: #d7c9a1;
    box-shadow: 0 0 0 2px rgba(215, 201, 161, 0.13);
  }

  .actions .menu-shop.active-menu {
    border-color: rgba(202, 164, 238, 0.74);
    color: #1d1028;
    background: #c89bea;
  }

  .actions .menu-shop.active-menu.new-items {
    border-color: rgba(255, 235, 152, 0.84);
    color: #1f1704;
    background: #ffe37b;
  }

  .actions .menu-inventory.active-menu {
    border-color: rgba(150, 226, 211, 0.62);
    color: #10221f;
    background: #8bd9ca;
  }

  .actions button.active-menu:hover {
    background: #e3d6b3;
  }

  .actions .menu-shop.active-menu:not(.new-items):hover {
    background: #d6aff4;
    box-shadow: 0 0 0 2px rgba(202, 164, 238, 0.18);
  }

  .actions .menu-inventory.active-menu:hover {
    background: #9de6d9;
    box-shadow: 0 0 0 2px rgba(150, 226, 211, 0.16);
  }

  .tutorial-dimmer {
    position: absolute;
    z-index: 20;
    inset: 0;
    border-radius: inherit;
    background: rgba(0, 0, 0, 0.7);
    cursor: default;
  }

  .tutorial-target {
    position: relative;
    z-index: 22;
    outline: 2px solid #fff0a8;
    outline-offset: 3px;
    background-color: rgba(30, 27, 22, 0.98);
    box-shadow:
      0 0 0 4px rgba(255, 217, 87, 0.14),
      0 0 20px rgba(255, 217, 87, 0.58);
    animation: tutorial-target-pulse 1.1s ease-in-out infinite;
  }

  .notification-button.tutorial-target {
    color: #1f1704;
    background: #ffd957;
    font-size: 0.82rem;
    line-height: 1;
  }

  .details.tutorial-target,
  .shop.tutorial-target,
  .inventory.tutorial-target,
  .settings.tutorial-target {
    border-radius: 8px;
  }

  .tutorial-step-action {
    position: absolute;
    z-index: 24;
    inset: 0;
    width: 100%;
    height: 100%;
    border: 0;
    padding: 0;
    background: transparent;
    cursor: pointer;
  }

  @keyframes tutorial-target-pulse {
    0%,
    100% {
      outline-color: #ffe57e;
      filter: brightness(1);
    }

    50% {
      outline-color: #fff8d5;
      filter: brightness(1.24);
    }
  }

  @keyframes unlock-card-reveal {
    0% {
      opacity: 0;
      transform: translateY(12px) scale(0.94);
    }

    70% {
      opacity: 1;
      transform: translateY(-1px) scale(1.015);
    }

    100% {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes unlock-card-shine {
    0% {
      opacity: 0;
      transform: translateX(0) rotate(20deg);
    }

    30% {
      opacity: 1;
    }

    100% {
      opacity: 0;
      transform: translateX(360px) rotate(20deg);
    }
  }

  @keyframes scene-stage-reveal {
    0% {
      opacity: 0.35;
      transform: scale(0.965);
      filter: brightness(1.7);
    }

    55% {
      opacity: 1;
      transform: scale(1.012);
      filter: brightness(1.15);
    }

    100% {
      transform: scale(1);
      filter: brightness(1);
    }
  }

  @keyframes scene-preview-arrive {
    0% {
      opacity: 0;
      transform: scale(0.78);
      filter: brightness(1.8);
    }

    100% {
      opacity: 1;
      transform: scale(1);
      filter: brightness(1);
    }
  }

  @keyframes scene-preview-glow {
    0%,
    100% {
      opacity: 0.45;
      transform: scale(0.82);
    }

    50% {
      opacity: 1;
      transform: scale(1.08);
    }
  }

  @keyframes title-rank-reveal {
    0% {
      opacity: 0;
      transform: translateY(5px) scale(0.92);
    }

    65% {
      opacity: 1;
      transform: translateY(-1px) scale(1.04);
    }

    100% {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .details {
    margin-top: 10px;
  }

  .dev-tools {
    display: grid;
    gap: 8px;
    margin-top: 10px;
    border: 1px dashed rgba(255, 217, 87, 0.36);
    border-radius: 6px;
    padding: 8px;
    background: rgba(51, 41, 17, 0.62);
  }

  .dev-tools button {
    min-height: 30px;
    border: 1px solid rgba(255, 217, 87, 0.26);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(36, 33, 29, 0.86);
    font: inherit;
    font-size: 0.74rem;
    font-weight: 800;
    cursor: pointer;
  }

  .dev-tools button:hover {
    background: rgba(54, 45, 24, 0.94);
  }

  .dev-tools p {
    margin: 0;
    color: #d7c9a1;
    font-size: 0.72rem;
    font-weight: 700;
  }

  dl {
    display: grid;
    gap: 6px;
    margin: 0;
  }

  dl div {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    border-top: 1px solid rgba(244, 240, 232, 0.12);
    padding-top: 6px;
  }

  dt {
    color: #c8c0b2;
    font-size: 0.8rem;
    font-weight: 700;
  }

  .stat-label {
    position: relative;
    border: 0;
    padding: 0;
    color: inherit;
    background: transparent;
    font: inherit;
    font-weight: inherit;
    text-align: left;
    cursor: pointer;
    outline: none;
  }

  .stat-tooltip {
    position: absolute;
    left: 0;
    bottom: calc(100% + 6px);
    z-index: 4;
    width: max-content;
    max-width: 220px;
    border: 1px solid rgba(215, 201, 161, 0.32);
    border-radius: 6px;
    padding: 7px 8px;
    color: #f4f0e8;
    background: rgba(45, 38, 28, 0.96);
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.32);
    font-size: 0.72rem;
    font-weight: 700;
    line-height: 1.25;
    opacity: 0;
    pointer-events: none;
    transform: translateY(2px);
    transition:
      opacity 120ms ease,
      transform 120ms ease;
  }

  .stat-label:hover .stat-tooltip,
  .stat-label:focus .stat-tooltip {
    opacity: 1;
    transform: translateY(0);
  }

  dd {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 800;
  }

  .shop,
  .inventory,
  .settings {
    width: 100%;
    margin-top: 10px;
  }

  .inventory,
  .settings {
    display: grid;
    gap: 10px;
  }

  .inventory li {
    display: grid;
    gap: 3px;
    padding: 9px;
  }

  .inventory li small {
    color: #aaa296;
    font-size: 0.7rem;
    font-weight: 700;
    line-height: 1.25;
  }

  .wardrobe,
  .items-category {
    display: grid;
    gap: 8px;
  }

  .wardrobe-heading,
  .inventory-category-heading {
    display: grid;
    gap: 2px;
  }

  .wardrobe-heading p,
  .inventory-category-heading p {
    margin: 0;
    color: #f4f0e8;
    font-size: 0.82rem;
    font-weight: 950;
  }

  .wardrobe-heading small,
  .inventory-category-heading small {
    color: #b9ad87;
    font-size: 0.68rem;
    font-weight: 750;
  }

  .items-category {
    border-top: 1px solid rgba(244, 240, 232, 0.12);
    padding-top: 10px;
  }

  .item-list {
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 6px;
  }

  .inventory .item-list .inventory-item-card {
    position: relative;
    min-width: 0;
    padding: 0;
  }

  .inventory-item-button {
    display: grid;
    place-items: center;
    width: 100%;
    min-height: 56px;
    border: 0;
    border-radius: inherit;
    padding: 4px;
    color: inherit;
    background: transparent;
    cursor: help;
  }

  .inventory-item-button:hover,
  .inventory-item-button:focus-visible {
    background: rgba(255, 255, 255, 0.04);
  }

  .item-icon {
    display: grid;
    place-items: center;
    width: 48px;
    height: 48px;
    border: 1px solid rgba(255, 217, 87, 0.16);
    border-radius: 8px;
    background:
      radial-gradient(circle, rgba(255, 217, 87, 0.1), transparent 68%),
      rgba(12, 11, 10, 0.3);
  }

  .item-icon img {
    width: 44px;
    height: 44px;
    object-fit: contain;
  }

  .wardrobe-list {
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 6px;
  }

  .inventory .wardrobe-list li {
    position: relative;
    padding: 0;
  }

  .inventory .wardrobe-list li.equipped-outfit {
    border-color: rgba(255, 217, 87, 0.68);
    box-shadow: 0 0 0 1px rgba(255, 217, 87, 0.12);
  }

  .wardrobe-item {
    display: grid;
    place-items: center;
    width: 100%;
    min-height: 52px;
    border: 0;
    border-radius: inherit;
    padding: 4px;
    color: inherit;
    background: transparent;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }

  .wardrobe-item:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .wardrobe-item[aria-pressed="true"] {
    background: rgba(255, 217, 87, 0.08);
    cursor: default;
  }

  .outfit-preview {
    display: grid;
    place-items: center;
    width: 42px;
    height: 42px;
  }

  .outfit-preview img {
    width: 42px;
    height: 42px;
    object-fit: contain;
    object-position: center;
  }

  .inventory-icon-tooltip {
    position: absolute;
    z-index: 30;
    left: 0;
    top: calc(100% + 7px);
    display: grid;
    width: 190px;
    gap: 3px;
    border: 1px solid rgba(215, 201, 161, 0.36);
    border-radius: 7px;
    padding: 8px;
    color: #f4f0e8;
    background: rgba(31, 27, 22, 0.98);
    box-shadow: 0 10px 24px rgba(0, 0, 0, 0.52);
    text-align: left;
    opacity: 0;
    pointer-events: none;
    transform: translateY(3px);
    transition:
      opacity 100ms ease,
      transform 100ms ease;
  }

  .wardrobe-list li:nth-child(4n + 3) .inventory-icon-tooltip,
  .wardrobe-list li:nth-child(4n + 4) .inventory-icon-tooltip,
  .item-list li:nth-child(4n + 3) .inventory-icon-tooltip,
  .item-list li:nth-child(4n + 4) .inventory-icon-tooltip {
    right: 0;
    left: auto;
  }

  .wardrobe-item:hover .inventory-icon-tooltip,
  .wardrobe-item:focus-visible .inventory-icon-tooltip,
  .inventory-item-button:hover .inventory-icon-tooltip,
  .inventory-item-button:focus-visible .inventory-icon-tooltip {
    opacity: 1;
    transform: translateY(0);
  }

  .inventory-icon-tooltip strong {
    margin: 0;
    color: #f4f0e8;
    font-size: 0.72rem;
    font-weight: 900;
    line-height: 1.15;
  }

  .inventory-icon-tooltip small {
    color: #aaa296;
    font-size: 0.64rem;
    font-weight: 700;
  }

  .item-tooltip {
    width: 210px;
  }

  .inventory-effect-details {
    display: grid;
    gap: 4px;
    margin: 3px 0 0;
  }

  .inventory-effect-details > span {
    display: grid;
    grid-template-columns: 62px minmax(0, 1fr);
    align-items: start;
    gap: 8px;
    border: 1px solid rgba(244, 240, 232, 0.1);
    border-radius: 4px;
    padding: 4px 5px;
    background: rgba(12, 11, 10, 0.32);
  }

  .inventory-effect-details b {
    margin: 1px 0 0;
    color: #8f887d;
    font-size: 0.55rem;
    font-weight: 900;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .inventory-effect-details small {
    margin: 0;
    color: #d8d1c5;
    font-size: 0.68rem;
    font-weight: 750;
    line-height: 1.25;
    overflow-wrap: anywhere;
  }

  .empty-panel-message {
    margin: 0;
    color: #8f887d;
    font-size: 0.74rem;
    font-weight: 700;
  }

  .settings-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    border: 1px solid rgba(244, 240, 232, 0.12);
    border-radius: 6px;
    padding: 9px;
    background: rgba(30, 28, 25, 0.82);
    cursor: pointer;
  }

  .settings-toggle span {
    display: grid;
    gap: 3px;
    min-width: 0;
  }

  .settings-toggle strong {
    color: #f4f0e8;
    font-size: 0.8rem;
    font-weight: 900;
  }

  .settings-toggle small {
    color: #aaa296;
    font-size: 0.7rem;
    font-weight: 700;
    line-height: 1.25;
  }

  .settings-toggle input {
    flex: 0 0 auto;
    width: 34px;
    height: 20px;
    margin: 0;
    appearance: none;
    border: 1px solid rgba(244, 240, 232, 0.16);
    border-radius: 999px;
    background: rgba(16, 16, 16, 0.86);
    cursor: pointer;
    transition:
      border-color 120ms ease,
      background 120ms ease;
  }

  .settings-toggle input::before {
    display: block;
    width: 14px;
    height: 14px;
    margin: 2px;
    border-radius: 999px;
    background: #8f887d;
    content: "";
    transition:
      transform 120ms ease,
      background 120ms ease;
  }

  .settings-toggle input:checked {
    border-color: rgba(215, 201, 161, 0.46);
    background: rgba(215, 201, 161, 0.22);
  }

  .settings-toggle input:checked::before {
    background: #d7c9a1;
    transform: translateX(14px);
  }

  .settings-toggle input:disabled {
    cursor: default;
    opacity: 0.62;
  }

  .settings-message {
    margin: 0;
    color: #e0b2b8;
    font-size: 0.75rem;
    font-weight: 700;
  }

  ul {
    display: grid;
    gap: 8px;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  li {
    border: 1px solid rgba(244, 240, 232, 0.12);
    border-radius: 6px;
    background: rgba(30, 28, 25, 0.82);
  }

  .shop li.new-shop-item {
    border-color: rgba(255, 217, 87, 0.82);
    box-shadow:
      0 0 0 2px rgba(255, 217, 87, 0.14),
      0 0 16px rgba(255, 217, 87, 0.3);
  }

  .shop-item {
    display: grid;
    width: 100%;
    gap: 4px;
    border: 0;
    border-radius: 6px;
    padding: 9px;
    color: inherit;
    background: transparent;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }

  .shop-item:hover:not(:disabled) {
    background: rgba(41, 37, 31, 0.92);
  }

  .shop-item:disabled {
    cursor: default;
    opacity: 0.72;
  }

  .shop-item span:first-child {
    font-size: 0.82rem;
    font-weight: 800;
  }

  .shop-item span:last-child {
    color: #c8c0b2;
    font-size: 0.75rem;
    font-weight: 700;
  }

  .shop-item small {
    color: #aaa296;
    font-size: 0.7rem;
    font-weight: 700;
    line-height: 1.25;
  }

  .confirm-purchase {
    display: grid;
    gap: 8px;
    border-top: 1px solid rgba(244, 240, 232, 0.12);
    padding: 9px;
    background: rgba(36, 33, 29, 0.86);
  }

  .confirm-purchase p {
    margin: 0;
    color: #d7c9a1;
    font-size: 0.78rem;
    font-weight: 800;
  }

  .confirm-purchase div {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .confirm-purchase button {
    min-height: 30px;
    border: 1px solid rgba(244, 240, 232, 0.14);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(24, 24, 24, 0.84);
    font: inherit;
    font-size: 0.75rem;
    font-weight: 800;
    cursor: pointer;
  }

  .confirm-purchase button:hover {
    background: rgba(48, 43, 37, 0.94);
  }

  .purchase-message {
    margin: 10px 0 0;
    color: #d7c9a1;
    font-size: 0.75rem;
    font-weight: 700;
  }

  .reset-toggle {
    width: 100%;
    margin-top: 8px;
    background: rgba(33, 24, 25, 0.84);
  }

  .reset-toggle:disabled {
    cursor: default;
    opacity: 0.72;
  }

  .confirm-reset {
    display: grid;
    gap: 8px;
    width: 100%;
    margin-top: 8px;
    border: 1px solid rgba(224, 178, 184, 0.22);
    border-radius: 6px;
    padding: 9px;
    box-sizing: border-box;
    background: rgba(33, 24, 25, 0.86);
  }

  .confirm-reset p {
    margin: 0;
    color: #e0b2b8;
    font-size: 0.78rem;
    font-weight: 800;
  }

  .confirm-reset div {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .confirm-reset button {
    min-height: 30px;
    border: 1px solid rgba(224, 178, 184, 0.24);
    border-radius: 6px;
    color: #f4f0e8;
    background: rgba(24, 24, 24, 0.84);
    font: inherit;
    font-size: 0.75rem;
    font-weight: 800;
    cursor: pointer;
  }

  .confirm-reset button:hover {
    background: rgba(48, 35, 38, 0.94);
  }

  .reset-message {
    margin: 8px 0 0;
    color: #e0b2b8;
    font-size: 0.75rem;
    font-weight: 700;
  }
</style>
