import type {
  Day, CreateDay, UpdateDay,
  City, UpdateCity,
  Accommodation, CreateAccommodation, UpdateAccommodation,
  AuthUser, UploadResponse
} from './types';

const BASE = 'http://localhost:8080';

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    credentials: 'include',
    headers: { 'Content-Type': 'application/json', ...options.headers as Record<string, string> },
    ...options,
  });
  if (!res.ok) throw new Error(`${res.status}: ${await res.text()}`);
  return res.json();
}

export const api = {
  auth: {
    me: () => request<AuthUser | null>('/api/auth/me'),
    loginUrl: `${BASE}/api/auth/login`,
    logoutUrl: `${BASE}/api/auth/logout`,
  },
  days: {
    list: () => request<Day[]>('/api/days'),
    get: (id: number) => request<Day>(`/api/days/${id}`),
    create: (data: CreateDay) => request<Day>('/api/days', { method: 'POST', body: JSON.stringify(data) }),
    update: (id: number, data: UpdateDay) => request<Day>(`/api/days/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
    delete: (id: number) => request<string>(`/api/days/${id}`, { method: 'DELETE' }),
  },
  cities: {
    list: () => request<City[]>('/api/cities'),
    get: (key: string) => request<City>(`/api/cities/${key}`),
    update: (key: string, data: UpdateCity) => request<City>(`/api/cities/${key}`, { method: 'PUT', body: JSON.stringify(data) }),
  },
  accommodations: {
    list: () => request<Accommodation[]>('/api/accommodations'),
    get: (key: string) => request<Accommodation>(`/api/accommodations/${key}`),
    create: (data: CreateAccommodation) => request<Accommodation>('/api/accommodations', { method: 'POST', body: JSON.stringify(data) }),
    update: (key: string, data: UpdateAccommodation) => request<Accommodation>(`/api/accommodations/${key}`, { method: 'PUT', body: JSON.stringify(data) }),
    delete: (key: string) => request<string>(`/api/accommodations/${key}`, { method: 'DELETE' }),
  },
  upload: async (file: File): Promise<UploadResponse> => {
    const form = new FormData();
    form.append('file', file);
    const res = await fetch(`${BASE}/api/upload`, { method: 'POST', body: form, credentials: 'include' });
    if (!res.ok) throw new Error(`Upload failed: ${res.status}`);
    return res.json();
  },
  exportUrl: `${BASE}/api/export`,
};
