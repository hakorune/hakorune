# MIRBUILDER-EXE-ACCEPTANCE-MERGE-PHI-DOMINANCE-D20
## Census: Loop-Carried Merge Without Phi — StringHelpers Function Family (D20)

Date: 2026-09-26
Status: in census
Family: callable / gate1 / Gate 1 unified lane / SSA dominance /
  EXE acceptance
Row reference: workstream row H (Gate 1 unified selfhost lane)
Blocking observation: after S6+S7 document publication landed,
`--emit-mir-json` on `apps/json-stream-aggregator/main.hako` now
fails strict verification with 13 SSA dominance violations — every
one inside `lang/src/shared/common/string_helpers.hako` functions:

```text
StringHelpers.index_of/3      %32,%33  def bb75  used bb71   (def AFTER use — backedge)
StringHelpers.int_to_str/1    %49      def bb88  used bb90
StringHelpers.json_quote/1    %29      def bb148 used bb150
StringHelpers.last_index_of/2 %62,%63  def bb184 used bb180  (def AFTER use — backedge)
StringHelpers.read_digits/2   %16      def bb200 used bb193  MergeUsesPredecessorValue
StringHelpers.skip_ws/2       %24      def bb211 used bb208,bb210 MergeUsesPredecessorValue
StringHelpers.to_i64/1        %163     def bb285 used bb276  MergeUsesPredecessorValue
```

## Boundary covered by this census

起点: LoopCond-family loop lowering emitting loop-carried bindings
(header/join merge convention); 終点: strict verifier dominance +
merge-Phi checks (`dom::check_dominance_with_policy`,
`cfg::check_merge_uses_with_policy`).
includes: which lane lowered each violating function's `loop(cond)`,
the merge convention it emitted (Phi vs edge-copy/predecessor value),
and which authority owns the merge convention contract.
excludes: call-edge domain policy (landed S7), artifact/document
completion (landed S6), VM-lane behavior, object/instance admission.

## Measured shape

- All 7 violating functions contain `loop(cond)` (LoopCond family)
  over local string-scan state (`i`, `j`, `pos`, `acc`, `out`).
- Two violation kinds, both PHI-shaped:
  - `DominatorViolation` with `def_block > use_block` — a value
    produced on the loop backedge is read at an earlier join/header
    block: the classic missing loop-carried Phi signature.
  - `MergeUsesPredecessorValue` — a merge block reads a value defined
    in exactly one predecessor without a Phi.
- Loop-carried bindings in these functions (`i`, `pos`, `acc`, `out`,
  `last`) are precisely the values named in the violations.

## The authority question

Is the non-Phi merge an **accepted MIR dialect** the document must
serialize as-is, or a **missing-Phi lowering defect**?

Evidence so far:

- `NYASH_MIR_NO_PHI` is opt-in; the default dialect is PHI-ON and the
  strict verifier requires Phi at merges (verification_flags.rs).
- Edge-copy is a real dialect
  (`builder_emit.rs:193` guards non-dominating `Copy` emission under
  strict/dev+planner_required), and `verify_allow_no_phi()` +
  `skip_phi_checks` exist to tolerate it — but only under an explicit
  non-strict + env policy, never by default.
- The module built for this app mixes conventions: other functions
  with `loop(...)` in `main.hako` itself pass the same checks, so at
  least one armed lane DOES insert loop-carried Phis correctly.
- Whether `StringHelpers` functions lowered through a different lane
  (different box membership, `me.` receiver, static-method body
  shape) or the same lane under a different source shape is the open
  question — the merge convention is decided inside lowering, not by
  verification.

## Census questions to close

1. Which route arm lowered each of the 7 functions' `loop(cond)` —
   same armed LoopCond lane as `main.hako`'s loops, or a sibling?
2. Does the emitted merge carry an edge-copy convention the module
   flags as no-phi, or is the header/join Phi simply absent?
3. Who owns the Phi/edge-copy decision for LoopCond joins — the
   normalizer's join emission, or a separate recipe contract?
4. Prior green evidence: did any acceptance gate previously compile
   these `StringHelpers` functions through the strict verifier, or
   is this the first strict traversal of this box?

## Options (to be weighed after census)

- (A) Missing-Phi defect in one lane's join emission — fix at the
  emission owner; no verifier change.
- (B) The lane legitimately emits edge-copy convention; document
  verification must consult the module's declared merge convention —
  requires an explicit convention record, not an env toggle.
- (C) Mixed: header Phis emitted for some bindings but not for
  values joining at loop exit/early-return merges — a partial
  coverage defect.

## Non-claims

- No claim this is a verifier defect: the strict checks implement
  the PHI-ON default contract correctly.
- No claim the fix is "insert Phis": if the lane's contract is
  edge-copy, the defect is the missing convention declaration, and
  silently adding Phis would mask it.
- No weakening of strict verification as the fix.
- VM `ingest/1` ledger-less spine; Gates 2–4; overall completion.
