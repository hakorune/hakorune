# RAW-SOURCE0 LOWER ROOT0 — FINAL0-S0 実行タスク

Status: **Closed implementation row — FINAL-DRAIN-prime-r1**
Date: 2026-07-24
Decision source: `cut0-i0-raw-source0-lower-root-final0-source-drain-handoff-consultation-question-2026-07-24.md`

## Decision lock

```text
RawDrainedInvocationV1
  -> prepare_finalization(self)
  -> RawFinalizedInvocationV1::{Script, App}
```

FINAL0 consumes the DRAIN0 owner directly. `RawDrainWitnessV1.manifest` is
the sole operational expected-inventory authority; the sealed ledger remains
retained provenance and is not reprojected. The opaque candidate module is
matched against the existing manifest and then paired with a prepared Builder
session. Preparation is mutation-free and the private typestate commit is
infallible.

The old `RawPhysicalCompleteInvocationV1` / `RawFinalizationInputV1` bridge
is not an adapter. Its callers stay zero. Lexical hard-coded inventory and
`into_draft_functions()` may remain in that old source until a later retirement
row, but neither is allowed in the new FINAL0 source or production call path.

## S0a — FINAL0-GUARD-SCOPE0

The guard-only prerequisite excludes `#[cfg(test)]`-registered modules before
measuring production callers. It prevents the existing `prod_activation_p0_r1`
fixture from being counted as a production finalization caller. This is a
guard-only change with no Rust behavior delta.

## S0b — FINAL-MANIFEST0

No new inventory manifest is generated. The Builder terminal consumes the
already sealed `RawDrainWitnessV1.manifest` and proves:

```text
manifest route == Script/App route evidence
helper receipt count == StaticHelper rows
callable disposition == CallableMainCompatibility row presence
candidate function count == manifest row count
every row symbol exists with matching signature name and arity
surplus candidate symbol = 0
module name == retained module name
token/session/witness brands and Raw family agree
```

No AST, source catalog, collector, `current_module`, or second ledger
projection is permitted.

## S0c — FINAL-PHYSICAL0

`RawDrainedPhysicalV1::prepare_raw_finalization` is the only Builder-side
physical terminal. It consumes the opaque unfinalized module and witness,
checks Builder readiness with `prepare_module_session()`, and returns a named
prepared product. The private commit converts:

```text
RawUnfinalizedModuleV1 -> RawFinalizedModuleV1
prepared Builder session + witness + parity -> RawFinalizedPhysicalV1
```

No bare `MirModule`, mutable module accessor, shell/collector/ledger tuple,
optimizer, verifier, postprocess, or external commit is exposed.

## S0d — FINAL-I0

`RawDrainedInvocationV1::prepare_finalization(self)` retains all route
evidence and produces typed Script/App success or discard-only rejection. App
callable-Main outcome, helper receipts, continuation, runtime snapshot,
module name, completion evidence, physical finalization product, witness, and
Builder readiness seal remain paired.

Rejection retains the exact drained owner and typed physical cause. Public
terminals are `stage()`, `error()`, and `discard(self)` only.

## S0e — FINAL-G0

The lane guard must measure production scope rather than repository-wide
lexical occurrence counts:

```text
new direct RawDrainedInvocationV1 finalizer = 1
old RawPhysicalCompleteInvocationV1 caller = 0
old RawModuleFinalizerV1 caller = 0
new FINAL0 hard-coded root inventory = 0
new FINAL0 into_draft_functions = 0
bare MirModule between DRAIN0 and FINAL0 = 0
source/catalog/current_module re-observation = 0
POST0 / external commit / public ingress consumers = 0
all modified/new source/check files < 800 lines
```

Repository-wide old-string removal is explicitly deferred to the retirement
row.

## Fixtures

```text
empty Script -> direct Raw finalization Script product
App + callable-Main NotSelected -> App product, callable row absent
App + callable-Main Selected -> App product, callable evidence retained
wrong module name / route / callable evidence -> typed rejection
missing or surplus candidate function -> typed rejection
symbol or arity drift -> typed rejection
Builder readiness failure -> exact drained owner retained
all failures -> module/manifest/witness unchanged, retry = 0
```

## Non-claims

```text
POST0 optimizer/refresh/verifier execution = 0
external commit = 0
public ingress / JSON bridge = 0
old finalizer source deletion = 0
legacy retirement = 0
production activation / CUT0 = 0
typed panic retention = 0
```

All touched source/check files remain below 800 lines. The next boundary is
POST0, which may consume only `RawFinalizedInvocationV1`.

## Evidence

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib                        = PASS
RUSTFLAGS='-Awarnings' cargo test -q raw_root_finalization_p0 --lib = PASS (4)
python3 ...final0_guard_scope.py                                   = PASS
python3 ...final0_guard.py                                         = PASS
bash tools/checks/current_state_pointer_guard.sh                   = PASS
git diff --check                                                    = PASS
```
