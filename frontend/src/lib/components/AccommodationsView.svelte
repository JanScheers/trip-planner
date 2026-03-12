<script lang="ts">
  import { api, staticUrl } from '../api';
  import type { Accommodation, Day } from '../types';

  let accommodations: Accommodation[] = $state([]);
  let days: Day[] = $state([]);

  $effect(() => {
    loadData();
  });

  async function loadData() {
    [accommodations, days] = await Promise.all([
      api.accommodations.list(),
      api.days.list(),
    ]);
  }
</script>

<div class="acc-list card">
  {#if accommodations.length === 0}
    <p class="text-muted">Loading...</p>
  {:else}
    <ul class="acc-list-ul">
      {#each accommodations as acc}
        <li>
          <a href="#/accommodations/{acc.key}" class="acc-list-link">
            {#if acc.hero_image}
              <img src={staticUrl(acc.hero_image)} alt="" class="acc-list-thumb" loading="lazy" />
            {:else}
              <span class="acc-list-emoji">{acc.emoji || '🏨'}</span>
            {/if}
            <span class="acc-list-name">{acc.name}</span>
            <span class="acc-list-nights"
              >{days.filter(d => d.accommodation_key === acc.key).length} night{days.filter(d => d.accommodation_key === acc.key).length !== 1 ? 's' : ''}</span
            >
            <span class="acc-list-arrow">&rarr;</span>
          </a>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .acc-list {
    padding: 0;
    overflow: hidden;
  }

  .acc-list-ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .acc-list-ul li {
    border-bottom: 1px solid var(--border);
  }

  .acc-list-ul li:last-child {
    border-bottom: none;
  }

  .acc-list-link {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    text-decoration: none;
    color: inherit;
    transition: background 0.2s, border-color 0.2s;
    border-left: 3px solid transparent;
  }

  .acc-list-link:hover {
    background: var(--bg-hover);
    border-left-color: var(--gold);
  }

  .acc-list-emoji {
    font-size: 24px;
    width: 56px;
    height: 56px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-hover);
    border-radius: var(--radius);
    flex-shrink: 0;
  }

  .acc-list-thumb {
    width: 56px;
    height: 56px;
    border-radius: var(--radius);
    object-fit: cover;
    flex-shrink: 0;
    border: 1px solid var(--border);
  }

  .acc-list-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .acc-list-nights {
    margin-left: auto;
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .acc-list-arrow {
    font-size: 14px;
    color: var(--text-muted);
    transition: transform 0.2s;
  }

  .acc-list-link:hover .acc-list-arrow {
    color: var(--gold);
    transform: translateX(4px);
  }

  .text-muted {
    color: var(--text-muted);
    padding: 24px 20px;
  }
</style>
