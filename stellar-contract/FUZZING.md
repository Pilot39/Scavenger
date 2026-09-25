# Contract Fuzzing Suite

## Approach

Since Soroban contracts target `no_std`/WASM, we use proptest-based fuzzing rather than libFuzzer/cargo-fuzz. Proptest generates randomized inputs and shrinks failing cases to minimal reproductions.

## Running

```bash
# Run comprehensive fuzzing suite
cargo test --package stellar-scavngr-contract --test fuzz_comprehensive -- --nocapture

# Run with extended cases (default: 32-128 per test)
PROPTEST_CASES=5000 cargo test --package stellar-scavngr-contract --test fuzz_comprehensive

# Run all fuzz tests (comprehensive + existing)
cargo test --package stellar-scavngr-contract fuzz_ -- --nocapture

# Run regression tests
cargo test --package stellar-scavngr-contract --test fuzz_regression

# Run full suite with script
./scripts/run_fuzz_tests.sh
```

## Fuzzing Targets

| File | Category | What it covers |
|------|----------|---------------|
| `fuzz_comprehensive.rs` | Boundary values | Extreme weights (0, MAX), invalid waste types, invalid roles, coordinate boundaries |
| `fuzz_comprehensive.rs` | State transitions | Submit-then-transfer, double transfer, nonexistent waste ops, unregistered user ops |
| `fuzz_comprehensive.rs` | Multi-participant | Chain transfers (R->C->M), self-transfer rejection, independent submissions |
| `fuzz_comprehensive.rs` | Incentives | Extreme reward/budget values, non-manufacturer rejection |
| `fuzz_regression.rs` | Regression | Deterministic edge case tests for discovered failures |
| `fuzz_contract_operations.rs` | Existing | Basic registration, submission, transfer fuzzing |
| `fuzz_waste_submission.rs` | Existing | Waste submission with varied inputs |
| `fuzz_waste_transfer.rs` | Existing | Transfer operation fuzzing |

## Adding New Fuzz Targets

1. Add a new `proptest! { }` block in `fuzz_comprehensive.rs`
2. Use `std::panic::catch_unwind` to catch expected panics
3. Use `prop_assert!` for invariant checks
4. Choose strategies that target boundaries, not just random ranges

## Creating Regression Tests

When proptest finds a failure, it saves the minimal case to `proptest-regressions/`. Convert these to deterministic tests in `fuzz_regression.rs`:

```rust
#[test]
#[should_panic]
fn regression_description_of_issue() {
    // Reproduce the exact inputs from the proptest failure
}
```

## Corpus Management

Proptest persists failure cases in `proptest-regressions/` directories (auto-created next to test files). These are replayed on every run to prevent regressions. Commit these files to version control.
