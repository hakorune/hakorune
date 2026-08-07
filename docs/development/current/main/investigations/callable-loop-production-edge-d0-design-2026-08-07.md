# Callable Loop Production Edge D0

Status: closed as `NoSafeSlice` after the production-edge census (2026-08-08).

Decision: do not promote the cfg(test)-only callable canary directly into
production. The census found no production caller that can supply the new
PreparedCallable -> profile-close -> Completion -> DraftSeal contract. This
row therefore closes as `NoSafeSlice`; the next row is a design-only
production-admission boundary. No selector, Generic G0, legacy scheduler,
retry, fallback, or production activation is opened.

## Sole source authority

```text
resolved callable source
  -> VerifiedCallableFunctionLoweringInputV1
  -> PreparedCallableLoopPhysicalizationV1
  -> profile-close evidence
  -> finish_profile_close
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

The existing callable chain is the only physical finish authority. The
cfg(test) canary is evidence, not a production input. The common physicalizer
remains unaware of callable Tail, ABI, Completion, Return, DraftSeal, and
production caller names.

## Non-authority and fail-fast boundary

Reject before any production switch when any of these is missing or foreign:

```text
exact callable owner/session/profile/ABI/Completion
one fresh unpublished function session
one profile-close receipt
one DraftSeal result
named production caller
```

Forbidden:

```text
AST or name re-walk
route-label or legacy-scheduler selection
Generic G0 adapter substitution
call-time discovery
retry/fallback
collector/module publication in this D0
```

## Required D0 evidence

1. Census one exact production caller candidate and its current old edge.
2. Record the source input, output receipt, owner/session boundary, and
   failure/discard behavior for that caller.
3. Define a thin named-caller adapter contract with no implementation.
4. Name the one old edge that a later I0/R0 will replace.
5. List parity and fresh-session fixtures required before opening the switch.

## Census result

The new callable physical products are not production inputs:

```text
src/mir/compiler/loop_physical_prepare.rs
  -> file-level cfg(test)
src/mir/compiler/callable_loop_physical_canary.rs
  -> file-level cfg(test)
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/callable_canary.rs
  -> cfg(test) consumer
```

Their consumers are tests only. They have no production caller capable of
providing `PreparedCallableLoopPhysicalizationV1`, a fresh canonical function
session, a profile-close receipt, a typed Completion handoff, and one
`CompletedFunctionDraftV1` output.

The closest old production edge is recorded for the next design row, but is
not a safe candidate for I0:

```text
RawInvocationChildPortV1::lower_loop
  src/mir/builder/recursive_child_lowering.rs:698-710
  -> PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1
     src/mir/builder/raw_loop_child_entry.rs:80-114
  -> lower_loop_or_freeze_v1
     src/mir/builder/control_flow/joinir/routing.rs:269-305
  -> MirBuilder::try_cf_loop_joinir / route_loop
     src/mir/builder/control_flow/joinir/routing.rs:552
```

The named production host is
`NormalCallableSemanticLoanPortV1::lower_normal_top_level_function`
(`src/mir/builder/normal_callable_semantic_loan_port.rs:246-278`). Its outer
failure/output contract is also legacy-owned:

```text
success:
  LegacyFunctionPendingSessionV1
  -> commit_legacy_symbol_pending
     (DraftPublicationPolicyV1::LegacyReplaceWholePair)
  -> root collector/module drain

failure:
  ModuleLoweringPortChildErrorV1::Session
  -> pending Drop / abort_and_restore
  -> caller context restored
```

This is a named production *host* only. It is not a safe callable-loop
candidate: the loop callsite supplies only the pre-effect schedule and raw
condition/body, then returns a loop `ValueId`/error. It never supplies the
new callable Prelude/Tail/ABI/Completion receipts or a `DraftSeal` output.

This edge only consumes a pre-effect `CallableSemanticLoopBindingSchedule`
and returns a `ValueId`/error through raw function lowering. It has no
callable Prelude/Tail/ABI/Completion receipt, no fresh function DraftSeal
owner, and no whole-function discard output. The existing DirectAccum and
NestedPredicate resolved cutovers are also not candidates: they are profile
specific old physicalizers, not consumers of the new callable physical
product.

Therefore no thin named-caller adapter can be honestly issued in this row.
Inventing one by route name, compatibility label, or test-only constructor is
forbidden.

## Later implementation order

```text
D0: production caller census + exact old-edge disposition = NoSafeSlice
D0: design one production callable admission/physicalization ingress
    with an explicit function-session/discard owner
I0/R0: one named caller switch only after that ingress exists; remove the
       selected old edge in the same slice
M10b: production activation only after Generic G0/all required route gates
M11/M12: legacy scheduler/fallback deletion after activation evidence
```

## Documentation requirement

This design row updates the callable physical-demand/session SSOT and current
mirrors only. Any later caller switch must update the exact reference
documentation, diagnostics, migration note, and current pointers in the same
commit as the implementation.
