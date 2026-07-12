# MirBuilder Hard Frontier Inventory and Taskization

Status: Parked design packet; not the active lane.
Date: 2026-07-12
Active lane remains: 3457 MapStoreI64 authority inventory.
Evidence method: focused source/fixture/consumer inventory; no semantic code
change is included in this packet.

## Result

The apparent backlog is three hard frontiers, not one task per Rust module:

```text
I64Value provenance authority
shared Hako AST/body snapshot capability
planner authority after Facts
```

The existing bool/string/loop Hako pilots are read-only token-snapshot
facades. They do not independently solve full AST traversal and must not be
counted as Source Selfhost or hard MirBuilder authority migration.

## Inventory

### HFI-A — MapStoreI64 dynamic integer Fact

```text
source candidate = MirType::Integer metadata branch
current status = pending
reason = type metadata does not identify source/fresh provenance
consumer = GenericMethodKeyRoute::I64Value / write route matcher
```

Decision: do not promote `MirType::Integer` directly to a hard Fact owner. The
next step is one consultation about a source-backed provenance Fact, with no
route-selection or runtime authority change.

### HFI-B — Shared Hako AST/body snapshot capability

Current Rust Fact owners include:

```text
src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs
src/mir/builder/control_flow/plan/facts/string_is_integer_facts.rs
src/mir/builder/control_flow/plan/facts/feature_facts.rs
src/mir/builder/control_flow/plan/facts/skeleton_facts.rs
```

Current Hako counterparts include:

```text
lang/src/compiler/lib/bool_predicate_scan_facts.hako
lang/src/compiler/lib/string_is_integer_facts.hako
lang/src/compiler/lib/loop_feature_facts.hako
```

Their common boundary is explicit: token snapshot in, read-only summary out;
full AST traversal, expression materialization, MIR mutation, route selection,
backend lowering, and ID allocation remain outside the Hako owner.

Decision: consolidate the missing full AST/body traversal into one shared
capability consultation. Do not create one task for each existing adoption
fixture or backlog label. Until the capability is accepted, keep these pilots
as scoped facades and keep Rust as the semantic oracle.

### HFI-C — Planner authority

The current planner candidates are:

```text
src/mir/builder/control_flow/plan/planner/outcome.rs::build_plan_with_facts_ctx
src/mir/builder/control_flow/plan/single_planner/mod.rs::try_build_outcome
```

The existing Hako surfaces only cover disposition and recipe-match gate DTOs.
Their adoption fixtures explicitly keep `build_plan_with_facts_ctx` and full
`try_build_outcome` migration at zero.

Decision: keep Plan authority blocked until the required Fact source and
recipe/plan boundary are complete. Do not open a planner implementation task
or move route execution, lowering, mutation, or allocation authority.

### HFI-D — Operational leftovers

Already taskized in
`repository-artifact-lifecycle-and-3511-followup-2026-07-12.md`:

```text
OLF-1 = repository artifact manifest refresh + PR-only strict ratchet
OLF-2 = 3511 evidence-label mapping + orphan test collection proof
```

Decision: retain both as parked implementation tasks. They are useful hygiene
work but do not select a MirBuilder semantic owner.

## Consolidated tasks

### HFI-1 — I64Value provenance authority consultation

Define one candidate only if it can name all of:

```text
source instruction/origin
freshness or rebuild owner
consumer boundary
fail-fast behavior
fixture and independent oracle
```

Do not implement from `MirType::Integer` alone.

### HFI-2 — Shared AST/body snapshot capability consultation

Answer whether Hako may receive a bounded AST/body snapshot DTO for one
read-only Fact owner family. The consultation must define:

```text
snapshot schema and source owner
allowed recursive shapes
analysis-only view boundary
Rust oracle/parity gate
unsupported-shape fail-fast
removal condition for token-only facades
```

This task covers bool-predicate, string-is-integer, and loop-feature/skeleton
full traversal gaps together. It does not activate a Fact owner, planner,
route, backend, mutation, or allocator.

### HFI-3 — OLF implementation queue

After the active design boundary is intentionally parked, execute OLF-1 then
OLF-2. Keep the PR gate Python-only and keep 3511 changes evidence-only.

## Ordering decision

```text
now:
  stop at 3457 / I64Value design boundary

next consultation:
  HFI-1 and HFI-2 as one compact authority/capability question packet

after consultation:
  select at most one code-facing owner

independent hygiene queue:
  OLF-1 -> OLF-2
```

No separate task is opened for `build_plan_with_facts_ctx` or full
`try_build_outcome`; those remain downstream of HFI-A/HFI-B and are recorded
as blocked planner authority.

## Explicit non-claims

```text
source_selfhost_claim = 0
full_ast_traversal_adopted = 0
mirtype_integer_hard_fact = 0
planner_authority_selected = 0
route_selection_authority_moved = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
id_allocation_authority = 0
mapstore_any_opened = 0
array_append_any_opened = 0
delete_opened = 0
```

## Consultation question

```text
3457のI64Const Fact slice後に残るhard frontierを、次の2群へ統合した。

1. MapStoreI64 dynamic I64Valueのsource/provenance Fact
2. bool/string/loop Fact pilotに共通するHako full AST/body snapshot capability

I64Valueは現在MirType::Integer metadataだけなので、hard Factには昇格しない。
既存Hako pilotもtoken snapshot facadeであり、full AST traversalやSource
Selfhostを意味しない。

次の方針として、以下を確認したい。

A. I64Valueについて、source instruction/originから再構築可能なprovenance
   Fact ownerを一件だけ選ぶ
B. Hako側は個別Factごとではなく、bounded AST/body snapshot DTOと
   analysis-only viewを共通capabilityとして先に設計する
C. Plannerのbuild_plan_with_facts_ctx / try_build_outcomeは、Factと
   snapshot boundaryが固まるまでblockedのままにする

このA+B+Cを次の設計境界として採用してよいか。
Plan、route、backend、mutation、ID allocation、Source Selfhost claimは
今回も開かない。
```
