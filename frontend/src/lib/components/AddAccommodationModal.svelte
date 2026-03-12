<script lang="ts">
  import { api } from "../api";
  import type { Accommodation, CreateAccommodation } from "../types";

  let {
    open,
    onClose,
    onSuccess,
  }: {
    open: boolean;
    onClose: () => void;
    onSuccess: (acc: Accommodation) => void | Promise<void>;
  } = $props();

  let addName = $state("");
  let addError = $state("");

  $effect(() => {
    if (open) {
      addName = "";
      addError = "";
    }
  });

  async function submitAdd() {
    addError = "";
    if (!addName.trim()) {
      addError = "Name is required";
      return;
    }
    try {
      const data: CreateAccommodation = { name: addName.trim() };
      const acc = await api.accommodations.create(data);
      onClose();
      await onSuccess(acc);
    } catch (err) {
      addError = err instanceof Error ? err.message : String(err);
    }
  }
</script>

{#if open}
  <div class="modal-overlay" role="dialog" aria-modal="true" aria-labelledby="add-acc-modal-title">
    <div class="modal-card modal-form">
      <h2 id="add-acc-modal-title" class="modal-title">Add stay</h2>
      <form
        onsubmit={(e) => {
          e.preventDefault();
          submitAdd();
        }}
      >
        <label for="add-acc-name">Name</label>
        <input
          id="add-acc-name"
          type="text"
          bind:value={addName}
          placeholder="Airbnb Kowloon"
          required
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
