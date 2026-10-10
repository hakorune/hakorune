# MIRBUILDER-STATIC-FORWARDED-ROOT-ENTRY-D0

Status: design stop; bounded source-only forwarded actual authority under audit
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
and `checked_static_input` proves the known forward chain. This source
census does not itself prove that every wrapper incoming actual is complete
or that the outgoing row has executable phase.

## Read-only audit and candidate Decision

The existing `finish_static_one_input_actuals_v1` enumerates executable
`source.incoming` and special `target_static` rows. The wrapper's
source-only outgoing call is outside both. `checked_completed_static_one_actuals_v1`
requires `ExecutableStaticOne`, while that edge remains `SourceStatic`.
The scalar finisher requires a MulOperand definition, and cannot lend its
authority to this forwarding edge. The prior S0 correctly refuses `Plain`.

```text
Decision candidate: complete one bounded source-only forwarded Static actual
  from the original one-formal, one-call CurrentOwner source, then route its
  existing Static packet/root entry. This expands executable eligibility,
  not source meaning. First verify the whole wrapper incoming cohort in the
  same ledger; do not infer readiness from four fixture spellings or one
  successful caller.
Source authority + canonical issuer: original StaticIncomingSourceV1,
  source_incoming inventory and static_arguments own identity; existing
  checked_static_input and issue_original_static_forwarded_actual_v1 own
  the forward proof; Completion, result contract/publication and Home ordered
  BorrowedActual corroborate it. Existing CallPacketSourceV1::static_i64
  and root Call emitter issue the physical packet and Invoke/NormalResult.
Non-authority: source-only phase by itself, terminal Call spelling, a Plain
  entry, one successful literal caller, freeze string, or rewritten .hako.
Fail-fast boundary: incomplete/mixed wrapper incoming cohort; changed
  caller formal, forwarding binding, target formal, ordinal or site; absent
  Completion/result/publication/Home observation; unselected SourceStatic.
Smallest next slice: after ledger census, one source-only forwarded actual
  finisher plus route selection for the exact one-input CurrentOwner cohort;
  reuse the existing Static packet/root entry and independent final verifier.
Non-claims: no whole SizeClassBox PASS, no general source-only Static
  promotion, no Bool/physical Mul completion, no mimalloc-lite EXE PASS.
```

## D0 acceptance before implementation

1. Inspect the actual package ledger for all four original wrapper incoming
   sites and the one wrapper-to-`good_size` outgoing site: retained source,
   phase, completion, target/formal binding and result publication. Record
   which proof is already available and the precise missing executable arm.
2. Confirm that the source-only outgoing edge can consume the existing
   complete incoming cohort without issuing a second source or result
   authority. If the join requires a new meaning, update this Decision
   before code.
3. Select a focused positive that pins executable forwarded actual, ordered
   BorrowedActual, one root Invoke/NormalResult after final MIR validation,
   and unchanged-source first-stop movement. Select negatives for one
   invalid wrapper actual and altered forwarding identity/ordinal; preserve
   refusal for an unselected SourceStatic edge. Reuse current family tests.

No implementation or source rewrite is authorized while these ledger facts
remain unconfirmed. The next D0 step is a read-only package census; once
the Decision is confirmed, select `MIRBUILDER-STATIC-FORWARDED-ROOT-ENTRY-S0`
as the bounded executable row.
