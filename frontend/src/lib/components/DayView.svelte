<script lang="ts">
  import "emoji-picker-element";
  import { api, staticUrl } from "../api";
  import { formatDateLong, isTypingElement } from "../format";
  import { navigate } from "../router";
  import type { Day, City, Accommodation, AuthUser, UpdateDay } from "../types";
  import ImageUpload from "./ImageUpload.svelte";
  import MarkdownEditor from "./MarkdownEditor.svelte";

  let {
    id,
    user,
    editMode,
    slideshowEl = $bindable(undefined),
    presentationMode = false,
  }: {
    id: number;
    user: AuthUser | null;
    editMode: boolean;
    slideshowEl?: HTMLDivElement | undefined;
    presentationMode?: boolean;
  } = $props();

  let day: Day | null = $state(null);
  let allDays: Day[] = $state([]);
  let cities: City[] = $state([]);
  let accommodations: Accommodation[] = $state([]);
  let showEmojiPicker = $state(false);
  let pickerEl: HTMLElement | null = $state(null);
  let emojiContainerEl: HTMLElement | null = $state(null);

  let dayIds = $derived(allDays.map((d) => d.id));
  let dayIndex = $derived.by(() =>
    day ? dayIds.indexOf(day.id) : -1,
  );
  let prevId = $derived(dayIndex > 0 ? dayIds[dayIndex - 1] : undefined);
  let nextId = $derived(
    dayIndex >= 0 && dayIndex < dayIds.length - 1 ? dayIds[dayIndex + 1] : undefined,
  );
  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));
  let accMap = $derived(
    Object.fromEntries(accommodations.map((a) => [a.key, a])),
  );
  let displayHeroImage = $derived.by(() =>
    day ? (day.hero_image ?? cityMap[day.city_key]?.hero_image ?? null) : null,
  );

  $effect(() => {
    loadData(id);
  });

  $effect(() => {
    id;
    slideshowEl?.scrollTo(0, 0);
  });

  async function loadData(dayId: number) {
    [day, allDays, cities, accommodations] = await Promise.all([
      api.days.get(dayId),
      api.days.list(),
      api.cities.list(),
      api.accommodations.list(),
    ]);
  }

  $effect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
      if (isTypingElement(e.target as HTMLElement)) return;
      const target = e.key === "ArrowLeft" ? prevId : nextId;
      if (target != null) {
        e.preventDefault();
        navigate(`/days/${target}`);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  });

  async function updateField(updates: Partial<UpdateDay>) {
    if (!day) return;
    day = await api.days.update(day.id, updates);
  }

  $effect(() => {
    const el = pickerEl;
    if (!el) return;
    const handler = (e: CustomEvent<{ unicode: string }>) => {
      if (e.detail?.unicode) {
        updateField({ emoji: e.detail.unicode });
        showEmojiPicker = false;
      }
    };
    el.addEventListener("emoji-click", handler as EventListener);
    return () => el.removeEventListener("emoji-click", handler as EventListener);
  });

  function handleClickOutside(e: MouseEvent) {
    if (showEmojiPicker && !emojiContainerEl?.contains(e.target as Node)) {
      showEmojiPicker = false;
    }
  }

  $effect(() => {
    if (showEmojiPicker) {
      document.addEventListener("click", handleClickOutside);
      return () => document.removeEventListener("click", handleClickOutside);
    }
  });
</script>

