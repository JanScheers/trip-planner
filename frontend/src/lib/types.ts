export interface Day {
  id: number;
  date: string;
  city_key: string;
  accommodation_key: string | null;
  tagline: string;
  notes: string;
  emoji: string | null;
  hero_image: string | null;
  travel: string | null;
}

export interface CreateDay {
  date: string;
  city_key: string;
  accommodation_key: string | null;
}

export interface UpdateDay {
  date?: string;
  city_key?: string;
  accommodation_key?: string | null;
  tagline?: string;
  notes?: string;
  emoji?: string | null;
  hero_image?: string | null;
  travel?: string | null;
}

export interface City {
  key: string;
  name: string;
  chinese_name: string;
  tagline: string;
  description: string;
  emoji: string | null;
  hero_image: string | null;
  lat: number | null;
  lng: number | null;
}

export interface CreateCity {
  name: string;
  chinese_name?: string;
  tagline?: string;
  lat?: number | null;
  lng?: number | null;
}

export interface UpdateCity {
  name?: string;
  chinese_name?: string;
  tagline?: string;
  description?: string;
  emoji?: string | null;
  hero_image?: string | null;
  lat?: number | null;
  lng?: number | null;
}

export interface Accommodation {
  key: string;
  name: string;
  link: string;
  notes: string;
  emoji: string | null;
  hero_image: string | null;
}

export interface CreateAccommodation {
  name: string;
}

export interface UpdateAccommodation {
  name?: string;
  link?: string;
  notes?: string;
  emoji?: string | null;
  hero_image?: string | null;
}

export interface AuthUser {
  email: string;
  name: string;
  picture: string | null;
  is_editor: boolean;
}

export interface UploadResponse {
  url: string;
}
