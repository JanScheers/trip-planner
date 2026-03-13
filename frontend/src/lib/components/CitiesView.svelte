<script lang="ts">
  import { api, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { navigate } from '../router';
  import type { AuthUser, City, Day } from '../types';
  import AddCityModal from './AddCityModal.svelte';
  import ListPageShell from './ListPageShell.svelte';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let cities: City[] = $state([]);
  let days: Day[] = $state([]);
  let showAddModal = $state(false);

  let canAdd = $derived(editMode && user?.is_editor);
  let stats = $derived(
    cities.length > 0
      ? [
          `${cities.length} ${cities.length === 1 ? 'city' : 'cities'}`,
          `${days.length} days planned`,
        ]
      : [],
  );

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

{#snippet addAction()}
  <button type="button" class="btn-gold btn-sm" onclick={openAddModal}>Add city</button>
{/snippet}

<ListPageShell
  eyebrow="Destinations"
  title="Cities"
  subtitle="Seven cities, each with its own story"
  stats={stats}
  action={canAdd ? addAction : undefined}
>

  <div class="city-list card card-brochure">
    {#if cities.length === 0}
      <p class="list-empty">Loading...</p>
    {:else}
      <ul class="media-list">
        {#each cities as city}
          {@const cityColor = getCityColor(city.key, cities)}
          {@const dayCount = days.filter((d) => d.city_key === city.key).length}
          <li>
            <a
              href="#/cities/{city.key}"
              class="media-list-link"
              style="--city-color: {cityColor};"
            >
              {#if city.hero_image}
                <img
                  src={staticUrl(city.hero_image)}
                  alt=""
                  class="media-list-tile"
                  loading="lazy"
                />
              {:else}
                <span
                  class="media-list-tile-placeholder"
                  style="--tint: {cityColor}; background: color-mix(in srgb, var(--tint) 18%, var(--gold-glow));"
                >
                  {city.emoji || '🏙️'}
                </span>
              {/if}
              <div class="media-list-body">
                <span class="media-list-title">{city.name}</span>
                {#if city.chinese_name || city.tagline}
                  <span class="media-list-meta">
                    {#if city.chinese_name}
                      <span class="chinese-text">{city.chinese_name}</span>
                    {/if}
                    {#if city.chinese_name && city.tagline}
                      <span class="meta-sep"> · </span>
                    {/if}
                    {#if city.tagline}
                      <span>{city.tagline}</span>
                    {/if}
                  </span>
                {/if}
              </div>
              <span class="media-list-badge">{dayCount} day{dayCount !== 1 ? 's' : ''}</span>
              <span class="media-list-arrow">&rarr;</span>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</ListPageShell>

<AddCityModal
  open={showAddModal}
  onClose={closeAddModal}
  onSuccess={handleCityAdded}
/>

<style>
  .city-list {
    padding: 0;
    overflow: hidden;
  }

  .media-list-link:hover {
    border-left-color: var(--city-color);
  }

  .list-empty {
    color: var(--text-muted);
    padding: 24px 24px;
  }
</style>
