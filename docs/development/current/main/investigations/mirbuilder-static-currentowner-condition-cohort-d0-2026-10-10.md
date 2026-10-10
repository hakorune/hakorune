# Static CurrentOwner condition cohort D0

Status: Home S0 landed; two-caller S1 target-scoped ingress mapping accepted
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-CONDITION-COHORT-D0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-closed-cohort-d0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-one-caller-s0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/README.md

## Selected frontier

The preceding S0 at `184422090e` closed the original one-caller
`size_to_bin -> normalize_size` local initializer. The unchanged mimalloc-lite
EXE still stops at Heap-to-Page `source-only-object-actuals`; that app stop is
not evidence that this Static upstream chain is complete.

`SizeClassBox.size_to_bin` has two selected original callers: the
`good_size -> size_to_bin` Body(0) local initializer and the `accepts ->
size_to_bin` Body(0) Eq condition Lhs. Home observes both after `2ba998a6b3`.
The original `accepts` condition is `me.size_to_bin(size) == me.huge_bin()`
in `lang/src/hako_alloc/memory/size_class_box.hako`. The current one-input
finisher deliberately rejects a cohort whose size is not one. Neither the
initializer nor the condition may be promoted alone.

## Read-only authority audit

The original Static source authority remains
`QualifiedStaticCallClaimIndexV1` and its retained CurrentOwner incoming
inventory. `qualified_static_call_claim.rs` issues the source from the
verified whole-source target/result and checks the exact source site.
`home_new_prefix_scalar_expression.rs` has a narrow one-argument
CurrentOwner `>= 0` arm and, after Home S0, a bounded pair for the original
Eq condition. `home_new_prefix_branch.rs` stages the Eq child actual. The physical route is still the
existing `CallPacketSourceV1::Static` with the selected Static packet owner.
This audit was read-only; no Cargo or shared-worktree edits were delegated.
The Eq has **two** call children: arity-one `size_to_bin` and zero-argument
`huge_bin`. The composed-expression Home owner emits observations only after
the whole expression preflight. The existing `>= 0` special case bypasses
that composed path, so widening it to any CurrentOwner call would lose the
Eq sibling and its failure ordering. The static home-neutral syntax scan is
not itself a Home or packet issuer.

The physical `dynamic_operator_contract` currently issues no
`Equal(NormalInteger, NormalInteger)` envelope. Generic Home scalar preflight
can type an integer Eq, but that does not authorize physical comparison.
The callee's loop/header/body/tail calls use the existing Static I64 V2
source/Recipe/Join path. The real-source collector test proves its standalone
physical draft, not the final module link or EXE. Neither fact grants the
two-caller transport owner.

## Integrated Decision and slice order

Source authority + canonical issuer: use the original CurrentOwner source
`Rc` and selected two-row incoming cohort. The existing
`observe_scalar_expression` composed-expression owner is the sole Home
issuer. Select a bounded source-only Home S0 for an exact If Eq whose Lhs is
a one-input CurrentOwner Static I64 call and whose RHS is a zero-input
CurrentOwner Static I64 call. Require the resolved If-region and exact binary
child sites, both original Static call claims, and the already staged
original actual. Check the whole eager Eq shape and both child claims before
projecting the Lhs `CurrentOwnerStaticSourceArguments`; return two
`LocalCallObservationV1` rows in Lhs/Rhs source order only after both are
available. The projection reads the existing pending-source port. The
branch's prior `Observe` remains source staging, not Home publication.

Non-authority: an Eq AST shape, `SourceStatic` actual, selected-site mask,
`good_size` Home alone, and the one-caller S0 token grant no executable
entry. Do not infer target/class from a method name or one caller.

Fail-fast boundary: wrong child site or operand side, a missing RHS claim,
nested-call argument, foreign receiver, short-circuit expression, and
source-argument mismatch yield no partial Home observation. No fallback.
Home S0 does not grant a physical Static packet or integer Eq execution.

Smallest next slice: Home S0 changes only composed Eq source observation.
The selected caller is the unchanged `accepts` condition. Its old source-only
Home responsibility is replaced by the two ordered observations; other
CurrentOwner condition forms keep their current boundary. Extend the
existing Home scalar owner, splitting its 716-line source by responsibility
before it reaches 800 lines.

