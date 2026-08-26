# Load Testing Guide

Comprehensive load testing suite for the Scavenger platform using k6.

## Overview

This guide covers load testing with 8+ scenarios:
1. Steady State (100 users)
2. Spike Test (sudden 1000 users)
3. Stress Test (gradual increase to 10000)
4. Endurance Test (sustained load)
5. Ramp Test (gradual increase)
6. Wave Test (multiple waves)
7. Peak Hour Simulation
8. Bottleneck Detection

## Prerequisites

- k6 installed: https://k6.io/docs/getting-started/installation/
- Node.js 18+
- Running Scavenger API

## Installation

```bash
# Install k6
# macOS
brew install k6

# Linux
sudo apt-get install k6

# Windows
choco install k6
```

## Running Load Tests

### Comprehensive Load Test (All Scenarios)

```bash
k6 run performance/k6-load-test-comprehensive.js
```

### Individual Scenarios

```bash
# Steady State Test (100 users)
k6 run --stage 2m:100 --stage 5m:100 --stage 2m:0 performance/load-test-scenarios.js

# Spike Test
k6 run --stage 1m:100 --stage 30s:1000 --stage 2m:1000 --stage 30s:100 --stage 1m:0 performance/load-test-scenarios.js

# Stress Test
k6 run --stage 2m:1000 --stage 2m:5000 --stage 2m:10000 --stage 5m:10000 --stage 2m:0 performance/load-test-scenarios.js

# Endurance Test (30 minutes)
k6 run --stage 5m:500 --stage 30m:500 --stage 5m:0 performance/load-test-scenarios.js

# Ramp Test
k6 run --stage 10m:5000 --stage 5m:5000 --stage 5m:0 performance/load-test-scenarios.js

# Wave Test
k6 run performance/load-test-scenarios.js

# Peak Hour Simulation
k6 run performance/load-test-scenarios.js

# Bottleneck Detection
k6 run performance/load-test-scenarios.js
```

### With Custom Configuration

```bash
# Set API URL
BASE_URL=http://api.example.com k6 run performance/k6-load-test-comprehensive.js

# Set VU count
k6 run -u 1000 -d 5m performance/k6-load-test-comprehensive.js

# Output to file
k6 run --out json=reports/load-test-results.json performance/k6-load-test-comprehensive.js
```

## Test Scenarios

### 1. Steady State (100 users)
- **Duration**: 9 minutes
- **Users**: 100 concurrent
- **Purpose**: Baseline performance under normal load
- **Thresholds**: p95 < 500ms, error rate < 5%

### 2. Spike Test
- **Duration**: 6.5 minutes
- **Peak Users**: 1000 concurrent
- **Purpose**: Test system response to sudden traffic spike
- **Thresholds**: p95 < 1000ms, error rate < 10%

### 3. Stress Test
- **Duration**: 13 minutes
- **Peak Users**: 10000 concurrent
- **Purpose**: Find breaking point of system
- **Thresholds**: p95 < 2000ms, error rate < 20%

### 4. Endurance Test
- **Duration**: 40 minutes
- **Users**: 500 concurrent
- **Purpose**: Detect memory leaks and degradation
- **Thresholds**: p95 < 500ms, error rate < 5%

### 5. Ramp Test
- **Duration**: 20 minutes
- **Peak Users**: 5000 concurrent
- **Purpose**: Gradual load increase to identify breaking point
- **Thresholds**: p95 < 1000ms, error rate < 10%

### 6. Wave Test
- **Duration**: 19 minutes
- **Peak Users**: 2000 concurrent
- **Purpose**: Multiple load waves to test recovery
- **Thresholds**: p95 < 1000ms, error rate < 10%

### 7. Peak Hour Simulation
- **Duration**: 45 minutes
- **Peak Users**: 2000 concurrent
- **Purpose**: Simulate realistic peak hour traffic
- **Thresholds**: p95 < 1000ms, error rate < 10%

### 8. Bottleneck Detection
- **Duration**: 15 minutes
- **Peak Users**: 10000 concurrent
- **Purpose**: Identify system bottlenecks
- **Thresholds**: p95 < 2000ms, error rate < 20%

## Metrics Collected

### Response Time Metrics
- `http_req_duration`: Total request duration
- `http_req_duration{staticAsset:yes}`: Static asset duration
- `http_req_duration{staticAsset:no}`: Dynamic content duration

### Error Metrics
- `http_req_failed`: Failed requests
- `errors`: Custom error rate
- `http_req_duration{expected_response:true}`: Expected responses

### Throughput Metrics
- `http_reqs`: Total requests
- `http_reqs_rate`: Requests per second
- `success_count`: Successful requests

### Connection Metrics
- `active_connections`: Active concurrent connections
- `http_conn_reused`: Reused connections

## Analyzing Results

### JSON Report

```bash
k6 run --out json=reports/results.json performance/k6-load-test-comprehensive.js
```

### HTML Report

```bash
# Install reporter
npm install -g @grafana/k6-reporter

# Generate report
k6 run --out json=reports/results.json performance/k6-load-test-comprehensive.js
k6-reporter reports/results.json
```

### Key Metrics to Review

1. **Response Time (p95, p99)**
   - Target: < 500ms for normal load
   - Target: < 1000ms for peak load

2. **Error Rate**
   - Target: < 5% for normal load
   - Target: < 10% for peak load

3. **Throughput**
   - Requests per second
   - Requests per user

4. **Resource Usage**
   - CPU utilization
   - Memory usage
   - Network bandwidth

## Bottleneck Identification

### Common Bottlenecks

1. **Database Queries**
   - Check query performance
   - Add indexes if needed
   - Consider caching

2. **API Endpoints**
   - Profile slow endpoints
   - Optimize business logic
   - Add rate limiting

3. **Network**
   - Check bandwidth usage
   - Optimize payload size
   - Use compression

4. **Server Resources**
   - Monitor CPU usage
   - Check memory leaks
   - Scale horizontally

## Performance Optimization

### Before Load Testing
- [ ] Profile application
- [ ] Identify slow queries
- [ ] Review code for inefficiencies
- [ ] Set up monitoring

### During Load Testing
- [ ] Monitor system resources
- [ ] Check error logs
- [ ] Track response times
- [ ] Identify bottlenecks

### After Load Testing
- [ ] Analyze results
- [ ] Identify issues
- [ ] Implement fixes
- [ ] Re-test improvements

## CI/CD Integration

### GitHub Actions

```yaml
name: Load Testing
on: [push, pull_request]

jobs:
  load-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: grafana/k6-action@v0.3.0
        with:
          filename: performance/k6-load-test-comprehensive.js
          cloud: true
```

## Troubleshooting

### High Error Rate
- Check API logs
- Verify database connectivity
- Check rate limiting
- Review error messages

### High Response Times
- Profile endpoints
- Check database queries
- Monitor server resources
- Review network latency

### Memory Issues
- Check for memory leaks
- Monitor garbage collection
- Review connection pooling
- Check cache size

## References

- [k6 Documentation](https://k6.io/docs/)
- [k6 Best Practices](https://k6.io/docs/testing-guides/load-testing/)
- [Performance Testing Guide](https://owasp.org/www-community/attacks/Performance_testing)
