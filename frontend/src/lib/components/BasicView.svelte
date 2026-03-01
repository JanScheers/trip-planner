<script lang="ts">
  import { api } from '../api';
  import type { Day, City, Accommodation, AuthUser } from '../types';

  let { user }: { user: AuthUser | null } = $props();

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
  let canEdit = $derived(user?.is_editor ?? false);

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

  function getCityColor(key: string): string {
    const colors: Record<string, string> = {
      beijing: '#e74c3c',
      xian: '#e67e22',
      chengdu: '#f1c40f',
      chongqing: '#2ecc71',
      zhangjiajie: '#1abc9c',
      guilin: '#3498db',
      hongkong: '#9b59b6',
    };
    return colors[key] || '#d4a843';
  }
</script>

<div class="page-header">
  <h1>Itinerary</h1>
  <div class="header-actions">
    <a href={api.exportUrl} class="btn-outline btn-sm" target="_blank">Export TSV</a>
  </div>
</div>

<div class="card table-card">
  <table>
    <thead>
      <tr>
        <th class="col-num">#</th>
        <th>Date</th>
        <th>City</th>
        <th>Accommodation</th>
        <th class="col-emoji"></th>
        {#if canEdit}<th class="col-actions"></th>{/if}
      </tr>
    </thead>
    <tbody>
      {#each days as day, i}
        {@const prevCity = i > 0 ? days[i - 1].city_key : null}
        {@const isNewCity = day.city_key !== prevCity}
        <tr class:new-city={isNewCity}>
          <td class="day-num">{i + 1}</td>
          <td>
            <a href="#/days/{day.id}" class="date-link">{formatDate(day.date)}</a>
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
              <a href="#/cities/{day.city_key}" class="city-link">
                <span class="city-indicator" style="background: {getCityColor(day.city_key)};"></span>
                {#if cityMap[day.city_key]?.emoji}{cityMap[day.city_key].emoji} {/if}{cityMap[day.city_key]?.name || day.city_key}
              </a>
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
              <a href="#/accommodations/{day.accommodation_key}">
                {accMap[day.accommodation_key]?.name || day.accommodation_key}
              </a>
            {:else}
              <span class="text-muted">—</span>
            {/if}
          </td>
          <td class="col-emoji">
            {#if day.emoji}<span>{day.emoji}</span>{/if}
          </td>
          {#if canEdit}
            <td>
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
  .header-actions {
    margin-left: auto;
  }

  .table-card {
    overflow-x: auto;
    padding: 0;
  }

  table {
    margin: 0;
  }

  th {
    padding: 14px 16px;
    background: var(--bg-secondary);
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

  .col-num { width: 40px; }
  .col-emoji { width: 40px; text-align: center; }
  .col-actions { width: 50px; }

  .day-num {
    color: var(--text-muted);
    font-size: 12px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .date-link {
    white-space: nowrap;
    font-weight: 500;
  }

  .city-link {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .city-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
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
