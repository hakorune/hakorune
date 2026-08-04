---
Status: D0 complete; D1 execution authorized; D2 gated on D1 green
Date: 2026-08-04
Parent: joinir-if-recipe-call-branch-implicit-design-stop-2026-08-04.md
Decision: if the design stop is accepted, execute only D0 facts/claims,
  D1 caller census, and D2 parity/candidate-abort evidence for one implicit
  then-only direct static i64 Call-RHS
---

# Implicit Call-RHS — bounded execution task

This card is an implementation checklist, not permission to broaden the
current shape. The parent design stop remains the source of scope and stop
conditions.

## D0 — facts, claims, and rejection matrix

Candidate production changes are limited to the existing If recipe contracts:

```text
resolved_value_profile/recipe_facts.rs
resolved_value_profile/recipe_mapper.rs
if_recipe_contract/verify.rs
if_recipe_contract/source_binding.rs
```

Remove only the explicit-else requirement for the exact implicit then-RHS
direct-call shape. Preserve rejection for any call outside that path and
preserve the existing implicit `IfThen` baseline/JoinSig representation.

Add focused facts/artifact/source-claim tests for:

* accepted implicit baseline + one then direct call;
* explicit else and wrong-path call rejection;
* two calls, call-in-condition, call-in-continuation, method/dynamic call,
  and unsupported result rejection.

### D0 evidence — 2026-08-04

The four existing contract owners now accept the exact implicit then-RHS
direct-call shape:

```text
facts finish                         = implicit direct-call admitted
recipe mapper                        = implicit profile admitted
IfRecipe/source-claim verifier       = baseline + direct-call order admitted
stale explicit-else reject variant   = deleted
new route/PHI/SSA/transaction owner  = 0
```

The focused facts/artifact test proves the source claim order
`[IfNode, Condition, ThenAssignment, ImplicitBaseline, DirectStaticCall]`,
the implicit baseline JoinSig representation, and a physical-ID-free
semantic artifact. Existing two-call and condition-call rejection tests stay
negative.

Green gates:

```text
resolved_value_profile             = 42 passed
if_recipe_contract                 = 10 passed
direct_call                        = 38 passed
if_recipe_candidate_abort_d2       = 2 passed
cargo check --lib                  = green
```

## D1 — caller census

Record production caller counts before/after the D0 change. The expected
delta is zero for all physical/capability owners:

```text
direct-call sealer      = 1
direct-call emitter     = 1
If recipe physicalizer  = 1
new route/transaction   = 0
new PHI/SSA writer      = 0
```

## D2 — parity and candidate abort

Mirror the completed explicit-else Call-RHS proof with `else_body = None`:

* true/false execution proves then-call versus header baseline;
* one Call and one `[header, then_exit]` PHI are present;
* capability, source claims, JoinSig, and interpreter results correspond;
* late seal failure preserves the live Builder fingerprint and unpublished
  candidate state;
* a fresh compile on the same compiler succeeds.

Reuse the existing candidate-session failure seam. Do not add a fault toggle,
rollback journal, second transaction, or production Builder snapshot API.

## Required gates

Use the focused existing suites plus the shared guards from the parent card:

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_value_profile -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_contract -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib if_recipe_candidate_abort_d2_tests -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The `vm-reference` success/parity gate is required for the production-shaped
fixture. Keep every touched file below 800 lines.

## Stop conditions

Stop and return to the design card if any new route, physicalizer, capability,
PHI/SSA owner, retry/fallback, ownership rule, or unsupported call family is
needed.
