import {
  SCENE_UNLOCK_LEVELS,
  WARDROBE_UNLOCK_LEVELS,
} from "$lib/unlockCatalog";

export type SceneLayer = {
  id: string;
  name: string;
  image: string;
  filter?: string;
};

export type CharacterSceneSelection = {
  location: SceneLayer;
  outfit: OutfitLayer;
  companion: SceneLayer | null;
};

type SceneProgressionEntry = {
  minLevel: number;
  location: SceneLayer;
};

export type OutfitLayer = SceneLayer & {
  minLevel: number;
  description: string;
};

export const farmLocation: SceneLayer = {
  id: "farm",
  name: "The Farm",
  image: "/scene/farm-sprites.png?v=20260609-2",
};

const officeClosetLocation: SceneLayer = {
  id: "office_closet",
  name: "Office Supply Closet",
  image: "/scene/office-closet.png",
};

const sharedOfficeLocation: SceneLayer = {
  id: "shared_office",
  name: "Shared Communications Office",
  image: "/scene/office-shared.png",
};

const sceneNames = [
  "Desk by the Broom Cupboard",
  "Window-Side Clerk Desk",
  "Royal Mail Corner",
  "Junior Copy Room",
  "Pamphlet Planning Nook",
  "Town Crier Dispatch Desk",
  "Village Campaign Office",
  "Festival Promotion Room",
  "Audience Research Annex",
  "Royal Campaign Office",
  "Campaign Briefing Room",
  "Public Sentiment Studio",
  "Senior Strategist Office",
  "Court Press Gallery",
  "Director's Planning Suite",
  "Marketing Director Suite",
  "Executive Audience Salon",
  "Royal Broadcast Room",
  "Kingdom Messaging Hub",
  "Influence Command Room",
  "Realm Strategy Chamber",
  "Grand Campaign War Room",
  "Chancellor's Briefing Hall",
  "Crown Communications Suite",
  "Legendary Influence Chamber",
  "Aetherwave Control Gallery",
  "Royal Reputation Vault",
  "Grand Audience Observatory",
  "High Chancellor's Office",
  "Crown Strategy Sanctum",
  "Kingmaker Strategy Room",
  "Throne of Public Opinion",
  "Royal Influence Apex",
] as const;

const sceneImages = [
  "/scene/office-shared.png",
  "/scene/office-shared.png",
  "/scene/office-shared.png",
  "/scene/office-shared.png",
  "/scene/office-shared.png",
  "/scene/office-campaign.png",
  "/scene/office-campaign.png",
  "/scene/office-campaign.png",
  "/scene/office-campaign.png",
  "/scene/office-campaign.png",
  "/scene/office-director.png",
  "/scene/office-director.png",
  "/scene/office-director.png",
  "/scene/office-director.png",
  "/scene/office-director.png",
  "/scene/office-command.png",
  "/scene/office-command.png",
  "/scene/office-command.png",
  "/scene/office-command.png",
  "/scene/office-command.png",
  "/scene/office-command.png",
  "/scene/office-legendary.png",
  "/scene/office-legendary.png",
  "/scene/office-legendary.png",
  "/scene/office-legendary.png",
  "/scene/office-legendary.png",
  "/scene/office-legendary.png",
  "/scene/office-kingmaker.png",
  "/scene/office-kingmaker.png",
  "/scene/office-kingmaker.png",
  "/scene/office-kingmaker.png",
  "/scene/office-kingmaker.png",
  "/scene/office-kingmaker.png",
] as const;

const sceneFilters = [
  "saturate(.82) brightness(.9)",
  "saturate(.9) brightness(.94)",
  "saturate(.96)",
  "saturate(1.02) brightness(1.02)",
  "saturate(1.08) brightness(1.04)",
] as const;

const sceneProgression: SceneProgressionEntry[] = SCENE_UNLOCK_LEVELS.map(
  (minLevel, index) => ({
    minLevel,
    location: {
      id: `scene_upgrade_${minLevel}`,
      name: sceneNames[index],
      image: sceneImages[index],
      filter: sceneFilters[index % sceneFilters.length],
    },
  }),
);

export const humbleRagsOutfit: OutfitLayer = {
  id: "humble_rags",
  name: "Humble Rags",
  image: "/scene/outfit-humble-rags.svg",
  minLevel: 0,
  description: "Threadbare clothes from before the Royal Influence Office noticed you.",
};

