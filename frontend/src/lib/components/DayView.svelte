<script lang="ts">
  import { api, staticUrl } from '../api';
  import type { Day, City, Accommodation, AuthUser } from '../types';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { id, user, editMode }: { id: number; user: AuthUser | null; editMode: boolean } = $props();

  let day: Day | null = $state(null);
  let cities: City[] = $state([]);
  let accommodations: Accommodation[] = $state([]);

  let canEdit = $derived(editMode);
  let cityMap = $derived(Object.fromEntries(cities.map(c => [c.key, c])));
  let accMap = $derived(Object.fromEntries(accommodations.map(a => [a.key, a])));
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
    [day, cities, accommodations] = await Promise.all([
      api.days.get(dayId),
      api.cities.list(),
      api.accommodations.list(),
    ]);
  }

  async function updateField(updates: Record<string, any>) {
    if (!day) return;
    day = await api.days.update(day.id, updates);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' });
  }
</script>

<a href="#/days" class="back-link">&larr; Back to Itinerary</a>

{#if day}
  <div class="page-header">
    {#if day.emoji}<span class="emoji-large">{day.emoji}</span>{/if}
    <div>
      <h1>{formatDate(day.date)}</h1>
      <div class="subtitle">
        <a href="#/cities/{day.city_key}">{cityMap[day.city_key]?.name || day.city_key}</a>
        {#if day.accommodation_key}
          &middot; <a href="#/accommodations/{day.accommodation_key}">{accMap[day.accommodation_key]?.name || day.accommodation_key}</a>
        {/if}
      </div>
    </div>
  </div>

  {#if canEdit}
    <ImageUpload
      currentImage={day.hero_image}
      onUpload={(url) => updateField({ hero_image: url })}
    />
  {:else if displayHeroImage}
    <img src={staticUrl(displayHeroImage)} alt="Hero" class="hero-image" loading="lazy" />
  {:else}
    <div class="hero-placeholder" aria-hidden="true">
      <span class="hero-placeholder-emoji">{day.emoji || '📅'}</span>
    </div>
  {/if}

  <div class="detail-grid">
    {#if canEdit}
      <div class="card">
        <div class="field-group">
          <label for="day-city">City</label>
          <select
            id="day-city"
            value={day.city_key}
            onchange={(e) => updateField({ city_key: (e.target as HTMLSelectElement).value })}
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
            value={day.accommodation_key || ''}
            onchange={(e) => updateField({ accommodation_key: (e.target as HTMLSelectElement).value || null })}
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
            value={day.emoji || ''}
            onchange={(e) => updateField({ emoji: (e.target as HTMLInputElement).value || null })}
            placeholder="e.g. 🏯"
            style="max-width: 80px;"
          />
        </div>
      </div>
    {/if}

    <div class="card">
      <h3 class="section-title">Notes</h3>
      <MarkdownEditor
        value={day.notes}
        readonly={!canEdit}
        onSave={(val) => updateField({ notes: val })}
      />
    </div>
  </div>
{:else}
  <p>Loading...</p>
{/if}

<style>
  .subtitle {
    font-size: 14px;
    color: var(--text-secondary);
    margin-top: 2px;
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
