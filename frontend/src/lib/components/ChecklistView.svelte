<script lang="ts">
  import { api, parseApiError } from "../api";
  import type { AuthUser, ChecklistItem, ChecklistEditor } from "../types";
  import AddChecklistItemModal from "./AddChecklistItemModal.svelte";
  import ConfirmModal from "./ConfirmModal.svelte";
  import ListPageShell from "./ListPageShell.svelte";

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } =
    $props();

  let items: ChecklistItem[] = $state([]);
  let editors: ChecklistEditor[] = $state([]);
  let checks: Record<string, boolean> = $state({});
  let showAddModal = $state(false);
  let deleteTarget: ChecklistItem | null = $state(null);
  let deleteError = $state("");
  let editingId: number | null = $state(null);
  let editingLabel = $state("");

  let canAdd = $derived(editMode && user?.is_editor);
  let canEdit = $derived(editMode && user?.is_editor);

  function editorInitials(email: string): string {
    const part = email.split("@")[0];
    const segments = part.split(/[._-]/);
    if (segments.length >= 2) {
      return (segments[0][0] + segments[1][0]).toUpperCase();
    }
    return part.slice(0, 2).toUpperCase();
  }

  function checkKey(itemId: number, editorEmail: string): string {
    return `${itemId}:${editorEmail}`;
  }

  function isChecked(itemId: number, editorEmail: string): boolean {
    return checks[checkKey(itemId, editorEmail)] ?? false;
  }

  function isMyColumn(editorEmail: string): boolean {
    return user?.email === editorEmail;
  }

  function canToggle(editorEmail: string): boolean {
    return user?.is_editor === true && isMyColumn(editorEmail);
  }

  $effect(() => {
    loadData();
  });

  async function loadData() {
    const [itemsData, editorsData, checksData] = await Promise.all([
      api.checklist.items(),
      api.checklist.editors(),
      api.checklist.checks(),
    ]);
    items = itemsData;
    editors = editorsData;
    checks = checksData;
  }

  function openAddModal() {
    showAddModal = true;
  }

  function closeAddModal() {
    showAddModal = false;
  }

  async function handleItemAdded() {
    await loadData();
    closeAddModal();
  }

  async function handleToggle(
    itemId: number,
    editorEmail: string,
    currentlyChecked: boolean,
  ) {
    if (!canToggle(editorEmail)) return;
    const newChecked = !currentlyChecked;
    try {
      await api.checklist.toggleCheck(itemId, newChecked);
      checks = { ...checks, [checkKey(itemId, editorEmail)]: newChecked };
    } catch {
      await loadData();
    }
  }

  function openDeleteModal(item: ChecklistItem) {
    deleteTarget = item;
    deleteError = "";
  }

  function closeDeleteModal() {
    deleteTarget = null;
    deleteError = "";
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    deleteError = "";
    try {
      await api.checklist.deleteItem(deleteTarget.id);
      closeDeleteModal();
      await loadData();
    } catch (err) {
      deleteError = parseApiError(err);
    }
  }

  function startEdit(item: ChecklistItem) {
    editingId = item.id;
    editingLabel = item.label;
  }

  function cancelEdit() {
    editingId = null;
    editingLabel = "";
  }

  async function saveEdit() {
    if (editingId == null) return;
    try {
      await api.checklist.updateItem(editingId, { label: editingLabel.trim() });
      items = items.map((i) =>
        i.id === editingId ? { ...i, label: editingLabel.trim() } : i,
      );
      cancelEdit();
    } catch {
      await loadData();
      cancelEdit();
    }
  }
</script>

