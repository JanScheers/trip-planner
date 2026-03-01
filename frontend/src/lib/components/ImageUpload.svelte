<script lang="ts">
  import { api, staticUrl } from '../api';

  let { currentImage, onUpload }: { currentImage: string | null; onUpload: (url: string) => void } = $props();

  let uploading = $state(false);

  async function handleFile(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    uploading = true;
    try {
      const result = await api.upload(file);
      onUpload(result.url);
    } catch (err) {
      alert('Upload failed');
    }
    uploading = false;
  }
</script>

<div class="image-upload">
  {#if currentImage}
    <img src={staticUrl(currentImage)} alt="Hero" class="hero-image" />
  {/if}
  <label class="upload-btn btn-outline btn-sm">
    {uploading ? 'Uploading...' : currentImage ? 'Change Image' : 'Upload Image'}
    <input type="file" accept="image/*" onchange={handleFile} hidden />
  </label>
</div>

<style>
  .image-upload {
    margin-bottom: 16px;
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
