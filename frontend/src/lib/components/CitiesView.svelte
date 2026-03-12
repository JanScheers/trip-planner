<script lang="ts">
  import { api, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { navigate } from '../router';
  import type { AuthUser, City, Day } from '../types';
  import AddCityModal from './AddCityModal.svelte';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let cities: City[] = $state([]);
  let days: Day[] = $state([]);
  let showAddModal = $state(false);

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
    showAddModal = true;
  }

  function closeAddModal() {
    showAddModal = false;
  }

  async function handleCityAdded(city: City) {
    await loadData();
    navigate(`/cities/${city.key}`);
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

<AddCityModal
  open={showAddModal}
  onClose={closeAddModal}
  onSuccess={handleCityAdded}
/>

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
