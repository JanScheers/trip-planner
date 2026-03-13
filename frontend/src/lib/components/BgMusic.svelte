<script lang="ts">
  import type { Snippet } from "svelte";
  import { setContext } from "svelte";
  import { BG_MUSIC_KEY } from "../bgMusic";

  let { children }: { children?: Snippet } = $props();

  const AUDIO_SRC = "/費玉清.webm";
  const DEFAULT_VOLUME = 0.3;

  let audioEl = $state<HTMLAudioElement | null>(null);
  let ctx = $state({ playing: false, toggle: () => {} });
  let userInteracted = $state(false);

  function unlockAndPlay() {
    if (!audioEl || userInteracted) return;
    userInteracted = true;
    audioEl.muted = false;
    audioEl.volume = DEFAULT_VOLUME;
    audioEl.play().then(() => {
      ctx.playing = true;
    }).catch(() => {});
  }

  function toggle() {
    if (!audioEl) return;
    if (!userInteracted) {
      unlockAndPlay();
      return;
    }
    if (ctx.playing) {
      audioEl.pause();
      ctx.playing = false;
    } else {
      audioEl.play().then(() => {
        ctx.playing = true;
      }).catch(() => {});
    }
  }

  ctx.toggle = toggle;
  setContext(BG_MUSIC_KEY, ctx);

  $effect(() => {
    const handler = () => unlockAndPlay();
    document.addEventListener("click", handler, { once: true });
    return () => document.removeEventListener("click", handler);
  });

  $effect(() => {
    const el = audioEl;
    if (!el) return;
    const onPlay = () => { ctx.playing = true; };
    const onPause = () => { ctx.playing = false; };
    el.addEventListener("play", onPlay);
    el.addEventListener("pause", onPause);
    return () => {
      el.removeEventListener("play", onPlay);
      el.removeEventListener("pause", onPause);
    };
  });
</script>

<audio
  bind:this={audioEl}
  src={AUDIO_SRC}
  loop
  muted
  preload="auto"
  aria-label="Background music"
></audio>

{#if children}
  {@render children()}
{/if}
