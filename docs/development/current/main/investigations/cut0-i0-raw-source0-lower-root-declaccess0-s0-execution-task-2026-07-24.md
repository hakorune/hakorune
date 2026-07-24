# RAW-SOURCE0 LOWER ROOT0 — DECLACCESS0-S0 execution task

Status: **Closed — `DECLACCESS0-S0` (`e2fb0839f5`); production consumers remain zero**
Date: 2026-07-24  
Decision: **DECLACCESS-prime-r1**

## Progress

The behavior-neutral preparation slice is landed in `c3f286fdfe`, and the
Builder-owned projection/co-install sub-slice is landed in `77983f2e95`:

```text
RawRootPlanV1 exposes only the source-derived manifest inputs needed by the
next exact-manifest constructor (callable rows, Script statement count,
App child/callable locators, and the eligible package view).
```

The exact manifest split, mutation-free Builder/shell preflight, and private
infallible co-install are covered by the disconnected guard and focused
tests. The compiler-side `declare_environment(self)` owner is now implemented
as the sole consuming terminal: it splits the ready owner once, derives the
Builder/shell projections from the same manifest, and returns a route-specific
declared owner or a discard-only rejected owner. Focused Script/App success
and dirty-Builder rejection fixtures pass. BODY0 and production consumers remain zero.

## Boundary

DECLACCESS0 consumes one `RawCallableMainReadyInvocationV1` and installs the
already sealed source-derived environment into the same candidate Builder and
physical shell. It does not lower the root body, publish `main/0`, reserve
the Main/condition batch, drain, finalize, postprocess, commit, or change a
public ingress.

```text
RawCallableMainReadyInvocationV1
  -> declare_environment(self)
       private mutation-free prepare
       private infallible commit
  -> DeclaredRawRootInvocationV1
       exact manifest + installed environment + typed body payload
  -> BODY0 later
```

The only caller-visible terminal is `declare_environment(self)`. `prepare`
and `commit` remain private implementation details of the same module.

## Locked decisions

### Q1 — one installation owner

Use one Raw-root-specific consuming terminal. The prepared aggregate owns both
the Builder-index projection and shell-metadata projection, plus the exact
body payload and manifest seal. Separate Builder and shell terminals are not
allowed.

### Q2 — exact manifest authority

Replace the summary-only environment handoff with one non-Clone
`RawRootEnvironmentManifestV1`. It owns:

```text
physical root identity and route
exact ScalarControl0 body payload
literal values, source path, and span
exact App Main locator and callable catalog
module declaration coverage
runtime-input snapshot and ingress-config witness
ProvedAbsent static-data / closure / process-slot seals
```

No whole-AST rescan, `OwnedRawSourceV1` re-read, `current_module` lookup,
classifier re-run, or emitted-MIR literal inference is permitted after the
manifest is sealed. The old name-only Script declaration acceptance must be
strengthened: first-slice Script has exact zero module declarations; App has
static Main only. Other declaration surfaces reject before physical effects.

### Q3 — prepare/commit sequence

The private sequence is fixed:

```text
borrow complete ready owner
-> validate token/family/session/physical/ledger/tracker brands
-> validate exact eligibility and manifest coverage
-> validate Raw-owned Builder/shell destination lanes
-> build both projections and body payload
-> consume owner into one prepared aggregate
-> private infallible commit
-> DeclaredRawRootInvocationV1
```

All fallible work completes before the prepared product exists. Failure
returns the complete ready owner unchanged. Commit returns no `Result` and
performs no lookup, allocation, or semantic validation.

### Q4 — first-slice capabilities

Admit only:

```text
ScalarControl0 exact located body payload
exact-zero Script declaration environment
plain static-Main App declaration environment
complete narrow callable catalog
snapshot-only imports/plugin signatures/runtime inputs
```

Static data, closures, and process-global slots remain typed `ProvedAbsent`
seals. Enum/brand/type-alias/global/top-level-function/non-Main-box and
partial-catalog shapes reject at ELIGIBILITY0; DECLACCESS0 does not become a
second eligibility authority.

### Q5 — BODY0 handoff

Success issues non-Clone `DeclaredRawRootInvocationV1`. Its only continuation
is `begin_body(self)`. The installed environment and exact body payload are
retained, while the owned source AST is no longer needed.

