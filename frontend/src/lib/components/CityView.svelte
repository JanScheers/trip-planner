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

  async function addDay() {
    if (cityDays.length === 0 || !city) return;
    const lastCityDay = cityDays[cityDays.length - 1];
    const nextDate = new Date(lastCityDay.date + 'T00:00:00');
    nextDate.setDate(nextDate.getDate() + 1);
    const insertionDate = nextDate.toISOString().slice(0, 10);
    const newDay = await api.days.create({
      date: insertionDate,
      city_key: city.key,
      accommodation_key: lastCityDay.accommodation_key,
    });
    const toShift = days.filter((d) => d.date >= insertionDate && d.id !== newDay.id);
    toShift.sort((a, b) => b.date.localeCompare(a.date));
    for (const d of toShift) {
      const dNext = new Date(d.date + 'T00:00:00');
      dNext.setDate(dNext.getDate() + 1);
      await api.days.update(d.id, { date: dNext.toISOString().slice(0, 10) });
    }
    await loadData(key);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }
</script>

{#if city}
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

    <div class="card">
      <h3 class="section-title">Days in {city.name} ({cityDays.length})</h3>
      <table>
        <thead>
          <tr>
            <th>Date</th>
            <th>Emoji</th>
          </tr>
        </thead>
        <tbody>
          {#each cityDays as day}
            <tr>
              <td><a href="#/days/{day.id}">{formatDate(day.date)}</a></td>
              <td>{day.emoji || ''}</td>
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
    margin-bottom: 12px;
  }
</style>
