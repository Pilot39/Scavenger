import { test, expect, Page } from '@playwright/test';

// Shared helper: wait for page to be visually stable
async function waitForStable(page: Page) {
  await page.waitForLoadState('networkidle');
  // Pause animations/transitions for deterministic screenshots
  await page.addStyleTag({
    content: `*, *::before, *::after {
      animation-duration: 0s !important;
      transition-duration: 0s !important;
    }`,
  });
}

// Public pages (no auth required)
test.describe('Public Pages', () => {
  test('landing page - desktop', async ({ page }) => {
    await page.goto('/');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('landing-desktop.png', { fullPage: true });
  });

  test('landing page - mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('landing-mobile.png', { fullPage: true });
  });

  test('landing page - tablet', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('landing-tablet.png', { fullPage: true });
  });

  test('login page', async ({ page }) => {
    await page.goto('/login');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('login.png', { fullPage: true });
  });

  test('login page - mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('login-mobile.png', { fullPage: true });
  });

  test('404 not found page', async ({ page }) => {
    await page.goto('/this-page-does-not-exist');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('not-found.png', { fullPage: true });
  });
});

// Authenticated pages — mock auth via localStorage before navigation
test.describe('Authenticated Pages', () => {
  test.beforeEach(async ({ page }) => {
    // Seed auth state so ProtectedLayout passes
    await page.addInitScript(() => {
      localStorage.setItem('auth_token', 'visual-test-token');
      localStorage.setItem('user_role', 'recycler');
    });
  });

  test('home / dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('dashboard.png', { fullPage: true });
  });

  test('recycler dashboard', async ({ page }) => {
    await page.goto('/dashboard/recycler');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('recycler-dashboard.png', { fullPage: true });
  });

  test('waste list page', async ({ page }) => {
    await page.goto('/wastes');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-list.png', { fullPage: true });
  });

  test('waste list page - mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/wastes');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-list-mobile.png', { fullPage: true });
  });

  test('incentives marketplace', async ({ page }) => {
    await page.goto('/incentives');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('incentives-marketplace.png', { fullPage: true });
  });

  test('collector dashboard', async ({ page }) => {
    await page.goto('/collect');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('collector-dashboard.png', { fullPage: true });
  });

  test('manufacturer dashboard', async ({ page }) => {
    await page.goto('/manufacturer');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('manufacturer-dashboard.png', { fullPage: true });
  });

  test('analytics page', async ({ page }) => {
    await page.goto('/analytics');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('analytics.png', { fullPage: true });
  });

  test('rewards page', async ({ page }) => {
    await page.goto('/rewards');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('rewards.png', { fullPage: true });
  });

  test('supply chain tracker', async ({ page }) => {
    await page.goto('/tracker');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('supply-chain-tracker.png', { fullPage: true });
  });

  test('waste map', async ({ page }) => {
    await page.goto('/map');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-map.png', { fullPage: true });
  });

  test('admin dashboard', async ({ page }) => {
    await page.addInitScript(() => localStorage.setItem('user_role', 'admin'));
    await page.goto('/admin');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('admin-dashboard.png', { fullPage: true });
  });

  test('verification page', async ({ page }) => {
    await page.goto('/verify');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('verification.png', { fullPage: true });
  });

  test('settings page', async ({ page }) => {
    await page.goto('/settings');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('settings.png', { fullPage: true });
  });

  test('community page', async ({ page }) => {
    await page.goto('/community');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('community.png', { fullPage: true });
  });

  test('recycling guide', async ({ page }) => {
    await page.goto('/recycling-guide');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('recycling-guide.png', { fullPage: true });
  });

  test('waste marketplace', async ({ page }) => {
    await page.goto('/marketplace');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-marketplace.png', { fullPage: true });
  });

  test('waste certification', async ({ page }) => {
    await page.goto('/certifications');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-certification.png', { fullPage: true });
  });

  test('predictive analytics', async ({ page }) => {
    await page.goto('/predictions');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('predictive-analytics.png', { fullPage: true });
  });

  test('route planner', async ({ page }) => {
    await page.goto('/route-planner');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('route-planner.png', { fullPage: true });
  });

  test('messaging page', async ({ page }) => {
    await page.goto('/messages');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('messaging.png', { fullPage: true });
  });

  test('waste comparison', async ({ page }) => {
    await page.goto('/compare');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('waste-comparison.png', { fullPage: true });
  });
});

// Dark mode variants
test.describe('Dark Mode', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.setItem('auth_token', 'visual-test-token');
      localStorage.setItem('theme', 'dark');
      document.documentElement.classList.add('dark');
    });
  });

  test('landing page - dark mode', async ({ page }) => {
    await page.goto('/');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('landing-dark.png', { fullPage: true });
  });

  test('dashboard - dark mode', async ({ page }) => {
    await page.goto('/dashboard');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('dashboard-dark.png', { fullPage: true });
  });

  test('settings page - dark mode', async ({ page }) => {
    await page.goto('/settings');
    await waitForStable(page);
    await expect(page).toHaveScreenshot('settings-dark.png', { fullPage: true });
  });
});
