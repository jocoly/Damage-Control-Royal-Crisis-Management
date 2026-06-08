export type TitleDefinition = {
  minLevel: number;
  maxLevel: number;
  name: string;
  description: string;
  flavor: string;
};

type TitleBand = {
  name: string;
  levels: Array<readonly [description: string, flavor: string]>;
};

export const loyalSubjectTitle: TitleDefinition = {
  minLevel: 0,
  maxLevel: 0,
  name: "Loyal Subject",
  description: "A citizen with no role in the Royal Influence Office.",
  flavor: "You follow the crown and pay taxes through a broken portal.",
};

const royalContractId = "royal_contract";
const royalContractTitleMaxLevel = 5;

const titleBands: TitleBand[] = [
  {
    name: "Court Marketing Intern",
    levels: [
      [
        "You file posts and forms for the Royal Influence Office.",
        "WELCOME1 somehow meets court security policy.",
      ],
    ],
  },
  {
    name: "Court Marketing Associate",
    levels: [
      [
        "You schedule posts for the kingdom's message board.",
        "The morning proclamation is delayed while a herald installs updates.",
      ],
      [
        "You answer routine comments beneath royal decrees.",
        "Most subjects ask whether the king's account was hacked again.",
      ],
      [
        "You prepare simple engagement reports for the Office.",
        "A thumbs-up from a duke counts as three ordinary reactions.",
      ],
      [
        "You schedule public notices and feast promotions.",
        "The algorithm strongly prefers posts containing illuminated cheese.",
      ],
    ],
  },
  {
    name: "Assistant Royal Page",
    levels: [
      [
        "You deliver messages across the enchanted network.",
        "The wax seal now includes a link nobody remembers authorizing.",
      ],
      [
        "You sort the monarch's overflowing enchanted inbox.",
        "Half the messages are princes offering investment opportunities.",
      ],
      [
        "You route urgent notifications between palace departments.",
        "Urgent has four meanings and twelve reaction icons.",
      ],
      [
        "You compare digital proclamations with parchment originals.",
        "The parchment never asks you to accept tracking runes.",
      ],
      [
        "You manage royal read receipts.",
        "The treasury has left the king on read for six consecutive quarters.",
      ],
    ],
  },
  {
    name: "Junior Campaign Coordinator",
    levels: [
      [
        "You coordinate campaigns for nearby villages.",
        "The village forum bans turnip after a bitter thread.",
      ],
      [
        "You recruit tavern streamers for royal campaigns.",
        "The Office calls influencers community heralds.",
      ],
      [
        "You draft campaign tags for festivals and tax deadlines.",
        "#BlessedByTheCrown trends until a bishop buys suspiciously many reposts.",
      ],
      [
        "You monitor wizard streams for campaign openings.",
        "Every fireball stream promotes mana powder.",
      ],
      [
        "You compare results across village feeds.",
        "One hamlet shares a single engaged account.",
      ],
    ],
  },
  {
    name: "Court Communications Clerk",
    levels: [
      [
        "You archive posts before officials edit history.",
        "The royal archive has a folder labeled FINAL_final_REAL_3.",
      ],
      [
        "You process requests to remove embarrassing court messages.",
        "Deleted by royal order still appears beneath every deleted post.",
      ],
      [
        "You moderate disputes on the palace's public channels.",
        "Two knights are suspended for arguing about shield aspect ratios.",
      ],
      [
        "You maintain the Office glossary for approved online language.",
        "Synergy is legal again, but only during sieges.",
      ],
      [
        "You summarize rumors spreading across the realm.",
        "The moon-tracking rumor has excellent retention.",
      ],
    ],
  },
  {
    name: "Senior Clerk",
    levels: [
      [
        "You review sensitive posts for the royal feed.",
        "A reply-all becomes a constitutional crisis.",
      ],
      [
        "You train junior clerks in archives and moderation.",
        "Lesson one is never quote-post a necromancer after midnight.",
      ],
      [
        "You investigate leaks from private court channels.",
        "The culprit is usually a scrying mirror left on during lunch.",
      ],
      [
        "You correct contradictions between official accounts.",
        "The army and navy briefly announce different Tuesdays.",
      ],
      [
        "You oversee the Office's sentiment records.",
        "The public wants fewer sentiment surveys.",
      ],
    ],
  },
  {
    name: "Royal Marketing Associate",
    levels: [
      [
        "You design campaigns with enchanted analytics.",
        "The crystal ball remembers your cookies.",
      ],
      [
        "You negotiate promotions with guilds and prophets.",
        "Prophets know exposure will not survive the apocalypse.",
      ],
      [
        "You test competing versions of royal announcements.",
        "Version B wins after replacing fiscal duty with treasure vibes.",
      ],
      [
        "You manage sponsored content across creator guilds.",
        "Sponsored quests must disclose free dragon armor.",
      ],
      [
        "You track how royal messaging moves the public.",
        "Attendance rises whenever the invitation includes free enchanted bandwidth.",
      ],
    ],
  },
  {
    name: "Campaign Supervisor",
    levels: [
      [
        "You supervise campaigns for several ministries.",
        "Every ministry demands the pinned slot.",
      ],
      [
        "You approve livestream plans for royal ceremonies.",
        "The coronation stream buffers at the precise moment the crown catches fire.",
      ],
      [
        "You settle disputes between heralds and network artificers.",
        "The heralds claim the latency ruins their dramatic pauses.",
      ],
      [
        "You regulate sponsored magical demonstrations.",
        "Polymorph tutorials require livestock warnings.",
      ],
      [
        "You report campaign results to Office leaders.",
        "Leadership requests a dashboard that feels more victorious.",
      ],
    ],
  },
  {
    name: "Regional Influence Captain",
    levels: [
      [
        "You direct Office campaigns across a province.",
        "The north prefers competitive embroidery streams.",
      ],
      [
        "You adapt royal messages for regional feeds.",
        "In the marshlands, every official post must include a legally binding fog emoji.",
      ],
      [
        "You counter regional misinformation.",
        "No, the queen is not three sprites operating a ceremonial gown.",
      ],
      [
        "You maintain channels during festivals and emergencies.",
        "The emergency channel becomes unusable after someone discovers animated banners.",
      ],
      [
        "You evaluate regional officers by trust and reach.",
        "One captain reports 140 percent trust and is promoted to audit subject.",
      ],
    ],
  },
  {
    name: "Royal Influence Officer",
    levels: [
      [
        "You command major Royal Influence Office campaigns.",
        "Your verified crown icon is larger than several baronies.",
      ],
      [
        "You answer viral events across the realm.",
        "The Office has forty minutes to explain why a griffin is wearing the treasury seal.",
      ],
      [
        "You manage public profiles for nobles.",
        "The duke's bio still says humble servant above a portrait of twelve yachts.",
      ],
      [
        "You investigate magical misinformation networks.",
        "All twelve suspicious accounts are operated by one broom in a rented cellar.",
      ],
      [
        "You represent the Office before royal councils.",
        "The council debates one reaction for two hours.",
      ],
    ],
  },
  {
    name: "Court Marketing Director",
    levels: [
      [
        "You direct the court's identity across every channel.",
        "The brand guide lists seventeen shades of majestic.",
      ],
      [
        "You allocate budgets and enchanted bandwidth.",
        "The dungeon receives faster service after threatening to switch providers.",
      ],
      [
        "You set communications policy for the court.",
        "Policy now requires captions whenever a wizard speaks in thunder.",
      ],
      [
        "You commission major creator partnerships.",
        "A famous bard charges per verse, repost, and heroic key change.",
      ],
      [
        "You present influence strategy to the crown.",
        "The king likes the chart because his portrait occupies the upward trend.",
      ],
    ],
  },
  {
    name: "Chief Campaign Strategist",
    levels: [
      [
        "You design the Office's long-term strategy.",
        "Your five-year plan includes a contingency for sentient hashtags.",
      ],
      [
        "You predict reactions with analytics and prophecy.",
        "The prophecy model is accurate except when Mercury enters sponsored content.",
      ],
      [
        "You align messages across diplomacy, trade, and defense.",
        "The same slogan must sell wheat, peace treaties, and siege insurance.",
      ],
      [
        "You reshape how the realm views the crown.",
        "The phrase relatable monarchy survives three committees and one abdication.",
      ],
      [
        "You brief the monarch on online culture.",
        "The king now knows what ratioed means and demands immediate arrests.",
      ],
    ],
  },
  {
    name: "Master of Royal Operations",
    levels: [
      [
        "You control the Office's publishing infrastructure.",
        "An underpaid apprentice guards the server crystal.",
      ],
      [
        "You coordinate staff and magical bandwidth.",
        "Every department claims its weekly newsletter is mission critical.",
      ],
      [
        "You plan for curses, outages, and scandals.",
        "The backup feed is powered by hamsters enchanted for plausible deniability.",
      ],
      [
        "You modernize obsolete court systems.",
        "The ancient oracle rejects the new terms.",
      ],
      [
        "You ensure royal messages reach their audience.",
        "The dead-letter queue is managed by an actual necromancer.",
      ],
    ],
  },
  {
    name: "High Chancellor of Messaging",
    levels: [
      [
        "You set messaging doctrine for the crown.",
        "Authenticity requires seventeen approval signatures.",
      ],
      [
        "You arbitrate the kingdom's public voice.",
        "The army wants bold, the treasury wants solvent, and the king wants taller.",
      ],
      [
        "You guide messaging through wars and migrations.",
        "The evacuation notice performs poorly until a minstrel adds a dance.",
      ],
      [
        "You train senior influence officers.",
        "The final exam is explaining the algorithm without inventing a demon.",
      ],
      [
        "You advise the crown on digital legitimacy.",
        "The throne remains popular, though its comments are wisely restricted.",
      ],
    ],
  },
  {
    name: "Grand Marshal of Campaigns",
    levels: [
      [
        "You command campaigns across every province.",
        "At dawn, ten thousand heralds post the same typo.",
      ],
      [
        "You deploy influence teams with diplomats and armies.",
        "The siege begins only after Legal approves the launch thread.",
      ],
      [
        "You counter propaganda on foreign networks.",
        "Enemy bots are easy to spot because they compliment the tax code.",
      ],
      [
        "You direct responses to cultural crises.",
        "A knight's apology video is filmed beside the horse he insulted.",
      ],
      [
        "You measure victory through public confidence.",
        "Morale rises six points after the Office patches the victory animation.",
      ],
    ],
  },
  {
    name: "Royal Brand Advisor",
    levels: [
      [
        "You advise the royal family on public identity.",
        "The prince's personal brand is mostly apologies issued by your office.",
      ],
      [
        "You shape the kingdom's image abroad.",
        "The official profile picture is retouched to remove three historic rebellions.",
      ],
      [
        "You protect the crown from endorsement scams.",
        "The queen did not launch a miracle tonic, despite the verified cauldron.",
      ],
      [
        "You regulate royal appearances across magical media.",
        "Scrying mirrors must now use the monarch's preferred side.",
      ],
      [
        "You turn tradition into a public identity.",
        "The new slogan tests well with peasants and poorly with historians.",
      ],
    ],
  },
  {
    name: "Archchancellor of Royal Reach",
    levels: [
      [
        "You expand the Office beyond the kingdom.",
        "Foreign audiences fear the royal cookie banner.",
      ],
      [
        "You negotiate access to distant magical platforms.",
        "The cloud giants offer excellent hosting but terrible weather guarantees.",
      ],
      [
        "You adapt campaigns for unfamiliar cultures.",
        "A translation rune renders beloved sovereign as moderately acceptable landlord.",
      ],
      [
        "You coordinate creators, diplomats, and network mages.",
        "The international livestream requires nine interpreters and one exorcist.",
      ],
      [
        "You make the kingdom known across the connected world.",
        "Even sea witches now mute the Office instead of marking it as spam.",
      ],
    ],
  },
  {
    name: "Supreme Court Marketer",
    levels: [
      [
        "You govern influence standards within the court.",
        "Your approval stamp becomes a speculative collectible.",
      ],
      [
        "You decide which trends the kingdom adopts.",
        "The Office denies creating cottagecore while selling official cottages.",
      ],
      [
        "You turn court policy into shareable stories.",
        "A thirty-page levy becomes a charming tale about one brave little tariff.",
      ],
      [
        "You turn campaigns into living folklore.",
        "Children now reenact the budget announcement with collectible ministers.",
      ],
      [
        "You weave influence into daily culture.",
        "The morning bells ring only after the royal post finishes uploading.",
      ],
    ],
  },
  {
    name: "Lord of Royal Influence",
    levels: [
      [
        "You control nearly every message from the crown.",
        "Ministers now ask permission before having opinions in public.",
      ],
      [
        "You shape alliances by directing attention.",
        "A well-timed repost prevents a war and accidentally promotes a bakery.",
      ],
      [
        "You command the Office's finest strategists and mages.",
        "Your staff can stabilize a kingdom before breakfast and a group chat by noon.",
      ],
      [
        "You manage the crown's reputation as an asset.",
        "Public trust is stored beside the gold because both keep disappearing.",
      ],
      [
        "You choose the stories that define a generation.",
        "Historians request edit access and are politely removed from the document.",
      ],
    ],
  },
  {
    name: "Aetherlord of Messaging",
    levels: [
      [
        "You direct influence across magical global networks.",
        "Crypto dragons call their hoards decentralized treasures.",
      ],
      [
        "You regulate enchanted assets and royal finance posts.",
        "The kingdom's coin rises after a dragon posts three flame emojis.",
      ],
      [
        "You counter attention-backed magical currencies.",
        "The treasury learns the moon was tokenized twice.",
      ],
      [
        "You coordinate the Office across scrying realities.",
        "One parallel kingdom has excellent engagement and no functioning monarchy.",
      ],
      [
        "You make the crown a force in the magical economy.",
        "Merchants check your morning post before prices, weather, or structural reality.",
      ],
    ],
  },
  {
    name: "Legend of the Royal Court",
    levels: [
      [
        "Your campaigns become milestones in Office history.",
        "Wizards react to your posts with dramatic gasps.",
      ],
      [
        "You train successors for the online kingdom.",
        "Every archived crisis uses the same dancing skeleton.",
      ],
      [
        "You advise rulers who depend on your systems.",
        "Crowns change, but your admin access remains.",
      ],
      [
        "You embody the history of royal influence.",
        "Every ancient meme bears your metadata.",
      ],
    ],
  },
  {
    name: "Kingmaker",
    levels: [
      [
        "You command enough influence to make or unmake a reign.",
        "Monarchs and markets wait for you to click Publish.",
      ],
    ],
  },
];

