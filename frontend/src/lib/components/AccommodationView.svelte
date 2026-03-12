<script lang="ts">
  import 'emoji-picker-element';
  import { api, parseApiError, staticUrl } from '../api';
  import { getCityColor } from '../cityColors';
  import { formatDate } from '../format';
  import { navigate } from '../router';
  import type { Accommodation, City, Day, AuthUser } from '../types';
  import ConfirmModal from './ConfirmModal.svelte';
  import ImageUpload from './ImageUpload.svelte';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { key, user, editMode }: { key: string; user: AuthUser | null; editMode: boolean } = $props();

  let acc: Accommodation | null = $state(null);
  let days: Day[] = $state([]);
  let accommodations: Accommodation[] = $state([]);
  let cities: City[] = $state([]);
  let showEmojiPicker = $state(false);
  let pickerEl: HTMLElement | null = $state(null);

  let canEdit = $derived(editMode);
  let accDays = $derived(days.filter(d => d.accommodation_key === key));
  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));

  function dayThumbUrl(day: Day): string | null {
    return day.hero_image || cityMap[day.city_key]?.hero_image || null;
  }
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
    const [accData, daysData, accsData, citiesData] = await Promise.all([
      api.accommodations.get(accKey),
      api.days.list(),
      api.accommodations.list(),
      api.cities.list(),
    ]);
    acc = accData;
    days = daysData;
    accommodations = accsData;
    cities = citiesData;
  }

  async function updateField(updates: Record<string, any>) {
    if (!acc) return;
    acc = await api.accommodations.update(acc.key, updates);
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

  let showDeleteModal = $state(false);
  let deleteError = $state('');

  function openDeleteModal() {
    deleteError = '';
    showDeleteModal = true;
  }

  function closeDeleteModal() {
    showDeleteModal = false;
    deleteError = '';
  }

  async function confirmDelete() {
    if (!acc) return;
    deleteError = '';
    try {
      await api.accommodations.delete(acc.key);
      closeDeleteModal();
      navigate('/accommodations');
    } catch (err) {
      deleteError = parseApiError(err);
    }
  }
</script>

{#if acc}
  {@const accColor = accDays[0] ? getCityColor(accDays[0].city_key, cities) : 'var(--gold)'}
  {@const hasHeroImage = !!acc.hero_image}
  <div class="acc-page" style="--acc-color: {accColor}">
    {#if hasHeroImage}
      <div
        class="acc-hero-bg"
        style="background-image: url({staticUrl(acc.hero_image!)})"
        aria-hidden="true"
      ></div>
    {:else}
      <div class="acc-hero-bg acc-hero-placeholder" aria-hidden="true"></div>
    {/if}
    <div class="acc-hero-overlay" aria-hidden="true"></div>

    <div class="nav-arrows">
      <button
        type="button"
        class="nav-arrow nav-arrow-left"
        disabled={prevKey == null}
        onclick={() => prevKey != null && navigate(`/accommodations/${prevKey}`)}
        aria-label="Previous stay"
      >
        ←
      </button>
      <button
        type="button"
        class="nav-arrow nav-arrow-right"
        disabled={nextKey == null}
        onclick={() => nextKey != null && navigate(`/accommodations/${nextKey}`)}
        aria-label="Next stay"
      >
        →
      </button>
    </div>

    {#if canEdit}
      <div class="hero-upload-overlay">
        <ImageUpload
          currentImage={acc.hero_image}
          onUpload={(url) => updateField({ hero_image: url })}
          compact
        />
      </div>
    {/if}

    <div class="acc-page-content">
      <div class="acc-title-card" class:has-image={hasHeroImage}>
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
                aria-label="Stay name"
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
              <h1 class="acc-name-gradient">{acc.name}</h1>
              {#if acc.link}
                <a href={acc.link} target="_blank" rel="noreferrer" class="ext-link">{acc.link}</a>
              {/if}
            {/if}
          </div>
          {#if canEdit}
            <button
              type="button"
              class="btn-delete-icon"
              onclick={openDeleteModal}
              aria-label="Delete stay"
              title="Delete stay"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
            </button>
          {/if}
        </div>
      </div>

      <div class="detail-grid">
        <div class="card section-card section-notes" style="--acc-color: {accColor}">
          <h3 class="section-title">Notes</h3>
          <MarkdownEditor
            value={acc.notes}
            readonly={!canEdit}
            onSave={(val) => updateField({ notes: val })}
          />
        </div>

        {#if accDays.length > 0}
          <div class="card table-card section-card" style="--acc-color: {accColor}">
            <table>
              <tbody>
                {#each accDays as day}
                  {@const thumb = dayThumbUrl(day)}
                  <tr
                    class="clickable-row"
                    style="--city-color: {getCityColor(day.city_key, cities)};"
                    onclick={(e: MouseEvent) => {
                      if ((e.target as HTMLElement).closest('a, button')) return;
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
                        <span class="day-row-thumb-placeholder">{day.emoji || '📅'}</span>
                      {/if}
                    </td>
                    <td class="col-emoji">
                      {#if day.emoji}<span>{day.emoji}</span>{:else}<span class="day-num">—</span>{/if}
                    </td>
                    <td>
                      <a href="#/days/{day.id}" class="date-text">{formatDate(day.date)}</a>
                      {#if day.tagline}
                        <div class="day-tagline">{day.tagline}</div>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    </div>
  </div>
{:else}
  <p>Loading...</p>
{/if}

<ConfirmModal
  open={showDeleteModal}
  title="Delete stay"
  message={accDays.length > 0
    ? `Delete ${acc?.name ?? key}? ${accDays.length} night(s) will have no stay assigned.`
    : `Delete ${acc?.name ?? key}? This cannot be undone.`}
  confirmLabel="Delete"
  danger={true}
  showConfirm={true}
  error={deleteError}
  onConfirm={confirmDelete}
  onCancel={closeDeleteModal}
/>

<style>
  /* Hero as full-page background, title & content on top, fade out towards bottom */
  .acc-page {
    position: relative;
    width: 100vw;
    margin-left: calc(50% - 50vw);
    min-height: 100%;
    padding-bottom: 32px;
  }

  .acc-hero-bg {
    position: fixed;
    inset: 0;
    z-index: 0;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
  }

  .acc-hero-bg.acc-hero-placeholder {
    background: linear-gradient(
      135deg,
      var(--gold-glow) 0%,
      color-mix(in srgb, var(--gold) 12%, var(--bg-card)) 100%
    );
  }

  .acc-hero-overlay {
    position: fixed;
    inset: 0;
    z-index: 1;
    pointer-events: none;
    background: linear-gradient(
      180deg,
      rgba(44, 42, 38, 0.4) 0%,
      rgba(44, 42, 38, 0.15) 25%,
      transparent 45%,
      var(--bg-primary) 85%,
      var(--bg-primary) 100%
    );
  }

  .nav-arrows {
    position: fixed;
    top: 50%;
    left: 0;
    right: 0;
    z-index: 10;
    transform: translateY(-50%);
    display: flex;
    justify-content: space-between;
    padding: 0 16px;
    pointer-events: none;
  }

  .nav-arrows .nav-arrow {
    pointer-events: auto;
    flex-shrink: 0;
    width: 48px;
    height: 48px;
    border-radius: 50%;
    border: 1px solid rgba(255, 255, 255, 0.5);
    background: rgba(44, 42, 38, 0.4);
    color: #fff;
    font-size: 22px;
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

  .nav-arrows .nav-arrow:hover:not(:disabled) {
    background: rgba(44, 42, 38, 0.6);
    border-color: var(--acc-color);
    color: var(--acc-color);
    transform: scale(1.08);
  }

  .nav-arrows .nav-arrow:active:not(:disabled) {
    transform: scale(0.96);
  }

  .nav-arrows .nav-arrow:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .hero-upload-overlay {
    position: fixed;
    top: calc(var(--header-height, 100px) + 16px);
    right: 16px;
    z-index: 15;
  }

  .acc-page-content {
    position: relative;
    z-index: 2;
    max-width: 720px;
    margin: 0 auto;
    padding: 24px 24px 0;
  }

  .acc-title-card {
    padding: 20px 28px;
    border-radius: var(--radius-lg);
    margin-bottom: 24px;
  }

  .acc-title-card.has-image {
    background: linear-gradient(
      180deg,
      rgba(255, 253, 248, 0.85) 0%,
      rgba(255, 253, 248, 0.7) 100%
    );
    backdrop-filter: blur(24px) saturate(150%);
    -webkit-backdrop-filter: blur(24px) saturate(150%);
    border: 1px solid rgba(255, 255, 255, 0.6);
    box-shadow:
      0 4px 24px rgba(44, 42, 38, 0.12),
      inset 0 1px 0 rgba(255, 255, 255, 0.5);
  }

  .acc-title-card:not(.has-image) {
    background: linear-gradient(
      135deg,
      #fff 0%,
      var(--bg-card-start) 50%,
      color-mix(in srgb, var(--gold) 6%, #fff) 100%
    );
    box-shadow: 0 4px 24px rgba(44, 42, 38, 0.15);
    border: 1px solid var(--border-gold);
  }

  .btn-delete-icon {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    padding: 0;
    border: none;
    border-radius: var(--radius);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: color 0.2s, background 0.2s;
  }

  .btn-delete-icon:hover {
    color: var(--danger);
    background: rgba(185, 28, 28, 0.08);
  }

  .page-header {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 0;
  }

  .header-fields {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
    min-width: 0;
  }

  .acc-name-gradient {
    font-size: 32px;
    font-weight: 700;
    margin: 0;
    color: var(--gold);
    background: linear-gradient(
      135deg,
      var(--gold) 0%,
      var(--gold-light) 50%,
      var(--gold-dim) 100%
    );
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
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

  .section-card.section-notes {
    border-left: 4px solid var(--acc-color);
  }

  .section-card.table-card {
    border-left: 4px solid var(--acc-color);
  }

  .section-title {
    color: var(--gold);
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 12px;
  }

  .table-card {
    overflow-x: auto;
    padding: 0;
    background: var(--gradient-card) !important;
    box-shadow: 0 2px 12px rgba(44, 42, 38, 0.06);
  }

  .table-card table {
    margin: 0;
  }

  .table-card td {
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

  .col-emoji {
    width: 40px;
    text-align: center;
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
</style>