{#snippet addAction()}
  <button type="button" class="btn-gold btn-sm" onclick={openAddModal}
    >Add item</button
  >
{/snippet}

<ListPageShell
  eyebrow="Group prep"
  title="Checklist"
  subtitle="Track what each traveler has done"
  stats={items.length > 0
    ? [`${items.length} items`, `${editors.length} travelers`]
    : []}
  action={canAdd ? addAction : undefined}
>
  <div class="checklist-card card card-brochure">
    {#if items.length === 0 && editors.length === 0}
      <p class="text-muted">Loading...</p>
    {:else if editors.length === 0}
      <p class="text-muted">
        No editors configured. Set EDITOR_EMAILS to see checklist columns.
      </p>
    {:else if items.length === 0}
      <p class="text-muted">No items yet. Add items to get started.</p>
    {:else}
      <div class="checklist-table-wrap">
        <table class="checklist-table">
          <thead>
            <tr>
              <th class="col-item">Item</th>
              {#each editors as editor}
                <th
                  class="col-check"
                  class:col-check-mine={isMyColumn(editor.email)}
                  title={editor.email}
                >
                  <span class="editor-chip">{editorInitials(editor.email)}</span
                  >
                </th>
              {/each}
              {#if canEdit}
                <th class="col-actions"></th>
              {/if}
            </tr>
          </thead>
          <tbody>
            {#each items as item}
              <tr>
                <td class="col-item">
                  {#if editingId === item.id}
                    <form
                      class="inline-edit-form"
                      onsubmit={(e) => {
                        e.preventDefault();
                        saveEdit();
                      }}
                    >
                      <input
                        type="text"
                        bind:value={editingLabel}
                        class="inline-edit-input"
                        onblur={saveEdit}
                        onkeydown={(e) => {
                          if (e.key === "Escape") cancelEdit();
                        }}
                      />
                    </form>
                  {:else if canEdit}
                    <button
                      type="button"
                      class="item-label editable"
                      onclick={() => startEdit(item)}
                      onkeydown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                          e.preventDefault();
                          startEdit(item);
                        }
                      }}
                    >
                      {item.label}
                    </button>
                  {:else}
                    <span class="item-label">{item.label}</span>
                  {/if}
                </td>
                {#each editors as editor}
                  <td class="col-check">
                    {#if canToggle(editor.email)}
                      <button
                        type="button"
                        class="check-btn"
                        class:checked={isChecked(item.id, editor.email)}
                        onclick={() =>
                          handleToggle(
                            item.id,
                            editor.email,
                            isChecked(item.id, editor.email),
                          )}
                        aria-label="Toggle {item.label} for {editor.email}"
                        aria-pressed={isChecked(item.id, editor.email)}
                      >
                        {#if isChecked(item.id, editor.email)}
                          ✓
                        {/if}
                      </button>
                    {:else}
                      <span
                        class="check-display"
                        class:checked={isChecked(item.id, editor.email)}
                        aria-hidden="true"
                      >
                        {#if isChecked(item.id, editor.email)}✓{/if}
                      </span>
                    {/if}
                  </td>
                {/each}
                {#if canEdit}
                  <td class="col-actions">
                    <button
                      type="button"
                      class="btn-icon btn-danger-icon"
                      onclick={() => openDeleteModal(item)}
                      aria-label="Delete {item.label}"
                    >
                      ×
                    </button>
                  </td>
                {/if}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</ListPageShell>

<AddChecklistItemModal
  open={showAddModal}
  onClose={closeAddModal}
  onSuccess={handleItemAdded}
/>

<ConfirmModal
  open={deleteTarget != null}
  title="Delete item"
  message="Remove this item from the checklist?"
  confirmLabel="Delete"
  danger={true}
  error={deleteError}
  onConfirm={confirmDelete}
  onCancel={closeDeleteModal}
/>

<style>
  .checklist-card {
    padding: 0;
    overflow-x: auto;
    border: var(--brochure-card-border);
    box-shadow: var(--brochure-card-shadow);
  }

  .checklist-table-wrap {
    overflow-x: auto;
  }

  .checklist-table {
    width: 100%;
    border-collapse: collapse;
    min-width: 400px;
  }

  .checklist-table thead {
    position: sticky;
    top: 0;
    z-index: 2;
    background: linear-gradient(
      180deg,
      var(--bg-hover) 0%,
      var(--bg-secondary) 100%
    );
  }

  .checklist-table th,
  .checklist-table td {
    padding: 14px 18px;
    text-align: left;
    border-bottom: 1px solid var(--border);
  }

  .checklist-table th {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .col-item {
    min-width: 200px;
    position: sticky;
    left: 0;
    z-index: 1;
    background: var(--bg-card);
  }

  .checklist-table thead .col-item {
    background: linear-gradient(
      180deg,
      var(--bg-hover) 0%,
      var(--bg-secondary) 100%
    );
  }

  .checklist-table tbody tr:hover .col-item {
    background: var(--bg-hover);
  }

  .col-check {
    width: 56px;
    text-align: center;
  }

  .col-check-mine {
    background: color-mix(in srgb, var(--gold-glow) 80%, var(--bg-secondary));
  }

  .editor-chip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    font-size: 11px;
    font-weight: 700;
    color: var(--gold-dim);
    background: var(--gold-glow);
    border: 1px solid var(--border-gold);
  }

  .col-check-mine .editor-chip {
    background: var(--gold-glow);
    border-color: var(--gold);
    color: var(--gold);
  }

  .col-actions {
    width: 48px;
    text-align: right;
  }

  .item-label {
    font-size: 14px;
    color: var(--text-primary);
  }

  .item-label.editable {
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px 4px;
    margin: -2px -4px;
    border-radius: var(--radius);
    font: inherit;
    text-align: left;
  }

  .item-label.editable:hover {
    background: var(--bg-hover);
  }

  .inline-edit-form {
    display: inline;
  }

  .inline-edit-input {
    font-size: 14px;
    padding: 4px 8px;
    border: 1px solid var(--border-gold);
    border-radius: var(--radius);
    width: 100%;
    max-width: 240px;
  }

  .check-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius);
    border: 2px solid var(--border-gold);
    background: transparent;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    color: transparent;
    transition: all 0.15s;
  }

  .check-btn:hover {
    border-color: var(--gold);
    background: var(--gold-glow);
  }

  .check-btn.checked {
    background: var(--gold);
    border-color: var(--gold);
    color: white;
  }

  .check-display {
    width: 28px;
    height: 28px;
    border-radius: var(--radius);
    border: 2px solid var(--border);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    color: transparent;
  }

  .check-display.checked {
    background: var(--gold-glow);
    border-color: var(--gold);
    color: var(--gold);
  }

  .btn-icon {
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 18px;
    line-height: 1;
    padding: 4px;
    color: var(--text-muted);
    border-radius: var(--radius);
  }

  .btn-icon:hover {
    color: var(--danger);
    background: rgba(185, 28, 28, 0.08);
  }

  .btn-danger-icon {
    font-weight: 300;
  }

  .text-muted {
    color: var(--text-muted);
    padding: 24px 20px;
  }
</style>
