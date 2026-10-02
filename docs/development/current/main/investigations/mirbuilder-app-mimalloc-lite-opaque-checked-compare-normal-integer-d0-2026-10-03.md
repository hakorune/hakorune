# mimalloc-lite opaque checked-compare Normal-Integer D0

Status: accepted Decision; S0 bounded to the checked-compare operand view only.
Scope: `MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-NORMAL-INTEGER-D0`.
Related: docs/development/RULES.md; CURRENT_STATE.toml;
  mirbuilder-app-mimalloc-opaque-formal-ingress-d0-2026-10-01.md (parent,
  "Remaining task 4" selection rule); mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md.

## Decision

A checked-compare lends a Normal-Integer *operand view* to a borrowed tagged
formal for the compared use and its dominated successors. The tagged carrier
(`borrowed_kind_payload_v1`) is retained end to end; the view is a site- and
binding-branded row in the borrowed-formal use ledger, not a `StoredLocal`
class change and not a new Receipt/Seal. Compare is the first admitted
operand-view use; the full `allocate` admission stays deferred to the task-4
D0 (ArraySet -> ordered Add -> constructor) that reuses this view.

Source authority + canonical issuer:
- `expression_source.rs` keeps the `ResolvedBinaryOperatorV1::Greater` row
  identity and site; lowering may re-express but source evaluation/Fault
  order is preserved.
- `callable_parameter_contract/issuer.rs` stays the formal/binding/ordinal
  authority; unannotated `requested_size` remains `OpaqueHandle`.
- `dynamic_operator_contract` (existing operation owner) gains a
  `Greater(NormalInteger, NormalInteger)` view envelope. It currently holds
  only `Add`/`Less` result envelopes with no operand refinement; this D0 adds
  the refinement *view*, issued through the existing borrowed-formal package
  co-seal (`draft_borrowed_formal_uses_v1` + the use ledger), which becomes
  its production consumer. No new authority is minted.
- Rhs Integer proof: existing `local_read_field` Scalar admission (usize
  satisfies `is_numeric_integer_type_name`) + ObjectFieldGet materialization;
  logical Integer is lent by the checked compare, never derived from the
  `usize`/U64Bits storage spelling.

Non-authority: `DynamicOperatorValueClassV1::I64` is a lane name, not the
logical-Integer proof; nonzero payload bits, caller literals, MirType and a
passing ignored-formal test do not classify. `dynamic_operator_contract` has
no production consumer today — extending the model alone is not acceptance.

## Pinned census (read-only worker, integrated)

Site: `lang/src/hako_alloc/memory/page_heap_box.hako:75`
`if requested_size > me.block_size { return null }` inside
`allocate(requested_size)` (L74, unannotated formal -> OpaqueHandle).
`me.block_size: usize` declared L37, written in `birth` L53. The compare
result is consumed only by `if`; never bound, copied or returned.

`requested_size` has four consumers in `allocate`: `>` operand (L75),
`ArrayBox.set` argument (L93), `+` operand (L96), `new HakoAllocHandle`
constructor argument (L102). Same-morphology siblings: `resizeInPlace`
L150/L154 (`<= 0`, `> me.block_size`), `realloc` L265, `reallocResult` L305.
The app reaches `allocate` through `HakoAllocHeap.allocate(size)` (unannotated
formal, forwarded one level) — ingress forwarding already works via the
tagged lanes landed in the ingress S0.

Empirical frontier (temporary probe, since removed): a minimal
`if requested > me.limit` box is declined before compare —
`check` is silently excluded (`BorrowedFormalUseDraftErrorV1::UnsupportedUse`),
the caller's `new Counter()` then fails at
`[freeze:contract][ordinary-new/local-commit/artifact-source-unavailable]
TerminalHomesUnavailable`. The decline propagates exactly as census predicted;
no raw-I64 fallback path is taken.

## Contract and fail-closed boundary

Envelope: `Greater` family, operand class `NormalInteger`, result
`TrivialBool`, fault the existing single variant
`TypeErrorBeforeResultNoOperandMutationNoRebind`. Checked-compare semantics:
kind != 1 lhs or a non-numeric-Integer rhs faults before result, with no
operand mutation or rebind. A negative kind-1 payload (e.g. -1) compares as
signed and continues the Normal path; the range Fault stays late, at the
usize/range-checked write — range authority remains at the original write
(`verification/numeric_substrate.rs`), never at parameter ingress or compare.

View lending discipline (parent rule 4):
- lent only to the same binding/ValueId that the compare consumed;
- scope = the region dominated by the compare's Normal edge, on both
  true/false successors (`join_branch` identical-class merge is the merge
  precedent; `compute_dominators`/`DominatorTree` the physical substrate);
- reject pre-check uses, Fault-edge uses, foreign-owner values and rebound
  loans — all remain `UnsupportedUse`/`forbidden-operand`.

Physical gates the S0 must clear, all named and fail-closed:
1. `draft_borrowed_formal_uses_v1`: admit a `CompareOperand` use kind only
   under a sealed view row; everything else stays `UnsupportedUse`.
2. `borrowed_call_uses`: whitelist `Compare` only when the instruction carries
   a proved view row; `forbidden-operand` otherwise.
3. `map_value_domains`: viewed operands count as `D::I64` for that Compare
   only (`Op::I64Compare`); the carrier and all other uses are untouched.
4. `invoke.rs` `object-field-read-definition-invalid` currently requires
   declared `i64`, contradicting the upstream usize Scalar admission — S0
   extends that gate explicitly under the view for numeric-integer declared
   fields; no silent bypass.
5. Published predicates stay signed (`sgt`); no unsigned lane is added.
   Signedness safety: the view requires the rhs field's selected writers to
   pass the checked usize write lane (negative i64 rejected), bounding stored
   usize to [0, i64::MAX] < 2^63; a field with an unproved writer declines.

## S0 acceptance boundary

- Focused positives: opaque formal `>` against a usize field read and against
  an Integer literal, both compare outcomes published and executed through
  the generated JSON path; -1 payload takes the signed Normal path.
- Negatives: kind != 1 actual -> checked Fault before result; non-numeric
  rhs -> decline/fault; use before the compare -> `UnsupportedUse`;
  Fault-edge, foreign-owner and rebound uses -> reject.
- The pinned app site is census evidence only: `allocate` still declines
  until the task-4 D0 admits `.set`/`+`/`new` uses via this same view.
- No retirement in this slice; additive view only, no caller-zero owed.

## Non-claims

No full `allocate`/`resizeInPlace`/`realloc` admission; no `LessEqual`/
`GreaterEqual`/`Equal` sibling envelopes (same-view extensions later);
no unsigned compare lane; no global S6C/static-I64 reinterpretation;
no `StoredLocal`/parameter-contract semantic change; no app EXE PASS or
MirBuilder completion claim. usize field reads under the view are scoped to
the compared operand — general usize ObjectFieldGet publication is not
claimed. Cohort immutable-index optimization remains separate BoxShape.

Next: MIRBUILDER-APP-MIMALLOC-LITE-OPAQUE-CHECKED-COMPARE-NORMAL-INTEGER-S0
(the bounded compare-operand view construction and acceptance above).
