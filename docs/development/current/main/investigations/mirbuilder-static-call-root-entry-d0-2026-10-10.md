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

## S0 closeout (2026-10-10)

The root Static packet now joins the exact Home Call terminal and Completion
exit to the original incoming source, completed inputs, selected result
publication, and ordered Home observation. The publication take uses the
original Call expression site after corroborating the Return statement's
caller lineage. The packet emits one root `Invoke`/`NormalResult`; the root
take moves only that exact site out of local binding-group accounting. A
terminal-shaped Call that is actually lowered locally keeps its local route.
Final Call visitation reuses the root packet validator for Static and Lexical
argument sources. `Plain` still cannot satisfy a selected Call terminal.

Evidence on the selected source revision before closeout:

- `cargo check --profile quick -p nyash-rust`: PASS.
- Focused final-MIR root Static positive: 1/1 PASS, one I64 Call Invoke and
  one NormalResult after independent final validation. This fixture is
  zero-input to isolate root transport; it is not a scalar whole-app claim.
- Original scalar incoming/terminal package positives: 2/2 PASS. Existing
  Static packet family: 9/9 PASS, including missing/changed actual and
  completion negatives. Lexical return: 2/2 PASS. Final Call visitor:
  4/4 PASS. Existing forwarded-I64 physical packet: 1/1 PASS.
- Unchanged `size_class_box.hako` with protected Eq WIP, diagnostic checkout:
  still red. Before S0 the first refusal was `call-entry-missing` at
  `SizeClassBox.bin_size_usize/1`, owner slot 9, `Body(0)`. After S0 the
  first refusal is the same named contract at owner slot 8, `Body(0)`, the
  `good_size_usize/1` wrapper. This proves first-stop movement, not passage
  of every later owner or whole-source publication. A temporary diagnostic
  also found and fixed an intermediate local binding overflow at
  `good_size/1` (`Body(1).IfCondition.Rhs` followed by `Body(2).Value`).
  The diagnostic overlay was removed; the three protected Eq files retained
  identical hashes and are the only remaining diagnostic checkout changes.
- A scalar-only physical fixture with a Mul callee reaches
  `borrowed-mul/operand-copy-missing`. That failure belongs to the later Mul
  physical-copy frontier and is not counted as a root-entry PASS.

S0 closes at the root Static packet and final MIR boundary. The next selected
decision must identify why the original `good_size_usize/1` direct Static
Return has a retained terminal Call but a `Plain` root entry, and whether its
callee's whole incoming/actual/result cohort can select the existing packet.
No source edit, Plain promotion, or fallback is authorized by this closeout.
