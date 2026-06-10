<script lang="ts">
  import {
    appearanceColor,
    hairColorOptions,
    skinToneOptions,
    type CharacterAppearance,
  } from "$lib/characterAppearance";
  import type { OutfitLayer } from "$lib/sceneCatalog";

  let {
    appearance,
    outfit = null,
  }: {
    appearance: CharacterAppearance;
    outfit?: OutfitLayer | null;
  } = $props();

  const skin = $derived(
    appearanceColor(skinToneOptions, appearance.skinTone, "#d6a06e"),
  );
  const hair = $derived(
    appearanceColor(hairColorOptions, appearance.hairColor, "#60452f"),
  );
</script>

<div class="avatar" style={`--skin: ${skin}; --hair: ${hair}`}>
  <svg viewBox="0 0 180 260" aria-hidden="true">
    <ellipse cx="90" cy="247" rx="55" ry="9" fill="#1b120b" opacity=".34"/>
    <path d="M66 148q12-5 23 1l-5 68q-1 9-11 9t-12-9z" fill="var(--skin)" stroke="#563522" stroke-width="5" stroke-linejoin="round"/>
    <path d="M91 149q11-6 23-1l5 69q-2 9-12 9t-11-9z" fill="var(--skin)" stroke="#563522" stroke-width="5" stroke-linejoin="round"/>
    {#if outfit}
      <path d="M62 215q11 5 22 0l-2 22q-1 7-10 7H47q-8 0-4-7zM96 215q11 5 22 0l19 22q4 7-4 7h-25q-9 0-10-7z" fill="#513526" stroke="#2d1d15" stroke-width="5" stroke-linejoin="round"/>
    {:else}
      <path d="M62 215q11 5 22 0l-1 19q-1 9-11 9H54q-9 0-8-7 2-8 16-21zM96 215q11 5 22 0 14 13 16 21 1 7-8 7h-18q-10 0-11-9z" fill="var(--skin)" stroke="#563522" stroke-width="5" stroke-linejoin="round"/>
    {/if}
    <path
      d="M68 98Q56 96 48 106q-4 6-7 17l-14 57q-3 12 5 17 8 3 13-5 2-4 4-12l11-46q3-11 11-19z"
      fill="var(--skin)"
      stroke="#563522"
      stroke-width="5"
      stroke-linejoin="round"
    />
    <path
      d="M112 98q12-2 20 8 4 6 7 17l14 57q3 12-5 17-8 3-13-5-2-4-4-12l-11-46q-3-11-11-19z"
      fill="var(--skin)"
      stroke="#563522"
      stroke-width="5"
      stroke-linejoin="round"
    />
    <path d="M68 96Q90 82 112 96l5 62q-27 16-54 0z" fill="var(--skin)" stroke="#563522" stroke-width="5"/>
    <rect x="22" y="178" width="27" height="22" rx="10" fill="var(--skin)" stroke="#563522" stroke-width="4"/>
    <rect x="131" y="178" width="27" height="22" rx="10" fill="var(--skin)" stroke="#563522" stroke-width="4"/>

    {#if !outfit}
      <path
        d="M67 96 78 85q4 13 12 13t12-13l11 11 4 62q-27 16-54 0z"
        fill="#fffaf0"
        stroke="#5a554c"
        stroke-width="4"
        stroke-linejoin="round"
      />
      <path d="M78 85q4 20 12 20t12-20" fill="var(--skin)" stroke="#5a554c" stroke-width="4"/>
      <path d="M62 153q28 11 56 0l4 34-21-2-11 12-11-12-21 2z" fill="#fffaf0" stroke="#665b52" stroke-width="5" stroke-linejoin="round"/>
      <path d="M90 160v37M66 162q24 8 48 0" fill="none" stroke="#c9bdb0" stroke-width="3"/>
      <path d="M59 184q10 7 21 1M100 185q11 6 21-1" fill="none" stroke="#665b52" stroke-linecap="round" stroke-width="5"/>
      <g fill="#c83f48">
        <path d="M70 169c-5-6-12 2 0 10 12-8 5-16 0-10z"/>
        <path d="M105 169c-5-6-12 2 0 10 12-8 5-16 0-10z"/>
        <path d="M87 181c-4-5-10 2 0 8 10-6 4-13 0-8z"/>
      </g>
    {/if}

    <rect x="76" y="68" width="28" height="27" rx="9" fill="var(--skin)" stroke="#563522" stroke-width="5"/>
    <ellipse cx="90" cy="48" rx="39" ry="43" fill="var(--skin)" stroke="#563522" stroke-width="5"/>
    <path d="M53 47q-8 2-7 13t12 14M127 47q8 2 7 13t-12 14" fill="var(--skin)" stroke="#563522" stroke-width="5" stroke-linecap="round"/>

    {#if appearance.hair !== "bald"}
      {#if appearance.hair === "waves"}
        <path d="M51 48Q50 5 89 3q39 2 40 43-10-9-18-7-8-15-18-2-11-15-20 1-10-6-22 10z" fill="var(--hair)" stroke="#3a291d" stroke-width="5" stroke-linejoin="round"/>
      {:else if appearance.hair === "mane"}
        <path
          d="M49 50Q50 8 90 3q40 5 41 47l-3 29q-6 8-15 10l1-31q0-15-7-24-4-5-8-8l-5 13-6-13-7 13-6-12q-9 10-9 25l1 31q-9-2-15-10z"
          fill="var(--hair)"
          stroke="#3a291d"
          stroke-width="5"
          stroke-linejoin="round"
        />
      {:else}
        <path d="M53 46Q54 4 90 4q31 0 39 31-16-13-33-10-26 5-43 21z" fill="var(--hair)" stroke="#3a291d" stroke-width="5" stroke-linejoin="round"/>
      {/if}
    {/if}

    {#if appearance.eyes === "bright"}
      <g fill="#29231f"><circle cx="75" cy="55" r="4"/><circle cx="105" cy="55" r="4"/></g>
    {:else if appearance.eyes === "calm"}
      <g fill="none" stroke="#3a291d" stroke-linecap="round" stroke-width="4"><path d="M66 56q9 7 18 0"/><path d="M96 56q9 7 18 0"/></g>
    {:else if appearance.eyes === "bold"}
      <g fill="#f8f3df" stroke="#563522" stroke-width="3"><ellipse cx="75" cy="56" rx="8" ry="6"/><ellipse cx="105" cy="56" rx="8" ry="6"/></g>
      <g fill="#29231f"><circle cx="76" cy="56" r="3"/><circle cx="104" cy="56" r="3"/></g>
      <g fill="none" stroke="#3a291d" stroke-linecap="round" stroke-width="4"><path d="M65 45l18 3"/><path d="m97 48 18-3"/></g>
    {:else}
      <g fill="#f8f3df" stroke="#563522" stroke-width="3"><ellipse cx="75" cy="55" rx="8" ry="7"/><ellipse cx="105" cy="55" rx="8" ry="7"/></g>
      <g fill="#29231f"><circle cx="76" cy="56" r="3"/><circle cx="104" cy="56" r="3"/></g>
    {/if}

    {#if appearance.nose === "straight"}
      <path d="M90 56v15l6 2" fill="none" stroke="#75452f" stroke-linecap="round" stroke-width="3"/>
    {:else if appearance.nose === "round"}
      <path d="M86 68q4 8 9 0" fill="none" stroke="#75452f" stroke-linecap="round" stroke-width="4"/>
    {:else if appearance.nose === "sharp"}
      <path d="m92 57-5 15 10-1" fill="none" stroke="#75452f" stroke-linecap="round" stroke-linejoin="round" stroke-width="3"/>
    {:else}
      <path d="M87 68q3 5 6 0" fill="none" stroke="#75452f" stroke-linecap="round" stroke-width="3"/>
    {/if}

    {#if appearance.mouth === "grin"}
      <path d="M76 72q14 13 28 0-4 12-14 12T76 72z" fill="#f8f3df" stroke="#563522" stroke-linejoin="round" stroke-width="3"/>
    {:else if appearance.mouth === "calm"}
      <path d="M78 80q12 3 24 0" fill="none" stroke="#563522" stroke-linecap="round" stroke-width="4"/>
    {:else if appearance.mouth === "confident"}
      <ellipse cx="90" cy="81" rx="6" ry="5" fill="#713b38" stroke="#563522" stroke-width="3"/>
      <ellipse cx="90" cy="82.5" rx="2.5" ry="1" fill="#d98f83"/>
    {:else}
      <path d="M76 76q14 11 28 0" fill="none" stroke="#563522" stroke-linecap="round" stroke-width="4"/>
    {/if}
  </svg>

  {#if outfit}
    <img
      class="outfit"
      src={outfit.image}
      style:filter={outfit.filter ?? "none"}
      alt=""
    />
  {/if}
</div>

<style>
  .avatar {
    position: relative;
    width: 100%;
    height: 100%;
  }

  svg,
  .outfit {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
  }

  .outfit {
    object-fit: contain;
  }
</style>
