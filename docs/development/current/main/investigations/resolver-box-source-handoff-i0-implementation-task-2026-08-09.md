---
Status: closed — bounded implementation landed
Date: 2026-08-09
Parent: `resolver-box-source-handoff-d0-design-task-2026-08-09.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# RESOLVER-BOX-SOURCE-HANDOFF-I0

## Objective

Consume one parser-private non-Clone `ParserBoxSourceSealV1` and issue one
opaque, AST-free source handoff for the bounded ordinary top-level Rust Box
cohort. This is the only implementation opened by the handoff D0.

```text
rich parser final product
  -> consuming handoff entry
  -> AST + non-Clone source handoff
```

The handoff preserves exact source member identity, method header syntax, typed
`CallableContractSyntaxV1::Query` carriage, and inventory placement for
diagnostics. It does not issue resolver semantic types, Home ABI, target,
Recipe, or physical facts.

## Allowed changes

```text
src/parser/source_resolver_handoff.rs       new bounded owner module
src/parser/mod.rs                          module/entry wiring only
src/parser/source_authority.rs              direct-site predicate only if needed
src/parser/callable_contract_syntax.rs     crate-visible typed carriage only
src/parser/*_tests.rs                      focused positive/negative tests
src/parser/README.md                       landed authority receipt
src/mir/resolved_semantics/README.md        resolver ingress receipt
docs/reference/language/callable-contracts.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/10-Now.md
this task card and task map
```

No parser source-authority rewrite, AST rewrite, resolver semantic product,
Home implementation, target, Recipe/CallSlot, Builder/MIR, provider/runtime,
fallback, or production selection is allowed.

## Canonical product

The public crate-visible handoff is non-Clone and has no arbitrary
constructor. Its rows are AST-free source syntax DTOs:

```text
ParserBoxResolverSourceHandoffV1 (non-Clone)
  source invocation brand (opaque)
  Box declaration rows

Box row
  exact top-level Box source site
  nominal Box source name
  explicit method rows only

Method row
  SourceBoxMethodSiteV1 transfer coordinate
  inventory ordinal (placement/diagnostic only)
  method name
  ordered parameter declaration syntax
  return type syntax
  static flag
  typed CallableContractSyntaxV1
```

Generated property/delegate rows are not emitted as explicit method rows.
Generated-only Boxes reject the handoff instead of producing an empty
authority. Any selected BuildGate path outside the bounded ordinary cohort
rejects this I0 and remains `NoSafeSlice`.

## Fail-fast matrix

```text
no source seal / unsupported cohort       -> NoSafeSlice
foreign source brand/site                -> Rejected
duplicate explicit source method         -> Rejected
relation/name/inventory mismatch        -> Rejected
generated-only Box                       -> Unresolved
missing typed Query is source Declined   -> preserved as None
post-consume handoff reuse               -> compile-time move failure
raw AST/JSON/HashMap/name reconstruction  -> forbidden by module boundary
```

`NoSafeSlice` is a development state, not a source disposition. The handoff
does not decide whether an unannotated method is a candidate; it only preserves
the optional typed source row for the later semantic issuer.

## Acceptance gates

1. Positive `box TextLike { @rune CallableContract(query) length(): i64 { ... } }`
   returns AST plus one handoff Box row and one direct method row.
2. The method row retains member site, name, zero parameters, `i64` return
   syntax, instance/static bit, inventory placement, and typed Query carriage.
3. Generated-only property rows do not become explicit resolver rows.
4. Foreign/stale/mismatched relations reject before handoff issuance.
5. The handoff has no AST field and cannot be cloned or issued twice.
6. No production resolver consumer is added; the next declaration/signature
   row consumes this handoff.
7. Focused parser tests, parser/resolver README receipts, language reference,
   task map, and current pointers update in this same commit.
8. New module remains below 650 lines; split before 760 and never cross 800.

## Required verification

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust source_resolver_handoff
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

No unrelated baseline guard is promoted by this row.

## Closeout receipt (2026-08-09)

Implemented in `src/parser/source_resolver_handoff.rs` with focused tests in
`src/parser/source_resolver_handoff_tests.rs`. The rich parser now returns the
AST separately from one non-Clone `ParserBoxResolverSourceHandoffV1`; the
handoff carries direct source sites, raw header syntax, typed Query carriage,
and placement ordinals, while generated-only and unsupported cohorts reject.

Green evidence:

```text
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust source_resolver_handoff
  3 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust parser::source_seal
  13 passed
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
  passed
```

No resolver semantic issuer, Home ABI, target, Recipe/CallSlot, body
conformance, Builder/MIR, provider/runtime, or production activation was
opened. The next design stop is the declaration semantic-signature/Home ABI
boundary.
