export type CharacterAppearance = {
  eyes: string;
  nose: string;
  mouth: string;
  hair: string;
  hairColor: string;
  skinTone: string;
};

export type AppearanceOption = {
  id: string;
  name: string;
  color?: string;
};

export const eyeOptions: AppearanceOption[] = [
  { id: "round", name: "Round" },
  { id: "bright", name: "Bright" },
  { id: "calm", name: "Calm" },
  { id: "bold", name: "Bold" },
];

export const noseOptions: AppearanceOption[] = [
  { id: "button", name: "Button" },
  { id: "straight", name: "Straight" },
  { id: "round", name: "Round" },
  { id: "sharp", name: "Sharp" },
];

export const mouthOptions: AppearanceOption[] = [
  { id: "smile", name: "Smile" },
  { id: "grin", name: "Grin" },
  { id: "calm", name: "Calm" },
  { id: "confident", name: "Confident" },
];

export const hairOptions: AppearanceOption[] = [
  { id: "bald", name: "Bald" },
  { id: "swept", name: "Swept" },
  { id: "waves", name: "Waves" },
  { id: "mane", name: "Long Hair" },
];

export const hairColorOptions: AppearanceOption[] = [
  { id: "black", name: "Black", color: "#29231f" },
  { id: "brown", name: "Brown", color: "#60452f" },
  { id: "blond", name: "Blond", color: "#c89b45" },
  { id: "red", name: "Red", color: "#8f432b" },
  { id: "gray", name: "Gray", color: "#8a8782" },
];

export const skinToneOptions: AppearanceOption[] = [
  { id: "light", name: "Light", color: "#f7d8ba" },
  { id: "warm", name: "Warm", color: "#e5b384" },
  { id: "tan", name: "Tan", color: "#cc9165" },
  { id: "brown", name: "Brown", color: "#aa704e" },
  { id: "deep", name: "Deep", color: "#80533d" },
];

export const defaultCharacterAppearance: CharacterAppearance = {
  eyes: "round",
  nose: "button",
  mouth: "smile",
  hair: "bald",
  hairColor: "brown",
  skinTone: "warm",
};

export function appearanceColor(
  options: AppearanceOption[],
  id: string,
  fallback: string,
) {
  return options.find((option) => option.id === id)?.color ?? fallback;
}
