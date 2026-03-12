<script lang="ts">
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

    <div class="slide-content">
      <!-- 1. Hero -->
      <div
        class="notion-hero"
        class:has-image={displayHeroImage != null && !canEdit}
        class:has-placeholder={!displayHeroImage && !canEdit}
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
        {/if}
      </div>

      <!-- 2. Title & subtext -->
      <div class="notion-header">
        <h1 class="notion-title">
          {#if day.emoji}<span class="emoji">{day.emoji}</span>{/if}
          {formatDate(day.date)}
        </h1>
        <p class="notion-subtext">
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

      <!-- 3. Content -->
      <div class="notion-body">
        {#if canEdit}
          <div class="edit-fields card">
            <div class="field-group">
              <label for="day-city">City</label>
              <select
                id="day-city"
                value={day.city_key}
                onchange={(e) =>
                  updateField({ city_key: (e.target as HTMLSelectElement).value })}
              >
                {#each cities as city}
                  <option value={city.key}>{city.name}</option>
                {/each}
              </select>
            </div>
            <div class="field-group">
              <label for="day-accommodation">Accommodation</label>
              <select
                id="day-accommodation"
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
            </div>
            <div class="field-group">
              <label for="day-emoji">Emoji</label>
              <input
                id="day-emoji"
                type="text"
                value={day.emoji || ""}
                onchange={(e) =>
                  updateField({
                    emoji: (e.target as HTMLInputElement).value || null,
                  })}
                placeholder="e.g. 🏯"
                style="max-width: 80px;"
              />
            </div>
          </div>
        {/if}

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
    overflow: hidden;
    background: transparent;
    min-height: calc(100vh - 56px);
  }

  .day-slideshow.presentation-mode {
    min-height: 100vh;
    background: transparent;
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
    width: 48px;
    height: 48px;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg-card) 90%, transparent);
    color: var(--text-primary);
    font-size: 24px;
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
    transform: scale(1.12);
  }

  .nav-arrow:active:not(:disabled) {
    transform: scale(0.96);
  }

  .nav-arrow:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .slide-content {
    position: relative;
    z-index: 1;
    max-width: 720px;
    margin: 0 auto;
    padding: 0 48px 48px;
  }

  .notion-hero {
    width: 100%;
    aspect-ratio: 16 / 9;
    max-height: 280px;
    background-size: cover;
    background-position: center;
    margin-bottom: 24px;
    border-radius: var(--radius);
    overflow: hidden;
  }

  .notion-hero.has-placeholder {
    background: linear-gradient(
      135deg,
      var(--gold-glow) 0%,
      color-mix(in srgb, var(--gold) 12%, var(--bg-card)) 100%
    );
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .hero-placeholder-emoji {
    font-size: 48px;
    opacity: 0.6;
  }

  .hero-upload {
    padding: 16px;
  }

  .notion-header {
    margin-bottom: 24px;
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

  .notion-subtext {
    font-size: 14px;
    color: var(--text-secondary);
    margin: 0 0 8px;
  }

  .notion-subtext a {
    color: var(--gold);
    text-decoration: none;
  }

  .notion-subtext a:hover {
    text-decoration: underline;
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
    color: var(--text-muted);
  }

  .travel-input {
    width: 100%;
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 14px;
    background: var(--bg-card);
  }

  .notion-body {
    margin-top: 0;
  }

  .edit-fields {
    margin-bottom: 20px;
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
</style>
