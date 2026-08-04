---
Status: D0 and D1 execution green; D2 adapter/parity execution authorized
Date: 2026-08-04
Parent: joinir-if-recipe-nested-one-level-design-stop-2026-08-04.md
Decision: if accepted, execute only depth-one nested pure fallthrough D0,
  D1 ownership census, and D2 parity/candidate-abort evidence
---

# One-level Nested If — bounded execution task

This card is not permission to widen the one-If recipe or to add recursive
production routing. The parent design stop owns the exact shape and stop
conditions.

## D0 — named nested facts and composition contract

Add a separate nested profile/product for exactly one outer and one inner
explicit-else If over one shared i64 binding. Keep the current one-If profile
immutable.

Required evidence:

* deterministic preorder facts and source claims for both If nodes;
* child artifact and outer artifact carry only portable identities;
* inner and outer JoinSig rows compose with the inner merge as the outer then
  edge;
* depth greater than one, implicit nested else, unsupported child operation,
  missing transfer/read, path mismatch, and multiple-binding fixtures reject
  before Builder effects.

D0 implementation evidence (2026-08-04):

* `nested_recipe_facts.rs` observes the two-node preorder in the same analyzer
  pass; the existing fixed-shell `finish()` still rejects `ifs.len() != 1`.
* `nested_schema.rs` / `nested_verify.rs` define a separate portable
  `ResolvedTrivialExplicitElseDepthOne` artifact with eight ordered source
  claims, deterministic node/value/binding keys, and no physical IDs.
* `nested_recipe_mapper.rs` consumes only the sealed sidecar plus the function
  origin, and `nested_join_sig.rs` composes the child merge into the outer
  `then` edge. No Builder, route, retry, or production physicalizer is wired.
* Focused tests cover accepted artifact/JoinSig composition plus depth-
  greater-than-one, implicit-child-else, and multiple-binding rejection.
  `RUSTFLAGS='-Awarnings' cargo test -q
  resolved_value_profile::nested_recipe_tests --lib` is green (3/3), and the
  existing in-place replacement guard remains green.

Disposition: D0 implementation is locally green; D1 owner census and D2
parity/candidate-abort evidence remain gated and are not claimed by this row.

## D1 — caller and owner census

Prove one nested producer/mapper/composition chain and keep the nested
physicalizer caller at **zero** while the profile is disconnected. Existing
CanonicalCfg, Binding SSA, and PhiTxn remain the sole physical owners. New
route, retry, transaction, and PHI/SSA owner counts must be zero. A future
production-consumer row may change the nested physicalizer caller from zero to
one only after a separate design stop; it must not be smuggled into this
census.

D1 census evidence (2026-08-04):

* Facts producer chain: one analyzer call to `nested_candidate()`, one sealed
  product sidecar, and one mapper input accessor.
* Mapper, verifier, and JoinSig composer have no production callers. Their
  only external execution caller is the focused nested test module; the
  verifier call remains inside the mapper.
* Nested artifact/mapper/composer/verifier symbols have zero references from
  `src/mir/builder/**` and zero production references from compiler/lowering
  routes. Nested physicalizer/adapter/route callers are zero.
* No touched file adds a CFG, Binding SSA, PHI transaction, PHI materializer,
  route, retry, or candidate owner. All touched source/test files remain below
  800 lines. Focused nested tests are 3/3 green, and the existing current-state
  and in-place replacement guards remain green.

## D2 — parity and candidate abort

D2 production-consumer design is accepted in
`joinir-if-recipe-nested-production-consumer-design-stop-2026-08-04.md`.
Implement only the one-shot nested proof adapter described there: it consumes
the verified artifact/JoinSig, while the existing canonical lowerer remains
the sole CFG/SSA/PHI physicalizer. Do not manufacture child
`CanonicalIfPhysicalDemandV1` values or add a second physicalizer.

Use the exact source fixture:

```text
local x = 0
if x < 10 { if x < 5 { x = 1 } else { x = 2 } } else { x = 3 }
return x
```

Prove results 1/2/3, two PHIs, exact predecessor/value correspondence, and
interpreter parity. Inject late failure after both nested PHIs using the
existing candidate seam; prove live state is unchanged and fresh reuse works.
If that fault point does not exist, stop and open a design row instead of
adding a new test-only fault mechanism.

## Post-implementation reference closeout — `JOINIR-IF-RECIPE-REFERENCE-CLOSEOUT0-DOC0`

This row is parked until the nested profile has a production consumer and its
physical/candidate gates are green. It is mandatory before the nested recipe
slice is declared complete.

Change:
  Synchronize the normative language, IR, and MIR reference pages with the
  production nested-If contract; keep the one-If V1 shell and all parked
  shapes explicitly distinguished.

Contract:
  Update only claims proved by the selected route. At minimum audit
  `docs/reference/language/EBNF.md`, `grammar-contract.md`, `statements.md`,
  `docs/reference/ir/ast-json-v0.md`, `docs/reference/ir/json_v0.md`, and
  `docs/reference/mir/phi_policy.md` / `phi_invariants.md`. Keep portable
  recipe names, recursive depth limits, JoinSig/PHI ownership, and backend
  fail-fast behavior consistent with the sealed product. Do not move
  implementation detail into the reference pages.

Done:
  Reference grammar and examples match parser-live syntax; nested depth and
  rejection boundaries are stated; JSON/MIR/PHI pages do not claim a second
  physical owner or an unimplemented recursive route; historical/provisional
  pages are labeled; the current-state pointer and active card point to the
  next real blocker.

Stop:
  Any mismatch that indicates a real parser, artifact, verifier, or runtime
  difference reopens the implementation/design row. Documentation must not
  hide an unproved route as historical prose. This closeout does not widen
  nested depth, activate ownership/Home rules, or add a new grammar surface.

## Required gates

Use the focused resolved profile, IfRecipe contract, canonical-session, and
candidate-abort suites plus:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
```

The production-shaped interpreter fixture must run with the existing
`vm-reference` feature. Keep every touched source/test file below 800 lines.

## Stop conditions

Stop and return to the design card if the implementation needs depth greater
than one, a new CFG/PHI/SSA owner, a second physicalizer, retry/fallback, raw
AST/name lookup, ownership rules, or a new fault/transaction API.
