import { test, expect } from '@playwright/test';
import { loginAsEditor } from './helpers';

test.describe('Export TSV', () => {
  test('export endpoint is accessible to authenticated users', async ({ context, page }) => {
    await loginAsEditor(context);
    // Make a direct API request to the export endpoint
    const response = await page.request.get('/api/export');
    expect(response.status()).toBe(200);
  });

  test('export returns tab-separated content with correct header', async ({ context, page }) => {
    await loginAsEditor(context);
    const response = await page.request.get('/api/export');
    const text = await response.text();
    expect(text).toMatch(/^date\tcity\taccommodation\n/);
  });

  test('export contains all 21 days', async ({ context, page }) => {
    await loginAsEditor(context);
    const response = await page.request.get('/api/export');
    const text = await response.text();
    const lines = text.trim().split('\n');
    // 1 header + 21 data rows
    expect(lines.length).toBe(22);
  });

  test('export contains beijing-hutong accommodation', async ({ context, page }) => {
    await loginAsEditor(context);
    const response = await page.request.get('/api/export');
    const text = await response.text();
    expect(text).toContain('beijing-hutong');
  });

  test('export contains all 7 cities', async ({ context, page }) => {
    await loginAsEditor(context);
    const response = await page.request.get('/api/export');
    const text = await response.text();
    for (const city of ['beijing', 'xian', 'chengdu', 'chongqing', 'zhangjiajie', 'guilin', 'hongkong']) {
      expect(text).toContain(city);
    }
  });

  test('export is rejected for unauthenticated requests', async ({ page }) => {
    const response = await page.request.get('/api/export');
    expect(response.status()).toBe(401);
  });

  test('Export TSV link is visible in the itinerary view', async ({ page }) => {
    await page.goto('/#/days');
    await expect(page.locator('a[href*="api/export"]')).toBeVisible({ timeout: 10_000 });
  });
});
