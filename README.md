# zkprim

A zero-knowledge primitive library built in Rust. `zkprim` implements foundational cryptographic
components used in ZK proving systems — field arithmetic, cryptographic hashing, and Merkle tree
construction with proof generation and verification.

This library is not a production prover. It is a focused systems programming project designed to
demonstrate fluency in Rust and cryptographic primitives at the protocol layer.

---
 
## Goals
 
### 1. Rust Systems Fluency
 
Develop deep, practical Rust fluency across the concepts that matter in performance-critical
cryptographic infrastructure:
 
- Ownership, borrowing, and lifetime semantics applied to field element operations
- Trait-based abstractions (`FieldElement`) that generalize over field representations
- Smart pointers (`Box`, `Arc`) for recursive data structures and thread-safe shared state
- Data parallelism via `rayon` for batch hashing and Merkle layer computation
- `unsafe` Rust for performance-critical Montgomery multiplication, safely encapsulated
- Asynchronous Rust via `tokio` for a proof-submission API
- Declarative macros to reduce field implementation boilerplate
 
### 2. ZK Primitive Implementation
 
Implement the core building blocks present in every ZK proving system:
 
- **Finite field arithmetic** over a prime field `Fp`, with correct modular semantics for all
  operations including negation (additive inverse) and inversion (multiplicative inverse)
- **Montgomery multiplication** for performant modular multiplication without expensive division
- **Merkle trees** with SHA-256 leaf hashing, sibling-path proof generation, and root-based
  proof verification
- **Generic field trait** that mirrors the abstraction model used in production libraries like
  `ark-ff`, enabling future extension to larger primes and different field representations

---

## Module Structure
 
```
zkprim/
├── src/
│   ├── lib.rs          # Public API, re-exports
│   ├── field/
│   │   ├── mod.rs      # FieldElement trait, Fp<M> implementation
│   │   └── montgomery.rs  # Montgomery multiplication
│   ├── hash/
│   │   └── mod.rs      # SHA-256 wrapper, Poseidon stub
│   └── tree/
│       └── mod.rs      # MerkleNode, proof generation, verification
├── tests/
│   └── integration_test.rs
├── benches/
│   └── field_bench.rs
└── README.md
```
 
---
 
## Dependencies
 
| Crate | Purpose |
|---|---|
| `sha2` | SHA-256 leaf hashing |
| `rand` | Random field element generation |
| `rayon` | Data parallelism for batch operations |
| `tokio` | Async runtime for proof submission API |
| `reqwest` | HTTP client for proof submission |
| `zeroize` | Reference implementation for `SecretBytes` |
| `criterion` | Benchmarking field and tree operations |
 
---
 
## Benchmarks
 
Benchmarks cover:
 
- Sequential vs parallel Merkle tree construction across leaf counts (256, 1024, 4096)
- Montgomery vs naive modular multiplication
- Batch field operations: sequential vs `rayon` parallel
 
Run with:
 
```bash
cargo bench
```

---
 
## Reference Codebases
 
The design of this library is informed by reading the following production ZK codebases:
 
- [`ark-ff`](https://github.com/arkworks-rs/algebra/tree/master/ff) — field trait abstraction and Montgomery backend
- [`plonky3`](https://github.com/Plonky3/Plonky3) — field and hash abstractions in a modern proving system
- [`risc0`](https://github.com/risc0/risc0/tree/main/risc0/zkp/src) — prover thread coordination and parallelism patterns
- [`blst`](https://github.com/supranational/blst/tree/master/bindings/rust) — FFI bindings pattern for performance-critical C libraries
 
---
