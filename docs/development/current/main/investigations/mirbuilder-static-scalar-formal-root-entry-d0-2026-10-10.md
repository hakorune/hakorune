# MIRBUILDER-STATIC-SCALAR-FORMAL-ROOT-ENTRY-D0

Status: D0 accepted; selected MIRBUILDER-STATIC-SCALAR-FORMAL-ROOT-ENTRY-S0
Date: 2026-10-10
Scope: unchanged `SizeClassBox.good_size_usize/1` direct Static Return
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-forwarded-root-entry-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/ordinary_new_borrowed_static_scalar_input_finish.rs
  - src/mir/callable_parameter_contract/issuer.rs
  - lang/src/hako_alloc/memory/size_class_box.hako

## Observed counterexample

The preceding D0 assumed that the wrapper's four literal callers formed a
borrowed-formal incoming cohort. A focused package test on the original
SizeClassBox plus those four callers failed at
`source_incoming.project(wrapper_formal_owner)` with `SourceIdentity`.
`good_size_usize(size: usize)` has a declared exact trivial formal; the
borrowed incoming inventory does not own its caller cohort. The source
spelling establishes four calls but cannot supply a borrowed-handle
forwarding receipt. No executable implementation was made under that
incorrect Decision. The corrected original-source package test now passes;
it is package evidence, not physical publication evidence.

The outgoing `me.good_size(size)` target formal is opaque. Existing
`SourceStatic` preparation recognizes `Scalar(binding, Integer)` as
integer evidence while spelling the target argument as Home
`BorrowedActual`. The existing static scalar finisher already joins
source, complete target cohort, scalar actual, signature, Completion and
result; its source-only owner selection is restricted to a MulOperand draft.
The target lacks that use. The read-only worker independently confirmed
the exact `usize` contract and this scalar path after the counterexample.

The original `good_size` result contract has no required-I64 input ordinal:
its result follows `size_to_bin` and `bin_size`, not a direct return of
argument 0. The scalar-forward entry therefore checks that result shape
separately from the declared `usize` input and its forwarding use. The
existing Mul entry keeps its required ordinal 0.

## Decision

```text
Decision: extend the existing static scalar finisher's source-only
  eligibility to the bounded exact-scalar-formal forwarding use. Require
  the original target cohort to project wholly and uniquely without veto;
  one CurrentOwner call, one opaque target formal at ordinal 0; the
  SourceStatic candidate must be Scalar(Integer) at the original argument
  site, rooted in the caller's declared ExactTrivial(USIZE) formal and
  matching its physical signature. Require the checked formal source/use
  chain for the opaque target plus original Completion, result/publication
  and Home ordered BorrowedActual. Reuse ExecutableStaticScalar and the
  existing Static packet/root entry. The borrowed Forwarded candidate is
  withdrawn.
Source authority + canonical issuer: callable parameter contract owns
  exact usize input; original StaticIncomingSource and source_incoming
  inventory own the target call cohort; SourceStatic actual and scalar
  formal-use evidence are joined by the existing package issuer.
  CallPacketSourceV1::static_i64 and root Call emitter remain physical owners.
Non-authority: literal-call count, borrowed source_incoming projection of
  the wrapper formal, checked_static_input on a nonborrowed formal, Plain
  root entry, .hako rewrite, or the freeze string.
Fail-fast boundary: unsupported/noninteger scalar actual, wrong caller
  formal or target binding, incomplete target cohort, absent Completion,
  result/publication or ordered Home observation; unselected SourceStatic
  must stay passive.
Selected production caller and old responsibility: the unchanged
  SizeClassBox.good_size_usize/1 Body(0) direct Static Return has a Home
  terminal Call but currently reaches Plain root entry. Replace that
  incomplete source-only scalar actual/route with one checked Static packet.
  Structurally identical exact-scalar formal forwarders enter only when the
  same checks hold; audit every selected caller affected by the change.
Smallest next slice: extend the existing scalar actual finisher and route
  for the exact-formal forwarding case, then use the existing Static packet
  and independent final verifier.
Non-claims: no whole SizeClassBox, Bool, Mul-copy or mimalloc-lite EXE PASS.
```

S0 acceptance: the corrected original-source package test must confirm the
exact `USIZE` formal, absence of a borrowed wrapper incoming owner, one
original wrapper-to-`good_size` Static source and its complete target
cohort. Add only the necessary positive to establish completed
`Scalar(Integer)` actual and ordered Home argument, one root
Invoke/NormalResult after final MIR validation, and unchanged-source
first-stop movement. A Bool/null or wrong binding/site/ordinal, missing
sibling target incoming, unchecked forward chain, or missing
Completion/result/publication must refuse. Preserve passive unselected
`SourceStatic`. Existing family tests cover much of this; add only
independent gaps. The test's green and production first-stop are S0
closeout evidence, not a construction prerequisite.

## S0 closeout evidence

- The focused original-source package test passes with four literal callers:
  it observes the exact `USIZE` formal, no borrowed incoming wrapper owner,
  one original wrapper-to-`good_size` target cohort, completed scalar actual,
  selected Static source and one packet actual.
- The existing scalar Mul test and all 11 `static_packet_entry::tests` pass,
  including scalar-actual drift, missing Completion and a non-`USIZE`
  declared-formal refusal. The negative case changes only the original
  wrapper's declared ABI to `i64`: the source package still issues, but
  route selection stays absent and packet lending refuses it.
- `current_state_pointer_guard.sh` and `git diff --check` pass.
- The unchanged `size_class_box.hako` published-view diagnostic used detached
  HEAD `33a4352f1a` plus the protected three-file Eq WIP and this scalar
  source diff (sha256 `7c1363a87a65635369ba77c16db2a6fbb3e3a3717a1caab862a7df4ec71fa1fb`).
  The same quick-profile test command,
  `cargo test --profile quick -p nyash-rust --lib imported_size_class_accepts_publishes_ordered_static_integer_eq`,
  stopped at `good_size_usize/1` slot 8 `Body(0)` without this scalar diff
  (build+link 5m22s, execution 0.01s). With this diff it selected the
  original `good_size` source, emitted and validated an I64 `Invoke` to
  `SizeClassBox.good_size/1` and matching `InvokeNormalResult` for the
  `good_size_usize/1` root, then stopped at `size_to_bin_usize/1` slot 6
  `Body(0)` (`selected=false`; incremental build+link 1m58s, execution
  0.02s). The test remains red; this is an exact first-stop change and
  root physical evidence, not whole-source publication.
- The non-`USIZE` refusal passed under quick profile (1/1; 9089 filtered).
  The rebuilt test binary's complete static-packet-entry family passed
  (11/11). No new guard or adapter was added.
- An attempt to lower the non-Main wrapper through the Main-oriented
  `lower_map_dependency_for_test` harness stopped at missing declaration
  catalog and then unavailable owner. That harness is not the production
  compiler route, so the temporary test was removed rather than counting
  the failure as a production refusal. The separate published-view probe
  used the production pipeline in the disposable worktree as described above.
