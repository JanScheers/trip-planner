<script lang="ts">
  import { api, staticUrl } from "../api";
  import { getCityColor } from "../cityColors";
  import type { Day, City } from "../types";
  import { navigate } from "../router";
  import ChinaMap from "./ChinaMap.svelte";

  let days: Day[] = $state([]);
  let cities: City[] = $state([]);

  const TRIP_START = new Date("2026-10-09T00:00:00");

  let now = $state(new Date());
  let countdown = $derived(getCountdown(now));

  function getCountdown(now: Date) {
    const diff = TRIP_START.getTime() - now.getTime();
    if (diff <= 0)
      return { days: 0, hours: 0, minutes: 0, seconds: 0, past: true };
    const d = Math.floor(diff / (1000 * 60 * 60 * 24));
    const h = Math.floor((diff / (1000 * 60 * 60)) % 24);
    const m = Math.floor((diff / (1000 * 60)) % 60);
    const s = Math.floor((diff / 1000) % 60);
    return { days: d, hours: h, minutes: m, seconds: s, past: false };
  }

  $effect(() => {
    const interval = setInterval(() => {
      now = new Date();
    }, 1000);
    return () => clearInterval(interval);
  });

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

  let totalDays = $derived(days.length);
  let citiesCount = $derived(new Set(days.map((d) => d.city_key)).size);
</script>

