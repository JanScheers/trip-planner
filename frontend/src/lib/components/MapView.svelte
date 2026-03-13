<script lang="ts">
  import { api } from "../api";
  import { navigate } from "../router";
  import ChinaMap from "./ChinaMap.svelte";
  import type { Day, City } from "../types";

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
</script>

<div class="map-page">
  <ChinaMap
    {cities}
    {days}
    fullHeight={true}
    onCityClick={(key) => navigate(`/cities/${key}`)}
  />
</div>

<style>
  .map-page {
    position: absolute;
    top: var(--header-height, 100px);
    left: 0;
    right: 0;
    bottom: 0;
  }
</style>
