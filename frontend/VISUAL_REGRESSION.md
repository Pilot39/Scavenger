# Visual Regression Testing

Playwright's built-in screenshot comparison is used — no external service required. Baselines are committed to the repo under `e2e/__snapshots__/`.

## Test coverage (32 tests)

| Group | Tests |
|---|---|
| Public pages | Landing (desktop/tablet/mobile), Login (desktop/mobile), 404 |
| Authenticated pages | Dashboard, Recycler, Waste List (desktop/mobile), Incentives, Collector, Manufacturer, Analytics, Rewards, Supply Chain, Map, Admin, Verification, Settings, Community, Recycling Guide, Marketplace, Certification, Predictive Analytics, Route Planner, Messaging, Comparison |
| Dark mode | Landing, Dashboard, Settings |

Each test runs across 3 browser projects: `visual-chromium`, `visual-firefox`, `visual-mobile` (Pixel 5).

## Running locally

```bash
# Run all visual tests (compare against baselines)
npm run visual

# Run only Chromium
npm run visual:chromium

# Open HTML diff report after a failure
npm run visual:report

# Update baselines after intentional UI changes
npm run visual:update

# Update a single project
./scripts/update-visual-baselines.sh chromium   # or firefox / mobile
```

## Baseline workflow

### First-time setup
Baselines don't exist yet — generate them:
```bash
npm run visual:update
git add e2e/__snapshots__
git commit -m "chore: add visual regression baselines"
```

### After intentional UI changes
1. Make your UI changes.
2. Run `npm run visual` — tests will fail showing the diff.
3. Review diffs in `playwright-report/` (`npm run visual:report`).
4. If the new visuals are correct, update baselines:
   ```bash
   npm run visual:update
   git add e2e/__snapshots__
   git commit -m "chore: update visual baselines for <feature>"
   ```
5. Push — CI will now pass.

### Approving baselines via CI (no local Playwright install)
1. Go to **Actions → Visual Regression** in GitHub.
2. Click **Run workflow**, set `update_baselines` to `true`.
3. The workflow regenerates snapshots and commits them back to the branch automatically.

## CI integration

The `visual-regression.yml` workflow runs on every PR that touches `frontend/src/**`. It:
- Runs all 3 browser projects in parallel.
- Uploads diff reports as artifacts (retained 30 days) when tests fail.
- Blocks the PR via the `Visual Regression Gate` job if any diff is detected.

### Reviewing a failure
1. Open the failed PR → **Actions** tab → click the failed run.
2. Download the `visual-diff-<project>` artifact.
3. Open `playwright-report/index.html` to see side-by-side diffs.
4. If the change is intentional, follow the baseline update steps above.

## Configuration

Key settings in `playwright.config.ts`:

| Setting | Value | Reason |
|---|---|---|
| `maxDiffPixelRatio` | `0.002` (0.2%) | Tolerates minor anti-aliasing differences across OS/GPU |
| `animations` | `disabled` | Prevents flaky diffs from in-flight CSS transitions |
| Snapshot path | `e2e/__snapshots__/{projectName}/` | Keeps per-browser baselines separate |

To tighten or loosen the threshold, edit `expect.toHaveScreenshot.maxDiffPixelRatio` in `playwright.config.ts`.
