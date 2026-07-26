import { Page, expect } from '@playwright/test';

/**
 * E2E Test Helper Utilities
 * Reusable functions for common test scenarios
 */

export class TestHelpers {
  constructor(private page: Page) {}

  /**
   * Mock wallet connection for testing
   */
  async mockWalletConnection() {
    await this.page.addInitScript(() => {
      // @ts-ignore
      window.freighter = {
        isConnected: () => Promise.resolve(true),
        getPublicKey: () => Promise.resolve('GTEST123456789ABCDEFGH...'),
        signTransaction: () => Promise.resolve({ signedTxXdr: 'mock_signed_tx' }),
      };
    });
  }

  /**
   * Mock Stellar contract calls
   */
  async mockContractCalls() {
    await this.page.route('**/api/contract/**', (route) => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: { txHash: 'mock_tx_hash' },
        }),
      });
    });
  }

  /**
   * Wait for transaction to complete
   */
  async waitForTransaction(timeout: number = 10000) {
    await this.page.waitForSelector('[data-testid="transaction-success"]', { timeout });
  }

  /**
   * Fill waste submission form
   */
  async fillWasteForm(data: {
    type: string;
    weight: string;
    latitude: string;
    longitude: string;
  }) {
    await this.page.selectOption('[data-testid="waste-type"]', data.type);
    await this.page.fill('[data-testid="waste-weight"]', data.weight);
    await this.page.fill('[data-testid="waste-latitude"]', data.latitude);
    await this.page.fill('[data-testid="waste-longitude"]', data.longitude);
  }

  /**
   * Fill participant registration form
   */
  async fillRegistrationForm(data: {
    name: string;
    role: string;
    latitude: string;
    longitude: string;
  }) {
    await this.page.fill('[data-testid="participant-name"]', data.name);
    await this.page.selectOption('[data-testid="participant-role"]', data.role);
    await this.page.fill('[data-testid="latitude"]', data.latitude);
    await this.page.fill('[data-testid="longitude"]', data.longitude);
  }

  /**
   * Check if element is visible with retry
   */
  async assertVisible(selector: string, timeout: number = 5000) {
    await expect(this.page.locator(selector)).toBeVisible({ timeout });
  }

  /**
   * Check if text content matches
   */
  async assertTextContains(selector: string, text: string) {
    await expect(this.page.locator(selector)).toContainText(text);
  }

  /**
   * Take screenshot with timestamp
   */
  async takeScreenshot(name: string) {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    await this.page.screenshot({
      path: `test-results/screenshots/${name}-${timestamp}.png`,
      fullPage: true,
    });
  }

  /**
   * Wait for API response
   */
  async waitForApiResponse(urlPattern: string, timeout: number = 5000) {
    return await this.page.waitForResponse(
      (response) => response.url().includes(urlPattern) && response.status() === 200,
      { timeout }
    );
  }

  /**
   * Navigate and wait for page load
   */
  async navigateTo(path: string) {
    await this.page.goto(path);
    await this.page.waitForLoadState('networkidle');
  }

  /**
   * Check for console errors
   */
  async checkConsoleErrors() {
    const errors: string[] = [];
    this.page.on('console', (msg) => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });
    return errors;
  }

  /**
   * Mock geolocation for testing
   */
  async mockGeolocation(latitude: number, longitude: number) {
    await this.page.context().setGeolocation({ latitude, longitude });
    await this.page.context().grantPermissions(['geolocation']);
  }

  /**
   * Fill search form
   */
  async searchWaste(query: string) {
    await this.page.fill('[data-testid="search-input"]', query);
    await this.page.click('[data-testid="search-button"]');
    await this.page.waitForSelector('[data-testid="search-results"]');
  }

  /**
   * Apply filters
   */
  async applyFilters(filters: Record<string, string>) {
    for (const [key, value] of Object.entries(filters)) {
      const selector = `[data-testid="filter-${key}"]`;
      await this.page.fill(selector, value);
    }
    await this.page.click('[data-testid="apply-filters"]');
  }

  /**
   * Verify table data
   */
  async verifyTableHasRows(selector: string, minRows: number = 1) {
    const rows = await this.page.locator(`${selector} tbody tr`).count();
    expect(rows).toBeGreaterThanOrEqual(minRows);
  }

  /**
   * Wait for loading to finish
   */
  async waitForLoadingComplete() {
    await this.page.waitForSelector('[data-testid="loading"]', { state: 'hidden' });
  }

  /**
   * Check accessibility violations
   */
  async checkAccessibility() {
    const violations = await this.page.evaluate(async () => {
      // @ts-ignore - axe-core is loaded separately
      if (typeof window.axe !== 'undefined') {
        const results = await window.axe.run();
        return results.violations;
      }
      return [];
    });
    return violations;
  }

  /**
   * Test keyboard navigation
   */
  async testKeyboardNav(tabCount: number) {
    for (let i = 0; i < tabCount; i++) {
      await this.page.keyboard.press('Tab');
      await this.page.waitForTimeout(100);
    }
  }

  /**
   * Mock date/time for testing
   */
  async mockDateTime(date: Date) {
    await this.page.addInitScript((timestamp) => {
      Date.now = () => timestamp;
    }, date.getTime());
  }

  /**
   * Fill and submit form
   */
  async submitForm(formSelector: string, data: Record<string, string>) {
    for (const [field, value] of Object.entries(data)) {
      await this.page.fill(`${formSelector} [name="${field}"]`, value);
    }
    await this.page.click(`${formSelector} [type="submit"]`);
  }

  /**
   * Verify notification appears
   */
  async verifyNotification(message: string, type: 'success' | 'error' | 'info' = 'success') {
    await expect(
      this.page.locator(`[data-testid="notification-${type}"]`)
    ).toContainText(message);
  }

  /**
   * Download and verify file
   */
  async downloadFile(buttonSelector: string) {
    const downloadPromise = this.page.waitForEvent('download');
    await this.page.click(buttonSelector);
    const download = await downloadPromise;
    const path = await download.path();
    expect(path).toBeTruthy();
    return download;
  }
}

