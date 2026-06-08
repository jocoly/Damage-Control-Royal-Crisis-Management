import { invoke } from "@tauri-apps/api/core";

export function startFocusedInputBridge() {
  function handleKeydown(event: KeyboardEvent) {
    if (event.repeat || isTextEntryTarget(event.target)) {
      return;
    }

    void invoke("record_focused_keypress", {
      eventAtMillis: Date.now(),
    });
  }

  document.addEventListener("keydown", handleKeydown, { capture: true });

  return () => {
    document.removeEventListener("keydown", handleKeydown, { capture: true });
  };
}

function isTextEntryTarget(target: EventTarget | null) {
  return (
    target instanceof Element &&
    target.closest("input, textarea, [contenteditable='true']") !== null
  );
}
