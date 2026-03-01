<script lang="ts">
  import { getCityColor } from '../cityColors';
  import type { City } from '../types';

  let { cities, onCityClick }: { cities: City[]; onCityClick: (key: string) => void } = $props();

  const cityPositions: Record<string, { x: number; y: number }> = {
    beijing:     { x: 370, y: 115 },
    xian:        { x: 305, y: 195 },
    chengdu:     { x: 240, y: 245 },
    chongqing:   { x: 275, y: 265 },
    zhangjiajie: { x: 330, y: 260 },
    guilin:      { x: 315, y: 310 },
    hongkong:    { x: 355, y: 340 },
  };

  const routeOrder = ['beijing', 'xian', 'chengdu', 'chongqing', 'zhangjiajie', 'guilin', 'hongkong'];

  function getRoutePath(): string {
    return routeOrder
      .map(key => cityPositions[key])
      .filter(Boolean)
      .map((p, i) => `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`)
      .join(' ');
  }

  function isCityVisited(key: string): boolean {
    const today = new Date().toISOString().slice(0, 10);
    return today >= '2026-10-09';
  }

  let cityMap = $derived(Object.fromEntries(cities.map(c => [c.key, c])));
</script>

<svg viewBox="0 0 500 420" class="china-map" xmlns="http://www.w3.org/2000/svg">
  <!-- Simplified China outline -->
  <path
    d="M 165 35 L 200 25 L 245 30 L 280 20 L 310 25 L 350 15 L 380 30 L 410 25
       L 435 45 L 440 75 L 420 95 L 430 115 L 420 140 L 405 155 L 415 175
       L 400 195 L 410 220 L 395 240 L 405 265 L 390 285 L 375 305
       L 360 325 L 370 345 L 350 360 L 330 340 L 310 355 L 290 330
       L 270 340 L 250 320 L 230 335 L 210 310 L 190 320 L 175 295
       L 160 280 L 150 255 L 135 240 L 120 215 L 110 190 L 95 175
       L 85 150 L 90 125 L 100 105 L 115 85 L 130 70 L 145 55 L 165 35 Z"
    fill="var(--bg-secondary)"
    stroke="var(--gold-dim)"
    stroke-width="1.5"
    opacity="0.8"
  />

  <!-- Route path -->
  <path
    d={getRoutePath()}
    fill="none"
    stroke="var(--gold)"
    stroke-width="2"
    stroke-dasharray="6 4"
    opacity="0.5"
  />

  <!-- City markers -->
  {#each routeOrder as key, i}
    {@const pos = cityPositions[key]}
    {@const city = cityMap[key]}
    {#if pos}
      <g
        class="city-marker"
        onclick={() => onCityClick(key)}
        role="button"
        tabindex="0"
        onkeydown={(e) => { if (e.key === 'Enter') onCityClick(key); }}
      >
        <circle
          cx={pos.x}
          cy={pos.y}
          r="18"
          fill="transparent"
          class="hit-area"
        />
        <circle
          cx={pos.x}
          cy={pos.y}
          r="6"
          fill={getCityColor(key, cities)}
          stroke="var(--bg-primary)"
          stroke-width="2"
        />
        <circle
          cx={pos.x}
          cy={pos.y}
          r="10"
          fill="none"
          stroke={getCityColor(key, cities)}
          stroke-width="1"
          opacity="0.3"
        />
        <text
          x={pos.x}
          y={pos.y - 14}
          text-anchor="middle"
          fill="var(--text-primary)"
          font-size="11"
          font-weight="600"
        >{city?.name || key}</text>
      </g>
    {/if}
  {/each}
</svg>

<style>
  .china-map {
    width: 100%;
    max-width: 500px;
    margin: 0 auto;
    display: block;
  }

  .city-marker {
    cursor: pointer;
  }

  .city-marker:hover circle {
    filter: brightness(1.3);
  }

  .city-marker:hover text {
    fill: var(--gold);
  }
</style>
