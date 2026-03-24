# Instance Storage

This example demonstrates `env.storage().instance()` — the middle ground between persistent and temporary storage in Soroban.

## What is Instance Storage?

Instance storage is scoped to the contract instance (the deployed address). All keys in instance storage share a single TTL (time-to-live) that covers the entire instance. This differs from persistent storage where each key has an independent TTL.

## Comparison with Other Storage Types

| Property | Persistent | Instance | Temporary |
|---|---:|---:|---:|
| Survives upgrade | ✅ Yes | ❌ No | ❌ No |
| TTL management | Per-key | Per-instance | Per-key |
| Relative cost | Highest | Medium | Lowest |
| Best for | Long-term critical data | Instance-wide config/counters | Single-invocation data |

## When to Use Instance Storage

- Use it for contract-wide configuration and counters that should expire with the instance.
- Avoid it for data that must survive contract upgrades (use persistent) or for ephemeral per-call data (use temporary).

## Key Patterns

Use a typed `#[contracttype]` enum for keys to avoid collisions and clarify the storage surface:

```rust
#[contracttype]
#[derive(Clone)]
pub enum InstanceKey {
    TxCounter,
    Config(Symbol),
}
```

## Common Operations

```rust
// Write a value and extend the instance TTL
env.storage().instance().set(&InstanceKey::TxCounter, &count);
env.storage().instance().extend_ttl(1_000, 10_000);

// Read with default
let count: u64 = env.storage().instance().get(&InstanceKey::TxCounter).unwrap_or(0);

// Check existence
let exists: bool = env.storage().instance().has(&InstanceKey::TxCounter);

// Remove
env.storage().instance().remove(&InstanceKey::TxCounter);
```

## Use Cases in this Example

1. Transaction counter — per-instance state that changes frequently and can expire with the instance.
2. Runtime configuration — operator-tunable parameters shared across calls but not required to survive upgrades.

## Build & Test

From this directory:

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
```

From repository root:

```bash
cargo test -p instance-storage
cargo build -p instance-storage --target wasm32-unknown-unknown --release
```

## Related Examples

- [02-storage-patterns](../02-storage-patterns/) — Compare all three storage types side-by-side
- [persistent-storage](../persistent-storage/) — Per-key TTL for user-specific data
- [temporary_storage](../temporary_storage/) — Ephemeral, single-ledger data

### TTL Management

Instance storage shares a single TTL for all entries. Calling `extend_ttl` on the instance refreshes the lifetime of _all_ instance keys at once.

```rust
const TTL_THRESHOLD: u32 = 1_000;
const TTL_EXTEND_TO: u32 = 10_000;

env.storage().instance().extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
```

## Use Cases in this Example

1.  **Transaction Counter**: A classic candidate for instance storage. It's per-instance state, changes often (benefiting from lower costs), and doesn't strictly need to survive upgrades.
2.  **Runtime Configuration**: Operator-tunable parameters (like fee rates or limits) that are shared across all invocations but can be reset if the contract is upgraded.

## Running the Example

### 1. Build the Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 2. Run Tests

```bash
cargo test
```

## Lessons Learned

- Instance storage is ideal for shared "instance-global" state.
- Single TTL management significantly simplifies housekeeping compared to persistent storage.
- Always call `extend_ttl` during both reads and writes to ensure the instance doesn't expire while in use.
>>>>>>> 0f17a2b (instance storage)
