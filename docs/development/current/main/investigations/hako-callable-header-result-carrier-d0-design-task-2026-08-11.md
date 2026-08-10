---
Status: accepted future Hako frontend producer and parity design. The current
bootstrap producer is the Rust final-source row; the Hako producer activates
only after H2/H3/H5 parity and atomically retires the Rust frontend authority.
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

The semantic authority is the explicit source annotation, not a particular
frontend implementation. The selected frontend emits one normalized callable-
header row; one frontend-neutral result-contract issuer consumes it together
with the existing opaque declaration identity and resolved batch row.

During bootstrap the Rust final-source producer is the only production
producer. After H2/H3/H5, `source_carrier_v1` emits the same normalized row as
offline parity evidence; one atomic selfhost cutover activates the Hako
producer and removes the Rust frontend producer from that production path.
Both producers are never admitted in one compilation and there is no retry or
fallback between them.

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

The normalized row contract is frontend-neutral. Its current production owner
is `VerifiedFinalCallableProgramSourceV1`; its future selfhost producer is the
Hako parser source-carrier/session path under
`lang/src/compiler/parser/source_carrier_v1/**`. Each producer must issue the
row in the same parse transaction as callable identity and final source slots.

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

## Accepted normalized producer contract

The bridge is a single downstream projection, not a second Hako parser and not
a compatibility provider:

```text
selected frontend normalized header row
  + same parser provenance + CallableDeclarationIdentityV1
  + existing resolved callable batch row
  -> sole DeclaredCallableResultContractIssuerV1
  -> frontend-neutral exact-I64 result receipt
```

The selected frontend owns syntax and parser provenance. The resolved callable
batch owns owner/forest/projection and exposes only the identity-to-batch
relation. The result issuer consumes that row and may not infer from method
name, body, `MirType`, `FunctionSignature`, ABI, runtime tag, or inventory
ordinal. H5 parity is test evidence, not a semantic issuer. If any edge is
unavailable, stop with `NoSafeSlice` rather than adding JSON, FuncScanner, a
text rescan, a second batch, or a frontend-specific result receipt.

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
  accepted: normalized row, producer staging, parity and cutover law

H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-BRIDGE-D0
  accepted and parked: no standalone bridge I0; issue+consume in the selected
  Dynamic cutover cell

H2-SELECTED-DYNAMIC-LOWERING-AUTHORITY-R0
  closed historical row: package is the sole Dynamic classifier

DYNAMIC-CALLABLE-RESULT-CONTRACT-I0
  CURRENT: add `: i64` to the canonical production declaration; transport the
  Rust final-source typed result row through the same declaration identity and
  batch mapping; issue one frontend-neutral exact-I64 contract

PHYSICAL-INPUT-AUTHORITY-I0
  project one ABI and co-seal Prelude/Tail with the exact multi-site
  Completion/operand set before Builder effects

DYNAMIC-EXIT-PHYSICAL-SESSION-P0
  consume exact multi-site Completion claims in one unpublished session;
  DraftSeal emits one Return per claimed exit block; no synthetic
  return-join/PHI

LOOP-UNIFICATION-AFTER-DYNAMIC-D0
  remove Recipe-derived transfer inference and repeated V1 evidence scans;
  keep Callable Tail/ABI/Completion outside the common physicalizer

H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0
  package program + local materialization + located Loop are co-sealed and
  consumed in one bounded V2 route; same-slice selected legacy-edge deletion;
  this row is the first-production-cutover milestone

H2-S2-S1-R1-REOPEN-AUDIT
  only after the bounded Dynamic dependency cutover; apply the existing
  parser-only product WIP and prove predecessor/gate parity

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

HAKO-CALLABLE-RESULT-ISSUER-CUTOVER-I0
  consume the H3-sealed normalized row after H5 parity, activate the Hako
  producer, and retire the Rust frontend producer with fallback/retry zero
```

Any earlier disconnected Rust-only receipt work remains non-authoritative. The
active bootstrap row must use the current final-source identity/batch path and
must not rebase an AST-rescan or frontend-specific result receipt.

H2/H3 are the selfhost parser-carrier implementation, not a prerequisite for
the Rust-fronted bootstrap cutover. The first H2 fixture may be
`length(): i64 { return 0 }`; it cannot claim `skip_while/4` parity because
locals, loops, conditionals, calls, and multiple returns require the later
complete body cohort.

## Dynamic loop unification parked lane

This Hako card only records the route; the canonical task text is the parked
section in `dynamic-fault-exit-transaction-d0-design-task-2026-08-10.md`, and
the global order is owned by `loop-common-physical-demand-and-session-ssot.md`:

```text
LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
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
and callable profile-close remain outside. Fixed-role topology retirement waits
for a caller-zero census after the segment route is proven. No V2-to-V1 adapter,
synthetic `ItemKey`, name/order repair, second JoinSig/Recipe, physical planner,
or Hako authority is allowed. This card does not schedule the parked lane.

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
