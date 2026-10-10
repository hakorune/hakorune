# Static CurrentOwner condition cohort D0

Status: selected design stop
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
size_to_bin` Body(0) Eq condition Lhs. Home observes only the initializer.
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
CurrentOwner `>= 0` arm; its generic scope and
`home_static_value_call.rs` admit only zero-argument CurrentOwner leaves.
`home_new_prefix_branch.rs` can stage the Eq child actual but does not issue
the corresponding `LocalCallObservationV1`. The physical route is still the
existing `CallPacketSourceV1::Static` with the selected Static packet owner.
This audit was read-only; no Cargo or shared-worktree edits were delegated.
The Eq has **two** call children: arity-one `size_to_bin` and zero-argument
`huge_bin`. The composed-expression Home owner emits observations only after
the whole expression preflight. The existing `>= 0` special case bypasses
that composed path, so widening it to any CurrentOwner call would lose the
Eq sibling and its failure ordering. The static home-neutral syntax scan is
not itself a Home or packet issuer.

## Decision to close before construction

Source authority + canonical issuer: use the original CurrentOwner source
`Rc` and selected two-row incoming cohort. Decide which existing Home
expression owner can issue a checked Eq-child `LocalCallObservationV1` for
the exact one-input call without deriving a second call claim. Then decide
how the post-signature finisher closes **both** rows against one selected
callee signature, result Completion, original ordered actuals, and physical
packet identity.

Non-authority: the existing source-only actual, an Eq AST shape, the
`good_size` Home observation alone, and a selected-site mask cannot grant
the condition call executable entry. The one-caller S0 token is not proof
for the two-caller cohort.

Fail-fast boundary: keep both rows source-only until the condition Home and
whole-cohort proof agree. Wrong site/operand side, non-I64 tag, duplicate or
missing caller, incomplete result/Completion, and a condition whose sibling
call changes effects must reject without a fallback or partial packet.

Smallest next design cell: freeze the composed Eq Home issuer, both ordered
call observations, each child's Fault edge, and the full two-row completion
rule in this card. Only then select an implementation S0. Keep
`bin_size` condition calls and Heap-to-Page outgoing transport separate.

Acceptance to define for S0: unchanged source must retain both original
rows; the Eq Lhs must receive a Home observation while an unsupported
condition context stays source-only; both calls must pass the same selected
callee contract before tagged entry and the sole Static packet; wrong
identity and non-I64 inputs reject; physical JSON/ABI, Normal/Fault, and
the unchanged app first stop are recorded honestly.

Non-claims: this D0 does not authorize an Eq condition implementation,
caller omission, generic reachability pruning, or whole-app completion.
