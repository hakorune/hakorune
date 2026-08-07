---
Status: SSOT
Date: 2026-07-13
Decision: accepted — root symbol facade is guarded; public module namespace convergence is parked until Loop production cutover
Scope: MIR root facade export contract.
Related:
  - src/mir/mod.rs
  - src/mir/README.md
  - tools/checks/mir_root_facade_guard.sh
  - tools/checks/mir_root_facade_allowlist.txt
  - tools/checks/mir_root_import_hygiene_guard.sh
  - docs/development/current/main/phases/phase-291x/291x-523-semantic-metadata-root-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-537-mir-root-facade-contract-card.md
  - docs/development/current/main/phases/phase-296x/archive/296x-976-MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-PLAN-SURFACE-001.md
  - docs/development/current/main/investigations/mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
  - docs/reference/mir/metadata-facts-ssot.md
---

# MIR Root Facade Contract

## Purpose

`src/mir/mod.rs` is a small public facade for core MIR infrastructure and
pipeline orchestration. It is not the owner of semantic metadata vocabulary.

The root can make common compiler plumbing easy to import, but it must not
hide where route/fact/proof/policy decisions live.

## Allowed Root Exports

Keep these categories available at the MIR root:

- Core MIR data model:
  - `MirModule`, `MirFunction`, `MirInstruction`, `BasicBlock`
  - `ValueId`, `LocalId`, core type/value/op enums
  - `Effect`, `EffectMask`, call/callee definitions
- Compiler facade entry points:
  - builder/compiler/optimizer/printer/query/verifier surfaces
  - small cross-cutting query helpers used as MIR infrastructure
  - opaque module-lowering input bundles and typed compiler-entry errors that
    external runners must pass to `MirCompiler`
- Refresh orchestration entry points:
  - `refresh_function_*`
  - `refresh_module_*`
  - whole-pipeline semantic refresh helpers

Refresh functions may remain at the root because callers are asking the MIR
pipeline to recompute metadata. They are orchestration entry points, not
semantic vocabulary ownership.

## Forbidden Root Exports

Do not re-export these from the MIR root:

- Domain semantic metadata structs/enums.
- Route/seed/window record types.
- Proof, policy, state, candidate, plan, selection, layout, contract, or
  provenance vocabulary.
- Types used only by JSON emitters, tests, fixtures, shims, or one backend
  helper family.

Consumers that construct or inspect those values must import the owner module
directly, for example:

```rust
use crate::mir::string_corridor::StringCorridorFact;
use crate::mir::string_corridor_placement::StringCorridorCandidate;
use crate::mir::sum_placement_selection::SumPlacementSelection;
```

## New Export Gate

Before adding a new `pub use` in `src/mir/mod.rs`, answer all of these:

- Is this a core MIR model type or compiler facade?
- If it is metadata, is it only a refresh entry point?
- Would importing the owner module be clearer?
- Is the export temporary? If yes, is there a phase card with a removal
  condition?
- Does the export make `.inc`, JSON, tests, or a backend helper depend on the
  MIR root instead of the owner?

If any answer points to semantic ownership, do not add the root export.

## Import Hygiene

Avoid broad root imports in new code:

```rust
use crate::mir::*;
```

Tests may use `use super::*` inside a small owner module when it improves local
readability, but not as a path back to pruned MIR-root vocabulary. If a test
constructs semantic metadata, import that metadata from its owner module.

## Review Commands

Use these during cleanup cards:

```bash
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/mir_root_import_hygiene_guard.sh
rg -n "use crate::mir::\\*;" src
rg -n "crate::mir::(StringCorridor|SumPlacement|ThinEntry|PlacementEffect|StorageClass|ValueConsumer)" src/mir src/runner -g'*.rs'
rg -n "pub use .*\\{[^}]*(Fact|Plan|Route|Candidate|Selection|Layout|Contract|Policy|Proof|State|Kind|Surface|Demand|Carrier|Reason|Provenance)" src/mir/mod.rs
```

Expected shape:

