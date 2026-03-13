<script lang="ts">
  let {
    eyebrow = '',
    title,
    subtitle = '',
    stats = [] as string[],
    action,
    children,
  }: {
    eyebrow?: string;
    title: string;
    subtitle?: string;
    stats?: string[];
    action?: import('svelte').Snippet;
    children?: import('svelte').Snippet;
  } = $props();
</script>

<div class="list-page-shell">
  <div class="list-page-header">
    {#if eyebrow}
      <span class="list-page-eyebrow">{eyebrow}</span>
    {/if}
    <h1 class="list-page-title">{title}</h1>
    {#if subtitle}
      <p class="list-page-subtitle">{subtitle}</p>
    {/if}
  </div>

  {#if stats.length > 0 || action}
    <div class="list-page-action-row">
      {#if stats.length > 0}
        <div class="list-page-stats">
          {#each stats as stat}
            <span class="list-page-stat-chip">{stat}</span>
          {/each}
        </div>
      {/if}
      {#if action}
        <div class="list-page-action">{@render action()}</div>
      {/if}
    </div>
  {/if}

  {#if children}
    {@render children()}
  {/if}
</div>
