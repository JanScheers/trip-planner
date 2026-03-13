<script lang="ts">
  import { api } from '../api';
  import type { AuthUser, Tips } from '../types';
  import MarkdownEditor from './MarkdownEditor.svelte';
  import ListPageShell from './ListPageShell.svelte';

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

<ListPageShell
  eyebrow="Reference"
  title="Tips"
  subtitle="Practical advice for the journey"
>
  {#if tips === null}
    <p class="tips-loading">Loading...</p>
  {:else}
    <div class="tips-guidebook card card-brochure">
      <MarkdownEditor
        value={tips.content}
        readonly={!canEdit}
        onSave={(val) => updateContent(val)}
      />
    </div>
  {/if}
</ListPageShell>

<style>
  .tips-loading {
    color: var(--text-muted);
    padding: 24px 0;
  }

  .tips-guidebook {
    padding: 28px 32px;
    max-width: 720px;
  }

  .tips-guidebook :global(.markdown-content) {
    font-size: 15px;
    line-height: 1.7;
  }

  .tips-guidebook :global(.markdown-content h1) {
    font-size: 22px;
    margin: 28px 0 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-gold);
  }

  .tips-guidebook :global(.markdown-content h1:first-child) {
    margin-top: 0;
  }

  .tips-guidebook :global(.markdown-content h2) {
    font-size: 18px;
    margin: 24px 0 10px;
  }

  .tips-guidebook :global(.markdown-content h3) {
    font-size: 16px;
    margin: 20px 0 8px;
  }

  .tips-guidebook :global(.markdown-content p) {
    margin-bottom: 12px;
  }

  .tips-guidebook :global(.markdown-content blockquote) {
    margin: 16px 0;
    padding: 12px 20px;
    border-left: 4px solid var(--gold);
    background: var(--gold-glow);
    border-radius: 0 var(--radius) var(--radius) 0;
    font-style: italic;
    color: var(--text-secondary);
  }

  .tips-guidebook :global(.markdown-content ul),
  .tips-guidebook :global(.markdown-content ol) {
    padding-left: 28px;
    margin-bottom: 12px;
  }

  .tips-guidebook :global(.markdown-content li) {
    margin-bottom: 4px;
  }

  .tips-guidebook :global(.markdown-content code) {
    background: var(--bg-hover);
    padding: 3px 8px;
    border-radius: 4px;
    font-size: 14px;
    border: 1px solid var(--border);
  }

  .tips-guidebook :global(.markdown-content pre) {
    margin: 16px 0;
    padding: 16px 20px;
    border-radius: var(--radius);
    border: 1px solid var(--border-gold);
    background: linear-gradient(
      180deg,
      var(--bg-hover) 0%,
      color-mix(in srgb, var(--gold-glow) 30%, var(--bg-card)) 100%
    );
  }

  .tips-guidebook :global(.markdown-content pre code) {
    background: transparent;
    padding: 0;
    border: none;
  }

  @media (max-width: 768px) {
    .tips-guidebook {
      padding: 20px 16px;
    }

    .tips-guidebook :global(.markdown-content) {
      font-size: 14px;
    }

    .tips-guidebook :global(.markdown-content h1) {
      font-size: 20px;
    }

    .tips-guidebook :global(.markdown-content h2) {
      font-size: 17px;
    }

    .tips-guidebook :global(.markdown-content h3) {
      font-size: 15px;
    }
  }
</style>
