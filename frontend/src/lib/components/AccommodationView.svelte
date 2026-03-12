<script lang="ts">
  import 'emoji-picker-element';
  import { api, staticUrl } from '../api';
  import { navigate } from '../router';
  import type { Accommodation, Day, AuthUser } from '../types';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { key, user, editMode }: { key: string; user: AuthUser | null; editMode: boolean } = $props();

  let acc: Accommodation | null = $state(null);
  let days: Day[] = $state([]);
  let accommodations: Accommodation[] = $state([]);
  let showEmojiPicker = $state(false);
  let pickerEl: HTMLElement | null = $state(null);

  let canEdit = $derived(editMode);
  let accDays = $derived(days.filter(d => d.accommodation_key === key));
  let accKeys = $derived(accommodations.map((a) => a.key));
  let prevKey = $derived.by(() => {
    const i = accKeys.indexOf(key);
    return i > 0 ? accKeys[i - 1] : undefined;
  });
  let nextKey = $derived.by(() => {
    const i = accKeys.indexOf(key);
    return i >= 0 && i < accKeys.length - 1 ? accKeys[i + 1] : undefined;
  });

  $effect(() => {
    const accKey = key;
    loadData(accKey);
  });

  async function loadData(accKey: string) {
    const [accData, daysData, accsData] = await Promise.all([
      api.accommodations.get(accKey),
      api.days.list(),
      api.accommodations.list(),
    ]);
    acc = accData;
    days = daysData;
    accommodations = accsData;
  }

  async function updateField(updates: Record<string, any>) {
    if (!acc) return;
    acc = await api.accommodations.update(acc.key, updates);
  }

  function formatDate(dateStr: string): string {
    const d = new Date(dateStr + 'T00:00:00');
    return d.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
  }

  $effect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== 'ArrowLeft' && e.key !== 'ArrowRight') return;
      if (e.key === 'ArrowLeft' && prevKey != null) {
        e.preventDefault();
        navigate(`/accommodations/${prevKey}`);
      } else if (e.key === 'ArrowRight' && nextKey != null) {
        e.preventDefault();
        navigate(`/accommodations/${nextKey}`);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  });

  function handleEmojiClick(e: CustomEvent<{ unicode: string }>) {
    const unicode = e.detail?.unicode;
    if (unicode) {
      updateField({ emoji: unicode });
      showEmojiPicker = false;
    }
  }

  $effect(() => {
    const el = pickerEl;
    if (!el) return;
    el.addEventListener('emoji-click', handleEmojiClick as EventListener);
    return () => el.removeEventListener('emoji-click', handleEmojiClick as EventListener);
  });

  let emojiContainerEl: HTMLElement | null = $state(null);

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as Node;
    if (showEmojiPicker && emojiContainerEl && !emojiContainerEl.contains(target)) {
      showEmojiPicker = false;
    }
  }

  $effect(() => {
    if (showEmojiPicker) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });
</script>

