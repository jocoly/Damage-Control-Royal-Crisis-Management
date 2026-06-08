# Copilot Instructions: Knight Shift

This is a small Tauri + Svelte + TypeScript desktop companion RPG.

Core rule:
All progression must trace back to real user keyboard or mouse activity.
Do not add passive idle generation, timers that create currency, login rewards, energy systems, or exponential click multipliers.

Tech stack:
- Tauri desktop shell
- Svelte frontend
- TypeScript game logic
- Rust Tauri backend for global input counting
- JSON save files for the prototype
- PixiJS or Canvas for the small pixel-art guild scene

Privacy:
- Never store key contents.
- Only count keypresses and mouse clicks.
- Do not log active window names, typed text, screenshots, URLs, or app names unless explicitly requested.

UI:
- App should remain tiny, roughly 250x250 to 400x400 px.
- Avoid full-screen assumptions.
- Keep CPU/memory usage low.

Development rules:
- Keep changes small.
- Prefer simple readable code.
- Add or update tests when practical.
- Do not introduce new dependencies without explaining why.
- After changes, run the relevant build/check command.
