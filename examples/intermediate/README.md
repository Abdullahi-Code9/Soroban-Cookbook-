# Intermediate Examples

This category contains examples that demonstrate common, real-world design patterns and use cases for Soroban smart contracts. These examples often combine multiple basic concepts to solve practical problems.

## What's Inside?

- **Access Control**: Implement patterns like multi-sig, Role-Based Access Control (RBAC), and timelocks.
- **Cross-Contract Communication**: See how to build systems with factory, proxy, and registry patterns.
- **Token Interactions**: Learn how to create contracts that interact with or wrap standard tokens.
- **Advanced Data Structures**: Examples of iterable maps, queues, and other complex data structures.

## Planned Examples

- `02-role-based-access-control`: An RBAC implementation for managing permissions.
- `03-factory-pattern` / **Ajo Factory**: Deploy new contract instances from within a contract (see `ajo-factory/`).
- `04-token-wrapper`: A contract that wraps a standard token to add new functionality.
- `05-upgradable-proxy`: A basic proxy pattern for contract upgradability.

### Cross-Contract Patterns

- **Ajo Factory** (./ajo-factory/) — Deploy new contract instances from within a contract
- **Proxy Pattern** — Upgradeable contract pattern
- **Registry** — Central registry for contract discovery

### Access Control

- **Multi-Sig Patterns** — Threshold signatures and multi-party authorization
- **Role-Based Access** — Implement RBAC (Role-Based Access Control)
- **Timelock** — Time-delayed execution for security

### Data Structures

- **Iterables** — Implement iterable mappings
- **Queues** — FIFO queue implementation
- **Priority Queue** — Heap-based priority queue

## Prerequisites

Before diving into intermediate examples, ensure you understand:

- [Basic Examples](../basics/) — Core Soroban concepts
- Rust ownership and borrowing
- Basic blockchain concepts (addresses, signatures, transactions)

## Learning Path

1. Start with **Token Interactions** to understand asset handling
2. Explore **Cross-Contract Patterns** for complex architectures
3. Master **Access Control** for secure applications
4. Study **Data Structures** for efficient storage patterns

## Building and Testing

```bash
# Navigate to any example
cd examples/intermediate/[example-name]

# Run tests
cargo test

# Build the contract
cargo build --target wasm32-unknown-unknown --release
```

## Next Steps

Once comfortable with intermediate patterns:

- [Advanced Examples](../advanced/) — Complex systems and protocols
- [DeFi Examples](../defi/) — Decentralized finance applications
- [Governance Examples](../governance/) — DAO and voting systems
