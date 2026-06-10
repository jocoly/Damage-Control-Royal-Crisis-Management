type StoryEventBase = {
  id: string;
  minLevel: number;
  title: string;
  body: string;
};

export type NotificationStoryEvent = StoryEventBase & {
  kind: "notification";
};

export type PurchaseStoryEvent = StoryEventBase & {
  kind: "purchase";
  itemId: string;
};

export type MenuTutorialTarget =
  | "anywhere"
  | "influence"
  | "scene"
  | "notifications"
  | "details"
  | "shop"
  | "inventory"
  | "settings";

export type MenuTutorialStep = {
  target: MenuTutorialTarget;
  body: string;
};

export type MenuTutorialStoryEvent = StoryEventBase & {
  kind: "menu_tutorial";
  steps: MenuTutorialStep[];
};

export type KingdomNamingStoryEvent = StoryEventBase & {
  kind: "kingdom_naming";
  eyebrow: string;
  inputLabel: string;
  inputHelp: string;
  submitLabel: string;
  submittingLabel: string;
};

export type StoryEvent =
  | NotificationStoryEvent
  | PurchaseStoryEvent
  | MenuTutorialStoryEvent
  | KingdomNamingStoryEvent;

export const kingdomNamingStoryEvent: KingdomNamingStoryEvent = {
  id: "rebrand_the_kingdom",
  kind: "kingdom_naming",
  minLevel: 1,
  eyebrow: "Rebrand the Kingdom",
  title: "Your First Day in the Royal Influence Office",
  body:
    "Your first task is simple: submit a stack of rebranding paperwork before the court notices it is late. On the top of the pile, you discover the NAME field on this very official form has been left blank. What will you call this kingdom?",
  inputLabel: "Kingdom of",
  inputHelp:
    "Up to 12 characters. Letters, numbers, spaces, apostrophes, and hyphens only.",
  submitLabel: "Rebrand Kingdom",
  submittingLabel: "Rebranding Kingdom",
};

export const hiredStoryEvent: NotificationStoryEvent = {
  id: "you_are_hired",
  kind: "notification",
  minLevel: 2,
  title: "You're Hired!",
  body:
    "Your appointment to the Royal Influence Office is now official. Your new title is Court Marketing Intern.",
};

export const royalContractStoryEvent: PurchaseStoryEvent = {
  id: "royal_contract_signed",
  kind: "purchase",
  itemId: "royal_contract",
  minLevel: 2,
  title: "Office Upgraded!",
  body:
    "The ink is barely dry, but the paperwork appears official enough. You leave the broom closet behind for a proper office and become Court Marketing Manager for the Royal Influence Office.",
};

export const storyEvents: StoryEvent[] = [
  kingdomNamingStoryEvent,
  royalContractStoryEvent,
  {
    id: "interactive_main_menu_tutorial",
    kind: "menu_tutorial",
    minLevel: 1,
    title: "A Quick Tour",
    body: "Click each highlighted button to inspect your new desk.",
    steps: [
      {
        target: "influence",
        body:
          "Influence rises with every mouse click or keypress. Click or press a key anywhere five times and watch the number grow.",
      },
      {
        target: "scene",
        body:
          "Your scene and character improve as you level. Click anywhere to proceed.",
      },
      {
        target: "notifications",
        body:
          "Notifications announce important updates such as new levels, titles, and story events. Two are waiting for you now. Click the highlighted notification button.",
      },
      {
        target: "details",
        body:
          "Details breaks down your keys, clicks, perks, and spending. Click anywhere to continue.",
      },
      {
        target: "shop",
        body:
          "Shop lets you spend Influence on upgrades to your marketing strategy. You'll unlock items as you level up. Click anywhere to continue.",
      },
      {
        target: "inventory",
        body:
          "Inventory holds purchased items and your wardrobe. Click anywhere to continue.",
      },
      {
        target: "settings",
        body:
          "Settings has customization options. Click anywhere to finish the tour.",
      },
    ],
  },
];

export function storyEventsForLevelRange(startLevel: number, endLevel: number) {
  return storyEvents.filter(
    (storyEvent): storyEvent is NotificationStoryEvent | MenuTutorialStoryEvent =>
      (storyEvent.kind === "notification" || storyEvent.kind === "menu_tutorial") &&
      storyEvent.minLevel >= startLevel &&
      storyEvent.minLevel <= endLevel,
  );
}
