<script lang="ts">
  import { marked } from 'marked';
  import DOMPurify from 'dompurify';

  let { value, readonly, onSave }: { value: string; readonly: boolean; onSave: (val: string) => void } = $props();

  let editing = $state(false);
  let draft = $state('');

  let rendered = $derived(DOMPurify.sanitize(marked.parse(value || '_No notes yet._') as string));

  function startEdit() {
    draft = value;
    editing = true;
  }

  function save() {
    onSave(draft);
    editing = false;
  }

  function cancel() {
    editing = false;
  }
</script>

<div class="md-editor">
  {#if editing}
    <textarea bind:value={draft} rows="10"></textarea>
    <div class="md-actions">
      <button class="btn-gold btn-sm" onclick={save}>Save</button>
      <button class="btn-outline btn-sm" onclick={cancel}>Cancel</button>
    </div>
  {:else}
    {#if !readonly}
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div
        class="markdown-content editable"
        onclick={startEdit}
        onkeydown={(e) => { if (e.key === 'Enter') startEdit(); }}
        role="button"
        tabindex={0}
      >
        {@html rendered}
      </div>
      <button class="btn-outline btn-sm" style="margin-top: 8px;" onclick={startEdit}>Edit Notes</button>
    {:else}
      <div class="markdown-content">
        {@html rendered}
      </div>
    {/if}
  {/if}
</div>

<style>
  .md-editor {
    min-height: 60px;
  }

  .md-actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .markdown-content {
    cursor: text;
    padding: 8px 0;
  }
</style>
