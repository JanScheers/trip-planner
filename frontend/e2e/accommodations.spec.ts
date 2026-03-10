import { test, expect } from '@playwright/test';

test.describe('Accommodation detail – public view', () => {
  test('beijing-hutong renders with correct name', async ({ page }) => {
    await page.goto('/#/accommodations/beijing-hutong');
    await expect(page.locator('h1')).toHaveText('Beijing Hutong', { timeout: 10_000 });
  });

  test('shows 4 nights in beijing-hutong from seed data', async ({ page }) => {
    await page.goto('/#/accommodations/beijing-hutong');
    await expect(page.locator('h1')).toContainText('Beijing Hutong', { timeout: 10_000 });
    // AccommodationView renders "Booked for 4 nights" section with a table
    await expect(page.locator('h3', { hasText: 'Booked for 4 nights' })).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('table tbody tr')).toHaveCount(4, { timeout: 10_000 });
  });

  test('emeishan-baoguo-lodge renders correctly', async ({ page }) => {
    await page.goto('/#/accommodations/emeishan-baoguo-lodge');
    await expect(page.locator('h1')).toHaveText('Emeishan Baoguo Temple Lodge', { timeout: 10_000 });
  });

  test('shows 4 nights in emeishan-baoguo-lodge', async ({ page }) => {
    await page.goto('/#/accommodations/emeishan-baoguo-lodge');
    await expect(page.locator('h1')).toContainText('Emeishan Baoguo Temple Lodge', { timeout: 10_000 });
    await expect(page.locator('h3', { hasText: 'Booked for 4 nights' })).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('table tbody tr')).toHaveCount(4, { timeout: 10_000 });
  });

  test('hongkong-central renders correctly', async ({ page }) => {
    await page.goto('/#/accommodations/hongkong-central');
    await expect(page.locator('h1')).toHaveText('Hongkong Central', { timeout: 10_000 });
  });

  test('back link returns to itinerary', async ({ page }) => {
    await page.goto('/#/accommodations/beijing-hutong');
    await expect(page.locator('h1')).toContainText('Beijing Hutong', { timeout: 10_000 });
    await page.locator('a.back-link').click();
    await expect(page).toHaveURL('/#/days');
  });
});