- root wildcard imports stay absent
- owner modules expose their own vocabulary
- root keeps refresh entry points and core MIR surfaces
- `tools/checks/mir_root_facade_guard.sh` reports the allowlisted export count
- `tools/checks/mir_root_import_hygiene_guard.sh` reports `ok`, including no
  root-path loop-canonicalizer detection bridge

## Current State

Phase 291x pruned the major semantic metadata root exports through the
root-export cleanup cards. The remaining MIR root surface should be treated as
a facade, not a semantic metadata catalog.

The Language v1 review corrective at card 3503 classifies explicit Array/Weak
write operation enums and IDs plus `LocalSlotId` as core MIR model vocabulary.
`ContractRefreshBoundary` and the two refresh-and-validate functions are
orchestration facade entries. Carrier summaries and refreshed bundle types stay
owned by `mir::semantic_refresh` and are not re-exported from the MIR root.

B0-L2a added the compiler facade inputs
`ResolvedModuleLoweringInputV1`, the opaque
`VerifiedResolvedSourceUnitV1`, and `CanonicalLoweringErrorV1`. Their internal
Raw provenance and sealed semantic-owner vocabulary are not root exports.
The resolved bundle has no production constructor in B0-L2a, so this facade
change activates neither semantic resolution nor canonical Lower.

`MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0-I0-R0` removes
`LegacyModuleLoweringInputV1` from the public facade. Public `MirCompiler`
compile methods now accept whole-file `Program` only and enter the typed normal
lifecycle. `NormalProgramCompileRequestErrorV1` and its opaque rejected owner
are the public pre-Builder failure transport. The Raw syntax carrier remains
crate-internal to its explicit NarrowV1 lifecycle and cannot select the public
compiler route.

`MIRBUILDER-PUBLIC-ROOT-API0-RET0` closes the explicit T2 breaking retirement of
`MirBuilder::build_module(ASTNode)`. Repository callers and the dead generic
root wrappers are zero; unknown downstream callers remain a recorded
non-claim. Whole-file users migrate to `MirCompiler::compile*`; arbitrary-root
behavior has no replacement public API and remains only as
responsibility-local test evidence. No deprecated, feature-gated,
`doc(hidden)`, private, or test-only facade prolongs the second module
lifecycle. The `MirBuilder` type, module, root re-export, typed Program
lowering, and live node kernels are separate contracts and remain unchanged.

P0c-B1 adds the compiler-facade pair
`VerifiedResolvedCallableProgramV1` and
`ResolvedCallableModuleLoweringInputV1`. The former is the opaque owned exact
Program carrier; the latter is its borrowed compiler input. Catalogs, callable
headers, resolved function units, activation witnesses, and transaction plans
remain private to their owner modules. This pair exposes one explicit
multi-function compiler ingress without turning the MIR root into a callable
semantic-vocabulary catalog.

`NORMAL-DEFAULT-PUBLISHED-PIPELINE0-I0-R0` adds only the opaque normal
compiler-front request. Its named constructors seal the two admission modes
and exact selected caller identity without exporting that provenance
vocabulary. The candidate session, current-normal result contract, and
sunset-bound general-module compatibility owner remain private to
`mir::compiler::normal_default_pipeline`.

## Public module namespace audit (2026-08-07)

Local `rg`/parser census at `42ec69ab84` records:

```text
src/mir/mod.rs lines                 = 391
module declarations                 = 198
  pub mod                           = 131
  pub(crate) mod                    = 62
  private mod                       = 5
guarded root pub-use symbols        = 128

module-name token inventory (non-exclusive):
  plan                              = 50
  seed                              = 12
  pilot                             = 4
  raw                               = 5
  compat                            = 1
  legacy                            = 0
```

The existing facade guard verifies the 128 `pub use` symbols exactly. It does
not classify or ratchet `pub mod`, so the root public module namespace is a
separate, currently unguarded navigation surface.

This census does **not** prove that tests, seeds, pilots, or compat modules are
production semantic authorities. `#[cfg(test)]` placement and filename tokens
are inventory evidence only. In particular, raw/compat modules in this census
are crate-internal, and seed retirement remains row-specific under
`docs/reference/mir/metadata-facts-ssot.md`.

