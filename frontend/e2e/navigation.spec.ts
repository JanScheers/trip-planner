import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('nav brand links to home', async ({ page }) => {
    await page.goto('/#/days');
    await page.locator('.nav-brand').click();
    await expect(page).toHaveURL(/\/#?\/?$/);
  });

  test('Itinerary nav link routes to #/days', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a[href="#/days"]').click();
    await expect(page).toHaveURL('/#/days');
    await expect(page.locator('h1')).toContainText('Itinerary');
  });

  test('Cities nav link routes to #/cities', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a[href="#/cities"]').click();
    await expect(page).toHaveURL('/#/cities');
  });

  test('clicking a city card navigates to city detail', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.city-card')).toHaveCount(7, { timeout: 10_000 });
    await page.locator('.city-card[href="#/cities/beijing"]').click();
    await expect(page).toHaveURL('/#/cities/beijing');
    await expect(page.locator('h1')).toContainText('Beijing');
  });

  test('clicking a date link in itinerary navigates to day view', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
    await page.locator('a.date-link').first().click();
    await expect(page).toHaveURL(/\/#\/days\/\d+/);
  });

  test('deep-linking to #/days/1 renders the day view', async ({ page }) => {
    await page.goto('/#/days/1');
    // Day view shows the formatted date in h1 and a back link
    await expect(page.locator('a.back-link')).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('h1')).toBeVisible();
  });

  test('Login button is visible when not authenticated', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav a[href*="api/auth/login"]')).toBeVisible();
  });
});
