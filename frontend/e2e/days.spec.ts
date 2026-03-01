import { test, expect } from '@playwright/test';
import { loginAsEditor } from './helpers';

test.describe('Days – public view', () => {
  test('itinerary table shows 21 rows from seed data', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
  });

  test('first row shows Oct 1 (Beijing)', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
    const firstRow = page.locator('table tbody tr').first();
    await expect(firstRow.locator('a.date-link')).toContainText('Oct 1');
  });

  test('accommodation links are visible in itinerary', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
    // First row (beijing) should link to beijing-hutong accommodation
    const firstRow = page.locator('table tbody tr').first();
    await expect(firstRow.locator('a[href="#/accommodations/beijing-hutong"]')).toBeVisible();
  });

  test('no Add Day button for anonymous users', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('button.btn-gold')).not.toBeVisible();
  });

  test('no delete buttons for anonymous users', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('button.btn-danger')).not.toBeVisible();
  });
});

test.describe('Days – editor view', () => {
  test.beforeEach(async ({ context }) => {
    await loginAsEditor(context);
  });

  test('shows Add Day button for editors', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
    await expect(page.locator('button.btn-gold', { hasText: '+ Add Day' })).toBeVisible();
  });

  test('shows delete buttons for each row when editor', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('button.btn-danger')).toHaveCount(21, { timeout: 10_000 });
  });

  test('shows editor badge in nav', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.editor-badge')).toBeVisible({ timeout: 10_000 });
  });

  test('add day appends a new row', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });

    await page.locator('button.btn-gold', { hasText: '+ Add Day' }).click();
    await expect(page.locator('table tbody tr')).toHaveCount(22, { timeout: 10_000 });

    // Clean up: delete the last row
    page.on('dialog', (d) => d.accept());
    await page.locator('button.btn-danger').last().click();
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
  });

  test('editor can navigate to day detail and see city link', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('table tbody tr')).toHaveCount(21, { timeout: 10_000 });
    await page.locator('a.date-link').first().click();
    await expect(page).toHaveURL(/\/#\/days\/\d+/);
    // Day view subtitle contains the city link
    await expect(page.locator('.subtitle a[href*="#/cities/beijing"]')).toBeVisible({
      timeout: 10_000,
    });
  });
});
