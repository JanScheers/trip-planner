<script lang="ts">
  import { parseHash, type Route } from "./lib/router";
  import { api } from "./lib/api";
  import type { AuthUser } from "./lib/types";
  import Nav from "./lib/components/Nav.svelte";
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
  <Nav
    {user}
    {editMode}
    isHomePage={route.page === "home"}
    ontoggleedit={() => {
      editMode = !editMode;
    }}
  />
{/if}

<main
  bind:this={mainEl}
  class="main-content"
  class:container={route.page !== "home" && route.page !== "day"}
  class:presentation-fullscreen={presentationMode && route.page === "day"}
  style="padding-top: {presentationMode ? 0 : route.page === 'home' ? 0 : 72}px; padding-bottom: {presentationMode ? 0 : 48}px;"
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
    <DayView
      id={Number(route.params.id)}
      {user}
      {editMode}
      {presentationMode}
      onEnterPresentation={() => {
        presentationMode = true;
        mainEl?.requestFullscreen().catch(() => {
          presentationMode = false;
        });
      }}
      onExitPresentation={() => {
        document.exitFullscreen().catch(() => {
          presentationMode = false;
        });
      }}
    />
  {:else if route.page === "city"}
    <CityView key={route.params.key} {user} {editMode} />
  {:else if route.page === "accommodation"}
    <AccommodationView key={route.params.key} {user} {editMode} />
  {/if}
</main>

<style>
  .main-content.presentation-fullscreen {
    position: fixed;
    inset: 0;
    z-index: 1000;
    overflow: auto;
    background: var(--bg-primary, #f5f3ef);
  }
</style>
