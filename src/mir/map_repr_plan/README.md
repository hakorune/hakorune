# map_repr_plan

Map representation planning for MIR metadata.

This directory owns proof-bearing Map representation metadata. It does not own
backend lowering, product `MapBox` storage policy, hasher policy, or MIRBuilder
object management.

## Module Boundaries

- `plans.rs`
  - facade for public plan vocabulary
- `plans/kind.rs`
  - `MapReprKind` tags only
- `plans/map_repr.rs`
  - `MapReprPlan` route-backed representation rows
- `plans/local_storage.rs`
  - LocalI64Map storage / direct-storage / entry-value tracking rows
  - no route scanning
  - no metadata refresh orchestration
- `candidates.rs`
  - local candidate detection from existing `GenericMethodRoute` rows
  - operand shape repair for current Map set/get routes
  - no public metadata writes
- `fastpath.rs`
  - positive `LocalFastPathFact` production only
  - fallback evidence is not a fact
- `refresh.rs`
  - function/module refresh orchestration
  - writes `FunctionMetadata` fields from the local builders
- `tests.rs`
  - facade for module tests
- `tests/fixtures.rs`
  - shared MIR fixture builders
- `tests/refresh_cases.rs`
  - public refresh contract cases

## Stop Line

Do not add backend-specific emission, helper-name inference, benchmark-name
branches, product `MapBox` storage changes, or hasher swaps here. Those require
separate design rows and backend-owned modules.
