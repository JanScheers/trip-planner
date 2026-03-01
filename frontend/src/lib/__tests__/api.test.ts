import { describe, it, expect, beforeEach, vi } from 'vitest';
import { staticUrl } from '../api';

// Ensure BASE is empty (no VITE_API_ORIGIN set) for predictable path tests.
vi.stubEnv('VITE_API_ORIGIN', '');

describe('staticUrl', () => {
  beforeEach(() => {
    vi.unstubAllEnvs();
    vi.stubEnv('VITE_API_ORIGIN', '');
  });

  it('returns empty string for null', () => {
    expect(staticUrl(null)).toBe('');
  });

  it('returns empty string for empty string', () => {
    expect(staticUrl('')).toBe('');
  });

  it('returns empty string for undefined', () => {
    expect(staticUrl(undefined)).toBe('');
  });

  it('returns path unchanged when it is a relative /static path (BASE is empty)', () => {
    expect(staticUrl('/static/img.png')).toBe('/static/img.png');
  });

  it('returns absolute URL unchanged (starts with http)', () => {
    expect(staticUrl('https://cdn.example.com/img.png')).toBe('https://cdn.example.com/img.png');
  });

  it('returns http URL unchanged', () => {
    expect(staticUrl('http://example.com/photo.jpg')).toBe('http://example.com/photo.jpg');
  });
});
