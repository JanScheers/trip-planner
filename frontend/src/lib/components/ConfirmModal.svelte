<script lang="ts">
  let {
    open,
    title,
    message,
    confirmLabel = 'Confirm',
    danger = false,
    showConfirm = true,
    error = '',
    onConfirm,
    onCancel,
  }: {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    danger?: boolean;
    showConfirm?: boolean;
    error?: string;
    onConfirm: () => void | Promise<void>;
    onCancel: () => void;
  } = $props();
</script>

{#if open}
  <div class="modal-overlay" role="dialog" aria-modal="true" aria-labelledby="modal-title">
    <div class="modal-card">
      <h2 id="modal-title" class="modal-title">{title}</h2>
      <p class="modal-message">{message}</p>
      {#if error}
        <p class="modal-error">{error}</p>
      {/if}
      <div class="modal-actions">
        <button type="button" class="btn-outline btn-sm" onclick={onCancel}>Cancel</button>
        {#if showConfirm}
          <button
            type="button"
            class="btn-sm"
            class:btn-danger={danger}
            class:btn-gold={!danger}
            onclick={async () => {
              await onConfirm();
            }}
          >
            {confirmLabel}
          </button>
        {/if}
      </div>
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

  .modal-card {
    background: var(--gradient-card);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 24px;
    max-width: 400px;
    width: 100%;
    box-shadow: 0 8px 32px rgba(44, 42, 38, 0.2);
  }

  .modal-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 12px;
  }

  .modal-message {
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0 0 20px;
  }

  .modal-error {
    font-size: 13px;
    color: var(--danger);
    margin: 0 0 16px;
  }

  .modal-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .btn-danger {
    background: transparent;
    color: var(--danger);
    border: 1px solid rgba(185, 28, 28, 0.4);
  }

  .btn-danger:hover {
    background: rgba(185, 28, 28, 0.08);
  }
</style>
