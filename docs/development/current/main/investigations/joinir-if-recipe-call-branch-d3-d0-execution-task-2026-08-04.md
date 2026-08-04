---
Status: Ready — D0 Call-leaf facts/recipe contract only
Date: 2026-08-04
Parent: joinir-if-recipe-call-branch-d3-design-stop-2026-08-04
Decision: admit one direct static i64 call only as one explicit-else branch
  assignment RHS; keep production If physicalization and call emission
  unchanged until the later D1/D2 rows
---

# D3-D0 execution task — Call-valued If leaf

## Scope

Implement the pre-Builder semantic contract for the selected Call-RHS shape.
This row may change facts, the portable If recipe schema/verifier/mapper, and
focused tests. It must not connect a new production call route, change the
selected physicalizer, retire a legacy writer, or broaden the accepted source
shape beyond one direct static i64 call in one explicit-else branch assignment
RHS.

## Required product

The same-pass facts must retain one owner-branded direct-call leaf instead of
marking every branch call unsupported. The portable artifact must represent
that leaf without AST nodes, callable headers, `ValueId`, `BasicBlockId`, or
runtime handles:

```text
If source claim: DirectStaticCall at the exact branch RHS site
Recipe operation: DirectStaticCall(result = recipe-local value key)
Co-sealed profile: VerifiedTrivialDirectCallV1
  target + arguments + InlineI64 result + ConservativeBarrier effect
```

Because D0 admits exactly one call, a new call key is not required; the exact
source claim and the existing direct-call profile row are the pairing key. If
implementation evidence shows that a source claim cannot be made unambiguous,
stop and reopen design before inventing a second identity system.

## Allowed implementation surface

Expected files (extract instead of growing a file past 800 lines):

```text
src/mir/resolved_value_profile/recipe_facts.rs
src/mir/resolved_value_profile/analyzer.rs
src/mir/resolved_value_profile/recipe_mapper.rs
src/mir/if_recipe_contract/schema.rs
src/mir/if_recipe_contract/verify.rs
src/mir/if_recipe_contract/normalize.rs (only if required by schema)
src/mir/resolved_value_profile/tests.rs
src/mir/if_recipe_contract/tests.rs
```

Do not modify `trivial_ssa/direct_call.rs`, the If physicalizer, raw call
routes, CorePlan/JoinIR, JSON-v0, or the production selected caller in this
row. The existing direct-call emitter remains the future D1 consumer.

## Contract rules

Accept only when all are true:

1. The function has one root If with an explicit else and no nested control,
   return, short-circuit, record, match, method, or dynamic call.
2. Exactly one direct static call exists in the whole If recipe and it is the
   RHS of exactly one branch assignment.
3. The call has only admitted trivial i64 arguments and an `InlineI64` result.
4. The other branch assignment is a pure admitted i64 expression.
5. The merge binding is homogeneous and has the existing post-merge read.
6. The source claim, call profile row, recipe operation result, JoinSig value
   class, and branch assignment all agree before Builder effects.

Reject before Builder effects when a call is in the condition, continuation,
both branches, an argument contains another call, the target is not a direct
static call, the result/effect is not the sealed profile, or the source claim
does not match the exact branch RHS.

## Focused tests

Add tests for:

* same-pass facts capture one Call leaf in an explicit-else branch RHS;
* mapper/verifier produce a portable Call operation with no physical IDs;
* source-claim/profile-site mismatch rejects before mapping;
* call in condition, call in continuation, two calls, and call-valued
  implicit-fallthrough remain typed `NotThisShape`/reject;
* existing no-call explicit/implicit recipe tests remain unchanged and green.

No candidate-abort or runtime parity test is added here; those belong to D2
after the D1 caller census and physicalizer handoff are accepted.

## Acceptance gates

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
```

Also verify every touched Rust/test file is below 800 lines and that the
selected production caller counts remain unchanged:

```text
trivial_ssa/direct_call::emit production callers = 1
If recipe physicalizer production callers       = 1
new Call-RHS production physicalizer callers    = 0
route/retry/fallback edges                      = 0
```

## Stop conditions

Stop and return to the D3 design card if this row needs:

* a second call resolver/emitter or a new SSA/PHI owner;
* raw AST/name lookup after facts are sealed;
* a call key/header/ABI copied into the portable artifact;
* implicit-fallthrough, nested, effect, return, record, match, or
  short-circuit support;
* production physicalizer wiring or candidate fault injection;
* `Option`, retry, fallback, or silent `FunctionCall -> unsupported` masking;
* any touched source/test file over 800 lines.

The next rows after this task are D1 caller census, then D2 production
physicalizer/parity/candidate-abort evidence. They are separate commits.
