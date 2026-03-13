<script lang="ts">
  import "emoji-picker-element";
  import { api, staticUrl } from "../api";
  import { formatDateLong, isTypingElement } from "../format";
  import { navigate } from "../router";
  import type { Day, City, Accommodation, AuthUser, UpdateDay } from "../types";
  import AddAccommodationModal from "./AddAccommodationModal.svelte";
  import AddCityModal from "./AddCityModal.svelte";
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
  let showAddCityModal = $state(false);
  let showAddAccModal = $state(false);

  let canAdd = $derived(editMode && user?.is_editor);
  let citySelectValue = $state("");
  let accSelectValue = $state("");

  $effect(() => {
    if (day) {
      citySelectValue = day.city_key;
      accSelectValue = day.accommodation_key ?? "";
    }
  });

  let dayIds = $derived(allDays.map((d) => d.id));
  let dayIndex = $derived.by(() => (day ? dayIds.indexOf(day.id) : -1));
  let prevId = $derived(dayIndex > 0 ? dayIds[dayIndex - 1] : undefined);
  let nextId = $derived(
    dayIndex >= 0 && dayIndex < dayIds.length - 1
      ? dayIds[dayIndex + 1]
      : undefined,
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
    return () =>
      el.removeEventListener("emoji-click", handler as EventListener);
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

  async function handleCityAdded(city: City) {
    await updateField({ city_key: city.key });
    cities = await api.cities.list();
  }

  async function handleAccommodationAdded(acc: Accommodation) {
    await updateField({ accommodation_key: acc.key });
    accommodations = await api.accommodations.list();
  }
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
    {#if editable}
      <div class="day-hero-subtext edit">
        <span class="inline-select-wrap">
          <select
            class="inline-select"
            aria-label="City"
            bind:value={citySelectValue}
            onchange={(e) => {
              const v = (e.target as HTMLSelectElement).value;
              if (v === "__add_city__") {
                showAddCityModal = true;
                citySelectValue = day!.city_key;
              } else {
                updateField({ city_key: v });
              }
            }}
          >
            {#each cities as city}
              <option value={city.key}>{city.name}</option>
            {/each}
            {#if canAdd}
              <option value="__add_city__">+ Add new city...</option>
            {/if}
          </select>
          {#if canAdd}
            <button
              type="button"
              class="btn-gold btn-sm add-city-btn"
              onclick={() => (showAddCityModal = true)}
              aria-label="Add city"
            >
              + Add city
            </button>
          {/if}
        </span>
      </div>
    {/if}
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
      <div class="day-city-display">
        {#if cityMap[day.city_key]?.chinese_name}
          <span class="day-city-chinese chinese-text">
            {cityMap[day.city_key].chinese_name}
          </span>
        {/if}
        <a href="#/cities/{day.city_key}" class="day-city-english">
          {cityMap[day.city_key]?.name || day.city_key}
        </a>
      </div>
    </div>

    {#if displayHeroImage}
      <div class="day-hero-spacer" aria-hidden="true"></div>
    {/if}

    <!-- 2. Content card (overlaps hero) -->
    <div class="slide-content">
      <div class="day-content-card">
        <div class="day-content-title">
          {@render heroContent(day, editMode)}
        </div>
        {#if day.tagline || editMode}
          <div class="tagline-row">
            {#if editMode}
              <input
                type="text"
                class="tagline-input"
                value={day.tagline || ""}
                onchange={(e) =>
                  updateField({
                    tagline: (e.target as HTMLInputElement).value,
                  })}
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
            <span class="sleep-subtitle">Overnight</span>
            {#if editMode}
              <div class="sleep-select-wrap">
                <select
                  class="sleep-select"
                  aria-label="Sleep"
                  bind:value={accSelectValue}
                  onchange={(e) => {
                    const v = (e.target as HTMLSelectElement).value;
                    if (v === "__add_acc__") {
                      showAddAccModal = true;
                      accSelectValue = day!.accommodation_key ?? "";
                    } else {
                      updateField({
                        accommodation_key: v || null,
                      });
                    }
                  }}
                >
                  <option value="">— None —</option>
                  {#each accommodations as acc}
                    <option value={acc.key}>{acc.name}</option>
                  {/each}
                  {#if canAdd}
                    <option value="__add_acc__">+ Add new stay...</option>
                  {/if}
                </select>
                {#if canAdd}
                  <button
                    type="button"
                    class="btn-gold btn-sm"
                    onclick={() => (showAddAccModal = true)}
                    aria-label="Add stay"
                  >
                    + Add stay
                  </button>
                {/if}
              </div>
            {:else if day.accommodation_key}
              <a
                href="#/accommodations/{day.accommodation_key}"
                class="sleep-link"
                >{accMap[day.accommodation_key]?.name ||
                  day.accommodation_key}</a
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

<AddCityModal
  open={showAddCityModal}
  onClose={() => (showAddCityModal = false)}
  onSuccess={handleCityAdded}
/>
<AddAccommodationModal
  open={showAddAccModal}
  onClose={() => (showAddAccModal = false)}
  onSuccess={handleAccommodationAdded}
/>

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

  .day-content-title {
    padding: 20px 24px 16px;
    margin-bottom: 0;
  }

  .day-content-title .day-hero-title {
    color: var(--text-primary);
  }

  .day-content-title .day-hero-subtext {
    color: var(--text-secondary);
  }

  .day-content-title .inline-select {
    color: var(--gold);
    border-color: var(--border);
  }

  .day-content-title .inline-select:hover {
    border-color: var(--border-gold);
    color: var(--gold-dim);
  }

  .day-content-title .inline-select:focus {
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

  /* City display overlapping hero (like Home page) */
  .day-city-display {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding: 72px 24px 32px;
    text-align: center;
    pointer-events: none;
  }

  .day-city-display .day-city-english {
    pointer-events: auto;
  }

  .day-city-chinese {
    font-size: 72px;
    font-weight: 700;
    letter-spacing: 0.05em;
    background: linear-gradient(
      135deg,
      var(--gold) 0%,
      var(--gold-light) 50%,
      var(--gold-dim) 100%
    );
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 6px;
  }

  .day-city-english {
    font-size: 20px;
    color: #fff;
    font-weight: 400;
    letter-spacing: 0.2em;
    text-transform: uppercase;
    text-decoration: none;
    text-shadow:
      0 0 3px rgba(138, 109, 10, 0.8),
      0 0 6px rgba(138, 109, 10, 0.5),
      0 1px 2px rgba(44, 42, 38, 0.4);
    transition: color 0.2s;
  }

  .day-city-english:hover {
    color: var(--gold-light);
  }

  /* Placeholder hero: darker text for contrast */
  .day-hero.has-placeholder .day-city-chinese {
    -webkit-text-fill-color: var(--gold);
    background: none;
  }

  .day-hero.has-placeholder .day-city-english {
    color: var(--text-primary);
    text-shadow: none;
  }

  .day-hero.has-placeholder .day-city-english:hover {
    color: var(--gold);
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
        0 8px 32px rgba(44, 42, 38, 0.1);
    }
  }

  .day-notes {
    margin-top: 0;
    padding: 0 24px 24px;
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

  .day-notes :global(.markdown-content hr) {
    display: none;
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
    transition:
      background 0.2s,
      border-color 0.2s;
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

  .inline-select-wrap {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-wrap: nowrap;
    width: 100%;
  }

  .inline-select-wrap .add-city-btn {
    margin-left: auto;
  }

  .sleep-select-wrap {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
  }

  .tagline-row {
    padding: 0 24px 16px;
    margin-bottom: 0;
  }

  .tagline {
    font-size: 18px;
    font-weight: 600;
    font-style: italic;
    color: var(--text-secondary);
    margin: 0;
  }

  .tagline-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 16px;
    font-weight: 600;
    font-style: italic;
    color: var(--text-primary);
    background: var(--bg-card);
    font-family: inherit;
    transition:
      border-color var(--transition),
      box-shadow var(--transition);
  }

  .tagline-input::placeholder {
    color: var(--text-muted);
    font-weight: 500;
    font-style: normal;
  }

  .tagline-input:hover {
    border-color: var(--border-gold);
  }

  .tagline-input:focus {
    outline: none;
    border-color: var(--gold);
    box-shadow: 0 0 0 2px var(--gold-glow);
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
    font-family: inherit;
    background: var(--gold-glow);
    padding: 8px 12px;
    margin-left: -24px;
    margin-right: -24px;
    padding-left: 24px;
    border-left: 4px solid var(--gold);
    border-radius: 0;
  }

  .travel-text {
    font-size: 16px;
    line-height: 1.65;
    color: var(--text-primary);
    font-family: inherit;
  }

  .travel-input {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 16px;
    color: var(--text-primary);
    background: var(--bg-card);
    font-family: inherit;
    transition:
      border-color var(--transition),
      box-shadow var(--transition);
  }

  .travel-input::placeholder {
    color: var(--text-muted);
  }

  .travel-input:hover {
    border-color: var(--border-gold);
  }

  .travel-input:focus {
    outline: none;
    border-color: var(--gold);
    box-shadow: 0 0 0 2px var(--gold-glow);
  }

  .sleep-section {
    padding: 12px 24px 24px;
    display: flex;
    flex-direction: column;
    gap: 8px;
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
    font-size: 15px;
    color: var(--text-primary);
    text-decoration: none;
  }

  .sleep-link:hover {
    text-decoration: underline;
    color: var(--gold);
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

    .day-content-title {
      padding: 16px 20px 12px;
    }

    .day-hero-title {
      font-size: 26px;
    }

    .day-city-chinese {
      font-size: 52px;
    }

    .day-city-english {
      font-size: 18px;
    }

    .day-city-display {
      padding: 56px 20px 24px;
    }

    .slide-content {
      margin-top: -60px;
      padding: 0 24px 32px;
    }
  }
</style>
