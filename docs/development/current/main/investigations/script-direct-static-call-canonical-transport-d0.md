---
Status: Closed NoSafeSlice / parked (2026-08-20)
Date: 2026-08-20
Exception: New T2 transport/lifetime boundary between existing semantic and canonical physical owners.
ParentCurrentCard: docs/development/current/main/investigations/script-direct-static-call-canonical-physical-input-i0.md
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  - src/mir/builder/normal_script_direct_static_join_handoff/physical_input.rs
  - src/mir/compiler/canonical_core_dispatch.rs
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-TRANSPORT-D0

## Current six-line brief

Decision: Design one move-only transport from the already-issued direct-static
physical input to the canonical detached Script entry; do not implement or
switch a caller until the carrier and identity handshake are closed.

Source authority + canonical issuer: the existing
`VerifiedScriptDirectStaticJoinHandoffV1` plus resolver-issued scalar operand
Recipe remain the semantic authority; one
`VerifiedScriptDirectStaticPhysicalInputV1::issue(join, operands)` call issues
the transported input.

Non-authority: canonical AST/source-plan values, `RawScriptBodyRecipeV1`,
`CanonicalCoreSourcePlanCompileRequestV1`, selected-normal claim state, names or
ordinals, consumer-side re-co-sealing, and any fallback route.

Fail-fast boundary: before detached Builder/Call effects, reject a missing or
duplicate carrier, source/owner/key/cardinality/site/target/representation or
terminal drift, profile mismatch, and simultaneous raw-recipe/input selection;
never fall back to AST re-resolution or the raw recipe.

Smallest next slice: specify one producer → move-only carrier → canonical
`compile_script` consumer handshake, including the existing Script session,
module transaction, caller, and old-edge disposition. No source-admission edit,
physical implementation, production switch, or raw retirement belongs here.

Non-claims: no canonical consumer yet, no production caller switch, no source
shape expansion, no compatibility/Deferred/raw retirement, no `MirInstruction::Call`
cleanup, no exit-owner/ABI/backend change, and no performance/C-parity claim.

## Evidence boundary

The landed physical-input producer is at
`normal_script_direct_static_join_handoff/physical_input.rs`; its detached
consumer is `script_physical_exit/direct_static_entry_kernel.rs`. The current
canonical path is `canonical_core_dispatch::compile_script` →
`SealedNormalScriptSourceV1::prepare_script_recipe` →
`OpenScriptPhysicalEntryV1`, which still consumes only `RawScriptBodyRecipeV1`.
No production caller currently invokes the new physical-input consumer.

The input is also narrower than the existing selected-normal claim route:
`PhysicalInput::issue` succeeds only when every argument belongs to the
resolver-issued integer literal/unary/binary cohort. The broader claim ledger
already lowers other exact-I64 calls through the ordered child driver. A
transport plan must therefore carry a source-issued, pairwise-disjoint cohort
selection; it may not let the canonical consumer and the claim ledger both
observe the same row or treat an unsupported scalar tree as an absent row.

## D0 done / stop

Done requires one source-backed carrier owner, one canonical consumer, exact
identity/cardinality checks before effects, and a named old-edge disposition;
the design must prove that raw recipe and physical input cannot both be used.
If the semantic source cannot reach `compile_script` without re-parsing,
re-resolving, or inventing a second authority, record `NoSafeSlice` and leave
the current physical-input row closed. The same stop applies when scalar-cohort
partition, canonical caller identity, or raw-recipe/input exclusivity cannot be
proven before the first physical effect.

## Closeout

The semantic Script source and the canonical detached `compile_script` path do
not share a source-backed carrier or caller identity. The landed physical input
therefore remains test-owned and detached. No source admission, re-resolution,
fallback, or production switch was added. Reopen only after a separate
Program-owned source inventory and one move-only producer-to-canonical-consumer
handshake are closed.
