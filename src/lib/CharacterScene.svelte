<script lang="ts">
  import CharacterAvatar from "$lib/CharacterAvatar.svelte";
  import type { CharacterAppearance } from "$lib/characterAppearance";
  import type { CharacterSceneSelection } from "$lib/sceneCatalog";

  let {
    selection,
    appearance,
  }: {
    selection: CharacterSceneSelection;
    appearance: CharacterAppearance;
  } = $props();
</script>

<div
  class="character-scene"
  style={`--scene-background: url("${selection.location.image}"); --scene-filter: ${selection.location.filter ?? "none"}`}
  aria-label={`${selection.location.name}, wearing ${selection.outfit.name}${selection.companion === null ? "" : ` with ${selection.companion.name}`}`}
>
  <div class="scene-art" aria-hidden="true"></div>
  <div class="character" aria-hidden="true">
    <CharacterAvatar {appearance} outfit={selection.outfit} />
    {#if selection.companion}
      <img class="companion-layer" src={selection.companion.image} alt="" />
    {/if}
  </div>
  <div class="ground-shadow" aria-hidden="true"></div>
</div>

<style>
  .character-scene {
    position: absolute;
    z-index: 0;
    inset: 0;
    overflow: hidden;
    border-radius: inherit;
  }

  .scene-art {
    position: absolute;
    inset: 0 0 34px;
    background-image: var(--scene-background);
    background-position: left center;
    background-repeat: no-repeat;
    background-size: contain;
    filter: var(--scene-filter);
  }

  .character-scene::after {
    position: absolute;
    inset: 0 auto 0 0;
    width: 58%;
    content: "";
    background: radial-gradient(
      ellipse at 22% 48%,
      rgba(18, 18, 18, 0.7),
      rgba(18, 18, 18, 0.28) 52%,
      transparent 76%
    );
  }

  .character {
    position: absolute;
    z-index: 1;
    right: 2px;
    bottom: 5px;
    width: 96px;
    height: 139px;
    filter: drop-shadow(0 4px 4px rgba(20, 12, 5, 0.38));
  }

  .companion-layer {
    position: absolute;
    z-index: 2;
    right: 104px;
    top: 50%;
    width: 60px;
    height: 60px;
    object-fit: contain;
    transform: translateY(-50%);
  }

  .ground-shadow {
    position: absolute;
    z-index: 0;
    right: 2px;
    bottom: 35px;
    width: 105px;
    height: 18px;
    border-radius: 50%;
    background: rgba(20, 12, 5, 0.32);
    filter: blur(4px);
  }
</style>
