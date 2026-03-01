<script lang="ts">
  import type { AuthUser } from '../types';
  import { api } from '../api';

  let { user }: { user: AuthUser | null } = $props();
</script>

<nav>
  <div class="nav-inner container">
    <div class="nav-left">
      <a href="#/" class="nav-brand">
        <span class="brand-icon">🏯</span>
        <span class="brand-text">China 2026</span>
      </a>
      <div class="nav-links">
        <a href="#/" class="nav-link">Home</a>
        <a href="#/days" class="nav-link">Itinerary</a>
      </div>
    </div>
    <div class="nav-right">
      {#if user}
        {#if user.picture}
          <img src={user.picture} alt={user.name} class="user-avatar" />
        {/if}
        <span class="user-name">{user.name}</span>
        {#if user.is_editor}
          <span class="editor-badge">Editor</span>
        {/if}
        <a href={api.auth.logoutUrl} class="btn-outline btn-sm">Logout</a>
      {:else}
        <a href={api.auth.loginUrl} class="btn-gold btn-sm" style="text-decoration:none;">Login</a>
      {/if}
    </div>
  </div>
</nav>

<style>
  nav {
    background: rgba(18, 18, 26, 0.85);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    z-index: 100;
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
  }

  .nav-inner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: 12px;
    padding-bottom: 12px;
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

  .brand-icon {
    font-size: 22px;
  }

  .nav-links {
    display: flex;
    gap: 20px;
  }

  .nav-link {
    color: var(--text-secondary);
    font-size: 14px;
    font-weight: 500;
    position: relative;
    padding: 2px 0;
  }

  .nav-link::after {
    content: '';
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
    color: var(--text-secondary);
  }

  .editor-badge {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: var(--gold-glow);
    color: var(--gold);
    padding: 2px 8px;
    border-radius: 99px;
    font-weight: 600;
    border: 1px solid var(--border-gold);
  }

  @media (max-width: 600px) {
    .nav-links { display: none; }
    .user-name { display: none; }
  }
</style>
