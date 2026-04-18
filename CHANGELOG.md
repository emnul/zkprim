# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `MerkleTree` struct backed by a 1-indexed, power-of-two-sized `Vec<Hash>`, representing a fixed-depth binary Merkle tree
  using SHA3-256 as the hash function. The tree layout places the root at index 1 with children at `2i` and `2i+1`.

- `MerkleProof` newtype wrapping `Vec<(bool, Hash)>`, where the `bool` indicates whether the current node is the left
  child, determining sibling ordering during verification.

- `MerkleTree::new(depth, initial_leaf_value)` with O(depth) initialization that exploits the uniform initial leaf value
  to compute each layer's hash exactly once rather than rehashing every node.

- `MerkleTree::set(leaf_index, value)` that writes a leaf and propagates updated parent hashes up to the root in O(depth)
  time via `set_recursively`.

- `MerkleTree::proof(leaf_index)` that collects the sibling hash at each level along the path from the target leaf to the
  root, returning a `MerkleProof` suitable for non-interactive verification.

- `MerkleTree::verify(proof, leaf_value)` that recomputes the root hash by folding the proof's sibling hashes over the
  leaf value, enabling verification without access to the full tree.

- `MerkleTree::num_leaves()` returning the number of leaves as `nodes.len() / 2`.

- Fp struct, representing a finite field over the [Mersenne prime](https://en.wikipedia.org/wiki/Mersenne_prime) 2^61-1. A
Mersenne prime was chosen for its [efficient modular reduction properties](https://hal.sorbonne-universite.fr/hal-02883333/file/BaDueprintversion.pdf)
to be explored in the future.

- `Modulus` trait with a single associated constant `VALUE: u64`, enabling compile-time
  parameterization of field arithmetic without runtime cost. Concrete moduli are defined as
  empty structs implementing this trait.
 
- `MersennePrime` modulus struct implementing `Modulus` with `VALUE = 2^61 - 1`, replacing the
  previously hardcoded constant on `Fp`.
 
- `Fp<M: Modulus>` generic field element parameterized over a modulus type via `PhantomData<M>`.
  `Fp` defaults to `Fp<MersennePrime>`, preserving backward compatibility with existing tests.
  Arithmetic between `Fp` instances over different moduli is rejected at compile time.
 
- `FieldElement` trait defining the interface for prime field arithmetic: `zero()`, `one()`,
  `inv()`, and `pow()`. The trait is bounded by `Sized + Clone + Mul<Output = Self>`, enforcing
  that any implementor supports multiplication before the trait can be implemented.
 
- `FieldElement` implementation for `Fp<M: Modulus>`, with `inv()` derived from Fermat's little
  theorem as `a^(p-2) mod p` and `pow()` implemented via square-and-multiply in O(log exp)
  multiplications.
 
- `batch_multiply<F: FieldElement>` generic function that applies a scalar multiplication across
  a slice of field elements, returning a new `Vec<F>`. Serves as the foundation for the parallel
  batch operations to be introduced in Week 2 Day 4.
 
- Manual implementations of `Clone`, `Copy`, and `PartialEq` for `Fp<M: Modulus>`, replacing
  derived implementations. Derived implementations incorrectly require `M: Clone` and `M: PartialEq`
  even though `M` is phantom and never stored. Manual implementations constrain only on `M: Modulus`
  as intended.
 
### Changed
 
- `Fp::PRIME_MODULUS` re-exposed as an associated constant on `Fp<M>` delegating to `M::VALUE`,
  preserving the original access pattern after modulus parameterization.
 
- `Mul` implementation updated to cast operands to `u128` before multiplication to prevent
  overflow prior to modular reduction, then truncates back to `u64`.


### Removed


