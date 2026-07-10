# MIR Type Contracts

This directory owns the Language v1 annotation guarantee vocabulary. It does
not parse source syntax, select storage layouts, or lower backend operations.

## Boundaries

- `guarantee_matrix.rs` is the closed annotation-site matrix.
- `proof.rs` defines verifier-backed proof records for activated slices.
- Site-specific classifiers remain with their existing semantic owner. The
  first owner is `mir/exact_numeric_field_contracts.rs`.
- Runtime and backend consumers may validate or enforce an activated contract;
  they do not infer contracts from source spelling or representation metadata.

Proofs in the first slice are rebuilt during semantic refresh. They are not a
cross-refresh cache and therefore do not carry invented CFG/SSA epochs.
