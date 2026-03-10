<script lang="ts">
  import { api, staticUrl } from '../api';
  import type { Accommodation, Day, AuthUser } from '../types';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { key, user, editMode }: { key: string; user: AuthUser | null; editMode: boolean } = $props();

  let acc: Accommodation | null = $state(null);
  let days: Day[] = $state([]);

  let canEdit = $derived(editMode);
  let accDays = $derived(days.filter(d => d.accommodation_key === key));

  $effect(() => {
    const accKey = key;
    loadData(accKey);
  });

  async function loadData(accKey: string) {
    [acc, days] = await Promise.all([
      api.accommodations.get(accKey),
      api.days.list(),
    ]);
  }

  async function updateField(updates: Record<string, any>) {
    if (!acc) return;
    acc = await api.accommodations.update(acc.key, updates);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }
</script>

<a href="#/days" class="back-link">&larr; Back to Itinerary</a>

{#if acc}
  <div class="page-header">
    {#if acc.emoji}<span class="emoji-large">{acc.emoji}</span>{/if}
    <div>
      <h1>{acc.name}</h1>
      {#if acc.link}
        <a href={acc.link} target="_blank" rel="noreferrer" class="ext-link">View listing &rarr;</a>
      {/if}
    </div>
  </div>

  {#if canEdit}
    <ImageUpload
      currentImage={acc.hero_image}
      onUpload={(url) => updateField({ hero_image: url })}
    />
  {:else if acc.hero_image}
    <img src={staticUrl(acc.hero_image)} alt={acc.name} class="hero-image" />
  {/if}

  <div class="detail-grid">
    {#if canEdit}
      <div class="card">
        <div class="field-group">
          <label for="acc-name">Name</label>
          <input
            id="acc-name"
            type="text"
            value={acc.name}
            onchange={(e) => updateField({ name: (e.target as HTMLInputElement).value })}
          />
        </div>
        <div class="field-group">
          <label for="acc-link">Listing Link</label>
          <input
            id="acc-link"
            type="url"
            value={acc.link}
            onchange={(e) => updateField({ link: (e.target as HTMLInputElement).value })}
            placeholder="https://airbnb.com/..."
          />
        </div>
        <div class="field-group">
          <label for="acc-emoji">Emoji</label>
          <input
            id="acc-emoji"
            type="text"
            value={acc.emoji || ''}
            onchange={(e) => updateField({ emoji: (e.target as HTMLInputElement).value || null })}
            placeholder="e.g. 🏠"
            style="max-width: 80px;"
          />
        </div>
      </div>
    {/if}

    <div class="card">
      <h3 class="section-title">Notes</h3>
      <MarkdownEditor
        value={acc.notes}
        readonly={!canEdit}
        onSave={(val) => updateField({ notes: val })}
      />
    </div>

    {#if accDays.length > 0}
      <div class="card">
        <h3 class="section-title">Booked for {accDays.length} night{accDays.length > 1 ? 's' : ''}</h3>
        <table>
          <thead>
            <tr>
              <th>Date</th>
            </tr>
          </thead>
          <tbody>
            {#each accDays as day}
              <tr>
                <td><a href="#/days/{day.id}">{formatDate(day.date)}</a></td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
{:else}
  <p>Loading...</p>
{/if}

<style>
  .ext-link {
    font-size: 13px;
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
