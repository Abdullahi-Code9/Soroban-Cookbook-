# Basic Examples

Core Soroban fundamentals, one concept per example. Perfect for beginners starting their journey with Soroban smart contract development.

## 🎯 Learning Path

Follow this recommended sequence to build your understanding progressively:

<<<<<<< HEAD
```
Beginner
  │
  ├── 01-hello-world          ← Start here: contract structure basics
  ├── basic-event-emission    ← Emit your first event
  ├── 02-storage-patterns     ← Understand all three storage tiers
  ├── instance-storage        ← Deep dive: instance storage
  ├── persistent-storage      ← Deep dive: persistent storage
  ├── temporary_storage       ← Deep dive: temporary storage
  │
Intermediate
  │
  ├── 03-custom-errors        ← Structured error handling
  ├── 05-error-handling       ← Error propagation patterns
  ├── 03-authentication       ← Auth with require_auth()
  ├── 05-auth-context         ← Cross-contract auth context
  ├── 04-events               ← Structured, production-grade events
  ├── events                  ← Counter contract with events
  ├── 11-event-filtering      ← Design events for off-chain filtering
  │
Advanced
  │
  ├── 06-soroban-types        ← Full type system overview
  ├── 06-type-conversions     ← Safe type conversion patterns
  ├── 09-primitive-types      ← Integer types and overflow safety
  ├── 10-data-types           ← Comprehensive data type reference
  ├── 11-collection-types     ← Vec and Map in depth
  ├── 06-validation-patterns  ← Input, state, and auth validation
  ├── 07-enum-types           ← Enums, state machines, dispatch
  └── 08-custom-structs       ← Complex on-chain data structures
=======
Your first Soroban contract - learn the basic structure and deployment process.

**Concepts:** Contract structure, functions, symbol types

### [02-storage-patterns](./02-storage-patterns/)

Learn the fundamentals of storing and retrieving data in Soroban contracts across all three storage types.

**Detailed Storage Examples:**

- **[Instance Storage](./instance-storage/)**: Deep dive into the middle ground storage pattern.
- **[Persistent Storage](./persistent-storage/)**: Learn long-term data persistence.
- **[Temporary Storage](./temporary_storage/)**: Use case for transient, single-ledger data.

# Basic Examples

Core Soroban fundamentals, one concept per example. Perfect for beginners starting their journey with Soroban smart contracts.

## 🎯 Learning Path

Follow this recommended sequence to build your understanding progressively:

1. Start with [01-hello-world](./01-hello-world/) to understand contract structure and deployment.
2. Learn storage patterns with [02-storage-patterns](./02-storage-patterns/) and its focused variants.
3. Study authentication and authorization in [03-authentication](./03-authentication/).
4. Add observability via events in [04-events] and [basic-event-emission].
5. Learn execution context with [05-auth-context].
6. Handle errors using [03-custom-errors] and [05-error-handling].
7. Explore types and conversions in [06-soroban-types], [06-type-conversions], and [10-data-types].

## Getting Started

Each example includes:

- Complete source code with inline documentation
- Comprehensive unit tests
- A README with usage and build instructions

To run an example locally:

```bash
cd examples/basics/[example-name]
cargo test
cargo build --target wasm32-unknown-unknown --release
```

## Example Index (selected)

- [01-hello-world](./01-hello-world/) — Beginner: Contract skeleton, tests
- [02-storage-patterns](./02-storage-patterns/) — Beginner: persistent/instance/temporary storage
- [03-authentication](./03-authentication/) — Intermediate: `require_auth`, RBAC, time-locks
- [04-events](./04-events/) — Intermediate: structured event topics and payloads
- [05-auth-context](./05-auth-context/) — Intermediate: invoker vs current contract
- [06-soroban-types](./06-soroban-types/) — Advanced: full type reference
- [06-type-conversions](./06-type-conversions/) — Advanced: safe conversions
- [10-data-types](./10-data-types/) — Advanced: data type trade-offs

For a complete list see the repository `examples/basics/` directory.

## Planned Examples

- Iterative Mappings — efficient iteration patterns
- Batch Processing — multiple operations per call
- State Machine Patterns — structured state transitions

## Difficulty Key
- State Machine Patterns — structured state transitions
