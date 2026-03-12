import { describe, it, expect, beforeEach } from 'vitest';
import { parseHash } from '../router';

// jsdom allows setting window.location.hash without navigation.
function setHash(hash: string) {
  window.location.hash = hash;
}

describe('parseHash', () => {
  beforeEach(() => {
    window.location.hash = '';
  });

  it('returns home for empty hash', () => {
    setHash('');
    expect(parseHash()).toEqual({ page: 'home', params: {} });
  });

  it('returns home for #/', () => {
    setHash('#/');
    expect(parseHash()).toEqual({ page: 'home', params: {} });
  });

  it('returns basic for #/days', () => {
    setHash('#/days');
    expect(parseHash()).toEqual({ page: 'basic', params: {} });
  });

  it('returns day with id for #/days/5', () => {
    setHash('#/days/5');
    expect(parseHash()).toEqual({ page: 'day', params: { id: '5' } });
  });

  it('returns cities for #/cities', () => {
    setHash('#/cities');
    expect(parseHash()).toEqual({ page: 'cities', params: {} });
  });

  it('returns city with key for #/cities/beijing', () => {
    setHash('#/cities/beijing');
    expect(parseHash()).toEqual({ page: 'city', params: { key: 'beijing' } });
  });

  it('returns accommodations list for #/accommodations', () => {
    setHash('#/accommodations');
    expect(parseHash()).toEqual({ page: 'accommodations', params: {} });
  });

  it('returns accommodation with key for #/accommodations/hotel-a', () => {
    setHash('#/accommodations/hotel-a');
    expect(parseHash()).toEqual({ page: 'accommodation', params: { key: 'hotel-a' } });
  });

  it('returns home for unknown segment', () => {
    setHash('#/unknown');
    expect(parseHash()).toEqual({ page: 'home', params: {} });
  });

  it('returns home for too many segments (#/days/5/extra)', () => {
    setHash('#/days/5/extra');
    expect(parseHash()).toEqual({ page: 'home', params: {} });
  });
});
