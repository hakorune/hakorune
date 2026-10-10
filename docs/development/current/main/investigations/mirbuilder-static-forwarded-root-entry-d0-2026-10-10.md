# MIRBUILDER-STATIC-FORWARDED-ROOT-ENTRY-D0

Status: superseded by scalar-formal D0 after original-source counterexample
Date: 2026-10-10
Scope: unchanged `SizeClassBox.good_size_usize/1` direct Static Return
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-call-root-entry-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_static_selected_actual.rs
  - lang/src/hako_alloc/memory/size_class_box.hako

## Current production frontier

The preceding root Static packet S0 landed at `dd1ab9c2cb`. With the
unchanged SizeClassBox source and protected Eq WIP in a separate diagnostic
checkout, the published-view first refusal moved from owner slot 9
`bin_size_usize/1 Body(0)` to slot 8 `good_size_usize/1 Body(0)`, both named
`ordinary-new/local-commit/call-entry-missing`. This is progress in the
selected source, not a whole-source PASS. The exact diagnostic environment
and S0 validation are recorded in the preceding card.

`good_size_usize(size)` returns `me.good_size(size)`. The unchanged policy
fixture has four literal calls to `good_size_usize`. Its sole syntactic
outgoing call forwards the wrapper's formal; `good_size` then forwards its
formal to `size_to_bin`. Existing package evidence finds the original
CurrentOwner Static source and `static_arguments` fact for the latter edge,
and `checked_static_input` proves the known forward chain. This is only
structural checked-use evidence. The separate `candidate_integer_agreement`
comes from classifying the sole incoming inventory across all callers,
including vetoes, and is required for this bounded executable admission.
Neither condition by itself grants physical payload or a packet.

## Read-only audit and Decision

The existing `finish_static_one_input_actuals_v1` enumerates executable
`source.incoming` and special `target_static` rows. The wrapper's
source-only outgoing call is outside both. `checked_completed_static_one_actuals_v1`
requires `ExecutableStaticOne`, while that edge remains `SourceStatic`.
The scalar finisher requires a MulOperand definition, and cannot lend its
authority to this forwarding edge. The prior S0 correctly refuses `Plain`.
The existing `issue_original_static_forwarded_actual_v1` corroborates the
same original source, formal, argument site, ordinal and staged
`SourceStatic` candidate; it does not independently classify every caller.
`candidate_integer_agreement` supplies that separate all-caller check.

The prior card incorrectly demanded the implemented ledger's final phase
before construction. RULES.md §4 requires authority, meaning/failure
boundary, production caller/old responsibility and focused acceptance;
green or caller-zero after implementation is not a construction prerequisite.
The S0 positive must observe the exact four fixture calls in the current
package inventory and the completed outgoing phase before closeout.

```text
Decision: complete one bounded source-only forwarded Static actual
  from the original one-formal, one-call CurrentOwner source, then route its
  existing Static packet/root entry. This expands executable eligibility,
  not source meaning. Require the existing original incoming projection and
  candidate_integer_agreement for the caller formal, checked_static_input
  for both ends of the forwarding chain, and the complete original target
  cohort. Do not infer readiness from fixture spelling or one caller.
Source authority + canonical issuer: original StaticIncomingSourceV1,
  source_incoming inventory and static_arguments own identity;
  candidate_integer_agreement owns all-caller source class agreement;
  checked_static_input and issue_original_static_forwarded_actual_v1 own
  the forward proof; Completion, result contract/publication and Home ordered
  BorrowedActual corroborate it. Existing CallPacketSourceV1::static_i64
  and root Call emitter issue the physical packet and Invoke/NormalResult.
Non-authority: source-only phase by itself, terminal Call spelling, a Plain
  entry, one successful literal caller, freeze string, or rewritten .hako.
Fail-fast boundary: incomplete/mixed wrapper incoming cohort; changed
  caller formal, forwarding binding, target formal, ordinal or site; absent
  Completion/result/publication/Home observation; unselected SourceStatic.
Selected production caller and old responsibility: the unchanged
  SizeClassBox.good_size_usize/1 Body(0) direct Static Return is currently
  retained as a Home Call but falls to Plain root entry. Replace that
  incomplete local route with one completed actual and existing root packet.
  Structurally identical source-only forwarders may enter only when every
  same-authority condition and final verifier holds; audit each affected
  selected caller in S0.
Smallest next slice: one source-only forwarded actual finisher plus route
  selection for the exact one-input CurrentOwner cohort; reuse existing
  Static packet/root entry and independent final verifier.
Non-claims: no whole SizeClassBox PASS, no general source-only Static
  promotion, no Bool/physical Mul completion, no mimalloc-lite EXE PASS.
```

## S0 acceptance

1. Focused positive inspects the actual package ledger for all original
   wrapper incoming sites, including the four literal fixture calls, and the
   wrapper-to-`good_size` outgoing site: retained source, integer agreement,
   completed phase, Completion, target/formal binding and result publication.
   A missing or vetoed sibling must prevent the selected packet.
2. Confirm in code and test that the outgoing edge consumes that same
   incoming inventory without issuing a second source or result authority.
   If this needs a new meaning, return to D0 before widening the slice.
3. Focused positive pins executable forwarded actual, ordered
   BorrowedActual, one root Invoke/NormalResult after final MIR validation,
   and unchanged-source first-stop movement. Select negatives for one
   invalid wrapper actual and altered forwarding identity/ordinal; preserve
   refusal for an unselected SourceStatic edge. Reuse current family tests.

No source rewrite or general SourceStatic promotion is authorized. The
package-ledger census and unchanged-source first-stop are S0 verification,
not D0 prerequisites. The source authority and bounded failure conditions
are now fixed, so construction may start.

## Counterexample and handoff

The focused original-source package test built the unchanged SizeClassBox
plus four literal calls to `good_size_usize`. Asking the borrowed
`source_incoming` inventory to project the wrapper formal owner returned
`SourceIdentity` before any implementation. The wrapper formal is declared
`usize`; callable parameter issuance classifies declared exact trivials
separately from `OpaqueHandle`. The borrowed inventory registers owners
with borrowed formal drafts. Consequently the four literal calls cannot be
used as the forwarded-handle cohort proposed above. The previous Decision
is withdrawn; no executable code was changed under it. The remaining
source-only outgoing edge may use an exact scalar formal actual, which needs
its own source and packet audit in the successor D0 card. This failed
diagnostic is an expected design counterexample, not a green S0 test.
