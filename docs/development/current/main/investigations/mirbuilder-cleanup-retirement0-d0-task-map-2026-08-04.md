# MIRBUILDER-CLEANUP-RETIREMENT0-D0

Status: parked BoxShape task map; audit complete, implementation not
authorized by this card.
Date: 2026-08-04

This card records the cleanup opportunities raised by the dead-code audit. It
is deliberately separate from the active If D0-A authority/PHI census. The
goal is to reduce duplicate shells and retire proven disconnected code without
changing route selection, Recipe semantics, SSA/PHI authority, or the JSON-v0
compatibility contract.

## Audit corrections

The headline estimate of “about 5,300 lines immediately deletable” is not a
safe deletion claim.

### Tier 1: `_p0` proof modules

All 28 `src/mir/compiler/*_p0.rs` modules are test-only/compile-only and have
zero production callers. They are still load-bearing contract evidence: many
have no live twin, and roughly 15 active guards reference their exact file or
symbol. The largest is 752 lines; all remain below the 800-line boundary.

Safe rule:

```text
stable replacement fixture/manifest
  -> retarget guard and prove old symbol excluded
  -> focused gate green
  -> delete one bounded module/registration in a buildable commit
```

Deletion follows the proof dependency DAG (independent drain/session proofs,
then canonical leaf-to-root, then raw public leaf-to-root). Loop candidate
abort and resolved DirectAccum hardening remain a separate final lane after
Loop production cutover. No `_p0` batch deletion is authorized here.

### `source_coverage.rs` is live

`src/mir/resolved_control_flow/source_coverage.rs` is not dead. `if_control.rs`
uses its coverage types and verifier in the production canonical preflight path;
the dedicated tests cover its rejection and ownership branches. Any wording
claiming “production consumer zero” must be narrowed to “no direct lowering or
analyzer consumer”; the file must remain.

### Stale `dead_code` masks

The three resolved modules contain real production owners mixed with unused
accessors, fields, and staged schema/re-exports:

```text
resolved_control_flow  = live completion/If control + a small accessor/re-export tail
resolved_region_flow   = live flow/port analysis + unused accessor/re-export tail
resolved_semantics     = live resolver/catalog/forest + staged SA0/SA1 surface
```

Do not remove all module-level allows in one change. First make a caller/lint
baseline; then narrow only stale imports/re-exports, followed by one accessor
group per behavior-neutral series. `resolved_semantics` requires a separate
submodule ownership ledger before any allow relocation.

## Ordered work packages

## Worker-verified execution order

The worker census confirms that the headline deletion list is a cleanup
program, not a single delete commit. The following order is now the task
board order:

```text
T1-D0 proof/mask baseline
  -> T1-S0 one proof module at a time
  -> T1-S1 narrow resolved-module masks
  -> T2-S0 route-neutral recipe bundle dedup
  -> T2-S1 trivial policy-matrix dedup
  -> T3-S0 test-facade/callable-result/naming lanes
  -> T4-R0 JSON-v0 bridge caller-zero retirement
```

`T1-D0` is the only prerequisite for deletion. `T2` and `T3` may be
parallelized after the baseline is recorded, but must not change route policy,
Recipe semantics, or PHI/SSA ownership. `T4` remains an independent
caller-zero lane because the bridge has its own Program-JSON compatibility
contract and direct PHI bypasses.

The worker inventory fixes three audit boundaries:

- Every `_p0` file is test/compile-only, but its tests and guards are evidence;
  the 28-file set is migrated and retired leaf-to-root, never batch-deleted.
- `source_coverage.rs` is a live production owner used by `if_control.rs`;
  it is excluded from the delete set. Only its “no direct lowering consumer”
  wording may be corrected.
- `resolved_control_flow` and `resolved_region_flow` can narrow stale masks
  after caller/lint baselines. `resolved_semantics` has staged live schema and
  needs its own ownership ledger before any module-level allow is removed.

The same audit also records the non-mechanical boundaries: `_for_test`
facades that forge brands or mutate Builder/SSA state stay owner-local, the
two `type_facts.rs` files are different domains until a naming decision is
written, and the JSON-v0 bridge is deleted only after its four caller families
are zero. These are explicit stop conditions, not implied cleanup work.

