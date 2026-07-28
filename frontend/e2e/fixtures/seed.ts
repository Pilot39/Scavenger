/**
 * E2E Seed Utility (#944)
 *
 * Provides database and local-state seeding helpers for the Playwright
 * happy-path test suite.  Because the E2E suite targets a running frontend
 * that mocks Stellar calls, this module provides:
 *
 *  1. Browser-context state (localStorage / sessionStorage) seeds.
 *  2. Mock API response fixtures injected via page.route().
 *  3. Freighter wallet stub injected via page.addInitScript().
 */

import { type Page, type BrowserContext } from '@playwright/test';

// ─── Constants ────────────────────────────────────────────────────────────────

export const TEST_WALLET_ADDRESS =
  'GTEST7SCAVNGR000000000000000000000000000000000000000000001';

export const TEST_CONTRACT_ID =
  'CSCAVNGR0000000000000000000000000000000000000000000000001';

// ─── Wallet seed ─────────────────────────────────────────────────────────────

/**
 * Inject a stub Freighter wallet into the browser context.
 * Must be called before page.goto() to take effect on first load.
 */
export async function seedWalletConnection(page: Page): Promise<void> {
  await page.addInitScript((address: string) => {
    /* eslint-disable @typescript-eslint/no-explicit-any */
    (window as any).freighter = {
      isConnected: () => Promise.resolve(true),
      isAllowed: () => Promise.resolve(true),
      getPublicKey: () => Promise.resolve(address),
      getNetwork: () => Promise.resolve('TESTNET'),
      getNetworkDetails: () =>
        Promise.resolve({ network: 'TESTNET', networkUrl: 'https://soroban-testnet.stellar.org' }),
      signTransaction: (_xdr: string) =>
        Promise.resolve({ signedTxXdr: 'AAAAAQAAAAAAAAAA' }),
      signAuthEntry: (_entryXdr: string) =>
        Promise.resolve({ signedEntryXdr: 'AAAAAQAAAAAAAAAA' }),
    };
    /* eslint-enable @typescript-eslint/no-explicit-any */
  }, TEST_WALLET_ADDRESS);
}

// ─── API route mocks ─────────────────────────────────────────────────────────

export interface MockParticipant {
  address: string;
  name: string;
  role: number;
  latitude: number;
  longitude: number;
  registered_at: number;
  is_active: boolean;
}

export interface MockWaste {
  waste_id: number;
  waste_type: number;
  weight: number;
  current_owner: string;
  latitude: number;
  longitude: number;
  recycled_timestamp: number;
  is_active: boolean;
  is_confirmed: boolean;
  confirmer: string;
}

const defaultParticipant: MockParticipant = {
  address: TEST_WALLET_ADDRESS,
  name: 'E2E Test Recycler',
  role: 0,
  latitude: 40712800,
  longitude: -74006000,
  registered_at: Math.floor(Date.now() / 1000),
  is_active: true,
};

const defaultWaste: MockWaste = {
  waste_id: 1,
  waste_type: 0,
  weight: 1500,
  current_owner: TEST_WALLET_ADDRESS,
  latitude: 40712800,
  longitude: -74006000,
  recycled_timestamp: Math.floor(Date.now() / 1000),
  is_active: true,
  is_confirmed: false,
  confirmer: '',
};

/**
 * Seed all API routes with mock responses.
 * Call once per test in beforeEach.
 */
export async function seedApiRoutes(
  page: Page,
  overrides: {
    participant?: Partial<MockParticipant>;
    waste?: Partial<MockWaste>;
    wastes?: MockWaste[];
    txHash?: string;
  } = {}
): Promise<void> {
  const participant = { ...defaultParticipant, ...overrides.participant };
  const waste = { ...defaultWaste, ...overrides.waste };
  const wastes = overrides.wastes ?? [waste];
  const txHash = overrides.txHash ?? '0000000000000000000000000000000000000000000000000000000000000001';

  // Health
  await page.route('**/api/health', (route) =>
    route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ status: 'ok' }) })
  );

  // Participant lookup
  await page.route(`**/api/participants/${TEST_WALLET_ADDRESS}`, (route) =>
    route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(participant) })
  );

  // Participant registration
  await page.route('**/api/participants', (route) => {
    if (route.request().method() === 'POST') {
      return route.fulfill({
        status: 201,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, participant }),
      });
    }
    return route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ participants: [participant], total: 1 }),
    });
  });

  // Waste list
  await page.route('**/api/wastes', (route) => {
    if (route.request().method() === 'POST') {
      return route.fulfill({
        status: 201,
        contentType: 'application/json',
        body: JSON.stringify({ success: true, waste, txHash }),
      });
    }
    return route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ wastes, total: wastes.length }),
    });
  });

  // Individual waste
  await page.route(`**/api/wastes/${waste.waste_id}`, (route) =>
    route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(waste) })
  );

  // Contract / transaction submission
  await page.route('**/api/contract/**', (route) =>
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ success: true, txHash }),
    })
  );

  // Stats / metrics
  await page.route('**/api/metrics', (route) =>
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ eventsProcessed: 42, totalWaste: wastes.length }),
    })
  );

  // Incentives
  await page.route('**/api/incentives**', (route) =>
    route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({ incentives: [], total: 0 }),
    })
  );
}

// ─── localStorage state seeds ────────────────────────────────────────────────

/**
 * Pre-populate localStorage with a connected-wallet state so pages that
 * gate on wallet connectivity skip the connect-wallet modal.
 */
export async function seedLocalStorageAuth(context: BrowserContext): Promise<void> {
  await context.addInitScript((address: string) => {
    localStorage.setItem('walletAddress', address);
    localStorage.setItem('walletConnected', 'true');
    localStorage.setItem('walletType', 'freighter');
  }, TEST_WALLET_ADDRESS);
}

/**
 * Clear all auth state from localStorage.
 */
export async function clearLocalStorageAuth(page: Page): Promise<void> {
  await page.evaluate(() => {
    localStorage.removeItem('walletAddress');
    localStorage.removeItem('walletConnected');
    localStorage.removeItem('walletType');
  });
}

// ─── Test state helpers ───────────────────────────────────────────────────────

/**
 * Wait for the page to reach a stable loaded state.
 * Waits for the network to be idle and the main content to appear.
 */
export async function waitForAppReady(page: Page): Promise<void> {
  await page.waitForLoadState('networkidle');
}

/**
 * Dismiss any toast / notification banners that may obscure test interactions.
 */
export async function dismissNotifications(page: Page): Promise<void> {
  try {
    const closeBtn = page.locator('[data-sonner-toast] button[aria-label="Close"]');
    if (await closeBtn.isVisible({ timeout: 500 })) {
      await closeBtn.click();
    }
  } catch {
    // No notifications visible — continue
  }
}
