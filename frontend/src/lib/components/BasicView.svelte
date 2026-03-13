<script lang="ts">
  import { api, staticUrl } from "../api";
  import { getCityColor } from "../cityColors";
  import { formatDate } from "../format";
  import { navigate } from "../router";
  import type { Day, City, Accommodation, AuthUser } from "../types";
  import ListPageShell from "./ListPageShell.svelte";

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } =
    $props();

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

  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));
  let accMap = $derived(
    Object.fromEntries(accommodations.map((a) => [a.key, a])),
  );
  let canEdit = $derived(editMode);
  let stats = $derived(
    days.length > 0
      ? [
          `${days.length} days`,
          `${new Set(days.map((d) => d.city_key)).size} cities`,
          `${new Set(days.map((d) => d.accommodation_key).filter(Boolean)).size} stays`,
        ]
      : [],
  );

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
</script>

{#snippet exportAction()}
  <a href={api.exportUrl} class="btn-gold btn-sm" target="_blank"
    >Export TSV</a
  >
{/snippet}

<ListPageShell
  eyebrow="The Route"
  title="Itinerary"
  subtitle="Beijing to Hong Kong — day by day"
  stats={stats}
  action={canEdit ? exportAction : undefined}
>
  <div class="card card-brochure table-card table-card-editorial" class:edit-mode={canEdit}>
    <table>
      <thead>
        <tr>
          <th class="col-thumb"></th>
          <th class="col-emoji"></th>
          <th class="col-date">Date</th>
          <th class="col-city">City</th>
          <th class="col-acc">Stays</th>
          <th class="col-travel">Travel</th>
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
            if ((e.target as HTMLElement).closest("select, button, a")) return;
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
              <span class="day-row-thumb-placeholder">{day.emoji || "📅"}</span>
            {/if}
          </td>
          <td class="col-emoji">
            {#if day.emoji}<span>{day.emoji}</span>{:else}<span class="day-num"
                >{i + 1}</span
              >{/if}
          </td>
          <td class="col-date">
            <span class="date-text">{formatDate(day.date)}</span>
            {#if day.tagline}
              <div class="day-tagline">{day.tagline}</div>
            {/if}
          </td>
          <td class="col-city">
            {#if canEdit}
              <select
                value={day.city_key}
                onchange={(e) =>
                  updateDayCity(day, (e.target as HTMLSelectElement).value)}
              >
                {#each cities as city}
                  <option value={city.key}>{city.name}</option>
                {/each}
              </select>
            {:else}
              <span class="cell-text">{cityMap[day.city_key]?.name || day.city_key}</span>
            {/if}
          </td>
          <td class="col-acc">
            {#if canEdit}
              <select
                value={day.accommodation_key || ""}
                onchange={(e) =>
                  updateDayAccommodation(
                    day,
                    (e.target as HTMLSelectElement).value,
                  )}
              >
                <option value="">— None —</option>
                {#each accommodations as acc}
                  <option value={acc.key}>{acc.name}</option>
                {/each}
              </select>
            {:else if day.accommodation_key}
              <a href="#/accommodations/{day.accommodation_key}" class="cell-link"
                >{accMap[day.accommodation_key]?.name || day.accommodation_key}</a
              >
            {:else}
              <span class="cell-text text-muted">—</span>
            {/if}
          </td>
          <td class="col-travel">
            {#if day.travel}
              <span class="travel-text">{day.travel}</span>
            {:else}
              <span class="text-muted">—</span>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  </div>
</ListPageShell>

<style>
  .table-card {
    overflow-x: auto;
    padding: 0;
  }

  .table-card-editorial {
    border: var(--brochure-card-border);
    box-shadow: var(--brochure-card-shadow);
  }

  .table-card {
    position: relative;
  }

  .table-card table {
    margin: 0;
    table-layout: fixed;
  }

  .table-card thead {
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .table-card th {
    padding: 16px 20px;
    background: linear-gradient(
      180deg,
      var(--bg-hover) 0%,
      var(--bg-secondary) 100%
    );
    color: var(--gold-dim);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .table-card th:first-child {
    border-radius: var(--radius-lg) 0 0 0;
  }

  .table-card th:last-child {
    border-radius: 0 var(--radius-lg) 0 0;
  }

  .table-card td {
    padding: 14px 20px;
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

  .col-date {
    width: 110px;
  }

  .col-city {
    width: 140px;
  }

  .col-acc {
    width: 200px;
  }

  .col-travel {
    width: 220px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .cell-text {
    display: block;
  }

  .cell-link {
    color: var(--gold);
    text-decoration: none;
  }

  .cell-link:hover {
    text-decoration: underline;
    color: var(--gold-light);
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
    width: 100%;
    min-width: 0;
    box-sizing: border-box;
    padding: 2px 6px;
    font-size: inherit;
    line-height: 1.6;
    border: none;
    background: transparent;
    border-radius: var(--radius);
    cursor: pointer;
  }

  .edit-mode select:hover,
  .edit-mode select:focus {
    background: var(--bg-hover);
    box-shadow: inset 0 0 0 1px var(--border);
  }

  .text-muted {
    color: var(--text-muted);
  }
</style>
