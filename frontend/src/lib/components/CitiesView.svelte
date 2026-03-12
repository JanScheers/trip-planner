<script lang="ts">
  import { api, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { navigate } from '../router';
  import type { AuthUser, City, CreateCity, Day } from '../types';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let cities: City[] = $state([]);
  let days: Day[] = $state([]);
  let showAddModal = $state(false);
  let addName = $state('');
  let addChineseName = $state('');
  let addTagline = $state('');
  let addLat = $state('');
  let addLng = $state('');
  let addError = $state('');

  let canAdd = $derived(editMode && user?.is_editor);

  $effect(() => {
    loadData();
  });

  async function loadData() {
    [cities, days] = await Promise.all([
      api.cities.list(),
      api.days.list(),
    ]);
  }

  function openAddModal() {
    addName = '';
    addChineseName = '';
    addTagline = '';
    addLat = '';
    addLng = '';
    addError = '';
    showAddModal = true;
  }

  function closeAddModal() {
    showAddModal = false;
  }

  async function submitAdd() {
    addError = '';
    if (!String(addName ?? '').trim()) {
      addError = 'Name is required';
      return;
    }
    const parseCoord = (s: string | number): number | null => {
      const v = parseFloat(String(s ?? '').trim());
      return isNaN(v) ? null : v;
    };
    const lat = parseCoord(addLat);
    const lng = parseCoord(addLng);
    if (String(addLat ?? '').trim() && lat === null) {
      addError = 'Latitude must be a valid number';
      return;
    }
    if (String(addLng ?? '').trim() && lng === null) {
      addError = 'Longitude must be a valid number';
      return;
    }
    if (lat !== null && (lat < -90 || lat > 90)) {
      addError = 'Latitude must be between -90 and 90';
      return;
    }
    if (lng !== null && (lng < -180 || lng > 180)) {
      addError = 'Longitude must be between -180 and 180';
      return;
    }
    try {
      const data: CreateCity = {
        name: String(addName ?? '').trim(),
        chinese_name: String(addChineseName ?? '').trim() || undefined,
        tagline: String(addTagline ?? '').trim() || undefined,
        lat: lat ?? undefined,
        lng: lng ?? undefined,
      };
      const city = await api.cities.create(data);
      closeAddModal();
      await loadData();
      navigate(`/cities/${city.key}`);
    } catch (err) {
      addError = err instanceof Error ? err.message : String(err);
    }
  }
</script>

<div class="cities-section">
  <div class="section-header">
    <h2 class="section-title">Cities</h2>
    {#if canAdd}
      <button type="button" class="btn-gold btn-sm" onclick={openAddModal}>Add city</button>
    {/if}
  </div>

  <div class="city-list card">
  {#if cities.length === 0}
    <p class="text-muted">Loading...</p>
  {:else}
    <ul class="city-list-ul">
      {#each cities as city}
        <li>
          <a href="#/cities/{city.key}" class="city-list-link" style="--city-color: {getCityColor(city.key, cities)};">
            {#if city.hero_image}
              <img src={staticUrl(city.hero_image)} alt="" class="city-list-thumb" loading="lazy" />
            {:else}
              <span class="city-list-dot" style="background: {getCityColor(city.key, cities)};"></span>
            {/if}
            <span class="city-list-name">{city.name}</span>
            {#if city.chinese_name}
              <span class="city-list-chinese chinese-text">{city.chinese_name}</span>
            {/if}
            <span class="city-list-days">{days.filter(d => d.city_key === city.key).length} days</span>
            <span class="city-list-arrow">&rarr;</span>
          </a>
        </li>
      {/each}
    </ul>
  {/if}
  </div>
</div>

{#if showAddModal}
  <div class="modal-overlay" role="dialog" aria-modal="true">
    <div class="modal-card modal-form">
      <h2 class="modal-title">Add city</h2>
      <form
        novalidate
        onsubmit={(e) => {
          e.preventDefault();
          submitAdd();
        }}
      >
        <label for="add-city-name">Name</label>
        <input
          id="add-city-name"
          type="text"
          bind:value={addName}
          placeholder="Shanghai"
          required
        />
        <label for="add-city-chinese">Chinese name (optional)</label>
        <input
          id="add-city-chinese"
          type="text"
          class="chinese-text"
          bind:value={addChineseName}
          placeholder="上海"
        />
        <label for="add-city-tagline">Tagline (optional)</label>
        <input
          id="add-city-tagline"
          type="text"
          bind:value={addTagline}
          placeholder="e.g. The Bund, French Concession & skyline"
        />
        <label for="add-city-lat">Latitude (optional)</label>
        <input
          id="add-city-lat"
          type="number"
          step="any"
          bind:value={addLat}
          placeholder="e.g. 39.9042"
        />
        <label for="add-city-lng">Longitude (optional)</label>
        <input
          id="add-city-lng"
          type="number"
          step="any"
          bind:value={addLng}
          placeholder="e.g. 116.4074"
        />
        {#if addError}
          <p class="form-error">{addError}</p>
        {/if}
        <div class="modal-actions">
          <button type="button" class="btn-outline btn-sm" onclick={closeAddModal}>Cancel</button>
          <button type="submit" class="btn-gold btn-sm">Add</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .cities-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .section-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .modal-overlay {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(44, 42, 38, 0.5);
    padding: 20px;
  }

  .modal-card.modal-form {
    max-width: 400px;
    width: 100%;
    background: var(--gradient-card);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 24px;
    box-shadow: 0 8px 32px rgba(44, 42, 38, 0.2);
  }

  .modal-form form {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 16px;
  }

  .modal-form label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .form-error {
    font-size: 13px;
    color: var(--danger);
    margin: 0;
  }

  .modal-form .modal-actions {
    margin-top: 8px;
  }

  .modal-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .city-list {
    padding: 0;
    overflow: hidden;
  }

  .city-list-ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .city-list-ul li {
    border-bottom: 1px solid var(--border);
  }

  .city-list-ul li:last-child {
    border-bottom: none;
  }

  .city-list-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    text-decoration: none;
    color: inherit;
    transition: background 0.2s, border-color 0.2s;
    border-left: 3px solid transparent;
  }

  .city-list-link:hover {
    background: var(--bg-hover);
    border-left-color: var(--city-color);
  }

  .city-list-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .city-list-thumb {
    width: 56px;
    height: 56px;
    border-radius: var(--radius);
    object-fit: cover;
    flex-shrink: 0;
    border: 1px solid var(--border);
  }

  .city-list-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .city-list-chinese {
    font-size: 13px;
    color: var(--text-muted);
  }

  .city-list-days {
    margin-left: auto;
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .city-list-arrow {
    font-size: 14px;
    color: var(--text-muted);
    transition: transform 0.2s;
  }

  .city-list-link:hover .city-list-arrow {
    color: var(--city-color);
    transform: translateX(4px);
  }

  .text-muted {
    color: var(--text-muted);
    padding: 24px 20px;
  }
</style>
