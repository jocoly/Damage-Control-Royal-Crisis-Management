export type LevelUnlockCategory = "wardrobe" | "item" | "scene";

export const WARDROBE_UNLOCK_LEVELS = Array.from(
  { length: 34 },
  (_, index) => index * 3 + 1,
);

export const ITEM_UNLOCK_LEVELS = Array.from(
  { length: 33 },
  (_, index) => index * 3 + 2,
);

export const SCENE_UNLOCK_LEVELS = Array.from(
  { length: 33 },
  (_, index) => index * 3 + 3,
);

export function unlockCategoryForLevel(level: number): LevelUnlockCategory {
  const normalizedLevel = Math.max(1, Math.min(100, Math.floor(level)));

  switch ((normalizedLevel - 1) % 3) {
    case 0:
      return "wardrobe";
    case 1:
      return "item";
    default:
      return "scene";
  }
}
