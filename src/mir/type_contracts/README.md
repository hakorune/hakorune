# MIR Type Contracts

This directory owns the Language v1 annotation guarantee vocabulary. It does
not parse source syntax, select storage layouts, or lower backend operations.

## Boundaries

- `guarantee_matrix.rs` is the closed annotation-site matrix.
- `proof.rs` defines verifier-backed proof records for activated slices.
- `parameter_entry.rs` owns typed final-callee entry carriers and drift checks.
- `return_exit.rs` owns typed final-outcome carriers and drift checks.
- `record_value.rs` owns record schema projection and construction/update
  carrier rebuild/validation. Builder, VM, JSON, and backend gates consume this
  owner instead of deriving contracts from record layout.
- `static_table.rs` owns readonly U16 source-spec, derived-plan, and load-site
  conformance. Parser, verifier, VM, JSON, and backends do not synthesize a
  contract from `StaticDataPlan`.
- Site-specific timing stays with its boundary owner. Shared exact-numeric
  value/range checking is subordinate and cannot infer a contract.
- Exact-numeric field writes remain owned by
  `mir/exact_numeric_field_contracts.rs`.
- Runtime and backend consumers may validate or enforce an activated contract;
  they do not infer contracts from source spelling or representation metadata.

Proofs in the first slice are rebuilt during semantic refresh. They are not a
cross-refresh cache and therefore do not carry invented CFG/SSA epochs.
