# CUT0-I0 RAW-SOURCE0-LOWER0 Consultation

Status: **Design stop — RAW-SOURCE0-LOWER0 requires a draft-only Raw lowering boundary**
Date: 2026-07-23
Scope: Raw package consumption through lowering/collection/root completion only.
No public ingress, executor, JSON behavior change, finalizer, postprocess,
external commit, or `MirBuilder::build_module` retirement is allowed here.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-consultation-2026-07-23.md`
- `src/mir/compiler/raw_source_binding.rs`
- `src/mir/builder/module_lifecycle.rs`
- `src/mir/builder/raw_physical_finalization.rs`

## Why the next row is a design stop

`RAW-SOURCE0-BIND0` now produces one compiler-owned, non-Clone
`SourceBoundRawPackageV1`. The existing Raw evidence products are also
present:

```text
RawExpansionReceiptLedgerV1
RawRootCompletionInputV1 -> RawCompleteInvocationV1
RawCompleteInvocationV1 -> RawPhysicalCompleteInvocationV1
RawPhysicalCompleteInvocationV1 -> RawFinalizationInputV1
```

But the source-to-physical lowering seam is not present. The current
`MirBuilder::lower_root` is an orchestrator over `current_module` and
`current_function`; `prepare_module` opens the live-style module state and
`finalize_module` publishes that state into a `MirModule`. It does not emit
unpublished drafts into `ModuleDraftCollectorV1`, reserve discovered work in
`RawExpansionReceiptLedgerV1`, or retain `CompletedRootBodyV1` through the
root batch.

The existing collector/ledger/root products therefore cannot be connected by
adding a call at the compiler boundary. Doing so would create two physical
owners:

```text
current_module/current_function + MirBuilder::finalize_module
collector/ledger/root completion chain
```

That would violate the SOURCE-FIRST owner law and make a future atomic CUT0
claim false. This is a genuine BoxShape/design boundary, not a missing
adapter.

## Evidence inventory

Existing source of truth:

```text
MirBuilder::prepare_module / lower_root / finalize_module
  -> direct current-function/module lifecycle

RawInvocationChildPortV1
  -> nested child lowering capability, but no module-root draft-only owner

ModuleDraftCollectorV1
  -> admission/receipt owner, no AST root traversal authority

RawExpansionReceiptLedgerV1
  -> discovered-work reservation/history owner, no AST traversal authority

RawRootCompletionInputV1
  -> prepared Main + condition batch and exact root witness, but requires
     already-produced drafts/reservations

RawPhysicalCompleteInvocationV1
  -> consumes a completed Raw root owner plus session/shell, not source AST
```

The missing product is a single Raw lowering owner that can borrow the
candidate Builder only while lowering and can route every completed draft to
the same collector/ledger/root owner without publishing a module early.

## Questions to lock

### Q1 — root lowering owner

Which product owns the Raw AST-to-draft traversal?

```text
1. A new Builder-side Raw draft-only module lowerer which consumes
   SourceBoundRawPackageV1 by value, owns the candidate session/shell/collector/
   ledger, and exposes only child/root completion terminals.

2. Reuse MirBuilder::lower_root and adapt its current function/module output
   afterward.

3. Let the compiler orchestrator traverse the AST and call Builder helpers
   directly.
```

Option 1 is the only candidate compatible with one physical owner. Options 2
and 3 preserve a second traversal/publication authority and are not safe
without a separate migration design.

### Q2 — draft-only seam

What is the minimal Builder seam for the root body and static children?

```text
1. `prepare_module`/`lower_root` are split into an unpublished Raw root
   session: root function state is borrowed during lowering, and completed
   functions leave through typed collector admissions.

2. Keep `lower_root` intact and add collector hooks around `try_add_function`.

3. Build a second Raw AST traversal beside `module_lifecycle.rs`.
```

The chosen seam must ensure `current_module` is never the expected-inventory
authority and `finalize_module` is not reached from the Raw draft-only path.

### Q3 — discovery and admission order

How are discovered children connected to the ledger and collector?

```text
1. Reserve a typed Raw draft request before each child descent, lower the
   child in a function-local session, admit/collect one receipt, then restore
   the parent. Root Main + condition are preflighted and collected as one
   atomic batch.

2. Lower all functions first, then reconstruct ledger events from the
   resulting module map.

3. Let the collector discover symbols from AST and let the ledger mirror it.
```

Option 1 preserves source discovery order, receipt provenance, and fail-fast
child error propagation. Options 2 and 3 create a second inventory authority.

### Q4 — root mode and callable Main

How does the lowerer select Script/App and callable-Main compatibility?

```text
1. Consume the sealed `OwnedRawRootProjectionV1` and
   `RawSourceContinuationV1`; Script has no callable-Main child, App uses the
   already sealed Omitted/Required disposition, and Selected failure aborts
   before inline root completion.

2. Re-run `VerifiedRawRootExpansionV1` inside Builder lowering.

3. Read `current_module` or ambient compatibility environment during body
   lowering.
```

Only option 1 preserves BIND0 source authority and one-time compatibility
selection.

### Q5 — failure and handoff

What does a failed lowering terminal return?

```text
1. A rejected Raw lowering owner retaining package/session/shell/collector/
   ledger/root prefix, with no sibling continuation, retry, or fallback.

2. Bare String error after dropping the physical owner.

3. Continue lowering fresh siblings and report the first error at finalization.
```

The production law must be option 1. Local function restoration remains
required, but the outer Raw invocation aborts on the first child/root/batch
failure.

## Non-claims while stopped

```text
RAW-SOURCE0-LOWER0 implementation = 0
production Raw consumer = 0
public outer executor = 0
MirBuilder::build_module retirement = 0
current_module -> collector reconstruction = 0
source re-resolution = 0
retry/fallback = 0
```

## Required decision output

The next design response must select Q1–Q5 and name the smallest executable
slice. It must explicitly define:

```text
one Raw draft-only Builder owner
one collector/ledger/root handoff
one source-derived root policy
one rejected-owner failure product
one retirement condition for direct lower_root/finalize_module use
```

Until that decision is locked, do not add a Raw lowering adapter or wire any
production consumer.