{#if acc}
  <div class="page-header-wrap">
    <button
      type="button"
      class="nav-arrow nav-arrow-left"
      disabled={prevKey == null}
      onclick={() => prevKey != null && navigate(`/accommodations/${prevKey}`)}
      aria-label="Previous accommodation"
    >
      ←
    </button>
    <div class="page-header">
      <div class="emoji-slot" bind:this={emojiContainerEl}>
        {#if canEdit}
          <button
            type="button"
            class="emoji-edit-trigger"
            onclick={(e) => { e.stopPropagation(); showEmojiPicker = !showEmojiPicker; }}
            aria-label="Change emoji"
            aria-expanded={showEmojiPicker}
            aria-haspopup="dialog"
          >
            <span class="emoji-large">{acc.emoji || '🏨'}</span>
          </button>
          {#if showEmojiPicker}
            <div class="emoji-picker-popover">
              <emoji-picker
                class="light"
                bind:this={pickerEl}
              ></emoji-picker>
            </div>
          {/if}
        {:else}
          {#if acc.emoji}<span class="emoji-large">{acc.emoji}</span>{/if}
        {/if}
      </div>
      <div class="header-fields">
        {#if canEdit}
          <input
            type="text"
            class="header-name-input"
            value={acc.name}
            onblur={(e) => updateField({ name: (e.target as HTMLInputElement).value })}
            onkeydown={(e) => e.key === 'Enter' && (e.target as HTMLInputElement).blur()}
            aria-label="Accommodation name"
          />
          <input
            type="url"
            class="header-link-input"
            value={acc.link || ''}
            placeholder="https://airbnb.com/..."
            onblur={(e) => updateField({ link: (e.target as HTMLInputElement).value || null })}
            onkeydown={(e) => e.key === 'Enter' && (e.target as HTMLInputElement).blur()}
            aria-label="Listing link"
          />
        {:else}
          <h1>{acc.name}</h1>
          {#if acc.link}
            <a href={acc.link} target="_blank" rel="noreferrer" class="ext-link">{acc.link}</a>
          {/if}
        {/if}
      </div>
    </div>
    <button
      type="button"
      class="nav-arrow nav-arrow-right"
      disabled={nextKey == null}
      onclick={() => nextKey != null && navigate(`/accommodations/${nextKey}`)}
      aria-label="Next accommodation"
    >
      →
    </button>
  </div>

  {#if canEdit}
    <ImageUpload
      currentImage={acc.hero_image}
      onUpload={(url) => updateField({ hero_image: url })}
    />
  {:else if acc.hero_image}
    <img src={staticUrl(acc.hero_image)} alt={acc.name} class="hero-image" loading="lazy" />
  {:else}
    <div class="hero-placeholder" aria-hidden="true">
      <span class="hero-placeholder-emoji">{acc.emoji || '🏨'}</span>
    </div>
  {/if}

  <div class="detail-grid">
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
  .page-header-wrap {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 24px;
  }

  .page-header-wrap .page-header {
    flex: 1;
    margin-bottom: 0;
  }

  .nav-arrow {
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg-card) 90%, transparent);
    color: var(--text-primary);
    font-size: 20px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      background 0.25s ease,
      border-color 0.25s ease,
      color 0.25s ease,
      opacity 0.25s ease,
      transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .nav-arrow:hover:not(:disabled) {
    background: color-mix(in srgb, var(--bg-card) 98%, transparent);
    border-color: var(--border-gold);
    color: var(--gold);
    transform: scale(1.08);
  }

  .nav-arrow:active:not(:disabled) {
    transform: scale(0.96);
  }

  .nav-arrow:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .header-fields {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
    min-width: 0;
  }

  .header-name-input,
  .header-link-input {
    font-family: inherit;
    font-size: 28px;
    font-weight: 700;
    color: var(--text-primary);
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius);
    padding: 2px 6px;
    margin: -2px -6px;
    width: 100%;
    transition: background 0.2s, border-color 0.2s;
  }

  .header-name-input:hover,
  .header-link-input:hover {
    background: var(--bg-hover);
    border-color: var(--border);
  }

  .header-name-input:focus,
  .header-link-input:focus {
    outline: none;
    background: var(--bg-card);
    border-color: var(--border-gold);
  }

  .header-link-input {
    font-size: 13px;
    font-weight: 400;
    color: var(--gold);
  }

  .ext-link {
    font-size: 13px;
    word-break: break-all;
    overflow-wrap: break-word;
  }

  .emoji-slot {
    position: relative;
    flex-shrink: 0;
  }

  .emoji-edit-trigger {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: var(--radius);
    background: transparent;
    border: 1px solid transparent;
    cursor: pointer;
    transition: background 0.2s, border-color 0.2s;
  }

  .emoji-edit-trigger:hover {
    background: var(--bg-hover);
    border-color: var(--border);
  }

  .emoji-picker-popover {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 8px;
    z-index: 100;
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .emoji-picker-popover emoji-picker {
    width: 320px;
    height: 400px;
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
