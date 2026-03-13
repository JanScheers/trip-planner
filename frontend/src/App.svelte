<script lang="ts">
  import { parseHash, type Route } from "./lib/router";
  import { api } from "./lib/api";
  import type { AuthUser } from "./lib/types";
  import Nav from "./lib/components/Nav.svelte";
  import DayNavBar from "./lib/components/DayNavBar.svelte";
  import Home from "./lib/components/Home.svelte";
  import BasicView from "./lib/components/BasicView.svelte";
  import DayView from "./lib/components/DayView.svelte";
  import CitiesView from "./lib/components/CitiesView.svelte";
  import CityView from "./lib/components/CityView.svelte";
  import AccommodationsView from "./lib/components/AccommodationsView.svelte";
  import AccommodationView from "./lib/components/AccommodationView.svelte";
  import TipsView from "./lib/components/TipsView.svelte";
  import ChecklistView from "./lib/components/ChecklistView.svelte";

  let route: Route = $state(parseHash());
  let user: AuthUser | null = $state(null);
  let editMode: boolean = $state(false);
  let presentationMode: boolean = $state(false);
  let mainEl: HTMLElement | undefined;
  let headerEl = $state<HTMLElement | undefined>(undefined);

  $effect(() => {
    const header = headerEl;
    if (!header) return;
    const observer = new ResizeObserver(() => {
      document.documentElement.style.setProperty(
        "--header-height",
        `${header.getBoundingClientRect().height}px`,
      );
    });
    observer.observe(header);
    document.documentElement.style.setProperty(
      "--header-height",
      `${header.getBoundingClientRect().height}px`,
    );
    return () => {
      observer.disconnect();
      document.documentElement.style.removeProperty("--header-height");
    };
  });

  $effect(() => {
    const handler = () => {
      if (!document.fullscreenElement) {
        presentationMode = false;
      }
    };
    document.addEventListener("fullscreenchange", handler);
    return () => document.removeEventListener("fullscreenchange", handler);
  });

  $effect(() => {
    if (!presentationMode) return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        document.exitFullscreen().catch(() => {
          presentationMode = false;
        });
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  });

  $effect(() => {
    const handler = () => {
      route = parseHash();
    };
    window.addEventListener("hashchange", handler);
    return () => window.removeEventListener("hashchange", handler);
  });

  $effect(() => {
    api.auth
      .me()
      .then((u) => {
        user = u;
      })
      .catch(() => {});
  });
</script>

{#if !presentationMode}
  <header class="app-header" bind:this={headerEl}>
    <Nav
      {user}
      {editMode}
      isHomePage={route.page === "home"}
      isDayPage={route.page === "day"}
      onEnterPresentation={
        route.page === "day"
          ? () => {
              presentationMode = true;
              mainEl?.requestFullscreen().catch(() => {
                presentationMode = false;
              });
            }
          : undefined
      }
      ontoggleedit={() => {
        editMode = !editMode;
      }}
    />
  </header>
{/if}

<main
  bind:this={mainEl}
  class="main-content"
  class:container={route.page !== "home" && route.page !== "day"}
  class:day-page={route.page === "day" && !presentationMode}
  class:with-nav={!presentationMode && route.page !== "home"}
  class:with-day-bar={!presentationMode && route.page === "day"}
  class:list-page={route.page === "basic" || route.page === "cities" || route.page === "accommodations" || route.page === "tips" || route.page === "checklist"}
  class:presentation-fullscreen={presentationMode && route.page === "day"}
  style="{(presentationMode || route.page === 'home' ? 'padding-top: 0px;' : '')} padding-bottom: {(presentationMode || route.page === 'day') ? 0 : 48}px;"
>
  {#if route.page === "home"}
    <Home />
  {:else if route.page === "basic"}
    <BasicView {user} {editMode} />
  {:else if route.page === "cities"}
    <CitiesView {user} {editMode} />
  {:else if route.page === "accommodations"}
    <AccommodationsView {user} {editMode} />
  {:else if route.page === "tips"}
    <TipsView {user} {editMode} />
  {:else if route.page === "checklist"}
    <ChecklistView {user} {editMode} />
  {:else if route.page === "day"}
    {#if presentationMode}
      <div class="presentation-viewport">
        <div class="presentation-content">
          <DayView
            id={Number(route.params.id)}
            {user}
            {editMode}
            {presentationMode}
          />
        </div>
        <DayNavBar
          currentDayId={Number(route.params.id)}
          presentationMode={true}
        />
      </div>
    {:else}
      <div class="day-page-viewport">
        <DayView
          id={Number(route.params.id)}
          {user}
          {editMode}
          {presentationMode}
        />
        <DayNavBar currentDayId={Number(route.params.id)} />
      </div>
    {/if}
  {:else if route.page === "city"}
    <CityView key={route.params.key} {user} {editMode} />
  {:else if route.page === "accommodation"}
    <AccommodationView key={route.params.key} {user} {editMode} />
  {/if}
</main>

<style>
  .app-header {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 100;
    display: flex;
    flex-direction: column;
  }

  .main-content.with-nav,
  .main-content.with-day-bar {
    padding-top: var(--header-height, 100px);
  }

  .main-content.list-page {
    padding-top: calc(var(--header-height, 100px) + 24px);
  }

  .main-content.day-page {
    background: var(--bg-primary, #f5f3ef);
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .day-page-viewport {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .day-page-viewport :global(.day-slideshow:not(.presentation-mode)) {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }

  .main-content.presentation-fullscreen {
    position: fixed;
    inset: 0;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--bg-primary, #f5f3ef);
  }

  .presentation-viewport {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .main-content.presentation-fullscreen .presentation-content {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
</style>
