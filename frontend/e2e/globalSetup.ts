import { execSync } from 'child_process';
import { rmSync } from 'fs';
import path from 'path';

const E2E_DB = path.resolve(__dirname, '../../backend/e2e-test.db');

export default function globalSetup() {
  // Kill any process already holding port 8080 so Playwright can start the
  // backend fresh. On macOS lsof is available; the command is a no-op if the
  // port is free.
  try {
    execSync('lsof -ti:8080 | xargs kill -9', { stdio: 'ignore' });
    // Give the OS a moment to release the port before cargo run binds it.
    execSync('sleep 0.5');
  } catch {
    // Port was not in use — nothing to do.
  }

  // Remove the stale database so the backend re-seeds from seed.tsv on startup.
  rmSync(E2E_DB, { force: true });
  rmSync(`${E2E_DB}-shm`, { force: true });
  rmSync(`${E2E_DB}-wal`, { force: true });
}
