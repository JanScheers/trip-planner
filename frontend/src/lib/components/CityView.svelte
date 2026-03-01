<script lang="ts">
  import { api } from '../api';
  import type { City, Day, AuthUser } from '../types';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { key, user }: { key: string; user: AuthUser | null } = $props();

  let city: City | null = $state(null);
  let days: Day[] = $state([]);

  let canEdit = $derived(user?.is_editor ?? false);
  let cityDays = $derived(days.filter(d => d.city_key === key));

  $effect(() => {
    loadData();
  });

  async function loadData() {
    [city, days] = await Promise.all([
      api.cities.get(key),
      api.days.list(),
    ]);
  }

  async function updateField(updates: Record<string, any>) {
    if (!city) return;
    city = await api.cities.update(city.key, updates);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }
</script>

<a href="#/" class="back-link">&larr; Back to Home</a>

{#if city}
  <div class="page-header">
    {#if city.emoji}<span class="emoji-large">{city.emoji}</span>{/if}
    <div>
      <h1>{city.name}</h1>
      {#if city.chinese_name}
        <span class="chinese-name chinese-text">{city.chinese_name}</span>
      {/if}
    </div>
  </div>

  {#if canEdit}
    <ImageUpload
      currentImage={city.hero_image}
      onUpload={(url) => updateField({ hero_image: url })}
    />
  {:else if city.hero_image}
    <img src="http://localhost:8080{city.hero_image}" alt={city.name} class="hero-image" />
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
      </div>
    {/if}

    <div class="card">
      <h3 class="section-title">Notes</h3>
      <MarkdownEditor
        value={city.notes}
        readonly={!canEdit}
        onSave={(val) => updateField({ notes: val })}
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
