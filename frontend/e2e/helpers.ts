import type { BrowserContext } from '@playwright/test';

/** Session cookie value pre-seeded by the backend via TEST_EDITOR_SESSION env var. */
export const EDITOR_SESSION = 'pw-editor-session';

/** Inject the editor session cookie so subsequent requests are authenticated as an editor. */
export async function loginAsEditor(context: BrowserContext) {
  await context.addCookies([
    {
      name: 'session',
      value: EDITOR_SESSION,
      domain: 'localhost',
      path: '/',
    },
  ]);
}