```text
root tracker activity = untouched before BODY0
physical main/0       = unpublished
condition_fn/1        = unpublished
root ledger reserves  = 0
ROOTBATCH0 input      = completed BODY0 product only
```

### Q6 — guard and proof budget

Use one reusable Raw-root lane guard with a DECLACCESS0 profile. Add no
one-off per-row shell guard. The active card records:

```text
ceremony_tier = T2 new authority
sunset_id = RAW-ROOT-DISCONNECTED-PROOFS
retirement_condition = production caller count reaches CUT0 target
sunset_row = RAW-ROOT-CUTOVER-G0
proof_inventory_before = PLAN0/OWNER0/ELIGIBILITY0/CHILDREN0/CALLMAIN0
new_proofs = exact manifest + co-install + rejection snapshots
retired_or_merged_proofs = none in this row
net_proof_delta = positive, repaid at CUTOVER-G0
```

## Implementation files

Keep every new or modified source/check file below 800 lines.

```text
ADD
  src/mir/compiler/raw_root_environment_manifest.rs
  src/mir/compiler/raw_root_decl_access.rs
  src/mir/compiler/raw_root_decl_access_p0.rs
  src/mir/builder/raw_root_environment_projection.rs
  src/mir/builder/raw_root_physical/environment_terminal.rs

EDIT narrowly
  src/mir/compiler/raw_root_plan0.rs
  src/mir/compiler/raw_root_callable_main.rs
  src/mir/compiler/raw_root_eligibility.rs
  src/mir/builder/raw_root_physical.rs
  src/mir/compiler/mod.rs
  src/mir/builder.rs
  tools/checks/lib/cut0_i0_root0_raw_lane_guard.py
```

Do not reuse or expose `MainPending/MainCaptured`,
`build_static_main_box_typed`, `lower_root`, `finalize_module`,
`current_module`, or a bounded source lookup port as the BODY0 authority.

## Required fixtures

### Success

```text
empty ScalarControl0 Script
plain static-Main App with zero helpers
plain static-Main App with helpers and selected callable Main already complete
App with callable Main NotSelected
literal/operator/name/path/span payload preserved exactly
Builder and shell projections derived from one manifest
```

### Rejection and atomicity

```text
Script declaration / top-level function / non-Main box
enum / brand / global / partial catalog
foreign token/session/physical/family
dirty Builder declaration/catalog/root lanes
dirty shell declaration/static/closure lanes
missing or duplicate source site/literal payload
invalid ProvedAbsent capability seal
```

For every failure:

```text
ready owner retained
Builder mutation = 0
shell mutation = 0
collector/ledger/tracker mutation = 0
BODY0 entry = 0
retry/resume/fallback/replacement = 0
```

## Reusable lane guard

The guard profile must prove:

```text
RawRootEnvironmentManifestV1 producer = 1
declare_environment(self) producer = 1
PreparedRawRootEnvironmentInstallV1 producer = 1
DeclaredRawRootInvocationV1 producer = 1
separate Builder/shell environment terminals = 0
manifest/prepared/declared Clone = 0
current_module / AST rescan after binding = 0
legacy MainPending/MainCaptured/root adapter = 0
BODY0/ROOTBATCH0/production consumers = 0
all touched source/check files < 800 lines
```

## Explicit non-claims

This row does not claim BODY0 lowering, root tracker completion,
Main/condition batching, physical drain, finalization, postprocess, external
commit, public ingress, JSON parity, legacy removal, or CUT0 activation.

## Evidence

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_decl_access --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_root --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q raw_source_binding --lib -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_declaccess0_s0_guard.py
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_declaccess_coinstall0_guard.py
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_callmain0_guard.py
git diff --check
```

All listed commands pass before commit. The implementation remains
disconnected: no BODY0, root batch, production executor, or public ingress
consumer is introduced.

## Closeout

`DECLACCESS0-S0` is closed. The compiler-side `declare_environment(self)`
terminal consumes the CALLMAIN-ready owner exactly once, derives the Builder
and shell projections from the retained manifest, performs mutation-free
preflight, and privately commits one installed environment. Success returns
the route-specific declared owner; every preflight failure retains the exact
unpublished chain in a discard-only rejected owner. BODY0, root batching,
drain, finalization, postprocess, external commit, public ingress, JSON
parity, and CUT0 activation remain zero.

The next design stop is
`RAW-SOURCE0-LOWER0-ROOT0-BODY0-CONSULT0`.
