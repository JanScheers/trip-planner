import { test, expect } from '@playwright/test';

test.describe('Home page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('displays the countdown timer', async ({ page }) => {
    await expect(page.locator('.countdown')).toBeVisible();
    // The countdown shows either the timer grid or a "journey has begun" label
    const hasGrid = await page.locator('.countdown-grid').isVisible();
    const hasLabel = await page.locator('.countdown-label').isVisible();
    expect(hasGrid || hasLabel).toBe(true);
  });

  test('renders 7 city cards from seed data', async ({ page }) => {
    // Wait for the API to return cities
    await expect(page.locator('.city-card')).toHaveCount(7, { timeout: 10_000 });
  });

  test('city cards link to city views', async ({ page }) => {
    await expect(page.locator('.city-card')).toHaveCount(7, { timeout: 10_000 });
    // Each card is an anchor pointing to #/cities/{key}
    const firstCard = page.locator('.city-card').first();
    const href = await firstCard.getAttribute('href');
    expect(href).toMatch(/^#\/cities\//);
  });

  test('shows stats section with 21 days', async ({ page }) => {
    // Hero stats show totalDays (from API), citiesCount, and hardcoded Adventure
    const statValues = page.locator('.stat-value');
    await expect(statValues.first()).toHaveText('21', { timeout: 10_000 });
  });

  test('renders the SVG route map', async ({ page }) => {
    // ChinaMap renders an SVG; wait for it to appear
    await expect(page.locator('svg')).toBeVisible({ timeout: 10_000 });
  });

  test('renders the timeline progress bar with 21 segments', async ({ page }) => {
    await expect(page.locator('.progress-segment')).toHaveCount(21, { timeout: 10_000 });
  });
});
