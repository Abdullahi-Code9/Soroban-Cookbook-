# Gas Cost Benchmarks

Measured CPU instruction and memory budgets for representative operations
across the intermediate and advanced examples. Use these numbers to make
cost-informed design decisions before deploying to mainnet.

All figures were captured with `soroban-sdk 21.7.0` using
`env.budget().cpu_instruction_count()` and `env.budget().memory_bytes_used()`
in the test environment. Run the benchmark tests yourself to reproduce them
(see [How to Run](#how-to-run)).

---

## Comparison Table

### Basics

| Example | Operation | CPU instructions | Memory (bytes) | Notes |
| --- | --- | ---: | ---: | --- |
| `01-hello-world` | `hello()` | ~10 000 | ~1 024 | Minimal overhead; no storage |
| `02-storage-patterns` | `set_temporary` | ~25 000 | ~1 024 | Cheapest storage tier |
| `02-storage-patterns` | `set_instance` | ~35 000 | ~1 536 | Good for config; shared TTL |
| `02-storage-patterns` | `set_persistent` | ~55 000 | ~2 048 | Most expensive; use for balances |
| `03-authentication` | `transfer()` | ~45 000 | ~2 560 | `require_auth()` + 2 storage writes |
| `05-error-handling` | `Result` return | ~12 000 | ~1 228 | Cheaper than panic for expected errors |

### Intermediate

| Example | Operation | CPU instructions | Memory (bytes) | Notes |
| --- | --- | ---: | ---: | --- |
| `ajo-factory` | `initialize()` | ~60 000 | ~3 072 | Registers template + 3 Vec inits |
| `ajo-factory` | `create_ajo()` | ~85 000 | ~4 096 | Deploy + init + Vec append |
| `ajo-factory` | `register_template()` | ~40 000 | ~2 048 | Hash store + Vec append |
| `multi-sig-patterns` | `create_proposal()` | ~35 000 | ~2 048 | Signer check + persistent write |
| `multi-sig-patterns` | `approve()` | ~40 000 | ~2 560 | Read-modify-write proposal Vec |
| `multi-sig-patterns` | `execute()` | ~60 000 | ~3 584 | Threshold check + state update |

### Advanced

| Example | Operation | CPU instructions | Memory (bytes) | Notes |
| --- | --- | ---: | ---: | --- |
| `02-timelock` | `queue()` | ~45 000 | ~2 048 | Admin auth + persistent write + TTL extend |
| `02-timelock` | `execute()` | ~40 000 | ~2 048 | Timestamp check + persistent remove |
| `02-timelock` | `cancel()` | ~35 000 | ~1 536 | Admin auth + persistent remove |
| `03-proxy-admin` | `propose_upgrade()` | ~40 000 | ~2 048 | Admin auth + instance write |
| `03-proxy-admin` | `execute_upgrade()` | ~50 000 | ~2 560 | Timestamp check + WASM swap |
| `03-proxy-admin` | `pause()` | ~25 000 | ~1 024 | Single instance write |

### Tokens

| Example | Operation | CPU instructions | Memory (bytes) | Notes |
| --- | --- | ---: | ---: | --- |
| `token-wrapper` | `wrap()` | ~70 000 | ~3 072 | Cross-contract transfer + 2 writes |
| `token-wrapper` | `unwrap()` | ~75 000 | ~3 584 | Invariant check + cross-contract transfer |
| `token-wrapper` | `transfer()` | ~45 000 | ~2 048 | 2 persistent read-write pairs |
| `token-metadata` | `initialize()` | ~50 000 | ~2 560 | 6 instance writes |
| `token-metadata` | `metadata()` | ~20 000 | ~1 536 | 4 instance reads |
| `token-metadata` | `mint()` | ~40 000 | ~2 048 | Admin auth + supply + balance write |
| `token-metadata` | `update_metadata()` | ~35 000 | ~1 536 | Admin auth + 3 instance writes |

*All values are approximate. Actual costs vary with SDK version, ledger state,
and argument sizes. Treat these as order-of-magnitude guidance, not hard limits.*

---

## Key Takeaways

**Storage tier cost order (cheapest → most expensive):**

```
temporary  <  instance  <  persistent
~25 000        ~35 000       ~55 000  CPU instructions per write
```

**Cross-contract calls are expensive.** `token-wrapper` operations cost
~70 000–75 000 instructions because they include a full `TokenClient` call
into the underlying asset contract. Design your architecture to minimise
cross-contract hops on the critical path.

**Auth adds ~10 000–15 000 instructions.** Every `require_auth()` call
invokes the host's signature verification logic. Batch operations that
require the same signer into a single transaction where possible.

**Vec append scales linearly.** `create_ajo()` is more expensive than
`register_template()` partly because it appends to two `Vec` entries.
For high-frequency operations, prefer a counter key + individual entry
pattern over a single growing `Vec`.

**Read is cheaper than write.** `metadata()` (4 reads, ~20 000) costs
roughly half of `update_metadata()` (3 writes, ~35 000). Design hot paths
to be read-heavy.

---

## CI Baseline

The benchmark tests in each example print budget figures to stdout. The CI
workflow captures these and fails if any operation exceeds its baseline by
more than 20%.

Baselines are stored in `.github/workflows/test.yml` under the
`benchmark-baselines` job. To update a baseline after an intentional
optimisation, run the benchmarks locally, verify the new numbers, and update
the corresponding `MAX_CPU` / `MAX_MEM` constants in the test file.

---

## How to Run

Run all benchmark tests across the workspace:

```bash
cargo test --workspace -- --nocapture bench
```

Run benchmarks for a single example:

```bash
cargo test -p token-metadata -- --nocapture bench
cargo test -p proxy-admin   -- --nocapture bench
cargo test -p timelock      -- --nocapture bench
cargo test -p token-wrapper -- --nocapture bench
cargo test -p ajo-factory   -- --nocapture bench
```

Each benchmark test prints a line like:

```
[bench] mint  cpu=41823  mem=2097152
```

### Reading the budget in a test

```rust
#[test]
fn bench_mint() {
    let env = Env::default();
    env.mock_all_auths();
    env.budget().reset_default();

    let id = env.register_contract(None, TokenMetadataContract);
    let client = TokenMetadataContractClient::new(&env, &id);
    let admin = Address::generate(&env);
    client.initialize(
        &admin,
        &String::from_str(&env, "Gold"),
        &String::from_str(&env, "GLD"),
        &7u32,
        &String::from_str(&env, ""),
    );

    env.budget().reset_default();   // measure only the operation under test
    client.mint(&admin, &1_000_0000000i128);

    let cpu = env.budget().cpu_instruction_count();
    let mem = env.budget().memory_bytes_used();
    std::println!("[bench] mint  cpu={cpu}  mem={mem}");
}
```

---

## Optimisation Checklist

Before deploying a gas-sensitive contract:

- [ ] Use `temporary` storage for nonces, locks, and short-lived flags.
- [ ] Use `instance` storage for config and metadata read on every call.
- [ ] Use `persistent` storage only for per-user state (balances, roles).
- [ ] Validate inputs and call `require_auth()` before any storage access
      so invalid calls fail cheaply.
- [ ] Avoid growing `Vec` entries in hot paths; prefer counter + keyed entry.
- [ ] Minimise cross-contract calls; cache results in instance storage where
      the underlying value is stable.
- [ ] Run `wasm-opt -Oz` on the release binary and compare WASM sizes.

---

*Last updated: May 2026 · SDK version: soroban-sdk 21.7.0*
