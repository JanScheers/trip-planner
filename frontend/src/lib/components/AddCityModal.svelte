<script lang="ts">
  import { api } from "../api";
  import type { City, CreateCity } from "../types";

  let {
    open,
    onClose,
    onSuccess,
  }: {
    open: boolean;
    onClose: () => void;
    onSuccess: (city: City) => void | Promise<void>;
  } = $props();

  let addName = $state("");
  let addChineseName = $state("");
  let addTagline = $state("");
  let addLat = $state("");
  let addLng = $state("");
  let addError = $state("");

  $effect(() => {
    if (open) {
      addName = "";
      addChineseName = "";
      addTagline = "";
      addLat = "";
      addLng = "";
      addError = "";
    }
  });

  async function submitAdd() {
    addError = "";
    if (!String(addName ?? "").trim()) {
      addError = "Name is required";
      return;
    }
    const parseCoord = (s: string | number): number | null => {
      const v = parseFloat(String(s ?? "").trim());
      return isNaN(v) ? null : v;
    };
    const lat = parseCoord(addLat);
    const lng = parseCoord(addLng);
    if (String(addLat ?? "").trim() && lat === null) {
      addError = "Latitude must be a valid number";
      return;
    }
    if (String(addLng ?? "").trim() && lng === null) {
      addError = "Longitude must be a valid number";
      return;
    }
    if (lat !== null && (lat < -90 || lat > 90)) {
      addError = "Latitude must be between -90 and 90";
      return;
    }
    if (lng !== null && (lng < -180 || lng > 180)) {
      addError = "Longitude must be between -180 and 180";
      return;
    }
    try {
      const data: CreateCity = {
        name: String(addName ?? "").trim(),
        chinese_name: String(addChineseName ?? "").trim() || undefined,
        tagline: String(addTagline ?? "").trim() || undefined,
        lat: lat ?? undefined,
        lng: lng ?? undefined,
      };
      const city = await api.cities.create(data);
      onClose();
      await onSuccess(city);
    } catch (err) {
      addError = err instanceof Error ? err.message : String(err);
    }
  }
</script>

{#if open}
  <div class="modal-overlay" role="dialog" aria-modal="true" aria-labelledby="add-city-modal-title">
    <div class="modal-card modal-form">
      <h2 id="add-city-modal-title" class="modal-title">Add city</h2>
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
          <button type="button" class="btn-outline btn-sm" onclick={onClose}>Cancel</button>
          <button type="submit" class="btn-gold btn-sm">Add</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
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
</style>
