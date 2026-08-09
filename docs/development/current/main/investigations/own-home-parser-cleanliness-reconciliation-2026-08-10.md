# OWN-HOME parser cleanliness reconciliation

Status: accepted parked task map; active JoinSig lane unchanged
Date: 2026-08-10
Source: external review reconciled against current Rust/Hako parser code

## Decision

The feedback is accepted with one clarification: the accepted language target
already defines `HTRIVIA` as horizontal whitespace or a comment containing no
line terminator, while the live bounded Release I0 reference deliberately
narrows that row to space/tab `HSPACE`. This is not silent drift, but it is an
unfinished parity boundary that must close before Take or Share activates.

The parser cleanup is parked behind the active Dynamic JoinSig lane. It does
not authorize edits to the current user-owned Hako parser worktree and does not
change ownership semantics.

## Confirmed findings

1. Hako `release` skips only space/tab; it has no same-line comment helper.
2. `with_direct_method_syntax` reconstructs the same Box/method association in
   `collect_body_rows` and `collect_syntax_lease`, then derives Release from the
   latter.
3. its callback already carries four positional products and would grow for
   Share/other body events.
4. the disconnected Hako parameter substrate stores `"Ordinary"` in a raw
   String and exposes a builder instance through `sealed_token()`.
5. the shared grammar corpus has only Release positive/projected/receiver rows;
   it does not cover ordinary call/binding/method/newline/comment cases.
6. nested Release currently reports only its containing top-level ordinal.
7. Dynamic local borrowing re-finds sealed roles; safe now, but a private slot
   index can remove that lookup later without adding a standalone ch product.

## Ordered parked tasks

```text
1. HOME-CONTEXTUAL-HTRIVIA-PARITY-R0
   accepted target HTRIVIA -> Rust/Hako same-line helper/parity corpus

2. PARSER-DIRECT-METHOD-OBSERVATION-RECUT-R0
   one parser-private traversal -> parts transport -> existing products

3. HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0
   decide closed Ordinary/Take issuer and parser-session/method co-seal

4. HAKO-PARAMETER-TRANSFER-TYPED-SEAL-R0
   only after D0; remove raw-string authority and consumer-visible builder token
   task: `hako-parameter-transfer-typed-seal-r0-task-2026-08-10.md`

5. OWN-HOME-TAKE-DECL-SYNTAX-I0
6. OWN-HOME-SHARE-REPRESENTATION-D0
7. OWN-HOME-SHARE-EXPR-SYNTAX-I0

P2 cleanup, not blockers:
  OWN-HOME-NESTED-RELEASE-SOURCE-PATH-P2
  LOOP-V2-DYNAMIC-LOCAL-SLOT-INDEX-P2
```

Share syntax never precedes its representation/destination Decision.

## Authority rules

```text
PreparedDirectMethodObservationBatchV1<'ast>:
  parser-private unpublished staging only
  one exact traversal
  not Clone/owned semantic authority
  never leaves the higher-ranked callback transaction

ParserDirectMethodObservationPartsV1<'ast>:
  transport bundle only
  handoff/body/syntax/release/share keep separate owners
  consuming destructure once in the callback

parameter transfer:
  closed typed syntax issued canonically
  raw "Take"/"Ordinary" text is not semantic evidence
  builder identity is not parser provenance
```

## Global hard stops

```text
no global take/share/release keyword
no generic skip_ws across line terminators
no post-transaction AST rescan
no public semantic batch/mega-product
no callback positional-product growth
no raw string -> Take classification downstream
no builder instance -> parser brand
no default/fallback ownership event
no source fixture narrowing
```