The conceptual labels `core / contracts / analysis / recipe / lowering /
passes / verification / compiler / compat` may guide navigation. They are not
permission for a nine-folder bulk move, and they must not collapse the existing
`Plan / Route / SeedRoute / Fact / Contract` distinctions in `src/mir/README.md`.

## Parked post-Loop task order

The current `RECIPE-COSEAL-I0-R0` row is closed, but its production Loop
cutover/retirement chain remains parked behind the typed function-finish
design stop. Do not move old Loop authorities that the future cutover will
delete.

### `MIR-TOPOLOGY-REBASE0-P0`

Change:
  Reproduce the local file/module/export census and emit one machine-readable
  root-module inventory. Delete or move nothing.

Contract:
  Classify declarations by visibility, owner family, active caller class, and
  lifecycle (`durable / temporary / bootstrap-compat / internal /
  retire-candidate`). A name is never deletion authority.

Done:
  Every declaration in the rebase snapshot has one owner and one lifecycle
  class; `pub use` and `pub mod` are reported separately. The report is
  evidence, not the lasting surface authority.

Stop:
  Any unresolved owner, generated declaration, or active production caller
  returns the row to classification. No folder design begins here.

### `MIR-ROOT-MODULE-SURFACE0-G0`

Change:
  Extend the existing root-facade guard family with a manifest-backed
  no-unreviewed-growth check for public module declarations.

Contract:
  Preserve module visibility and behavior. The guard owns surface drift only,
  never semantic acceptance or retirement policy. Its checked manifest is the
  sole root public-module surface authority after this row.

Done:
  Root symbol exports and public module declarations each have one stable,
  index-listed guard entry; current inventory is reproduced exactly.

Stop:
  Do not add another per-row shell guard or infer keep/retire from a suffix.

### `MIR-ROOT-INTERNAL-HELPER-PRIVATIZE0-R0`

Change:
  Select one classified semantic family and privateize only that family's
  module visibility behind its existing owner facade. Delete the old public
  edge atomically after repository caller-zero proof.

Contract:
  BoxShape-only; no accepted source/MIR shape, route selection, optimizer
  behavior, backend behavior, or physical clustering change. Broad `src/mir`
  movement is forbidden. Repository caller-zero does not prove unknown external
  caller-zero; every formerly public module needs an explicit API disposition.

Done:
  Focused tests and root guards are green. The implementation commit updates
  the target module README, this facade contract, and the exact
  `docs/reference/**` owner when its public MIR contract changes; otherwise it
  records `reference_delta = 0` with the reason.

Stop:
  More than one owner family, a required compatibility re-export without a
  retire condition, unknown downstream compatibility without an accepted
  breaking disposition, or an acceptance delta requires a new bounded decision.

### `MIR-TEMPORARY-SURFACE-NEXT0-P0`

Change:
  Select exactly one seed/pilot row whose reference retirement condition and
  production caller-zero evidence make it eligible for a dedicated decision.
  Change no source or visibility.

Contract:
  No filename-based batch deletion. Seed/pilot rows remain independent until
  their exact metadata/reference contracts prove otherwise. This P0 does not
  choose promote/quarantine/retire inside an implementation row.

Done:
  One row-specific D0/R1 is issued with its caller inventory, public API
  disposition, replacement owner, exact reference update, and atomic delete or
  keep boundary.

Stop:
  No eligible singleton, a live backend consumer, missing parity fixture, or
  ambiguous replacement leaves every row classified and unmodified.

### `MIR-ROOT-TOPOLOGY-CLOSEOUT0-G0`

Change:
  Re-run module/export/temporary-surface inventories after the bounded family
  series and close only the proven delta.

Contract:
  A smaller number is evidence, not the goal. One owner per semantic truth and
  one discoverable entry per durable family are the completion criteria.

Done:
  `src/mir/README.md`, owner READMEs, root facade contract, exact reference
  ledgers, and reusable guards agree; no old facade survives without an explicit
  compatibility owner and retire condition.

Stop:
  Crate split and broad rename remain separate decisions.
