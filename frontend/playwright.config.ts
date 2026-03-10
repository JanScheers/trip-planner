import { defineConfig, devices } from '@playwright/test';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const backendDir = path.resolve(__dirname, '../backend');

/**
 * E2E tests run against a real backend + frontend.
 *
 * globalSetup (e2e/globalSetup.ts) runs before anything else:
 *   1. Kills any process on port 8080 (ensures a clean backend start)
 *   2. Deletes e2e-test.db (forces re-seeding from seed/)
 *
 * The backend is started with:
 *   - TEST_EDITOR_SESSION=pw-editor-session  (pre-seeds an editor session)
 *   - EDITOR_EMAILS=e2e-editor@example.com
 *   - DATABASE_URL pointing to e2e-test.db (auto-seeded on first run)
 *   - SEED_DIR=../seed
 *
 * Editor specs inject the session cookie `session=pw-editor-session` before each test.
 */
export const EDITOR_SESSION = 'pw-editor-session';
export const EDITOR_EMAIL = 'e2e-editor@example.com';
const E2E_DB = path.resolve(backendDir, 'e2e-test.db');

export default defineConfig({
  testDir: './e2e',
  globalSetup: './e2e/globalSetup.ts',
  fullyParallel: false, // tests share backend state; run sequentially
  retries: 0,
  reporter: 'list',
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: [
    {
      command: `TEST_EDITOR_SESSION=${EDITOR_SESSION} EDITOR_EMAILS=${EDITOR_EMAIL} DATABASE_URL=sqlite:${E2E_DB}?mode=rwc SEED_DIR=../seed cargo run`,
      cwd: backendDir,
      port: 8080,
      timeout: 120_000,
      // Always start fresh — globalSetup already freed the port.
      reuseExistingServer: false,
    },
    {
      command: 'npm run dev',
      port: 5173,
      timeout: 30_000,
      reuseExistingServer: !process.env.CI,
    },
  ],
});