Home S0 acceptance: the unchanged `accepts` source yields exactly the Lhs
one-input and RHS zero-input Home observations in source order, with original
argument identity and both Static claim sites. Changing either child site,
removing RHS source, or replacing Eq with a short-circuit expression yields
no partial Home observation. Existing zero-input and `>= 0` condition
families remain green. No physical JSON, EXE, or app first-stop advance is
claimed for this source-only S0.

After Home S0, close the whole two-caller `size_to_bin` source/actual cohort
before publishing physical Eq. `ordinary_new_borrowed_static_one_input_finish`
still requires `cohort.len() == 1`; the original `good_size` initializer and
`accepts` Eq Lhs together have two incoming identities. Both must agree on
the original actuals, callee signature/result and existing Completion.
`good_size` cannot be promoted on its own.

The read-only physical-owner audit fixes the remaining authority split:
`QualifiedStaticCallClaimIndexV1` owns original Static source identity;
`CallPacketSourceV1::static_i64` and `LexicalCallProjectionV1` own executable
call packet/actual projection; `dynamic_operator_contract` alone issues the
integer Eq execution envelope. Its current issuer has no
`Equal(NormalInteger, NormalInteger)` arm. `physical_program_json` can spell
ordinary `compare/eq`, but JSON spelling is not an execution proof. The
existing `Equal(Dynamic, Null)` envelope cannot substitute for integer Eq.

The preceding assertion that callee Completion must first be implemented was
wrong. `raw_loop_child_entry/static_i64_v2.rs` already joins Completion with
the loop V2 product, and `normal_callable_semantic_loan_port/static_i64_entry.rs`
has a real-source positive/negative physical collector test. The direct
missing boundary is `seed_static_transport_owners_v1`: it requires a single
CurrentOwner initializer, while `borrowed_formal_incoming.rs` marks the Eq
caller as unsupported static context. The later one-input finisher also
requires a one-row cohort. Source seeding precedes Home: a broad seed followed
by Home-skip would already alter transport owners and the incoming graph.
Reuse the Home Eq source selector as a read-only pair query for exact
pre-Home eligibility, then require both Home rows in the later finisher.
`issue_original_static_forwarded_actual_v1` already
checks each member of a two-row source cohort; do not mint another receipt.

Decision: select the bounded two-caller source/actual closure S1 in the
related execution card. Source authority is the original
`QualifiedStaticCallClaimIndexV1` and whole incoming inventory; the canonical
actual finisher reuses the existing signature/result/Completion and Home rows.
The production replacement is both original `good_size` and `accepts`
`SourceStatic`-only outgoing call responsibilities, never either caller alone.
Wrong/missing/duplicate original caller, site, target, class, Home argument,
signature or Completion rejects the whole cohort. An unrelated noninitializer
caller remains vetoed. This S1 does not create a new Static claim, source
receipt, physical Eq envelope, or final module-link claim.

After S1, issue the integer Eq envelope through the sole operator issuer. Physical lowering
must then show Lhs Invoke Normal -> RHS Invoke Normal -> Compare/Branch, with
either child's Fault bypassing later steps and no `borrowed_null_compare`
route. Only the fully linked chain can count as physical/EXE acceptance.
`bin_size` condition calls and Heap-to-Page outgoing transport remain
separate.

Home S0 landed at `2ba998a6b3` with both ordered `accepts` Home rows and
focused positive/negative acceptance. It did not issue a physical Eq
envelope or executable two-caller entry. Read-only loop/physical audit
resolved the next replacement mapping without new source authority. A
global-seed S1 attempt then activated unrelated Heap/Page actuals and failed
at `ordinary-new/borrowed-actual/entry-source`. That implementation was
removed. The related S1 card now owns the target-scoped source/actual/entry
Decision and acceptance. Construction may proceed on that mapping; Home green
alone does not grant execution permission.

Non-claims: this D0 does not authorize physical Eq, two-caller execution,
caller omission, generic reachability pruning, or whole-app completion.
