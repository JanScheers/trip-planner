<script lang="ts">
  import { parseHash, type Route } from './lib/router';
  import { api } from './lib/api';
  import type { AuthUser } from './lib/types';
  import Nav from './lib/components/Nav.svelte';
  import Home from './lib/components/Home.svelte';
  import BasicView from './lib/components/BasicView.svelte';
  import DayView from './lib/components/DayView.svelte';
  import CitiesView from './lib/components/CitiesView.svelte';
  import CityView from './lib/components/CityView.svelte';
  import AccommodationView from './lib/components/AccommodationView.svelte';

  let route: Route = $state(parseHash());
  let user: AuthUser | null = $state(null);

  $effect(() => {
    const handler = () => { route = parseHash(); };
    window.addEventListener('hashchange', handler);
    return () => window.removeEventListener('hashchange', handler);
  });

  $effect(() => {
    api.auth.me().then(u => { user = u; }).catch(() => {});
  });
</script>

<Nav {user} />

<main class="container" style="padding-top: 24px; padding-bottom: 48px;">
  {#if route.page === 'home'}
    <Home />
  {:else if route.page === 'basic'}
    <BasicView {user} />
  {:else if route.page === 'cities'}
    <CitiesView />
  {:else if route.page === 'day'}
    <DayView id={Number(route.params.id)} {user} />
  {:else if route.page === 'city'}
    <CityView key={route.params.key} {user} />
  {:else if route.page === 'accommodation'}
    <AccommodationView key={route.params.key} {user} />
  {/if}
</main>
