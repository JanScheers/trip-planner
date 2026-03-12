<script lang="ts">
  import { api } from "../api";
  import { getCityColor } from "../cityColors";
  import type { Day, City } from "../types";

  let {
    currentDayId = null,
    presentationMode = false,
  }: {
    currentDayId?: number | null;
    presentationMode?: boolean;
  } = $props();

  let days: Day[] = $state([]);
  let cities: City[] = $state([]);

  $effect(() => {
    api.days.list().then((d) => {
      days = d;
    });
    api.cities.list().then((c) => {
      cities = c;
    });
  });

  let cityMap = $derived(Object.fromEntries(cities.map((c) => [c.key, c])));

  function getDayProgress(day: Day) {
    const today = new Date().toISOString().slice(0, 10);
    if (day.date < today) return "past";
    if (day.date === today) return "current";
    return "future";
  }
</script>

{#if days.length > 0}
  <div
    class="day-nav-bar"
    class:presentation-mode={presentationMode}
    role="navigation"
    aria-label="Day navigation"
  >
    <div class="progress-bar">
      {#each days as day}
        <a
          href="#/days/{day.id}"
          class="progress-segment {getDayProgress(day)}"
          class:viewing={currentDayId != null && day.id === currentDayId}
          style="--seg-color: {getCityColor(day.city_key, cities)};"
          title="{day.date} — {cityMap[day.city_key]?.name || day.city_key}"
        >
          <span class="segment-label">
            {new Date(day.date + "T00:00:00").getDate()}
          </span>
        </a>
      {/each}
    </div>
  </div>
{/if}

<style>
  .day-nav-bar {
    display: flex;
    align-items: stretch;
    flex-shrink: 0;
    box-shadow: 0 2px 8px rgba(44, 42, 38, 0.06);
  }

  .day-nav-bar.presentation-mode {
    flex-shrink: 0;
  }

  .progress-bar {
    flex: 1;
    display: flex;
    gap: 0;
    margin: 0;
  }

  .progress-segment {
    flex: 1;
    height: var(--day-nav-bar-height);
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    transition:
      filter 0.15s,
      transform 0.15s;
    position: relative;
  }

  .progress-segment.past,
  .progress-segment.current {
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--seg-color) 70%, white) 0%,
      color-mix(in srgb, var(--seg-color) 55%, var(--bg-secondary)) 100%
    );
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.25);
  }

  .progress-segment.future {
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--seg-color) 45%, var(--bg-secondary)) 0%,
      color-mix(in srgb, var(--seg-color) 35%, var(--bg-secondary)) 100%
    );
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.15);
  }

  .progress-segment:hover {
    transform: scaleY(1.15);
    z-index: 1;
    filter: brightness(1.2);
  }

  .progress-segment.current {
    box-shadow:
      inset 0 1px 0 rgba(255, 255, 255, 0.25),
      0 0 16px color-mix(in srgb, var(--seg-color) 50%, transparent);
  }

  .progress-segment.viewing {
    z-index: 1;
  }

  .progress-segment.viewing::before {
    content: '';
    position: absolute;
    inset: 0;
    border: 2px solid var(--gold);
    border-radius: inherit;
    pointer-events: none;
  }

  .segment-label {
    font-size: 12px;
    font-weight: 600;
    color: white;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  }

  .future .segment-label {
    color: var(--text-muted);
    text-shadow: none;
  }

  @media (max-width: 640px) {
    .segment-label {
      font-size: 11px;
    }

    .progress-segment {
      height: 48px;
    }
  }
</style>
