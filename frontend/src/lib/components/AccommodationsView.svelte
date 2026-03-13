<script lang="ts">
  import { api, staticUrl } from '../api';
  import { navigate } from '../router';
  import type { Accommodation, AuthUser, Day } from '../types';
  import AddAccommodationModal from './AddAccommodationModal.svelte';
  import ListPageShell from './ListPageShell.svelte';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let accommodations: Accommodation[] = $state([]);
  let days: Day[] = $state([]);
  let showAddModal = $state(false);

  let canAdd = $derived(editMode && user?.is_editor);
  let stats = $derived(
    accommodations.length > 0
      ? [
          `${accommodations.length} ${accommodations.length === 1 ? 'stay' : 'stays'}`,
          `${days.filter((d) => d.accommodation_key).length} night${days.filter((d) => d.accommodation_key).length !== 1 ? 's' : ''} booked`,
        ]
      : [],
  );

  $effect(() => {
    loadData();
  });

  async function loadData() {
    [accommodations, days] = await Promise.all([
      api.accommodations.list(),
      api.days.list(),
    ]);
  }

  function openAddModal() {
    showAddModal = true;
  }

  function closeAddModal() {
    showAddModal = false;
  }

  async function handleAccommodationAdded(acc: Accommodation) {
    await loadData();
    navigate(`/accommodations/${acc.key}`);
  }
</script>

{#snippet addAction()}
  <button type="button" class="btn-gold btn-sm" onclick={openAddModal}>Add stay</button>
{/snippet}

<ListPageShell
  eyebrow="Where we stay"
  title="Stays"
  subtitle="Accommodations across the journey"
  stats={stats}
  action={canAdd ? addAction : undefined}
>
  <div class="acc-list card card-brochure">
    {#if accommodations.length === 0}
      <p class="list-empty">Loading...</p>
    {:else}
      <ul class="media-list">
        {#each accommodations as acc}
          {@const nightCount = days.filter((d) => d.accommodation_key === acc.key).length}
          <li>
            <a href="#/accommodations/{acc.key}" class="media-list-link">
              {#if acc.hero_image}
                <img
                  src={staticUrl(acc.hero_image)}
                  alt=""
                  class="media-list-tile"
                  loading="lazy"
                />
              {:else}
                <span class="media-list-tile-placeholder">
                  {acc.emoji || '🏨'}
                </span>
              {/if}
              <div class="media-list-body">
                <span class="media-list-title">{acc.name}</span>
                {#if acc.notes}
                  <span class="media-list-meta">{acc.notes}</span>
                {/if}
              </div>
              <span class="media-list-badge"
                >{nightCount} night{nightCount !== 1 ? 's' : ''}</span
              >
              <span class="media-list-arrow">&rarr;</span>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</ListPageShell>

<AddAccommodationModal
  open={showAddModal}
  onClose={closeAddModal}
  onSuccess={handleAccommodationAdded}
/>

<style>
  .acc-list {
    padding: 0;
    overflow: hidden;
  }

  .media-list-link:hover {
    border-left-color: var(--gold);
  }

  .list-empty {
    color: var(--text-muted);
    padding: 24px 24px;
  }
</style>
