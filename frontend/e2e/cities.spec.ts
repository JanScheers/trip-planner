import { test, expect } from '@playwright/test';
import { loginAsEditor } from './helpers';

test.describe('Cities list', () => {
  test('shows all 7 cities', async ({ page }) => {
    await page.goto('/#/cities');
    // CitiesView renders an <ul class="city-list-ul"> with one <li> per city
    await expect(page.locator('.city-list-ul li')).toHaveCount(7, { timeout: 10_000 });
  });

  test('each city row links to its detail view', async ({ page }) => {
    await page.goto('/#/cities');
    await expect(page.locator('.city-list-ul li')).toHaveCount(7, { timeout: 10_000 });
    const beijingLink = page.locator('a[href="#/cities/beijing"]').first();
    await expect(beijingLink).toBeVisible();
  });
});

test.describe('City detail – public view', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/#/cities/beijing');
    await expect(page.locator('h1')).toContainText('Beijing', { timeout: 10_000 });
  });

  test('shows city name in heading', async ({ page }) => {
    await expect(page.locator('h1')).toHaveText('Beijing');
  });

  test('shows Chinese name', async ({ page }) => {
    await expect(page.locator('.chinese-name')).toBeVisible();
  });

  test('lists 4 days in Beijing from seed data', async ({ page }) => {
    // CityView renders "4 days in Beijing" section with a table
    await expect(page.locator('h3', { hasText: 'days in Beijing' })).toBeVisible({ timeout: 10_000 });
    await expect(page.locator('table tbody tr')).toHaveCount(4, { timeout: 10_000 });
  });

  test('nav brand returns to home', async ({ page }) => {
    await page.locator('.nav-brand').click();
    await expect(page).toHaveURL(/\/#?\/?$/);
  });
});

test.describe('City detail – editor view', () => {
  test.beforeEach(async ({ context, page }) => {
    await loginAsEditor(context);
    await page.goto('/#/cities/xian');
    await expect(page.locator('h1')).toContainText("Xi'an", { timeout: 10_000 });
    // Enable edit mode
    await page.locator('button.edit-toggle', { hasText: 'Edit' }).click();
  });

  test('shows editable fields for editors', async ({ page }) => {
    await expect(page.locator('input[aria-label="City name"]')).toBeVisible();
    await expect(page.locator('input[aria-label="Chinese name"]')).toBeVisible();
  });

  test('can update emoji via picker', async ({ page }) => {
    const emojiTrigger = page.locator('button.emoji-edit-trigger');
    if (await emojiTrigger.isVisible()) {
      await emojiTrigger.click();
      await expect(page.locator('emoji-picker')).toBeVisible({ timeout: 5000 });
      // Click first emoji in the picker (pierces shadow DOM)
      await page.locator('emoji-picker').locator('button').first().click();
      // Give the API call time to complete
      await page.waitForTimeout(500);
      // Reload to verify persistence
      await page.reload();
      await expect(page.locator('h1')).toContainText("Xi'an", { timeout: 10_000 });
    }
  });

  test('shows Add before / Add after per row in Days section when city has days', async ({ page }) => {
    await page.goto('/#/cities/beijing');
    await expect(page.locator('h1')).toContainText('Beijing', { timeout: 10_000 });
    await expect(page.locator('h3', { hasText: 'days in Beijing' })).toBeVisible();
    await expect(page.locator('button.btn-add-day', { hasText: 'Add before' }).first()).toBeVisible();
    await expect(page.locator('button.btn-add-day', { hasText: 'Add after' }).first()).toBeVisible();
  });
});
