<script lang="ts">
  import { onMount } from "svelte";
  import maplibregl from "maplibre-gl";
  import "maplibre-gl/dist/maplibre-gl.css";
  import { getCityColor } from "../cityColors";
  import type { City, Day } from "../types";

  let {
    cities,
    days,
    onCityClick,
  }: { cities: City[]; days: Day[]; onCityClick: (key: string) => void } =
    $props();

  let routeOrder = $derived(
    [...new Set(days.map((d) => d.city_key))],
  );

  function getCityCoords(key: string): [number, number] | null {
    const city = cities.find((c) => c.key === key);
    if (city?.lng != null && city?.lat != null) return [city.lng, city.lat];
    return null;
  }

  let mapContainer: HTMLDivElement;
  let map: maplibregl.Map | null = null;

  function citiesGeoJSON(): GeoJSON.FeatureCollection {
    return {
      type: "FeatureCollection",
      features: routeOrder
        .filter((key) => getCityCoords(key) !== null)
        .map((key) => ({
          type: "Feature",
          geometry: { type: "Point", coordinates: getCityCoords(key)! },
          properties: {
            key,
            name: cities.find((c) => c.key === key)?.name || key,
            chinese_name: cities.find((c) => c.key === key)?.chinese_name || "",
            color: getCityColor(key, cities),
          },
        })),
    };
  }

  function routeGeoJSON(): GeoJSON.Feature {
    return {
      type: "Feature",
      properties: {},
      geometry: {
        type: "LineString",
        coordinates: routeOrder
          .map((k) => getCityCoords(k))
          .filter((c): c is [number, number] => c !== null),
      },
    };
  }

  function initLayers() {
    if (!map) return;

    // Route line
    map.addSource("route", { type: "geojson", data: routeGeoJSON() });
    map.addLayer({
      id: "route-line",
      type: "line",
      source: "route",
      paint: {
        "line-color": "#b8860b",
        "line-width": 2.5,
        "line-dasharray": [5, 4],
        "line-opacity": 0.7,
      },
    });

    // City points — rendered by WebGL just like the route line,
    // so positioning is guaranteed to match (no DOM bounding-rect issues).
    map.addSource("cities", { type: "geojson", data: citiesGeoJSON() });

    // Outer ring
    map.addLayer({
      id: "city-rings",
      type: "circle",
      source: "cities",
      paint: {
        "circle-radius": 10,
        "circle-color": "rgba(0,0,0,0)",
        "circle-stroke-width": 1.5,
        "circle-stroke-color": ["get", "color"],
        "circle-stroke-opacity": 0.5,
      },
    });

    // Inner dot
    map.addLayer({
      id: "city-dots",
      type: "circle",
      source: "cities",
      paint: {
        "circle-radius": 6,
        "circle-color": ["get", "color"],
        "circle-stroke-width": 2,
        "circle-stroke-color": "rgba(255,255,255,0.9)",
      },
    });

    // English label above dot (light map: dark text, light halo)
    map.addLayer({
      id: "city-labels",
      type: "symbol",
      source: "cities",
      layout: {
        "text-field": ["get", "name"],
        "text-font": ["Open Sans Bold", "Arial Unicode MS Bold"],
        "text-size": 11,
        "text-offset": [0, -1.5],
        "text-anchor": "bottom",
        "text-allow-overlap": true,
        "text-ignore-placement": true,
      },
      paint: {
        "text-color": "#2c2a26",
        "text-halo-color": "rgba(255,255,255,0.95)",
        "text-halo-width": 1.5,
      },
    });

    // Chinese name below the English label
    map.addLayer({
      id: "city-labels-chinese",
      type: "symbol",
      source: "cities",
      filter: ["!=", ["get", "chinese_name"], ""],
      layout: {
        "text-field": ["get", "chinese_name"],
        "text-font": ["Arial Unicode MS Bold"],
        "text-size": 10,
        "text-offset": [0, -3.2],
        "text-anchor": "bottom",
        "text-allow-overlap": true,
        "text-ignore-placement": true,
      },
      paint: {
        "text-color": "#5c5852",
        "text-halo-color": "rgba(255,255,255,0.95)",
        "text-halo-width": 1.5,
      },
    });

    // Click handlers
    const clickHandler = (
      e: maplibregl.MapMouseEvent & {
        features?: maplibregl.MapGeoJSONFeature[];
      },
    ) => {
      const key = e.features?.[0]?.properties?.key;
      if (key) onCityClick(key);
    };
    map.on("click", "city-dots", clickHandler);
    map.on("click", "city-rings", clickHandler);

    map.on("mouseenter", "city-dots", () => {
      map!.getCanvas().style.cursor = "pointer";
    });
    map.on("mouseleave", "city-dots", () => {
      map!.getCanvas().style.cursor = "";
    });
  }

  function fitRouteBounds(animate = false) {
    if (!map) return;
    const coords = routeOrder
      .map((k) => getCityCoords(k))
      .filter((c): c is [number, number] => c !== null);
    if (coords.length === 0) return;
    const lngs = coords.map((c) => c[0]);
    const lats = coords.map((c) => c[1]);
    map.fitBounds(
      [
        [Math.min(...lngs), Math.min(...lats)],
        [Math.max(...lngs), Math.max(...lats)],
      ],
      { padding: 90, animate },
    );
  }

  onMount(() => {
    map = new maplibregl.Map({
      container: mapContainer,
      style: {
        version: 8,
        glyphs: "https://fonts.openmaptiles.org/{fontstack}/{range}.pbf",
        sources: {
          carto: {
            type: "raster",
            tiles: [
              "https://a.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png",
              "https://b.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png",
              "https://c.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png",
              "https://d.basemaps.cartocdn.com/rastertiles/light_all/{z}/{x}/{y}.png",
            ],
            tileSize: 256,
            attribution:
              '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors &copy; <a href="https://carto.com/attributions">CARTO</a>',
          },
        },
        layers: [{ id: "carto-layer", type: "raster", source: "carto" }],
      },
      center: [108, 30],
      zoom: 3,
      minZoom: 2,
      maxZoom: 12,
      attributionControl: false,
    });

    map.addControl(
      new maplibregl.AttributionControl({ compact: true }),
      "bottom-right",
    );
    map.addControl(
      new maplibregl.NavigationControl({ showCompass: false }),
      "top-right",
    );

    map.on("load", initLayers);

    return () => map?.remove();
  });

  // Update sources when cities/days props change
  $effect(() => {
    const _dep =
      cities.map((c) => `${c.key}${c.name}${c.chinese_name}${c.lat}${c.lng}`).join(",") +
      routeOrder.join(",");

    const citySrc = map?.getSource("cities") as
      | maplibregl.GeoJSONSource
      | undefined;
    citySrc?.setData(citiesGeoJSON());

    const routeSrc = map?.getSource("route") as
      | maplibregl.GeoJSONSource
      | undefined;
    routeSrc?.setData(routeGeoJSON());

    fitRouteBounds();
  });
