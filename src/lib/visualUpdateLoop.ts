export const ACTIVE_VISUAL_UPDATE_MS = 100;
export const IDLE_AFTER_MS = 30_000;
export const IDLE_VISUAL_CHECK_MS = 1_000;

export type InputSnapshot = {
  influence: number;
  xp: number;
  level: number;
  xp_for_current_level: number;
  xp_for_next_level: number;
  keys: number;
  clicks: number;
  bonus_influence: number;
  spent_influence: number;
  power_event_sequence: number;
  last_power_event_at_millis: number;
  last_power_event_amount: number;
  last_power_event_item_level: number;
  inventory_item_ids: string[];
  seen_story_event_ids: string[];
  seen_shop_item_ids: string[];
  kingdom_name: string | null;
  last_input_at_millis: number;
};

export type VisualFrame = InputSnapshot & {
  isIdle: boolean;
  lastInputAgeMs: number | null;
};

type VisualUpdateLoopOptions = {
  readSnapshot: () => Promise<InputSnapshot>;
  onFrame: (frame: VisualFrame) => void;
  onError: () => void;
};

export function startVisualUpdateLoop({
  readSnapshot,
  onFrame,
  onError,
}: VisualUpdateLoopOptions) {
  let stopped = false;
  let timeoutId: number | undefined;
  let wasIdle: boolean | undefined;

  async function tick() {
    try {
      const snapshot = await readSnapshot();
      const lastInputAgeMs =
        snapshot.last_input_at_millis === 0
          ? null
          : Math.max(0, Date.now() - snapshot.last_input_at_millis);
      const isIdle = lastInputAgeMs === null || lastInputAgeMs >= IDLE_AFTER_MS;

      if (!isIdle || wasIdle !== isIdle) {
        onFrame({
          ...snapshot,
          spent_influence: spentInfluenceForSnapshot(snapshot),
          seen_shop_item_ids: snapshot.seen_shop_item_ids ?? [],
          kingdom_name: snapshot.kingdom_name ?? null,
          isIdle,
          lastInputAgeMs,
        });
      }

      wasIdle = isIdle;
      scheduleNext(isIdle ? IDLE_VISUAL_CHECK_MS : ACTIVE_VISUAL_UPDATE_MS);
    } catch {
      onError();
      scheduleNext(IDLE_VISUAL_CHECK_MS);
    }
  }

  function scheduleNext(delay: number) {
    if (stopped) {
      return;
    }

    timeoutId = window.setTimeout(tick, delay);
  }

  tick();

  return () => {
    stopped = true;

    if (timeoutId !== undefined) {
      window.clearTimeout(timeoutId);
    }
  };
}

function spentInfluenceForSnapshot(snapshot: InputSnapshot) {
  const calculatedSpent = snapshot.xp - snapshot.influence;

  if (Number.isFinite(calculatedSpent)) {
    return Math.max(0, calculatedSpent);
  }

  return Number.isFinite(snapshot.spent_influence) ? snapshot.spent_influence : 0;
}
