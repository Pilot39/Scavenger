# Performance Testing Infrastructure

This directory contains the performance testing suite for Scavenger.

## Framework
We use [k6](https://k6.io/) for load, stress, and endurance testing.

## Directory Structure
- `scenarios/`: k6 test scripts for different testing types (load, stress, endurance).
- `lib/`: Shared functions and configurations.
- `baselines/`: Performance baseline data for regression detection.
- `reports/`: Generated test results and summary reports.

## Test Types
1. **Load Test** (`load.js`): Tests the system under expected normal load.
2. **Stress Test** (`stress.js`): Tests the system's limits by gradually increasing load until it breaks or reaches a high threshold.
3. **Endurance Test** (`endurance.js`): Tests system stability over an extended period.

## Running Tests

Requires a running environment (see the
[Developer Onboarding Guide](../docs/DEVELOPER_ONBOARDING.md#development-environment-setup))
plus [k6](https://k6.io) installed locally.

To run the full suite:
```bash
./performance/run-perf-tests.sh
```

To run a specific test:
```bash
k6 run performance/scenarios/load.js
```

## Baselines and Alerts
The suite includes a baseline comparison tool.
- To generate a new baseline: `GENERATE_BASELINE=true ./performance/run-perf-tests.sh`
- The `analyze-results.js` script automatically compares current results with the baseline and reports regressions.

## Metrics Tracked
- `http_req_duration`: End-to-end request time (p95, avg).
- `errors`: Rate of non-200/409 responses.
- `api_duration`: Custom trend for API specific timing.
