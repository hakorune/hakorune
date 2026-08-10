---
Status: revise / design stop; bridge owner incomplete
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

`DYNAMIC-CALLABLE-RESULT-CONTRACT-I0` cannot close yet. The canonical Hako
owner is fixed as the existing `source_carrier_v1` lifecycle, but the bridge
from its sealed row to normalized Rust/Hako parity, the resolved callable
batch, and the sole result issuer is not yet closed. The existing H2/H3
sequence is the parser substrate; it does not replace this bridge Decision.

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

The canonical owner is the Hako parser source-carrier/session path under
`lang/src/compiler/parser/source_carrier_v1/**`, next to the existing typed
callable declaration/source handoff. It must be issued during the same parse
transaction as the callable declaration identity and final source slots.

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
ParserMethodResultSyntaxV1
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

The H2-S3/H2-I0/H3-I0 sequence may open only when all items below are fixed;
none of these rows may use compatibility fallback:

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
  current design stop: bridge owner/parity/batch relation and prerequisite audit

H2-S2-S1-R1
  re-audit its recorded GenericLoop blocker before reopening the expression
  product; do not repair the compatibility guard as a shortcut

H2-S2-S1-I0
  close only after the prerequisite is genuinely green

H2-S3-I0
  unpublished direct-method transaction:
  exact method site + parameters + body + typed result syntax; no final seal

H2-I0
  bounded ordinary Box direct-method parser connection

H3-I0
  final atomic declaration/source seal and complete coverage

H5
  test-only normalized Rust/Hako parity

HAKO-CALLABLE-HEADER-RESULT-CARRIER-I0
  consume the H3-sealed row, prove normalized parity, and connect one row to
  the existing resolved batch/result issuer; parser reimplementation forbidden

DYNAMIC-CALLABLE-RESULT-CONTRACT-I0
  resume only after H2/H3/H5 are green; selected skip_while remains outside
  this bounded body cohort until its full source/body transaction exists
  issue one source-backed exact-I64 result receipt

PHYSICAL-INPUT-AUTHORITY-I0
  resume only after the result/ABI boundary is closed
```

The earlier Rust-only receipt work is retained in a reversible stash and is not
an active production or selfhost claim. Rebase it only after H3/H5 provide the
same Hako source product and the bridge I0 is accepted.

H2/H3 are lower parser-carrier rows, not a substitute for the bridge. The
first H2 fixture may be `length(): i64 { return 0 }`; it cannot claim
`skip_while/4` parity because locals, loops, conditionals, calls, and multiple
returns require a later complete body cohort.

## Dynamic loop unification parked lane

This Hako card only records the route; the canonical task text is the parked
section in `dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md`, and
the global order is owned by `loop-common-physical-demand-and-session-ssot.md`:

```text
LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-PHYSICAL-ALWAYS/IF/EXIT-COVERAGE-I0
```

The bounded common protocol is:

```text
verified Recipe placement + JoinSig logical transfer
  -> prepared physical layout
complete operation/source-effect ledger
  -> complete physical demand
```

The cleanup must remove Recipe-derived Predicate/Jump/Backedge transfer
reconstruction, Recipe condition rescans in `segment_allocator`, repeated V1
Recipe/evidence scans, and Callable profile counts from the common physicalizer.
`ReadyLoopAfterContinuationV1` is the common stop line; Tail, ABI, Completion,
and callable profile-close remain outside. No V2-to-V1 adapter, synthetic
`ItemKey`, name/order repair, second JoinSig/Recipe, physical planner, or Hako
authority is allowed. This card does not schedule the parked lane.

The common compiler flow remains the existing:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower
        -> Seal -> Collect -> Atomic Publish
```

See `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`
for the global flow and `docs/reference/mir/loop-recipe-contract.md` for the
portable Recipe/JoinSig boundary.

## Non-claims / hard stops

```text
no live ordinary parser branch claim before H2-I0/H3-I0
no canonical Hako owner from FuncScanner/Stage-B JSON
no GenericLoop fix just to make the compatibility guard green
no Rust success as Hako parity
no result receipt from source text, loop shape, body return, MirType,
  FunctionSignature, runtime tag, selector name, or physical ABI
no selected Dynamic result I0 or PHYSICAL-INPUT-AUTHORITY-I0 resume
no physical session, DraftSeal, Collector, publication, retry, or fallback
```