### T1-D0 — proof-module and mask census (design stop)

Freeze the 28-file manifest, guard dependencies, live-twin/proof-owner status,
and the three resolved-module lint baseline. Record `source_coverage.rs` as a
live owner. No deletion and no route/PHI changes.

### T1-S0 — proof migration and guarded `_p0` retirement

Work from leaf to root in small buildable commits. For each row, add or point
to a stable contract test, retarget every guard, run the focused gate, then
delete only the now-redundant file/registration. Acceptance is caller-zero,
guard-zero, twin/proof coverage, and fast-gate green. Keep source coverage and
Loop-specific p0 proofs until their independent lanes close.

### T1-S1 — narrow stale-mask cleanup

After the relevant `_p0` rows are retired, remove stale re-exports/imports in
`resolved_control_flow` and `resolved_region_flow`, then retire accessor groups
only with caller-zero guards. Treat `resolved_semantics` as its own staged API
series; never use a global `force-warn` result as permission to delete schema.

### T2-D0/S0 — route-neutral Recipe carrier dedup

The 11 route-specific `XRecipe { arena, root }` structs are structurally
identical. Introduce one `BuiltRecipeTree` (name provisional) with aliases,
then migrate in at most five refactor commits and delete the aliases. This is
shape dedup only: route Facts, matcher/composer policy, and portable
`LoopRecipeArtifactV1` remain separate authorities. Guard old type names at
zero and the new bundle at one shared definition.

### T2-D1/S1 — trivial canonical policy-matrix consolidation

The four `analyze_trivial_canonical_*` wrappers are policy-matrix delegators,
not strict byte-identical functions. Add a neutral
`TrivialCanonicalAnalysisModeV1` (`ordinary/main × closed/finite-direct-call`),
migrate capability callers and tests, then collapse the shared analyzer kernel.
Keep `main` role data in the mode even if currently unused. Guard the four old
symbols at zero and the new production entry at one capability owner.

### T3-D0..S3 — test facade and naming cleanup

First manifest every `_for_test` facade with cfg scope, owner, mutation class,
all callers, and candidate reachability. Delete only confirmed USE=0 helpers.
Move pure fixtures/observers into owner-local `#[cfg(test)] test_support`;
keep sealed-brand forges, failure injection, Builder/SSA mutation, reset, and
private lifecycle helpers owner-local. Normalize mixed re-exports only after
the owner boundary is guarded. Do not move PHI/SSA helpers in this lane.

Callable-result exports get a separate C0→C1→C2 series: split direct
submodule/test-only imports first, then narrow the nine allows, and remove root
exports only after caller-zero proof. `has_scope_box_lineage` remains a
route-specific aggregation over a shared site primitive; do not replace it with
a global body-wide boolean. The two `type_facts.rs` files are distinct
domains; naming audit precedes any mechanical rename.

### T4-D0..R0 — JSON-v0 bridge retirement

This is a separate phase-29ci/29cj lane, not a unification of MIR lowering.
First freeze four caller families (runtime direct MIR JSON, Stage-1 handoff,
selfhost producer/consumer, Program JSON-v0 compatibility). Route direct loop
PHI bypasses in `json_v0_bridge/lowering/loop_.rs` and `loop_range.rs` through
LoopForm or explicitly quarantine them before bridge deletion. Preserve
Program-specific import-bundle/trace behavior until all tools, probes, stage1
wrappers, and compat callers are zero. Only then hard-delete the bridge.

## Cross-lane rules and acceptance

- This card never authorizes If/Loop production wiring or PHI owner adoption.
- Semantic PHI/SSA SSOT exists (`BindingSsaBuilderV1`/`PhiTxn` and the LoopForm
  owner), but exclusive production physicalization is not yet complete.
- Every refactor series is behavior-neutral, buildable, and below 800 lines;
  BoxCount and BoxShape changes are not mixed.
- Each deletion has a stable replacement proof, caller/guard census, focused
  gate, and explicit rollback boundary.
- The current blocker remains the If D0-B2 facts-mapper design stop; this cleanup card is parked
  and may run in parallel only as read-only audit/design work until a bounded
  row is promoted.
