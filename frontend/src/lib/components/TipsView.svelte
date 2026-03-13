<script lang="ts">
  import { api } from '../api';
  import type { AuthUser, Tips } from '../types';
  import MarkdownEditor from './MarkdownEditor.svelte';

  let { user, editMode }: { user: AuthUser | null; editMode: boolean } = $props();

  let tips: Tips | null = $state(null);

  let canEdit = $derived(editMode && user?.is_editor);

  $effect(() => {
    loadData();
  });

  async function loadData() {
    tips = await api.tips.get();
  }

  async function updateContent(content: string) {
    if (!tips) return;
    tips = await api.tips.update(content);
  }
</script>

<div class="tips-section">
  {#if tips === null}
    <p class="text-muted">Loading...</p>
  {:else}
    <div class="card section-card section-tips">
      <MarkdownEditor
        value={tips.content}
        readonly={!canEdit}
        onSave={(val) => updateContent(val)}
      />
    </div>
  {/if}
</div>

<style>
  .tips-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-card.section-tips {
    padding: 20px;
  }
</style>
