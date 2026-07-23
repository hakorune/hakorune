# CUT0-I0 RAW-SOURCE0 LOWER0 execution task

Status: **Active executable row — `RAW-SOURCE0-LOWER0-S0`**
Date: 2026-07-23
Scope: one disconnected Raw child-draft owner; no production consumer.

Related consultation:

- `cut0-i0-raw-source0-lower-consultation-2026-07-23.md`
- `cut0-i0-raw-source0-consultation-2026-07-23.md`
- `CURRENT_STATE.toml`

## Objective

Implement the smallest source-to-draft Raw seam selected by LOWER0-D0:

```text
SourceBoundRawPackageV1
-> one Builder-owned RawDraftInvocationV1
-> one source-locator-derived child work request
-> reserve before descent
-> function-local capture/lower/restore
-> branded collector admission
-> ledger completion
```

This is a disconnected proof. It must not wire a public ingress, outer
executor, JSON bridge, root completion, physical drain, finalizer,
postprocessor, external commit, retry, fallback, or `MirBuilder::build_module`
retirement.

## Ownership contract

### `RawDraftInvocationV1`

Create one non-Clone Builder-side owner which consumes the package by value and
opens the candidate `ModuleBuilderInvocationSessionV1` with Raw's
`ContinueLive` Core-ID policy. The owner creates exactly one branded empty
shell, one branded collector, and one `RawExpansionReceiptLedgerV1` for the
same token. Do not create a second ordinal/domain or rewrap a test token.

The package handoff must avoid duplicated projection authority. If the current
BIND0 package stores the projection both in `OwnedRawSourceV1` and in the
continuation, add a private consuming split or retain one owned projection
only; do not clone it for the lowerer.

### `RawChildWorkRequestV1`

Derive one request from the sealed source locator/role. It owns the child role,
semantic key, physical symbol, and arity needed by both the ledger reservation
and the collector admission. Callers must not construct independent
`RawExpansionDraftRequestV1` and `LegacyChildDraftAdmissionV1` values.

The child terminal is source ordered:

```text
derive request -> reserve -> capture pending -> lower
-> close header loan -> prepare/collect branded receipt
-> complete ledger -> restore parent
```

On any failure, the failed draft is not collected. The reservation is aborted
into ledger evidence and the outer owner returns a rejected owner immediately.

### Branded child terminal

Add a Raw-specific branded child completion terminal (or an equivalent
invocation-port sibling) because the existing `complete_legacy_child` returns
an unbranded receipt. The new terminal must preserve collector brand and
complete the matching ledger event without exposing loose token/key/policy
arguments.

## S0 fixture boundary

The positive fixture may use one Script/top-level child shape that is already
representable by the owned source AST and deterministic traversal. It must
prove:

```text
same token brand across session/shell/collector/ledger
reserve-before-descent
one child draft collected by branded receipt
ledger completion uses that exact receipt
parent restoration exactly once
```

Negative fixtures:

```text
foreign token/brand before mutation
child capture/session failure
collector admission mismatch
ledger completion mismatch
primary + cleanup error retention
failed child is not collected
no later sibling descent
```

Do not add App-wide declaration inventory in S0. PLAN0 currently stores only
Script count and selected App locators; non-Main boxes, free functions,
instance methods, static data, closure metadata, and root Main/condition
batch require later `RAW-SOURCE0-LOWER0-ROOT0`/ACCESS rows.

## Explicit non-claims

```text
production Raw consumer/executor = 0
public wrapper/config prewrite = 0
AST JSON behavior change = 0
Program(JSON v0) merge = 0
current_module expected inventory = 0
source re-resolution = 0
root Main/condition completion = 0
physical drain/finalizer/postprocess/commit = 0
retry/fallback/catch_unwind = 0
```

## Acceptance and retirement

The focused S0 test and guard must show one Raw draft owner, one branded
collector/ledger handoff, mutation-free rejection, and all production caller
counts zero. Every new or touched source/check file remains below 800 lines.

`MirBuilder::lower_root` and `MirBuilder::finalize_module` remain in the
legacy path until all non-test Raw ingresses (legacy AST, AST-JSON parity, and
the separate Program(JSON v0) design lane) migrate and an atomic CUT0 census
proves zero direct callers. S0 alone cannot claim retirement or production
parity.

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_source_lower0_s0 --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_s0_guard.py
```

The guard must also verify that no public executor or Raw production caller
appears while this row is disconnected.

## LOWER0-S0 closeout (2026-07-23)

S0 is landed and pushed as commit `433392d918`.

The disconnected proof now exercises one real owner chain:

```text
SourceBoundRawPackageV1
-> RawDraftInvocationV1
-> owned source locator
-> reserve before child descent
-> function-local capture/lower/restore
-> branded collector admission
-> matching Raw ledger completion
```

The package projection is not cloned or re-resolved. The candidate session,
shell, collector, ledger, token, source, and continuation remain under one
non-Clone owner. Child failure returns a discard-only rejected owner and does
not continue to a sibling or fallback path.

Evidence is green:

```text
raw_s0 focused tests: 2 passed
raw_bind focused tests: 5 passed
selected_callable_main focused tests: 2 passed
cargo check --lib: green
diff --check: green
LOWER0-S0 guard: green
pointer guard: green
```

This closeout does not claim root Script/App lowering, declaration/static-data
or closure metadata admission, root Main/`condition_fn` batching, physical
drain, finalization, postprocess, public ingress, JSON parity, or production
Raw consumers. The next design stop is
`RAW-SOURCE0-LOWER0-ROOT-CONSULT0`.