</script>

<div class="map-wrap">
  <div bind:this={mapContainer} class="map-container"></div>
</div>

<style>
  .map-wrap {
    position: relative;
    height: 480px;
  }

  .map-container {
    width: 100%;
    height: 100%;
  }

  /* ── MapLibre chrome overrides (light mode) ── */
  :global(.maplibregl-ctrl-attrib) {
    background: rgba(255, 255, 255, 0.9) !important;
    color: #7a756e !important;
    font-size: 10px !important;
    border: 1px solid var(--border);
  }
  :global(.maplibregl-ctrl-attrib a) {
    color: var(--gold) !important;
  }
  :global(.maplibregl-ctrl-group) {
    background: #fff !important;
    border: 1px solid var(--border) !important;
    border-radius: 8px !important;
    box-shadow: 0 2px 12px rgba(44, 42, 38, 0.1) !important;
    overflow: hidden;
  }
  :global(.maplibregl-ctrl-group button) {
    background-color: transparent !important;
    width: 32px !important;
    height: 32px !important;
    display: flex !important;
    align-items: center !important;
    justify-content: center !important;
    transition: background-color 0.2s ease !important;
  }
  :global(.maplibregl-ctrl-group button:hover) {
    background-color: var(--bg-hover) !important;
  }
  :global(.maplibregl-ctrl-group button:active) {
    background-color: #e5e2dc !important;
  }
  :global(.maplibregl-ctrl-group button + button) {
    border-top: 1px solid var(--border) !important;
  }
  :global(.maplibregl-ctrl-icon) {
    filter: none;
  }
</style>
