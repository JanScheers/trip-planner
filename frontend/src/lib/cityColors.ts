/**
 * Assign colors by city order (first date → last date).
 * The palette runs warm → cool, echoing dawn-to-dusk over the journey.
 * Works for any number of cities; wraps around if there are more cities than palette entries.
 */
const PALETTE = [
  '#e05252', // crimson
  '#e07c3a', // burnt orange
  '#d4b83a', // golden
  '#5ab56e', // jade green
  '#3aada8', // teal
  '#3a82d4', // sky blue
  '#7b5dd4', // violet
  '#c45db0', // magenta
  '#e05285', // rose
  '#5db0c4', // light blue
  '#7bc45d', // lime
  '#c4a05d', // bronze
];

export function getCityColor(key: string, cities: { key: string }[]): string {
  const index = cities.findIndex(c => c.key === key);
  if (index === -1) return '#d4a843';
  return PALETTE[index % PALETTE.length];
}