/**
 * Test data factory
 */
export class TestDataFactory {
  static generateWasteData() {
    return {
      type: 'Plastic',
      weight: (Math.random() * 10000 + 1000).toFixed(0),
      latitude: (Math.random() * 180 - 90).toFixed(6),
      longitude: (Math.random() * 360 - 180).toFixed(6),
    };
  }

  static generateParticipantData(role: string) {
    return {
      name: `Test ${role} ${Math.random().toString(36).substring(7)}`,
      role,
      latitude: (Math.random() * 180 - 90).toFixed(6),
      longitude: (Math.random() * 360 - 180).toFixed(6),
    };
  }

  static generateIncentiveData() {
    return {
      wasteType: 'Metal',
      rewardPoints: (Math.random() * 200 + 50).toFixed(0),
      budget: (Math.random() * 100000 + 10000).toFixed(0),
    };
  }
}

/**
 * Performance measurement utilities
 */
export class PerformanceHelpers {
  static async measurePageLoad(page: Page, url: string) {
    const startTime = Date.now();
    await page.goto(url);
    await page.waitForLoadState('networkidle');
    return Date.now() - startTime;
  }

  static async measureRenderTime(page: Page, selector: string) {
    const startTime = Date.now();
    await page.waitForSelector(selector);
    return Date.now() - startTime;
  }

  static async measureApiResponseTime(page: Page, apiUrl: string, action: () => Promise<void>) {
    const startTime = Date.now();
    const responsePromise = page.waitForResponse((response) => response.url().includes(apiUrl));
    await action();
    await responsePromise;
    return Date.now() - startTime;
  }

  static async getMemoryUsage(page: Page) {
    return await page.evaluate(() => {
      // @ts-ignore
      return performance.memory ? performance.memory.usedJSHeapSize : 0;
    });
  }
}

/**
 * Visual regression helpers
 */
export class VisualRegressionHelpers {
  static async compareScreenshot(page: Page, name: string, options?: any) {
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(500); // Let animations settle
    return await page.screenshot({
      path: `test-results/screenshots/${name}.png`,
      fullPage: true,
      ...options,
    });
  }

  static async compareElement(page: Page, selector: string, name: string) {
    const element = page.locator(selector);
    await element.waitFor({ state: 'visible' });
    return await element.screenshot({
      path: `test-results/screenshots/${name}-element.png`,
    });
  }
}
