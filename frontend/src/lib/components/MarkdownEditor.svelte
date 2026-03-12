<script lang="ts">
  import { marked } from 'marked';
  import DOMPurify from 'dompurify';

  let { value, readonly, onSave }: { value: string; readonly: boolean; onSave: (val: string) => void } = $props();

  let draft = $state('');

  let rendered = $derived(DOMPurify.sanitize(marked.parse(value || '_No notes yet._') as string));

  $effect(() => {
    if (!readonly) draft = value;
  });

  function save() {
    onSave(draft);
  }

  function cancel() {
    draft = value;
  }
</script>

<div class="md-editor">
  {#if readonly}
    <div class="markdown-content">
      {@html rendered}
    </div>
  {:else}
    <textarea bind:value={draft} rows="10"></textarea>
    <div class="md-actions">
      <button class="btn-gold btn-sm" onclick={save}>Save</button>
      <button class="btn-outline btn-sm" onclick={cancel}>Cancel</button>
    </div>
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
