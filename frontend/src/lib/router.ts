export interface Route {
  page: string;
  params: Record<string, string>;
}

export function parseHash(): Route {
  const hash = window.location.hash.slice(1) || '/';
  const parts = hash.split('/').filter(Boolean);

  if (parts.length === 0) return { page: 'home', params: {} };
  if (parts[0] === 'days' && parts.length === 1) return { page: 'basic', params: {} };
  if (parts[0] === 'days' && parts.length === 2) return { page: 'day', params: { id: parts[1] } };
  if (parts[0] === 'cities' && parts.length === 1) return { page: 'cities', params: {} };
  if (parts[0] === 'cities' && parts.length === 2) return { page: 'city', params: { key: parts[1] } };
  if (parts[0] === 'accommodations' && parts.length === 1) return { page: 'accommodations', params: {} };
  if (parts[0] === 'accommodations' && parts.length === 2) return { page: 'accommodation', params: { key: parts[1] } };
  if (parts[0] === 'tips' && parts.length === 1) return { page: 'tips', params: {} };
  if (parts[0] === 'checklist' && parts.length === 1) return { page: 'checklist', params: {} };
  if (parts[0] === 'map' && parts.length === 1) return { page: 'map', params: {} };
  return { page: 'home', params: {} };
}

export function navigate(path: string) {
  window.location.hash = path;
}