let nextLevel = 1;

export const titles: TitleDefinition[] = titleBands.flatMap((band) =>
  band.levels.map(([description, flavor]) => {
    const level = nextLevel;
    nextLevel += 1;

    return {
      minLevel: level,
      maxLevel: level,
      name: band.name,
      description,
      flavor,
    };
  }),
);

function validateTitleCatalog(): void {
  if (titles.length !== 100) {
    throw new Error(`Expected 100 title levels, found ${titles.length}.`);
  }

  titles.forEach((title, index) => {
    const expectedLevel = index + 1;
    if (title.minLevel !== expectedLevel || title.maxLevel !== expectedLevel) {
      throw new Error(`Title catalog is missing level ${expectedLevel}.`);
    }
  });

  const descriptions = new Set(titles.map((title) => title.description));
  const flavors = new Set(titles.map((title) => title.flavor));
  if (descriptions.size !== titles.length || flavors.size !== titles.length) {
    throw new Error("Every title level must have unique description and flavor text.");
  }
}

validateTitleCatalog();

export function getTitleForLevel(level: number): TitleDefinition {
  const normalizedLevel = Math.max(1, Math.min(100, Math.floor(level)));
  return titles[normalizedLevel - 1];
}

export function getTitleForLevelWithContract(
  level: number,
  hasRoyalContract: boolean,
): TitleDefinition {
  if (!hasRoyalContract) {
    return loyalSubjectTitle;
  }

  return getTitleForLevel(level);
}

export function titleForLevel(level: number): TitleDefinition {
  return getTitleForLevel(level);
}

export function titleForLevelAndInventory(
  level: number,
  inventoryItemIds: readonly string[],
): TitleDefinition {
  const baseTitle = getTitleForLevel(level);

  if (
    level <= royalContractTitleMaxLevel &&
    inventoryItemIds.includes(royalContractId)
  ) {
    return {
      ...baseTitle,
      name: "Court Marketing Manager",
      description: "Official marketing manager for the Aethernet royal court.",
      flavor: "The contract is mostly legitimate, depending on who asks.",
    };
  }

  return baseTitle;
}

export function titleHoverText(title: TitleDefinition): string {
  return title.flavor;
}

export function isPromotionLevel(level: number): boolean {
  const normalizedLevel = Math.max(1, Math.min(100, Math.floor(level)));
  return (
    normalizedLevel === 1 ||
    getTitleForLevel(normalizedLevel).name !== getTitleForLevel(normalizedLevel - 1).name
  );
}
