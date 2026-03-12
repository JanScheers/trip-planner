<script lang="ts">
  import "emoji-picker-element";
  import { api, staticUrl } from "../api";
  import { navigate } from "../router";
  import type { Day, City, Accommodation, AuthUser } from "../types";
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

  let canEdit = $derived(editMode);
  let dayIds = $derived(allDays.map((d) => d.id));
  let prevId = $derived.by(() => {
    if (!day) return undefined;
    const i = dayIds.indexOf(day.id);
    return i > 0 ? dayIds[i - 1] : undefined;
  });
  let nextId = $derived.by(() => {
    if (!day) return undefined;
    const i = dayIds.indexOf(day.id);
    return i >= 0 && i < dayIds.length - 1 ? dayIds[i + 1] : undefined;
  });
  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));
  let accMap = $derived(
    Object.fromEntries(accommodations.map((a) => [a.key, a])),
  );
  let displayHeroImage = $derived.by(() => {
    const d = day;
    if (!d) return null;
    return d.hero_image || (cityMap[d.city_key]?.hero_image ?? null) || null;
  });

  $effect(() => {
    const dayId = id;
    loadData(dayId);
  });

  async function loadData(dayId: number) {
    [day, allDays, cities, accommodations] = await Promise.all([
      api.days.get(dayId),
      api.days.list(),
      api.cities.list(),
      api.accommodations.list(),
    ]);
  }

  function isTypingElement(el: EventTarget | null): boolean {
    if (!el || !(el instanceof HTMLElement)) return false;
    const tag = el.tagName.toLowerCase();
    const role = el.getAttribute("role");
    if (tag === "input" || tag === "textarea" || tag === "select") return true;
    if (el.isContentEditable) return true;
    if (role === "textbox" || role === "searchbox") return true;
    return false;
  }

  $effect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
      if (isTypingElement(e.target as HTMLElement)) return;
      if (e.key === "ArrowLeft" && prevId != null) {
        e.preventDefault();
        navigate(`/days/${prevId}`);
      } else if (e.key === "ArrowRight" && nextId != null) {
        e.preventDefault();
        navigate(`/days/${nextId}`);
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  });

  async function updateField(updates: Record<string, any>) {
    if (!day) return;
    day = await api.days.update(day.id, updates);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + "T00:00:00");
    return d.toLocaleDateString("en-US", {
      weekday: "long",
      month: "long",
      day: "numeric",
      year: "numeric",
    });
  }

  function handleEmojiClick(e: CustomEvent<{ unicode: string }>) {
    const unicode = e.detail?.unicode;
    if (unicode) {
      updateField({ emoji: unicode });
      showEmojiPicker = false;
    }
  }

  $effect(() => {
    const el = pickerEl;
    if (!el) return;
    el.addEventListener("emoji-click", handleEmojiClick as EventListener);
    return () =>
      el.removeEventListener("emoji-click", handleEmojiClick as EventListener);
  });

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as Node;
    if (
      showEmojiPicker &&
      emojiContainerEl &&
      !emojiContainerEl.contains(target)
    ) {
      showEmojiPicker = false;
    }
  }

  $effect(() => {
    if (showEmojiPicker) {
      document.addEventListener("click", handleClickOutside);
      return () =>
        document.removeEventListener("click", handleClickOutside);
    }
  });
</script>

