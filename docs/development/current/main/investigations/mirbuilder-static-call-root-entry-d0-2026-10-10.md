# MIRBUILDER-STATIC-CALL-ROOT-ENTRY-D0

Status: D0 accepted; selected MIRBUILDER-STATIC-CALL-ROOT-ENTRY-S0
Date: 2026-10-10
Scope: unchanged SizeClassBox published-view first stop after direct Static Call terminal retention
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-root-exit-source-missing-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/validation.rs
  - lang/src/hako_alloc/memory/size_class_box.hako

## Observed first stop

With the original `size_class_box.hako` and protected Eq WIP in a detached
diagnostic checkout, the published-view probe moved from
`root-exit-source-missing` to `ordinary-new/local-commit/call-entry-missing`
after the direct Static Call relation was retained. Log:
`/tmp/hako-static-call-terminal-integration.log`. This is still red, and the
whole source and mimalloc-lite EXE have not passed.

`validate_root_call_entry` returns this refusal when it has a Call source but
the recorded root exit entry is `Plain`, with a legacy terminal source and no
instance-call exception. That is a source/executable-entry mismatch, not
permission to infer an invoke from the terminal relation. The prior exact
missing relation was `SizeClassBox.bin_size_usize/1` at `[Body(0)]`; this new
refusal likely concerns its `me.bin_size(bin)` return, but the generic error
alone does not prove the current owner and exit.

## Decision

```text
Decision: issue one root Static Call entry from the existing original source
  and packet path, never by accepting a Plain entry. The terminal Return's
  exact owner/exit/value call site must match the original CurrentOwner
  StaticIncomingSourceV1. Borrow the complete finished input cohort and
  selected result publication, then use CallPacketSourceV1::static_i64 as
  the packet issuer. Reuse RootExitIngress::Call for the outer Invoke and
  NormalResult, and record that packet in root exit progress.
Source authority + canonical issuer: Home's TerminalRelationV1::Call and
  Completion/Home exit own the Return relation; StaticIncomingSourceV1 and
  its original incoming inventory own the call/target/actual identity;
  checked scalar cohort owns completed inputs; selected publication owns
  result handoff. CallPacketSourceV1::static_i64 owns the physical packet;
  root exit emitter owns Invoke/NormalResult and Normal/Fault cleanup.
  The existing final MIR verifier remains independent.
Required validation: a Static packet cannot reuse the Lexical terminal
  argument accessor. The exact Home LocalCallObservation carries ordered
  BorrowedActual arguments, whereas the terminal Call relation has no
  Lexical arguments. Add a Static corroboration arm in root packet/entry
  validation that checks the same original source Rc, owner/exit/call site,
  ordered actuals, publication, packet and recorded physical bindings.
  Preserve the current Lexical branch unchanged.
Non-authority: the freeze string, AST method name, terminal spelling alone,
  any one successful caller, local emitted ValueId, or a Plain entry.
Fail-fast boundary: missing/mixed sibling, noninteger actual, owner/site or
  ordinal drift, absent/wrong publication, missing/changed Invoke or
  NormalResult, or Fault cleanup bypass rejects. No fallback to generic
  Call or old direct-call loan.
Smallest next slice: MIRBUILDER-STATIC-CALL-ROOT-ENTRY-S0, join the exact
  source and packet for the selected direct Static return, emit one root
  Call entry and validate its Static ordered argument source. Use the
  original bin_size_usize/1 Body(0) as production frontier, confirming its
  owner/exit at the new refusal during implementation. Reuse existing
  lexical/packet tests, add only an independent Static positive/negative,
  and compare the unchanged published-view first stop before/after.
Non-claims: no whole SizeClassBox publication, Bool physical result, Eq
  physical completion or mimalloc-lite EXE PASS.
```

The read-only worker found the key trap: the existing Static packet is sound,
but the current root validator calls `lexical_arguments()` and would reject a
Static `BorrowedActual` packet. The direct-call loan is for App Main and its
selected children, not a general CurrentOwner Static terminal issuer. Keep
this as one root-entry responsibility with a Static-specific validation arm;
do not broaden the old direct-call loan or erase the Lexical distinction.

The previous S0 was closed at the retained source relation boundary; this
entry and whole-source publication remain separate responsibilities.
