<script lang="ts">
  import { api, staticUrl } from "../api";

  let {
    currentImage,
    onUpload,
  }: { currentImage: string | null; onUpload: (url: string) => void } =
    $props();

  let uploading = $state(false);
  let fileInput: HTMLInputElement;

  async function handleFile(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    uploading = true;
    try {
      const result = await api.upload(file);
      onUpload(result.url);
    } catch (err) {
      console.error(err);
      alert("Upload failed");
    } finally {
      uploading = false;
      input.value = "";
    }
  }

  function triggerUpload() {
    if (!uploading) fileInput?.click();
  }
</script>

<div class="image-upload">
  <input
    type="file"
    accept="image/*"
    bind:this={fileInput}
    onchange={handleFile}
    hidden
  />
  {#if currentImage}
    <button
      type="button"
      class="image-overlay-trigger"
      onclick={triggerUpload}
      disabled={uploading}
      aria-label="Change image"
    >
      <img src={staticUrl(currentImage)} alt="Hero" class="hero-image" />
      <span class="image-overlay">
        {uploading ? "Uploading..." : "Change Image"}
      </span>
    </button>
  {:else}
    <button
      type="button"
      class="upload-btn btn-outline btn-sm"
      onclick={() => fileInput?.click()}
      disabled={uploading}
    >
      {uploading ? "Uploading..." : "Upload Image"}
    </button>
  {/if}
</div>

<style>
  .image-upload {
    margin-bottom: 16px;
  }

  .image-overlay-trigger {
    position: relative;
    display: block;
    width: 100%;
    padding: 0;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .image-overlay-trigger:disabled {
    cursor: wait;
  }

  .image-overlay-trigger .hero-image {
    display: block;
    margin-bottom: 0;
  }

  .image-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.4);
    color: white;
    font-size: 15px;
    font-weight: 500;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .image-overlay-trigger:hover .image-overlay,
  .image-overlay-trigger:focus-visible .image-overlay,
  .image-overlay-trigger:disabled .image-overlay {
    opacity: 1;
  }

  .image-overlay-trigger:focus-visible {
    outline: 2px solid var(--gold);
    outline-offset: 2px;
  }

  .upload-btn {
    display: inline-block;
    cursor: pointer;
    padding: 6px 14px;
    border-radius: var(--radius);
    border: 1px solid var(--border-gold);
    color: var(--gold);
    font-size: 13px;
    transition: all 0.2s;
  }

  .upload-btn:hover {
    background: var(--gold-glow);
  }
</style>
