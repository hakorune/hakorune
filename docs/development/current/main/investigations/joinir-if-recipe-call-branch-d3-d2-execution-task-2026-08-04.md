---
Status: Ready for implementation — D2 candidate-abort and parity evidence
Date: 2026-08-04
Parent: joinir-if-recipe-call-branch-d3-design-stop-2026-08-04.md
Decision: open exactly two D2 proofs for the selected direct static i64
  Call-RHS shape; reuse the existing canonical If physicalizer, direct-call
  emitter, and unpublished whole-compile candidate boundary
---

# D3-D2 execution task — Call-valued If parity and candidate abort

## Scope

Close the selected Call-RHS shape with two focused proofs:

1. successful production physicalization has the same branch/PHI/value
   correspondence as the existing direct-call oracle;
2. a late failure after Call + If + PHI work drops the unpublished candidate,
   leaves the live `MirBuilder` unchanged, and permits a fresh compile.

This row does not broaden the source shape, add a route, or retire any
unselected caller. D1 caller/capability ownership is already recorded in the
parent design stop and must remain exact-once.

## Selected source shape

Keep the D3 envelope unchanged:

```text
one resolved static callable module
one root If with explicit else
one outer i64 binding assigned in each branch
one direct static i64 call in exactly one branch assignment RHS
one pure i64 expression in the other branch
one post-merge read/return
```

The call target and argument rows come only from the co-sealed
`VerifiedTrivialDirectCallV1` profile. The portable recipe and physical receipt
must not contain callable headers, raw AST nodes, `ValueId`, `BasicBlockId`, or
runtime handles.

## Proof A — success and parity

Add one production-shaped fixture using the existing resolved callable-module
front door. The fixture must contain a direct static helper with an `InlineI64`
result and an explicit-else If whose one branch assigns the helper result.

Verify, in order:

* the same-pass facts and D0 recipe contain exactly one direct-call row;
* `VerifiedTrivialDirectCallV1` is sealed once and its target/argument/result
  contract is the row consumed by the existing `trivial_ssa::direct_call::emit`;
* the selected If physicalizer is the only If physicalizer caller;
* MIR contains one direct call, one branch join, and one two-input PHI whose
  predecessor/value pairs match the sealed JoinSig;
* the post-merge result and diagnostics match the existing direct-call oracle;
* no `Option`, retry, fallback, raw lookup, or second SSA/PHI owner appears.

The parity oracle may inspect MIR metadata and an interpreter result, but it
must compare the sealed source/branch/value correspondence rather than infer a
route from emitted instruction names.

## Proof B — candidate abort and fresh reuse

Reuse the existing test-only seam:

```text
CanonicalModuleLoweringSessionV1::open
  -> lower_resolved_trivial_function_draft_with_seal_failure_for_test
  -> drop unpublished candidate
```

Use the same selected Call-RHS fixture (or a source-equivalent single-function
fixture if the module front door cannot expose the seam). The injected failure
must occur after the call, branch blocks, and PHI work have been produced but
before external publication.

The proof must assert:

* `MirBuilder::loop_candidate_test_fingerprint()` is identical before/after;
* `current_module`, current function, and entry block are unpublished/empty;
* the failure is a typed terminal `BuilderContract`/draft-seal failure;
* no retry or alternate route is attempted;
* a fresh compile on the same compiler succeeds and produces the expected
  function/branch/PHI shape.

Do not add a second transaction, rollback journal, fault environment variable,
live Builder snapshot API, or production failure branch.

## Authorized files

Prefer the existing test modules and small extracted helpers:

```text
src/mir/compiler/if_recipe_candidate_abort_d2_tests.rs
src/mir/compiler/capability_tests.rs (only if the success oracle belongs here)
src/mir/compiler/<small call-rhs test helper>.rs (only if needed)
```

Keep every touched Rust/test file below 800 lines. If a fixture helper would
grow a large module, extract a test-only helper instead of extending a
production owner.

## Acceptance gates

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_abort_d2_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib direct_call -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Also re-run the exact caller ledger:

```text
VerifiedTrivialDirectCallV1::seal production callers = 1
trivial_ssa::direct_call::emit production callers     = 1
If recipe physicalizer production callers             = 1
new Call-RHS physicalizer production callers           = 0
route/retry/fallback edges                             = 0
```

## Stop conditions

Return to the D3 design stop if any proof needs:

* a second direct-call resolver/emitter or another PHI/SSA owner;
* raw/name lookup after the sealed facts/profile boundary;
* a new transaction, rollback journal, production fault toggle, or live
  Builder snapshot;
* implicit-fallthrough Call-RHS, nested/effect/return/record/match/
  short-circuit calls, or more than one call operation;
* `None`, retry, fallback, or route reselection after a selected call fails;
* a touched file over 800 lines.

## Non-claims

This task proves only the selected Call-RHS shape. It does not establish
global PHI/SSA sole-writer status, all-call route retirement, JSON-v0 parity,
Loop-family convergence, ownership/Home activation, or property retirement.