<div class="home">
  <div class="hero">
    <div class="hero-bg">
      <div class="hero-bg-image" role="img" aria-label="China travel"></div>
      <div class="hero-bg-overlay"></div>
      <div class="lantern lantern-1"></div>
      <div class="lantern lantern-2"></div>
      <div class="lantern lantern-3"></div>
    </div>

    <div class="hero-content">
      <div class="hero-badge">An adventure awaits</div>
      <h1 class="hero-title">
        <span class="chinese-text title-chinese">中国之旅</span>
        <span class="hero-subtitle">China 2026</span>
      </h1>

      <div class="countdown">
        {#if countdown.past}
          <div class="countdown-label live-pulse">The journey has begun</div>
        {:else}
          <div class="countdown-label">Departure in</div>
          <div class="countdown-grid">
            <div class="countdown-item">
              <span class="countdown-value">{countdown.days}</span>
              <span class="countdown-unit">days</span>
            </div>
            <div class="countdown-sep">:</div>
            <div class="countdown-item">
              <span class="countdown-value"
                >{String(countdown.hours).padStart(2, "0")}</span
              >
              <span class="countdown-unit">hours</span>
            </div>
            <div class="countdown-sep">:</div>
            <div class="countdown-item">
              <span class="countdown-value"
                >{String(countdown.minutes).padStart(2, "0")}</span
              >
              <span class="countdown-unit">min</span>
            </div>
            <div class="countdown-sep">:</div>
            <div class="countdown-item">
              <span class="countdown-value"
                >{String(countdown.seconds).padStart(2, "0")}</span
              >
              <span class="countdown-unit">sec</span>
            </div>
          </div>
        {/if}
      </div>

      <div class="hero-stats">
        <div class="stat">
          <span class="stat-value">{totalDays}</span>
          <span class="stat-label">Days</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">{citiesCount}</span>
          <span class="stat-label">Cities</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">1</span>
          <span class="stat-label">Adventure</span>
        </div>
      </div>
    </div>
  </div>

  <div class="container home-sections">
    <section class="section">
      <div class="section-header">
        <h2>The Route</h2>
        <p class="section-desc">
          Beijing to Hong Kong — a journey through 5,000 years of history
        </p>
      </div>
      <div class="city-grid-fullwidth">
        <div class="map-card card">
          <ChinaMap
          {cities}
          {days}
          onCityClick={(key) => navigate(`/cities/${key}`)}
        />
        <div class="progress-bar">
          {#each days as day}
            <a
              href="#/days/{day.id}"
              class="progress-segment {getDayProgress(day)}"
              style="--seg-color: {getCityColor(day.city_key, cities)};"
              title="{day.date} — {cityMap[day.city_key]?.name || day.city_key}"
            >
              <span class="segment-label"
                >{new Date(day.date + "T00:00:00").getDate()}</span
              >
            </a>
          {/each}
        </div>
        </div>
      </div>
    </section>

    <section class="section">
      <div class="section-header">
        <h2>Destinations</h2>
        <p class="section-desc">Seven cities, each with its own story</p>
      </div>
      <div class="city-grid-fullwidth">
        <div class="city-grid">
        {#each cities as city}
          <a
            href="#/cities/{city.key}"
            class="city-card"
            style="--city-color: {getCityColor(city.key, cities)};"
          >
            <div
              class="city-card-image"
              role="img"
              aria-label="{city.name}"
              style="background-image: url('{staticUrl(city.hero_image) ||
              "https://images.unsplash.com/photo-1751688472649-ed13c46ccfe1?auto=format&fit=crop&w=800&q=80"}');"
            ></div>
            <div class="city-card-overlay"></div>
            <div class="city-card-body">
              {#if city.emoji}
                <span class="city-emoji">{city.emoji}</span>
              {/if}
              <h3 class="city-name">{city.name}</h3>
              {#if city.chinese_name}
                <span class="city-chinese chinese-text">{city.chinese_name}</span>
              {/if}
              <p class="city-desc">{city.tagline}</p>
              <div class="city-card-footer">
                <span class="city-days-count"
                  >{days.filter((d) => d.city_key === city.key).length} days</span
                >
                <span class="city-arrow">&rarr;</span>
              </div>
            </div>
          </a>
        {/each}
        </div>
      </div>
    </section>

    <div class="cta-strip">
      <a href="#/days" class="btn-gold cta-btn">View Full Itinerary &rarr;</a>
    </div>
  </div>
</div>

<style>
  .home {
    display: flex;
    flex-direction: column;
    gap: 48px;
    padding-bottom: 32px;
  }

  .home-sections {
    display: flex;
    flex-direction: column;
    gap: 48px;
  }

  /* --- Hero --- */
  .hero {
    position: relative;
    text-align: center;
    padding: 0 0 48px;
    overflow: hidden;
    min-height: 100vh;
  }

  .hero-bg {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .hero-bg-image {
    position: absolute;
    inset: 0;
    background-image: url("https://images.unsplash.com/photo-1751688472649-ed13c46ccfe1?auto=format&fit=crop&w=1920&q=80");
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
  }

  .hero-bg-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      180deg,
      rgba(44, 42, 38, 0.5) 0%,
      rgba(44, 42, 38, 0.35) 25%,
      rgba(44, 42, 38, 0.15) 50%,
      rgba(245, 243, 239, 0.4) 75%,
      rgba(245, 243, 239, 0.92) 100%
    );
  }

  .lantern {
    position: absolute;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--gold);
    opacity: 0;
    animation: float-lantern 6s ease-in-out infinite;
  }

  .lantern-1 {
    left: 15%;
    top: 20%;
    animation-delay: 0s;
  }
  .lantern-2 {
    left: 80%;
    top: 35%;
    animation-delay: 2s;
  }
  .lantern-3 {
    left: 50%;
    top: 10%;
    animation-delay: 4s;
  }

  @keyframes float-lantern {
    0%,
    100% {
      opacity: 0;
      transform: translateY(0);
    }
    50% {
      opacity: 0.4;
      transform: translateY(-20px);
    }
  }

  .hero-content {
    position: relative;
    z-index: 1;
    padding-top: 72px;
  }

  .hero-badge {
    display: inline-block;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.2em;
    color: var(--gold-dim);
    border: 1px solid var(--border-gold);
    padding: 4px 16px;
    border-radius: 99px;
    margin-bottom: 24px;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.9) 0%,
      rgba(232, 213, 163, 0.25) 100%
    );
    box-shadow: 0 2px 12px rgba(184, 134, 11, 0.1);
  }

  .hero-title {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
  }

  .title-chinese {
    font-size: 56px;
    background: linear-gradient(
      135deg,
      var(--gold) 0%,
      var(--gold-light) 50%,
      var(--gold-dim) 100%
    );
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    font-weight: 700;
    letter-spacing: 0.05em;
  }

  .hero-subtitle {
    font-size: 15px;
    color: #fff;
    font-weight: 400;
    letter-spacing: 0.2em;
    text-transform: uppercase;
    text-shadow:
      0 0 3px rgba(138, 109, 10, 0.8),
      0 0 6px rgba(138, 109, 10, 0.5),
      0 1px 2px rgba(44, 42, 38, 0.4);
  }

  /* --- Countdown --- */
  .countdown {
    margin-top: 36px;
  }

  .countdown-label {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: #fff;
    margin-bottom: 14px;
    text-shadow:
      0 0 3px rgba(138, 109, 10, 0.8),
      0 0 6px rgba(138, 109, 10, 0.5),
      0 1px 2px rgba(44, 42, 38, 0.4);
  }

  .live-pulse {
    color: var(--gold);
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .countdown-grid {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
  }

  .countdown-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-width: 72px;
    background: linear-gradient(
      180deg,
      #ffffff 0%,
      var(--bg-hover) 50%,
      rgba(232, 213, 163, 0.08) 100%
    );
    border: 1px solid var(--border-gold);
    border-radius: var(--radius-lg);
    padding: 12px 8px 10px;
    box-shadow:
      0 2px 4px rgba(0, 0, 0, 0.04),
      0 4px 16px rgba(184, 134, 11, 0.08);
  }

  .countdown-value {
    font-size: 36px;
    font-weight: 700;
    color: var(--gold);
    line-height: 1;
    font-variant-numeric: tabular-nums;
  }

  .countdown-unit {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--text-muted);
    margin-top: 6px;
  }

  .countdown-sep {
    font-size: 28px;
    color: var(--gold-dim);
    font-weight: 300;
    margin-bottom: 18px;
    padding: 0 2px;
  }

  /* --- Stats --- */
  .hero-stats {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 24px;
    margin-top: 36px;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .stat-value {
    font-size: 24px;
    font-weight: 700;
    color: #fff;
    -webkit-text-stroke: 2px rgba(138, 109, 10, 0.9);
    paint-order: stroke fill;
    text-shadow:
      0 0 4px rgba(138, 109, 10, 0.6),
      0 0 8px rgba(138, 109, 10, 0.4);
  }

  .stat-label {
    font-size: 13px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #fff;
    -webkit-text-stroke: 2px rgba(138, 109, 10, 0.9);
    paint-order: stroke fill;
    text-shadow:
      0 0 4px rgba(138, 109, 10, 0.6),
      0 0 8px rgba(138, 109, 10, 0.4);
  }

  .stat-divider {
    width: 3px;
    height: 36px;
    background: rgba(255, 255, 255, 0.8);
    border-radius: 2px;
  }

  /* --- Sections --- */
  .section {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .section-header {
    text-align: center;
  }

  .section-header h2 {
    background: linear-gradient(90deg, var(--gold-dim), var(--gold));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    font-size: 14px;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    margin-bottom: 6px;
  }

  .section-desc {
    font-size: 14px;
    color: var(--text-secondary);
  }

  .map-card {
    padding: 0;
    width: 100%;
    max-width: 1000px;
    margin: 0 auto;
    overflow: hidden;
    background: linear-gradient(
      180deg,
      #fff 0%,
      var(--bg-card-start) 50%,
      rgba(232, 213, 163, 0.05) 100%
    ) !important;
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.04),
      0 8px 24px rgba(44, 42, 38, 0.08) !important;
    border: 1px solid rgba(184, 134, 11, 0.12);
  }

  /* --- City Grid --- */
  .city-grid-fullwidth {
    width: 100vw;
    margin-left: calc(-50vw + 50%);
    padding-left: 40px;
    padding-right: 40px;
    box-sizing: border-box;
  }

  .city-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 32px;
    max-width: 1000px;
    margin: 0 auto;
  }

  .city-card {
    position: relative;
    display: block;
    aspect-ratio: 16 / 9;
    border-radius: var(--radius-lg);
    overflow: hidden;
    text-decoration: none;
    color: inherit;
    transition: all 0.25s ease;
    box-shadow:
      0 2px 8px rgba(0, 0, 0, 0.06),
      0 8px 24px rgba(44, 42, 38, 0.1);
  }

  .city-card:hover {
    transform: translateY(-6px);
    box-shadow:
      0 4px 12px rgba(0, 0, 0, 0.08),
      0 20px 48px rgba(44, 42, 38, 0.18);
  }

  .city-card-image {
    position: absolute;
    inset: 0;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    transition: transform 0.4s ease;
  }

  .city-card:hover .city-card-image {
    transform: scale(1.04);
  }

  .city-card-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      180deg,
      transparent 20%,
      rgba(44, 42, 38, 0.3) 50%,
      rgba(44, 42, 38, 0.9) 100%
    );
    transition: opacity 0.25s;
  }

  .city-card-body {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 24px;
    color: #fff;
  }

  .city-emoji {
    font-size: 24px;
    line-height: 1;
    display: block;
    margin-bottom: 6px;
  }

  .city-name {
    font-size: 22px;
    font-weight: 700;
    color: #fff;
    line-height: 1.2;
    margin-bottom: 4px;
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.5);
  }

  .city-chinese {
    font-size: 14px;
    opacity: 0.9;
    display: block;
    margin-bottom: 8px;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  }

  .city-desc {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.9);
    line-height: 1.5;
    margin-bottom: 12px;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  }

  .city-card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .city-days-count {
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.9;
  }

  .city-arrow {
    font-size: 16px;
    opacity: 0.9;
    transition: all 0.2s;
  }

  .city-card:hover .city-arrow {
    transform: translateX(4px);
  }

  /* --- Timeline (merged into map card) --- */
  .progress-bar {
    display: flex;
    gap: 0;
    margin: 0;
  }

  .progress-segment {
    flex: 1;
    height: 44px;
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
      var(--seg-color) 100%
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

  .segment-label {
    font-size: 11px;
    font-weight: 600;
    color: white;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.4);
  }

  .future .segment-label {
    color: var(--text-muted);
    text-shadow: none;
  }

  /* --- CTA --- */
  .cta-strip {
    text-align: center;
    padding: 8px 0 16px;
  }

  .cta-btn {
    display: inline-block;
    padding: 12px 32px;
    font-size: 15px;
    text-decoration: none;
    border-radius: var(--radius-lg);
    transition: all 0.25s;
  }

  .cta-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 28px rgba(184, 134, 11, 0.35);
  }

  /* --- Responsive --- */
  @media (max-width: 640px) {
    .hero {
      padding: 48px 0 32px;
    }
    .title-chinese {
      font-size: 40px;
    }
    .countdown-value {
      font-size: 26px;
    }
    .countdown-item {
      min-width: 56px;
      padding: 10px 6px 8px;
    }
    .hero-stats {
      gap: 16px;
    }
    .stat-value {
      font-size: 20px;
    }
    .segment-label {
      font-size: 9px;
    }
    .progress-segment {
      height: 36px;
    }
    .city-grid {
      grid-template-columns: 1fr;
    }
    .home {
      gap: 36px;
    }
  }
</style>
