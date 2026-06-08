export type LevelDefinition = {
  level: number;
  xpRequired: number;
  xpRequiredForNextLevel: number;
  xpToNextLevel: number;
};

export const MAX_CATALOG_LEVEL = 100;

function xpNeededToAdvanceFromLevel(level: number) {
  const earlyRequirements = [
    0,
    500,
    2_000,
    6_250,
    10_000,
    8_750,
    7_500,
    8_750,
    11_250,
    12_500,
    22_500,
    23_750,
    23_750,
    25_000,
    27_500,
    31_250,
    35_000,
  ];

  if (level < earlyRequirements.length) {
    return earlyRequirements[level];
  }

  let requirement = earlyRequirements[16];

  for (let completedLevel = 17; completedLevel < level; completedLevel += 1) {
    requirement = Math.floor((requirement * 108) / 100);
  }

  return requirement;
}

function calculateXpRequiredForLevel(level: number) {
  let xpRequired = 0;

  for (let completedLevel = 1; completedLevel < level; completedLevel += 1) {
    xpRequired += xpNeededToAdvanceFromLevel(completedLevel);
  }

  return xpRequired;
}

export const levels: LevelDefinition[] = Array.from(
  { length: MAX_CATALOG_LEVEL },
  (_, index) => {
    const level = index + 1;
    const xpRequired = calculateXpRequiredForLevel(level);
    const xpRequiredForNextLevel = calculateXpRequiredForLevel(level + 1);

    return {
      level,
      xpRequired,
      xpRequiredForNextLevel,
      xpToNextLevel: xpRequiredForNextLevel - xpRequired,
    };
  },
);

export function levelDefinition(level: number) {
  const normalizedLevel = Math.max(1, Math.floor(level));

  return (
    levels[normalizedLevel - 1] ?? {
      level: normalizedLevel,
      xpRequired: calculateXpRequiredForLevel(normalizedLevel),
      xpRequiredForNextLevel: calculateXpRequiredForLevel(normalizedLevel + 1),
      xpToNextLevel: xpNeededToAdvanceFromLevel(normalizedLevel),
    }
  );
}