{#if day}
  {#snippet heroContent(d: Day, editable: boolean)}
    <h1 class="day-hero-title">
      {#if editable}
        <div class="emoji-slot" bind:this={emojiContainerEl}>
          <button
            type="button"
            class="emoji-edit-trigger"
            onclick={(e) => {
              e.stopPropagation();
              showEmojiPicker = !showEmojiPicker;
            }}
            aria-label="Change emoji"
            aria-expanded={showEmojiPicker}
            aria-haspopup="dialog"
          >
            <span class="emoji">{d.emoji || "📅"}</span>
          </button>
          {#if showEmojiPicker}
            <div class="emoji-picker-popover">
              <emoji-picker class="light" bind:this={pickerEl}></emoji-picker>
            </div>
          {/if}
        </div>
      {:else if d.emoji}
        <span class="emoji">{d.emoji}</span>
      {/if}
      {formatDateLong(d.date)}
    </h1>
    <p class="day-hero-subtext" class:edit={editable}>
      {#if editable}
        <select
          class="inline-select"
          aria-label="City"
          value={d.city_key}
          onchange={(e) =>
            updateField({ city_key: (e.target as HTMLSelectElement).value })}
        >
          {#each cities as city}
            <option value={city.key}>{city.name}</option>
          {/each}
        </select>
      {:else}
        <a href="#/cities/{d.city_key}"
          >{cityMap[d.city_key]?.name || d.city_key}</a
        >
      {/if}
    </p>
  {/snippet}

  <div
    bind:this={slideshowEl}
    class="day-slideshow"
    class:presentation-mode={presentationMode}
    role="region"
    aria-label="Day slide"
  >
    {#if displayHeroImage}
      <div
        class="day-hero-bg-fixed"
        style="background-image: url({staticUrl(displayHeroImage)})"
        aria-hidden="true"
      ></div>
    {/if}
    <div class="nav-arrows">
      <button
        class="nav-arrow nav-arrow-left"
        disabled={prevId == null}
        onclick={() => prevId != null && navigate(`/days/${prevId}`)}
        aria-label="Previous day"
      >
        ←
      </button>
      <button
        class="nav-arrow nav-arrow-right"
        disabled={nextId == null}
        onclick={() => nextId != null && navigate(`/days/${nextId}`)}
        aria-label="Next day"
      >
        →
      </button>
    </div>

    <!-- 1. Hero (overlay + content; image is on fixed layer above) -->
    <div
      class="day-hero"
      class:has-image={displayHeroImage != null}
      class:has-placeholder={!displayHeroImage}
    >
      {#if editMode}
        <div class="hero-upload-overlay">
          <ImageUpload
            currentImage={day.hero_image}
            onUpload={(url) => updateField({ hero_image: url })}
            compact
          />
        </div>
      {/if}
      {#if !displayHeroImage}
        <span class="hero-placeholder-emoji">{day.emoji || "📅"}</span>
      {/if}
      {#if displayHeroImage}
        <div class="day-hero-overlay" aria-hidden="true"></div>
      {/if}
      <div class="day-hero-content">
        <div class="day-hero-title-strip">
          {@render heroContent(day, editMode)}
        </div>
      </div>
    </div>

    {#if displayHeroImage}
      <div class="day-hero-spacer" aria-hidden="true"></div>
    {/if}

    <!-- 2. Content card (overlaps hero) -->
    <div class="slide-content">
      <div class="day-content-card">
        {#if day.tagline || editMode}
          <div class="tagline-row">
            {#if editMode}
              <input
                type="text"
                class="tagline-input"
                value={day.tagline || ""}
                onchange={(e) =>
                  updateField({ tagline: (e.target as HTMLInputElement).value })}
                placeholder="Tagline"
              />
            {:else if day.tagline}
              <p class="tagline">{day.tagline}</p>
            {/if}
          </div>
        {/if}
        {#if day.travel || editMode}
          <div class="travel-section">
            <span class="travel-subtitle">Travel</span>
            {#if editMode}
              <label for="day-travel" class="sr-only">Travel</label>
              <input
                id="day-travel"
                type="text"
                class="travel-input"
                value={day.travel || ""}
                onchange={(e) =>
                  updateField({
                    travel: (e.target as HTMLInputElement).value || null,
                  })}
                placeholder="Travel"
              />
            {:else if day.travel}
              <span class="travel-text">{day.travel}</span>
            {/if}
          </div>
        {/if}
        <div class="day-notes">
          <div class="notes-block">
            <MarkdownEditor
              value={day.notes}
              readonly={!editMode}
              onSave={(val) => updateField({ notes: val })}
            />
          </div>
        </div>
        {#if day.accommodation_key || editMode}
          <div class="sleep-section">
            <span class="sleep-subtitle">Sleep</span>
            {#if editMode}
              <select
                class="sleep-select"
                aria-label="Sleep"
                value={day.accommodation_key || ""}
                onchange={(e) =>
                  updateField({
                    accommodation_key:
                      (e.target as HTMLSelectElement).value || null,
                  })}
              >
                <option value="">— None —</option>
                {#each accommodations as acc}
                  <option value={acc.key}>{acc.name}</option>
                {/each}
              </select>
            {:else if day.accommodation_key}
              <a href="#/accommodations/{day.accommodation_key}" class="sleep-link"
                >{accMap[day.accommodation_key]?.name || day.accommodation_key}</a
              >
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{:else}
  <p>Loading...</p>
{/if}

<style>
  .day-slideshow {
    position: relative;
    background: transparent;
    min-height: calc(100vh - 56px);
    isolation: isolate;
  }

  .day-slideshow.presentation-mode {
    min-height: 100vh;
    background: transparent;
    overflow: hidden;
  }

  .nav-arrows {
    position: fixed;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    z-index: 2;
    display: flex;
    justify-content: space-between;
    padding: 0 16px;
    pointer-events: none;
  }

  .nav-arrow {
    pointer-events: auto;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg-card) 90%, transparent);
    color: var(--text-primary);
    font-size: 20px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      background 0.25s ease,
      border-color 0.25s ease,
      color 0.25s ease,
      opacity 0.25s ease,
      transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .nav-arrow:hover:not(:disabled) {
    background: color-mix(in srgb, var(--bg-card) 98%, transparent);
    border-color: var(--border-gold);
    color: var(--gold);
    transform: scale(1.08);
  }

  .nav-arrow:active:not(:disabled) {
    transform: scale(0.96);
  }

  .nav-arrow:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  /* Full-bleed hero */
  .day-hero {
    position: relative;
    width: 100%;
    background-size: cover;
    background-position: center;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
  }

  .day-hero-bg-fixed {
    position: fixed;
    inset: 0;
    z-index: 0;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    pointer-events: none;
  }

  .day-hero.has-image {
    position: absolute;
    inset: 0;
    min-height: 100%;
  }

  .day-hero-spacer {
    height: min(50vh, 420px);
  }

  .day-hero.has-placeholder {
    min-height: min(50vh, 400px);
    aspect-ratio: 16 / 7;
    max-height: min(65vh, 560px);
    background: linear-gradient(
      135deg,
      var(--gold-glow) 0%,
      color-mix(in srgb, var(--gold) 12%, var(--bg-card)) 100%
    );
  }

  .day-hero-overlay {
    position: absolute;
    inset: 0;
    pointer-events: none;
    background: linear-gradient(
      180deg,
      rgba(44, 42, 38, 0.5) 0%,
      rgba(44, 42, 38, 0.25) 40%,
      transparent 70%
    );
  }

  .day-hero-content {
    position: relative;
    z-index: 1;
    width: 100%;
    max-width: 720px;
    margin: 0 auto;
    padding: 32px 48px 0;
    text-align: left;
  }

  .day-hero-title-strip {
    padding: 20px 28px;
    border-radius: var(--radius-lg);
  }

  /* Placeholder: solid card */
  .day-hero.has-placeholder .day-hero-title-strip {
    background: linear-gradient(
      135deg,
      #fff 0%,
      var(--bg-card-start) 50%,
      color-mix(in srgb, var(--gold) 6%, #fff) 100%
    );
    box-shadow: 0 4px 24px rgba(44, 42, 38, 0.15);
    border: 1px solid var(--border-gold);
  }

  .day-hero.has-placeholder .day-hero-title-strip .day-hero-title,
  .day-hero.has-placeholder .day-hero-title-strip .day-hero-subtext {
    color: var(--text-primary);
  }

  .day-hero.has-placeholder .day-hero-title-strip .day-hero-subtext a {
    color: var(--gold);
  }

  .day-hero.has-placeholder .day-hero-title-strip .day-hero-subtext a:hover {
    color: var(--gold-light);
  }

  .day-hero.has-placeholder .day-hero-title-strip .inline-select {
    color: var(--gold);
    border-color: var(--border);
  }

  .day-hero.has-placeholder .day-hero-title-strip .inline-select:hover {
    border-color: var(--border-gold);
    color: var(--gold-dim);
  }

  .day-hero.has-placeholder .day-hero-title-strip .inline-select:focus {
    border-color: var(--gold);
  }

  /* On image: glass card with dark text */
  .day-hero.has-image .day-hero-title-strip {
    background: linear-gradient(
      180deg,
      rgba(255, 253, 248, 0.75) 0%,
      rgba(255, 253, 248, 0.6) 100%
    );
    backdrop-filter: blur(24px) saturate(150%);
    -webkit-backdrop-filter: blur(24px) saturate(150%);
    border: 1px solid rgba(255, 255, 255, 0.6);
    border-top: 1px solid rgba(255, 255, 255, 0.9);
    box-shadow:
      0 4px 24px rgba(44, 42, 38, 0.12),
      inset 0 1px 0 rgba(255, 255, 255, 0.5);
  }

  .day-hero.has-image .day-hero-title-strip .day-hero-title {
    color: var(--text-primary);
    -webkit-text-fill-color: var(--text-primary);
    text-shadow: none;
  }

  .day-hero.has-image .day-hero-title-strip .day-hero-subtext,
  .day-hero.has-image .day-hero-title-strip .day-hero-subtext a {
    color: var(--text-secondary);
    font-weight: 600;
    text-shadow: none;
  }

  .day-hero.has-image .day-hero-title-strip .day-hero-subtext a:hover {
    color: var(--gold);
  }

  .day-hero.has-image .day-hero-title-strip .inline-select {
    color: var(--gold);
    border-color: var(--border);
    background: var(--bg-hover);
  }

  .day-hero.has-image .day-hero-title-strip .inline-select:hover {
    border-color: var(--border-gold);
    color: var(--gold-dim);
  }

  .day-hero.has-image .day-hero-title-strip .inline-select:focus {
    border-color: var(--gold);
  }

  .day-hero-title {
    font-size: 32px;
    font-weight: 700;
    margin: 0 0 4px;
  }

  .day-hero-subtext {
    font-size: 14px;
    margin: 0;
  }

  .day-hero-subtext.edit {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0 6px;
  }

  .day-hero-subtext a {
    text-decoration: none;
  }

  .day-hero-subtext a:hover {
    text-decoration: underline;
  }

  .hero-placeholder-emoji {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 64px;
    opacity: 0.5;
  }

  .hero-upload-overlay {
    position: absolute;
    bottom: 16px;
    right: 16px;
    z-index: 2;
  }

  .slide-content {
    position: relative;
    z-index: 2;
    max-width: 720px;
    margin: -80px auto 0;
    padding: 0 48px 48px;
  }

  .day-content-card {
    background: linear-gradient(
      180deg,
      var(--bg-card-start) 0%,
      var(--bg-card) 100%
    );
    border-radius: var(--radius-lg);
    padding: 0;
    box-shadow:
      0 2px 4px rgba(0, 0, 0, 0.04),
      0 8px 32px rgba(44, 42, 38, 0.1);
    border: 1px solid var(--border);
  }

  @supports (backdrop-filter: blur(1px)) or (-webkit-backdrop-filter: blur(1px)) {
    .day-content-card {
      background: linear-gradient(
        180deg,
        rgba(255, 253, 248, 0.75) 0%,
        rgba(255, 253, 248, 0.6) 50%,
        rgba(255, 253, 248, 0.55) 100%
      );
      backdrop-filter: blur(24px) saturate(150%);
      -webkit-backdrop-filter: blur(24px) saturate(150%);
      box-shadow:
        0 2px 4px rgba(0, 0, 0, 0.04),
        0 8px 32px rgba(44, 42, 38, 0.1),
        inset 0 1px 0 rgba(255, 255, 255, 0.6);
      border: 1px solid rgba(255, 255, 255, 0.5);
      border-top: 1px solid rgba(255, 255, 255, 0.9);
    }
  }

  .day-notes {
    margin-top: 0;
    padding: 20px 24px 24px;
  }

  /* Section headers (markdown rendered in child) */
  .day-notes :global(.markdown-content h2),
  .day-notes :global(.markdown-content h3) {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--text-primary);
    margin: 24px 0 8px;
    font-weight: 600;
    background: var(--gold-glow);
    padding: 8px 12px;
    margin-left: -24px;
    margin-right: -24px;
    padding-left: 24px;
    border-left: 4px solid var(--gold);
    border-radius: 0;
  }

  .day-notes :global(.markdown-content h2:first-of-type),
  .day-notes :global(.markdown-content h3:first-of-type) {
    margin-top: 0;
  }

  .day-notes :global(.markdown-content p) {
    margin-bottom: 12px;
    line-height: 1.65;
  }

  .emoji {
    margin-right: 6px;
  }

  .emoji-slot {
    position: relative;
    display: inline-block;
    margin-right: 6px;
  }

  .emoji-edit-trigger {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px 4px;
    border-radius: var(--radius);
    background: transparent;
    border: 1px solid transparent;
    cursor: pointer;
    transition: background 0.2s, border-color 0.2s;
  }

  .emoji-edit-trigger:hover {
    background: var(--bg-hover);
    border-color: var(--border);
  }

  .emoji-picker-popover {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 8px;
    z-index: 100;
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .emoji-picker-popover emoji-picker {
    width: 320px;
    height: 400px;
  }

  .inline-select {
    font-family: inherit;
    font-size: 14px;
    color: var(--gold);
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius);
    padding: 2px 4px;
    cursor: pointer;
    max-width: 200px;
  }

  .inline-select:hover {
    background: var(--bg-hover);
    border-color: var(--border);
  }

  .inline-select:focus {
    outline: none;
    border-color: var(--border-gold);
  }

  .tagline-row {
    padding: 20px 24px 12px;
    margin-bottom: 0;
  }

  .tagline {
    font-size: 18px;
    font-weight: 600;
    font-style: italic;
    color: var(--text-primary);
    margin: 0;
  }

  .tagline-input {
    width: 100%;
    padding: 6px 0;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 16px;
    background: var(--bg-card);
  }

  .travel-section {
    padding: 0 24px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .travel-subtitle {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--text-primary);
    font-weight: 600;
    background: var(--gold-glow);
    padding: 8px 12px;
    margin-left: -24px;
    margin-right: -24px;
    padding-left: 24px;
    border-left: 4px solid var(--gold);
    border-radius: 0;
  }

  .travel-text {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .travel-input {
    width: 100%;
    padding: 6px 0;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 14px;
    background: var(--bg-card);
  }

  .sleep-section {
    padding: 20px 24px 24px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    border-top: 1px solid var(--border);
  }

  .sleep-subtitle {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--text-primary);
    font-weight: 600;
    background: var(--gold-glow);
    padding: 8px 12px;
    margin-left: -24px;
    margin-right: -24px;
    padding-left: 24px;
    border-left: 4px solid var(--gold);
    border-radius: 0;
  }

  .sleep-select {
    font-family: inherit;
    font-size: 14px;
    color: var(--text-secondary);
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 8px 12px;
    cursor: pointer;
    width: 100%;
    max-width: 320px;
  }

  .sleep-select:hover {
    border-color: var(--border-gold);
  }

  .sleep-select:focus {
    outline: none;
    border-color: var(--gold);
  }

  .sleep-link {
    font-size: 13px;
    color: var(--gold);
    text-decoration: none;
  }

  .sleep-link:hover {
    text-decoration: underline;
    color: var(--gold-light);
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  @media (max-width: 640px) {
    .day-hero.has-placeholder {
      min-height: min(40vh, 320px);
      max-height: min(50vh, 400px);
    }

    .day-hero-content {
      padding: 24px 24px 0;
    }

    .day-hero-title-strip {
      padding: 16px 20px;
    }

    .day-hero-title {
      font-size: 26px;
    }

    .slide-content {
      margin-top: -60px;
      padding: 0 24px 32px;
    }
  }
</style>
