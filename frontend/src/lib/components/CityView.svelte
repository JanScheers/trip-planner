<script lang="ts">
  import { api, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { navigate } from '../router';
  import type { City, Day, AuthUser } from '../types';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { key, user, editMode }: { key: string; user: AuthUser | null; editMode: boolean } = $props();

  let city: City | null = $state(null);
  let days: Day[] = $state([]);
  let cities: City[] = $state([]);

  let canEdit = $derived(editMode);
  let cityDays = $derived(days.filter(d => d.city_key === key));
  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));
  let cityKeys = $derived(cities.map((c) => c.key));
  let prevKey = $derived.by(() => {
    const i = cityKeys.indexOf(key);
    return i > 0 ? cityKeys[i - 1] : undefined;
  });
  let nextKey = $derived.by(() => {
    const i = cityKeys.indexOf(key);
    return i >= 0 && i < cityKeys.length - 1 ? cityKeys[i + 1] : undefined;
  });

  function dayThumbUrl(day: Day): string | null {
    return day.hero_image || cityMap[day.city_key]?.hero_image || null;
  }

  $effect(() => {
    const cityKey = key;
    loadData(cityKey);
  });

  async function loadData(cityKey: string) {
    const [cityData, daysData, citiesData] = await Promise.all([
      api.cities.get(cityKey),
      api.days.list(),
      api.cities.list(),
    ]);
    city = cityData;
    days = daysData;
    cities = citiesData;
  }

  async function updateField(updates: Record<string, any>) {
    if (!city) return;
    city = await api.cities.update(city.key, updates);
  }

  async function shiftDaysFrom(insertionDate: string, excludeId: number) {
    const toShift = days.filter((d) => d.date >= insertionDate && d.id !== excludeId);
    toShift.sort((a, b) => b.date.localeCompare(a.date));
    for (const d of toShift) {
      const dNext = new Date(d.date + 'T00:00:00');
      dNext.setDate(dNext.getDate() + 1);
      await api.days.update(d.id, { date: dNext.toISOString().slice(0, 10) });
    }
  }

  async function addDayAfter(day: Day) {
    if (!city) return;
    const nextDate = new Date(day.date + 'T00:00:00');
    nextDate.setDate(nextDate.getDate() + 1);
    const insertionDate = nextDate.toISOString().slice(0, 10);
    const newDay = await api.days.create({
      date: insertionDate,
      city_key: city.key,
      accommodation_key: day.accommodation_key,
    });
    await shiftDaysFrom(insertionDate, newDay.id);
    await loadData(key);
  }

  async function addDayBefore(day: Day) {
    if (!city) return;
    const insertionDate = day.date;
    const idx = cityDays.findIndex((d) => d.id === day.id);
    const prevCityDay = idx > 0 ? cityDays[idx - 1] : null;
    const accommodation_key = prevCityDay?.accommodation_key ?? day.accommodation_key;
    const newDay = await api.days.create({
      date: insertionDate,
      city_key: city.key,
      accommodation_key,
    });
    await shiftDaysFrom(insertionDate, newDay.id);
    await loadData(key);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }

  $effect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight') return;
      if (e.key === 'ArrowLeft' && prevKey != null) {
        e.preventDefault();
        navigate(`/cities/${prevKey}`);
      } else if (e.key === 'ArrowRight' && nextKey != null) {
        e.preventDefault();
        navigate(`/cities/${nextKey}`);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  });
</script>

