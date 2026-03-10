<script lang="ts">
  import { api, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { navigate } from '../router';
  import type { Day, City, Accommodation, AuthUser } from '../types';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let days: Day[] = $state([]);
  let cities: City[] = $state([]);
  let accommodations: Accommodation[] = $state([]);

  $effect(() => {
    loadData();
  });

  async function loadData() {
    [days, cities, accommodations] = await Promise.all([
      api.days.list(),
      api.cities.list(),
      api.accommodations.list(),
    ]);
  }

  let cityMap = $derived(Object.fromEntries(cities.map(c => [c.key, c])));
  let accMap = $derived(Object.fromEntries(accommodations.map(a => [a.key, a])));
  let canEdit = $derived(editMode);
  function dayThumbUrl(day: Day): string | null {
    return day.hero_image || cityMap[day.city_key]?.hero_image || null;
  }

  async function updateDayCity(day: Day, cityKey: string) {
    await api.days.update(day.id, { city_key: cityKey });
    await loadData();
  }

  async function updateDayAccommodation(day: Day, accKey: string) {
    const key = accKey || null;
    await api.days.update(day.id, { accommodation_key: key });
    await loadData();
  }

  async function addDay() {
    const last = days[days.length - 1];
    if (!last) return;
    const nextDate = new Date(last.date + 'T00:00:00');
    nextDate.setDate(nextDate.getDate() + 1);
    const dateStr = nextDate.toISOString().slice(0, 10);
    await api.days.create({
      date: dateStr,
      city_key: last.city_key,
      accommodation_key: last.accommodation_key,
    });
    await loadData();
  }

  async function deleteDay(id: number) {
    if (!confirm('Delete this day?')) return;
    await api.days.delete(id);
    await loadData();
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }
</script>

<div class="card table-card">
  <table>
    <thead>
      <tr>
        <th class="col-thumb"></th>
        <th class="col-emoji"></th>
        <th>Date</th>
        <th>City</th>
        <th>Accommodation</th>
        <th>Travel</th>
        {#if canEdit}
          <th class="col-actions">
            <a href={api.exportUrl} class="btn-outline btn-sm" target="_blank">Export TSV</a>
          </th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each days as day, i}
        {@const prevCity = i > 0 ? days[i - 1].city_key : null}
        {@const isNewCity = day.city_key !== prevCity}
        {@const thumb = dayThumbUrl(day)}
        <tr
          class="clickable-row"
          class:new-city={isNewCity}
          style="--city-color: {getCityColor(day.city_key, cities)};"
          onclick={(e: MouseEvent) => {
            if ((e.target as HTMLElement).closest('select, button, a')) return;
            navigate(`/days/${day.id}`);
          }}
        >
          <td class="col-thumb">
            {#if thumb}
              <img src={staticUrl(thumb)} alt="" class="day-row-thumb" loading="lazy" />
            {:else}
              <span class="day-row-thumb-placeholder">{day.emoji || '📅'}</span>
            {/if}
          </td>
          <td class="col-emoji">
            {#if day.emoji}<span>{day.emoji}</span>{:else}<span class="day-num">{i + 1}</span>{/if}
          </td>
          <td>
            <span class="date-text">{formatDate(day.date)}</span>
            {#if day.tagline}
              <div class="day-tagline">{day.tagline}</div>
            {/if}
          </td>
          <td>
            {#if canEdit}
              <select
                value={day.city_key}
                onchange={(e) => updateDayCity(day, (e.target as HTMLSelectElement).value)}
              >
                {#each cities as city}
                  <option value={city.key}>{city.name}</option>
                {/each}
              </select>
            {:else}
              {cityMap[day.city_key]?.name || day.city_key}
            {/if}
          </td>
          <td>
            {#if canEdit}
              <select
                value={day.accommodation_key || ''}
                onchange={(e) => updateDayAccommodation(day, (e.target as HTMLSelectElement).value)}
              >
                <option value="">— None —</option>
                {#each accommodations as acc}
                  <option value={acc.key}>{acc.name}</option>
                {/each}
              </select>
            {:else if day.accommodation_key}
              <span>{accMap[day.accommodation_key]?.name || day.accommodation_key}</span>
            {:else}
              <span class="text-muted">—</span>
            {/if}
          </td>
          <td class="col-travel">
            {#if day.travel}
              <span class="travel-text">{day.travel}</span>
            {:else}
              <span class="text-muted">—</span>
            {/if}
          </td>
          {#if canEdit}
            <td class="col-actions">
              <button class="btn-danger btn-sm" onclick={() => deleteDay(day.id)}>×</button>
            </td>
          {/if}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

{#if canEdit}
  <div style="margin-top: 16px; display: flex; gap: 8px;">
    <button class="btn-gold" onclick={addDay}>+ Add Day</button>
  </div>
{/if}

<style>
  .table-card {
    overflow-x: auto;
    padding: 0;
    background: var(--gradient-card) !important;
    box-shadow: 0 2px 12px rgba(44, 42, 38, 0.06);
  }

  .col-actions {
    text-align: right;
    white-space: nowrap;
  }

  table {
    margin: 0;
  }

  th {
    padding: 14px 16px;
    background: linear-gradient(180deg, var(--bg-hover) 0%, var(--bg-secondary) 100%);
    color: var(--gold-dim);
  }

  th:first-child {
    border-radius: var(--radius-lg) 0 0 0;
  }

  th:last-child {
    border-radius: 0 var(--radius-lg) 0 0;
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

  .col-emoji { width: 40px; text-align: center; }

  .col-travel {
    max-width: 220px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .travel-text {
    white-space: normal;
    line-height: 1.3;
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
    font-variant-numeric: tabular-nums;
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

  .new-city td {
    border-top: 2px solid var(--border-gold);
  }

  select {
    width: auto;
    min-width: 120px;
    padding: 4px 8px;
    font-size: 13px;
  }

  .text-muted {
    color: var(--text-muted);
  }
</style>
