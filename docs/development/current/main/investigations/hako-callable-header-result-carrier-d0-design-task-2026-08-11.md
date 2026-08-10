---
Status: DESIGN STOP / NoSafeSlice
Date: 2026-08-11
Scope: canonical Hako callable-header result annotation authority only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/DOCS_LAYOUT.md
---

# HAKO-CALLABLE-HEADER-RESULT-CARRIER-D0

## Decision

`DYNAMIC-CALLABLE-RESULT-CONTRACT-I0` cannot close yet. The Rust final-source
and resolved-batch path can observe an explicit `: i64` annotation, but the
canonical Hako parser transaction does not currently retain that annotation in
the typed callable-header/source seal that feeds resolver identity and the
semantic batch.

Therefore the Hako axis is `NoSafeSlice`. This is a design stop, not an
invitation to repair the compatibility scanner or to infer a result downstream.

The next design question is:

```text
canonical Hako parser session
  -> typed callable-header result row
  -> same parser provenance / declaration identity
  -> final source seal
  -> resolved callable batch
  -> one source-backed result-contract issuer
```

The selected bounded fixture remains:

```hako
skip_while(src, pos, end, pred_chars): i64 { ... }
```

No new Rune, generic result-disposition syntax, loop/body inference, MIR
inference, runtime-tag inference, ABI reverse inference, fallback, or retry is
allowed.

## Authority boundary

The canonical owner must live in the Hako parser source-carrier/session path
under `lang/src/compiler/parser/**`, next to the existing typed callable
declaration/source handoff. It must be issued during the same parse transaction
as the callable declaration identity and final source slots.

The following are explicitly not canonical owners:

```text
lang/src/compiler/entry/func_scanner*.hako
StageBJsonBuilderBox
Program(JSON v0) return_type metadata
k2_wide_stageb_return_type_annotation_alignment_guard.sh
source-text rescan after the parser seal
```

Those may remain compatibility/diagnostic probes. They cannot issue a
`Verified*` result receipt or serve as Rust/Hako parity evidence for this row.

## Minimum product shape

```text
ParserCallableHeaderResultSyntaxV1
  declaration identity
  parameter/header source site
  optional declared TYPE_REF spelling
  parser provenance

VerifiedHakoFinalCallableProgramSourceV1
  complete callable rows
  final slots
  typed header-result row
  same opaque declaration identity

Hako source row + Rust source row normalized parity
  -> one resolved callable row
  -> DeclaredCallableResultContractIssuerV1
  -> VerifiedDeclaredExactI64CallableResultContractV1
```

The result receipt owns only the declared semantic result (`I64`, with the
bounded `ExactScalar` representation contract). It is not a body-conformance
proof, MIR `MirType`, physical ABI, return writer, Completion, or publication
receipt.

## Acceptance criteria

This D0 may move to I0 only when all items below are decided and the canonical
owner is implementable without compatibility fallback:

1. The exact Hako parser/session owner is identified in `lang/src/compiler/parser/**`.
2. The typed result row is issued in the same parser transaction as callable
   identity/source slots; no post-seal AST or text rescan is needed.
3. Rust and Hako normalized rows agree on declaration identity, method mode,
   parameter/header shape, and declared result spelling.
4. The Hako row reaches the resolver batch through the same source provenance
   and opaque identity relation as Rust.
5. Exactly one canonical result issuer can issue the bounded `: i64` receipt
   for the selected `StaticBoxMethod` row.
6. Missing annotation, `void`, other types, foreign source identity, foreign
   owner, and duplicate rows fail with typed rejection; none becomes an
   inferred result.
7. Compatibility scanner probes are recorded as informational only. A failing
   generic-loop probe is not repaired as part of this task.
8. Parser README, source-carrier README, language/reference notes, parity
   corpus, and the active task card are updated in the same implementation
   slice.

## Task ladder

```text
HAKO-CALLABLE-HEADER-RESULT-CARRIER-D0
  design stop / owner census / no code

HAKO-CALLABLE-HEADER-RESULT-CARRIER-I0
  typed Hako source row + same-pass seal + Rust/Hako parity

DYNAMIC-CALLABLE-RESULT-CONTRACT-I0
  resume only after Hako owner/parity is green
  issue one source-backed exact-I64 result receipt

PHYSICAL-INPUT-AUTHORITY-I0
  resume only after the result/ABI boundary is closed
```

The Rust-only receipt work is retained in a reversible stash and is not an
active production or selfhost claim. It may be rebased onto this task only
after the Hako source product exists.

## Dynamic loop unification parked lane

The dynamic loop can share a common physical protocol after its bounded
physical-input/demand products are complete. This is a BoxShape cleanup, not a
new result or Hako authority:

```text
verified Recipe placement
  + JoinSig logical transfer view
  -> prepared physical layout

complete operation/source-effect ledger
  -> complete physical demand
```

The following must be removed before that lane is called unified:

```text
physical_layout / recursive_after:
  no Predicate/Jump/Backedge reconstruction from LoopConditionV1/as_recipe()

segment_allocator:
  no Recipe condition rescan to decide Header/Body

common physicalizer:
  stop at ReadyLoopAfterContinuationV1
  no Callable profile counts/symbols, Tail, ABI, or Completion
```

The parked follow-ups remain bounded and separate:

```text
LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  move callable profile-close/Tail/ABI/Completion to callable owner

LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
  census fixed-role receipts versus segment receipts;
  delete only after old production/test callers reach zero

LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  consume the common transfer/placement boundary if the census proves one
```

No V2-to-V1 adapter, synthetic `ItemKey`, name/order repair, second JoinSig,
second Recipe, or physical planner is allowed. The common compiler flow remains
the existing:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower
        -> Seal -> Collect -> Atomic Publish
```

See `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
for the global flow and `docs/reference/mir/loop-recipe-contract.md` for the
portable Recipe/JoinSig boundary.

## Non-claims / hard stops

```text
no canonical Hako owner from FuncScanner/Stage-B JSON
no GenericLoop fix just to make the compatibility guard green
no Rust success as Hako parity
no result receipt from source text, loop shape, body return, MirType,
  FunctionSignature, runtime tag, selector name, or physical ABI
no PHYSICAL-INPUT-AUTHORITY-I0 resume
no physical session, DraftSeal, Collector, publication, retry, or fallback
```
