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

  let route: Route = $state(parseHash());
  let user: AuthUser | null = $state(null);
  let editMode: boolean = $state(false);
  let presentationMode: boolean = $state(false);
  let mainEl: HTMLElement | undefined;

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
  <header class="app-header">
    <Nav
      {user}
      {editMode}
      isHomePage={route.page === "home"}
      ontoggleedit={() => {
        editMode = !editMode;
      }}
    />
    {#if route.page === "day"}
      <DayNavBar
        currentDayId={Number(route.params.id)}
        onEnterPresentation={() => {
          presentationMode = true;
          mainEl?.requestFullscreen().catch(() => {
            presentationMode = false;
          });
        }}
      />
    {/if}
  </header>
{/if}

<main
  bind:this={mainEl}
  class="main-content"
  class:container={route.page !== "home" && route.page !== "day"}
  class:day-page={route.page === "day" && !presentationMode}
  class:with-nav={!presentationMode && route.page !== "home"}
  class:with-day-bar={!presentationMode && route.page === "day"}
  class:presentation-fullscreen={presentationMode && route.page === "day"}
  style="{(presentationMode || route.page === 'home' ? 'padding-top: 0px;' : '')} padding-bottom: {presentationMode ? 0 : 48}px;"
>
  {#if route.page === "home"}
    <Home />
  {:else if route.page === "basic"}
    <BasicView {user} {editMode} />
  {:else if route.page === "cities"}
    <CitiesView />
  {:else if route.page === "accommodations"}
    <AccommodationsView />
  {:else if route.page === "day"}
    {#if presentationMode}
      <DayNavBar
        currentDayId={Number(route.params.id)}
        presentationMode={true}
        onExitPresentation={() => {
          document.exitFullscreen().catch(() => {
            presentationMode = false;
          });
        }}
      />
      <div class="presentation-content">
        <DayView
          id={Number(route.params.id)}
          {user}
          {editMode}
          {presentationMode}
        />
      </div>
    {:else}
      <DayView
        id={Number(route.params.id)}
        {user}
        {editMode}
        {presentationMode}
      />
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

  .main-content.with-nav {
    padding-top: 72px;
  }

  .main-content.with-day-bar {
    padding-top: 116px;
  }

  @media (max-width: 640px) {
    .main-content.with-day-bar {
      padding-top: 108px;
    }
  }

  .main-content.day-page {
    background: var(--bg-primary, #f5f3ef);
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

  .main-content.presentation-fullscreen .presentation-content {
    flex: 1;
    min-height: 0;
    overflow: auto;
  }
</style>