{#if day}
  <div
    bind:this={slideshowEl}
    class="day-slideshow"
    class:presentation-mode={presentationMode}
    role="region"
    aria-label="Day slide"
  >
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

    <!-- 1. Hero (full-bleed, image behind) -->
    <div
      class="day-hero"
      class:has-image={displayHeroImage != null && !canEdit}
      class:has-placeholder={!displayHeroImage && !canEdit}
      class:edit-mode={canEdit}
      style={displayHeroImage && !canEdit
        ? `background-image: url(${staticUrl(displayHeroImage)})`
        : undefined}
    >
      {#if canEdit}
        <div class="hero-upload">
          <ImageUpload
            currentImage={day.hero_image}
            onUpload={(url) => updateField({ hero_image: url })}
          />
        </div>
      {:else if !displayHeroImage}
        <span class="hero-placeholder-emoji">{day.emoji || "📅"}</span>
        <div class="day-hero-content on-placeholder">
          <h1 class="day-hero-title">
            {#if day.emoji}<span class="emoji">{day.emoji}</span>{/if}
            {formatDate(day.date)}
          </h1>
          <p class="day-hero-subtext">
            <a href="#/cities/{day.city_key}"
              >{cityMap[day.city_key]?.name || day.city_key}</a
            >
            {#if day.accommodation_key}
              <span class="sep">·</span>
              <a href="#/accommodations/{day.accommodation_key}"
                >{accMap[day.accommodation_key]?.name || day.accommodation_key}</a
              >
            {/if}
          </p>
        </div>
      {:else}
        <div class="day-hero-overlay" aria-hidden="true"></div>
        <div class="day-hero-content on-image">
          <h1 class="day-hero-title">
            {#if day.emoji}<span class="emoji">{day.emoji}</span>{/if}
            {formatDate(day.date)}
          </h1>
          <p class="day-hero-subtext">
            <a href="#/cities/{day.city_key}"
              >{cityMap[day.city_key]?.name || day.city_key}</a
            >
            {#if day.accommodation_key}
              <span class="sep">·</span>
              <a href="#/accommodations/{day.accommodation_key}"
                >{accMap[day.accommodation_key]?.name || day.accommodation_key}</a
              >
            {/if}
          </p>
        </div>
      {/if}
    </div>

    <!-- 2. Content card (overlaps hero) -->
    <div class="slide-content">
      <div class="day-content-card">
        {#if canEdit}
          <div class="notion-header edit-header">
            <h1 class="notion-title">
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
                  <span class="emoji">{day.emoji || "📅"}</span>
                </button>
                {#if showEmojiPicker}
                  <div class="emoji-picker-popover">
                    <emoji-picker
                      class="light"
                      bind:this={pickerEl}
                    ></emoji-picker>
                  </div>
                {/if}
              </div>
              {formatDate(day.date)}
            </h1>
            <p class="notion-subtext notion-subtext-edit">
              <select
                class="inline-select"
                aria-label="City"
                value={day.city_key}
                onchange={(e) =>
                  updateField({
                    city_key: (e.target as HTMLSelectElement).value,
                  })}
              >
                {#each cities as city}
                  <option value={city.key}>{city.name}</option>
                {/each}
              </select>
              <span class="sep">·</span>
              <select
                class="inline-select"
                aria-label="Accommodation"
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
            </p>
          </div>
        {/if}
        {#if day.tagline || canEdit}
          <div class="tagline-row">
            {#if canEdit}
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
        {#if day.travel || canEdit}
          <div class="travel-row">
            {#if canEdit}
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
      </div>

      <!-- 3. Description (outside card, countdown-label styling) -->
      <div class="day-notes">
        <div class="notes-block">
          <MarkdownEditor
            value={day.notes}
            readonly={!canEdit}
            onSave={(val) => updateField({ notes: val })}
          />
        </div>
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
  }

  .day-slideshow.presentation-mode {
    min-height: 100vh;
    background: transparent;
    overflow: hidden;
  }

  /* When used as scroll container (day page, not presentation), parent sets flex + overflow-y: auto */
  .day-slideshow:not(.presentation-mode) {
    overflow: visible;
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
    min-height: min(50vh, 400px);
    aspect-ratio: 16 / 7;
    max-height: min(65vh, 560px);
    background-size: cover;
    background-position: center;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
  }

  .day-hero.edit-mode {
    min-height: 200px;
    aspect-ratio: auto;
    max-height: 280px;
    justify-content: center;
  }

  .day-hero.has-placeholder {
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
      rgba(44, 42, 38, 0.4) 0%,
      rgba(44, 42, 38, 0.2) 30%,
      transparent 55%,
      rgba(245, 243, 239, 0.5) 85%,
      var(--bg-primary) 100%
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

  .day-hero-content.on-image .day-hero-title,
  .day-hero-content.on-image .day-hero-subtext,
  .day-hero-content.on-image .day-hero-subtext a {
    color: #fff;
    text-shadow:
      0 0 4px rgba(44, 42, 38, 0.8),
      0 1px 3px rgba(0, 0, 0, 0.5);
  }

  .day-hero-content.on-image .sep {
    color: rgba(255, 255, 255, 0.85);
  }

  .day-hero-content.on-image .day-hero-subtext a:hover {
    color: var(--gold-soft);
  }

  .day-hero-content.on-placeholder .day-hero-title,
  .day-hero-content.on-placeholder .day-hero-subtext {
    color: var(--text-primary);
  }

  .day-hero-content.on-placeholder .day-hero-subtext a {
    color: var(--gold);
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

  .hero-upload {
    padding: 16px;
  }

  .slide-content {
    position: relative;
    z-index: 2;
    max-width: 720px;
    margin: -80px auto 0;
    padding: 0 48px 48px;
  }

  .day-content-card {
    background: var(--bg-card);
    border-radius: var(--radius-lg);
    padding: 24px;
    box-shadow: 0 4px 24px rgba(44, 42, 38, 0.12);
    border: 1px solid var(--border);
  }

  .day-notes {
    margin-top: 24px;
  }

  /* Departure-in style for section headers (markdown rendered in child) */
  .day-notes :global(.markdown-content h2),
  .day-notes :global(.markdown-content h3) {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--gold-dim);
    margin: 24px 0 8px;
    font-weight: 600;
  }

  .day-notes :global(.markdown-content h2:first-of-type) {
    margin-top: 0;
  }

  .day-notes :global(.markdown-content p) {
    margin-bottom: 12px;
    line-height: 1.65;
  }

  .notion-header {
    margin-bottom: 24px;
  }

  .notion-header.edit-header {
    margin-bottom: 16px;
  }

  .notion-title {
    font-size: 32px;
    font-weight: 700;
    margin: 0 0 4px;
    color: var(--text-primary);
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

  .notion-subtext-edit {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0 6px;
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

  .notion-subtext {
    font-size: 14px;
    color: var(--text-primary);
    margin: 0 0 8px;
  }

  .sep {
    margin: 0 6px;
    color: var(--text-muted);
  }

  .tagline-row {
    margin-bottom: 6px;
  }

  .tagline {
    font-size: 16px;
    font-weight: 500;
    color: var(--text-primary);
    margin: 0;
  }

  .tagline-input {
    width: 100%;
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 16px;
    background: var(--bg-card);
  }

  .travel-row {
    margin-bottom: 0;
  }

  .travel-text {
    font-size: 13px;
    color: var(--text-secondary);
  }

  .travel-input {
    width: 100%;
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 14px;
    background: var(--bg-card);
  }

  .notes-block {
    margin-top: 0;
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
    .day-hero {
      min-height: min(40vh, 320px);
      max-height: min(50vh, 400px);
    }

    .day-hero-content {
      padding: 24px 24px 0;
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