{#if city}
  <div class="page-header-wrap">
    <button
      type="button"
      class="nav-arrow nav-arrow-left"
      disabled={prevKey == null}
      onclick={() => prevKey != null && navigate(`/cities/${prevKey}`)}
      aria-label="Previous city"
    >
      ←
    </button>
    <div class="page-header">
      {#if city.emoji}<span class="emoji-large">{city.emoji}</span>{/if}
      <div>
        <h1>{city.name}</h1>
        {#if city.chinese_name}
          <span class="chinese-name chinese-text">{city.chinese_name}</span>
        {/if}
        {#if city.tagline}
          <p class="city-tagline">{city.tagline}</p>
        {/if}
      </div>
    </div>
    <button
      type="button"
      class="nav-arrow nav-arrow-right"
      disabled={nextKey == null}
      onclick={() => nextKey != null && navigate(`/cities/${nextKey}`)}
      aria-label="Next city"
    >
      →
    </button>
  </div>

  {#if canEdit}
    <ImageUpload
      currentImage={city.hero_image}
      onUpload={(url) => updateField({ hero_image: url })}
    />
  {:else if city.hero_image}
    <img src={staticUrl(city.hero_image)} alt={city.name} class="hero-image" loading="lazy" />
  {:else}
    <div class="hero-placeholder" aria-hidden="true">
      <span class="hero-placeholder-emoji">{city.emoji || '🏙️'}</span>
    </div>
  {/if}

  <div class="detail-grid">
    {#if canEdit}
      <div class="card">
        <div class="field-group">
          <label for="city-name">Name</label>
          <input
            id="city-name"
            type="text"
            value={city.name}
            onchange={(e) => updateField({ name: (e.target as HTMLInputElement).value })}
          />
        </div>
        <div class="field-group">
          <label for="city-chinese-name">Chinese Name</label>
          <input
            id="city-chinese-name"
            type="text"
            value={city.chinese_name}
            onchange={(e) => updateField({ chinese_name: (e.target as HTMLInputElement).value })}
            class="chinese-text"
            placeholder="e.g. 北京"
          />
        </div>
        <div class="field-group">
          <label for="city-emoji">Emoji</label>
          <input
            id="city-emoji"
            type="text"
            value={city.emoji || ''}
            onchange={(e) => updateField({ emoji: (e.target as HTMLInputElement).value || null })}
            placeholder="e.g. 🏯"
            style="max-width: 80px;"
          />
        </div>
        <div class="field-group">
          <label for="city-tagline">Tagline</label>
          <input
            id="city-tagline"
            type="text"
            value={city.tagline}
            onchange={(e) => updateField({ tagline: (e.target as HTMLInputElement).value })}
            placeholder="e.g. The Forbidden City, Great Wall & imperial grandeur"
          />
        </div>
      </div>
    {/if}

    <div class="card">
      <h3 class="section-title">About</h3>
      <MarkdownEditor
        value={city.description}
        readonly={!canEdit}
        onSave={(val) => updateField({ description: val })}
      />
    </div>

    <div class="card table-card">
      <div class="table-card-header">
        <h3 class="section-title">{cityDays.length} {cityDays.length === 1 ? 'day' : 'days'} in {city.name}</h3>
      </div>
      <table>
        <tbody>
          {#each cityDays as day}
            {@const thumb = dayThumbUrl(day)}
            <tr
              class="clickable-row"
              style="--city-color: {getCityColor(city.key, cities)};"
              onclick={(e: MouseEvent) => {
                if ((e.target as HTMLElement).closest('a, button, .col-actions')) return;
                navigate(`/days/${day.id}`);
              }}
            >
              <td class="col-thumb">
                {#if thumb}
                  <img
                    src={staticUrl(thumb)}
                    alt=""
                    class="day-row-thumb"
                    loading="lazy"
                  />
                {:else}
                  <span class="day-row-thumb-placeholder">{day.emoji || '📅'}</span>
                {/if}
              </td>
              <td class="col-emoji">
                {#if day.emoji}<span>{day.emoji}</span>{:else}<span class="day-num">—</span>{/if}
              </td>
              <td>
                <a href="#/days/{day.id}" class="date-text">{formatDate(day.date)}</a>
                {#if day.tagline}
                  <div class="day-tagline">{day.tagline}</div>
                {/if}
              </td>
              {#if canEdit}
                <td class="col-actions">
                  <button type="button" class="btn-add-day" onclick={(e) => { e.stopPropagation(); addDayBefore(day); }}>Add before</button>
                  <span class="actions-sep">·</span>
                  <button type="button" class="btn-add-day" onclick={(e) => { e.stopPropagation(); addDayAfter(day); }}>Add after</button>
                </td>
              {/if}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
{:else}
  <p>Loading...</p>
{/if}

<style>
  .page-header-wrap {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
  }

  .page-header-wrap .page-header {
    flex: 1;
    margin-bottom: 0;
  }

  .nav-arrow {
    flex-shrink: 0;
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

  .chinese-name {
    font-size: 18px;
    color: var(--text-secondary);
  }

  .city-tagline {
    font-size: 14px;
    color: var(--text-muted);
    font-style: italic;
    margin-top: 4px;
  }

  .detail-grid {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-title {
    color: var(--gold);
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0;
  }

  .table-card {
    overflow-x: auto;
    padding: 0;
    background: var(--gradient-card) !important;
    box-shadow: 0 2px 12px rgba(44, 42, 38, 0.06);
  }

  .table-card-header {
    padding: 24px 24px 12px;
  }

  table {
    margin: 0;
  }

  td {
    padding: 12px 16px;
  }

  .col-thumb {
    width: 48px;
    padding: 8px 12px;
    vertical-align: middle;
  }

  .day-row-thumb {
    width: 40px;
    height: 40px;
    border-radius: var(--radius);
    object-fit: cover;
    display: block;
    border: 1px solid var(--border);
  }

  .day-row-thumb-placeholder {
    width: 40px;
    height: 40px;
    border-radius: var(--radius);
    background: var(--gold-glow);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
  }

  .col-emoji {
    width: 40px;
    text-align: center;
  }

  .clickable-row {
    cursor: pointer;
    background: color-mix(in srgb, var(--city-color) 8%, var(--bg-card));
    transition: background 0.15s ease;
  }

  .clickable-row:hover {
    background: color-mix(in srgb, var(--city-color) 18%, var(--bg-card));
  }

  .clickable-row:hover :global(td) {
    background: transparent;
  }

  .day-num {
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 600;
  }

  .date-text {
    white-space: nowrap;
    font-weight: 500;
  }

  .day-tagline {
    color: var(--text-muted);
    font-size: 12px;
    margin-top: 2px;
  }

  .col-actions {
    white-space: nowrap;
    text-align: right;
    padding-right: 16px;
    vertical-align: middle;
  }

  .btn-add-day {
    background: none;
    border: none;
    padding: 0;
    font-size: 12px;
    color: var(--text-muted);
    cursor: pointer;
    text-decoration: none;
  }

  .btn-add-day:hover {
    color: var(--gold);
    text-decoration: underline;
  }

  .actions-sep {
    color: var(--border);
    margin: 0 6px;
    font-size: 12px;
  }
</style>
