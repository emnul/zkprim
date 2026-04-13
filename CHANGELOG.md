# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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


