<script lang="ts">
  import { getContext } from "svelte";
  import type { AuthUser } from "../types";
  import { api } from "../api";
  import { BG_MUSIC_KEY } from "../bgMusic";

  const bgMusic = getContext<{ playing: boolean; toggle: () => void }>(BG_MUSIC_KEY);

  let {
    user,
    editMode,
    isHomePage,
    isDayPage = false,
    onEnterPresentation,
    ontoggleedit,
  }: {
    user: AuthUser | null;
    editMode: boolean;
    isHomePage: boolean;
    isDayPage?: boolean;
    onEnterPresentation?: () => void;
    ontoggleedit: () => void;
  } = $props();

  let scrollY = $state(0);
  let navOpacity = $derived(isHomePage ? Math.min(1, scrollY / 120) : 1);

  $effect(() => {
    const onScroll = () => {
      scrollY = window.scrollY;
    };
    window.addEventListener("scroll", onScroll, { passive: true });
    scrollY = window.scrollY;
    return () => window.removeEventListener("scroll", onScroll);
  });
</script>

<nav
  style="opacity: {navOpacity}; transition: opacity 0.2s ease; pointer-events: {navOpacity <
  0.01
    ? 'none'
    : 'auto'};"
>
  <div class="nav-inner">
    <div class="nav-left">
      <a href="#/" class="nav-brand">
        <span class="brand-text">中国</span>
      </a>
      <div class="nav-links">
        <a href="#/days" class="nav-link">Itinerary</a>
        <a href="#/map" class="nav-link">Map</a>
        <a href="#/cities" class="nav-link">Cities</a>
        <a href="#/accommodations" class="nav-link">Stays</a>
        <a href="#/tips" class="nav-link">Tips</a>
        <a href="#/checklist" class="nav-link">Checklist</a>
      </div>
    </div>
    <div class="nav-right">
      {#if bgMusic}
        <button
          type="button"
          class="nav-action-btn nav-music-btn"
          onclick={bgMusic.toggle}
          title={bgMusic.playing ? "Pause music" : "Play music"}
          aria-label={bgMusic.playing ? "Pause background music" : "Play background music"}
        >
          {#if bgMusic.playing}
            <svg class="nav-music-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
              <path d="M15.54 8.46a5 5 0 0 1 0 7.07"/>
              <path d="M19.07 4.93a10 10 0 0 1 0 14.14"/>
            </svg>
          {:else}
            <svg class="nav-music-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
              <line x1="23" y1="9" x2="17" y2="15"/>
              <line x1="17" y1="9" x2="23" y2="15"/>
            </svg>
          {/if}
        </button>
      {/if}
      {#if isDayPage && onEnterPresentation}
        <button
          class="nav-action-btn"
          onclick={onEnterPresentation}
          title="Full screen"
          aria-label="Full screen"
        >
          ⛶
        </button>
      {/if}
      {#if user}
        {#if user.picture}
          <img src={user.picture} alt={user.name} class="user-avatar" />
        {/if}
        <span class="user-name">{user.name}</span>
        {#if user.is_editor}
          <button
            class="nav-action-btn edit-toggle"
            class:active={editMode}
            onclick={ontoggleedit}
          >
            {editMode ? "Editing" : "Edit"}
          </button>
        {/if}
        <a href={api.auth.logoutUrl} class="btn-outline btn-sm">Logout</a>
      {:else}
        <a
          href={api.auth.loginUrl}
          class="btn-gold btn-sm nav-login-btn"
          style="text-decoration:none;">Login</a
        >
      {/if}
    </div>
  </div>
</nav>

<style>
  nav {
    background: linear-gradient(180deg, #ffffff 0%, var(--bg-primary) 100%);
    border-bottom: 1px solid var(--border);
    box-shadow: 0 2px 20px rgba(44, 42, 38, 0.06);
  }

  .nav-inner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    min-height: var(--nav-bar-height);
    width: 100%;
  }

  .nav-left {
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .nav-brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 700;
    font-size: 18px;
    color: var(--gold) !important;
    transition: opacity 0.2s;
  }

  .nav-brand:hover {
    opacity: 0.85;
  }

  .nav-links {
    display: flex;
    gap: 20px;
  }

  .nav-link {
    color: var(--text-primary);
    font-size: 14px;
    font-weight: 500;
    position: relative;
    padding: 2px 0;
  }

  .nav-link::after {
    content: "";
    position: absolute;
    bottom: -2px;
    left: 0;
    right: 0;
    height: 1px;
    background: var(--gold);
    transform: scaleX(0);
    transition: transform 0.2s ease;
  }

  .nav-link:hover {
    color: var(--gold);
  }

  .nav-link:hover::after {
    transform: scaleX(1);
  }

  .nav-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .user-avatar {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 1px solid var(--border-gold);
  }

  .user-name {
    font-size: 13px;
    color: var(--text-primary);
  }

  .nav-action-btn {
    font-size: 12px;
    background: transparent;
    padding: 4px 10px;
    border-radius: 99px;
    font-weight: 600;
    border: 1px solid var(--border-gold);
    cursor: pointer;
    color: var(--gold-dim);
    transition:
      color 0.15s,
      border-color 0.15s,
      background 0.15s;
  }

  .nav-action-btn:hover {
    color: var(--gold);
    border-color: var(--gold);
  }

  .nav-action-btn.edit-toggle {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .nav-action-btn.edit-toggle.active {
    background: var(--gold-glow);
    color: var(--gold);
    border-color: var(--border-gold);
  }

  .nav-music-btn {
    padding: 6px 8px;
  }

  .nav-music-icon {
    width: 18px;
    height: 18px;
    display: block;
  }

  .nav-login-btn {
    border-radius: 99px;
  }

  @media (max-width: 600px) {
    .nav-links {
      display: none;
    }
    .user-name {
      display: none;
    }
  }
</style>