const outfitNames = [
  "Office Intern Tunic",
  "Ink-Stained Work Tunic",
  "Junior Clerk's Tabard",
  "Royal Mail Jerkin",
  "Pamphleteer's Vest",
  "Town Crier Liaison Coat",
  "Village Campaign Doublet",
  "Festival Herald Doublet",
  "Audience Researcher's Coat",
  "Royal Campaign Doublet",
  "Briefing Room Bluecoat",
  "Public Sentiment Tailcoat",
  "Senior Strategist's Coat",
  "Court Press Formalwear",
  "Deputy Director's Coat",
  "Director's Gold-Trimmed Coat",
  "Executive Courtwear",
  "Royal Broadcast Uniform",
  "Kingdom Messaging Robes",
  "Influence Commander Robes",
  "Realm Strategist Vestments",
  "Grand Campaign Mantle",
  "Chancellor's Briefing Robes",
  "Crown Communications Regalia",
  "Legendary Influence Regalia",
  "Aetherwave Ceremonial Coat",
  "Royal Reputation Vestments",
  "Grand Audience Regalia",
  "High Chancellor's Robes",
  "Crown Strategist Vestments",
  "Kingmaker Black-and-Gold",
  "Public Opinion Sovereign Robes",
  "Royal Influence Apex Regalia",
  "The Final Kingmaker Vestments",
] as const;

const outfitImages = [
  "/scene/outfit-intern-tunic.svg",
  "/scene/outfit-clerk-tabard.svg",
  "/scene/outfit-campaign-doublet.svg",
  "/scene/outfit-director-coat.svg",
  "/scene/outfit-ceremonial-sash.svg",
  "/scene/outfit-chancellor-robes.svg",
  "/scene/outfit-legendary-regalia.svg",
  "/scene/outfit-grand-mantle.svg",
  "/scene/outfit-kingmaker-vestments.svg",
  "/scene/outfit-sovereign-regalia.svg",
] as const;

const outfitImageSequence = [
  0, 1, 0, 2, 1, 3, 2, 4, 3, 2, 4, 5, 3, 4, 5, 6, 4,
  7, 5, 6, 7, 8, 6, 7, 9, 8, 7, 9, 6, 8, 9, 7, 8, 9,
] as const;

const milestoneOutfits: OutfitLayer[] = WARDROBE_UNLOCK_LEVELS.map(
  (minLevel, index) => {
    const imageIndex = outfitImageSequence[index];

    return {
      id: `milestone_outfit_${minLevel}`,
      name: outfitNames[index],
      image: outfitImages[imageIndex],
      filter:
        index === 0
          ? ""
          : `hue-rotate(${(index * 47) % 360}deg) saturate(${1.05 + (index % 4) * 0.18}) brightness(${0.88 + (index % 5) * 0.06}) contrast(${1 + (index % 3) * 0.08})`,
      minLevel,
      description: `Unlocked at level ${minLevel}. Role-appropriate courtwear for ${outfitNames[index].toLowerCase()}.`,
    };
  },
);

export const outfits: OutfitLayer[] = [humbleRagsOutfit, ...milestoneOutfits];

export const startingSceneSelection: CharacterSceneSelection = {
  location: farmLocation,
  outfit: humbleRagsOutfit,
  companion: null,
};

export function sceneSelectionForProgress(
  level: number,
  hasBeenHired: boolean,
  hasRoyalContract: boolean,
  selectedOutfitId = humbleRagsOutfit.id,
): CharacterSceneSelection {
  if (!hasBeenHired) {
    return startingSceneSelection;
  }

  const normalizedLevel = Math.max(1, Math.min(100, Math.floor(level)));
  const unlockedOutfits = outfitsForProgress(normalizedLevel, hasBeenHired);
  const selectedOutfit =
    unlockedOutfits.find((outfit) => outfit.id === selectedOutfitId) ??
    humbleRagsOutfit;
  const location = hasRoyalContract
    ? [...sceneProgression]
        .reverse()
        .find((entry) => normalizedLevel >= entry.minLevel)?.location ??
      sharedOfficeLocation
    : officeClosetLocation;

  return {
    location,
    outfit: selectedOutfit,
    companion: null,
  };
}

export function outfitsForProgress(
  level: number,
  hasBeenHired: boolean,
): OutfitLayer[] {
  if (!hasBeenHired) {
    return [humbleRagsOutfit];
  }

  const normalizedLevel = Math.max(1, Math.min(100, Math.floor(level)));
  return outfits.filter((outfit) => outfit.minLevel <= normalizedLevel);
}

export function outfitForId(outfitId: string): OutfitLayer {
  return outfits.find((outfit) => outfit.id === outfitId) ?? humbleRagsOutfit;
}

export function outfitUnlockedAtLevel(level: number): OutfitLayer | undefined {
  return milestoneOutfits.find((outfit) => outfit.minLevel === level);
}

export function sceneUnlockedAtLevel(level: number): SceneLayer | undefined {
  return sceneProgression.find((entry) => entry.minLevel === level)?.location;
}
