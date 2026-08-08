---
Status: closed — bounded semantic declaration/signature implementation landed
Date: 2026-08-09
Parent: `loop-resolver-instance-declaration-and-contract-receipts-d0-design-task-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# RESOLVER-INSTANCE-DECLARATION-SIGNATURE-I0

## Objective

Implement the first resolver-owned semantic declaration/signature issuer for
the bounded ordinary Rust Box cohort.

```text
ParserBoxResolverSourceHandoffV1 (consume by value)
  + resolver-owned nominal/type environment
  -> one fresh resolver catalog/type brand
  -> non-Clone AST-free declaration catalog
```

The positive witness is:

```hako
box TextLike {
    @rune CallableContract(query)
    length(): i64 { return 0 }
}
```

The issuer owns declaration identity and semantic parameter/result types only.
Home ABI, Query behavior, body conformance, target, Recipe/CallSlot,
Builder/MIR, physical ABI, provider/runtime, fallback, and production remain
closed.

## Authority contract

```text
source authority:
  ParserBoxResolverSourceHandoffV1::into_parts(self)

nominal/type authority:
  resolver-owned ResolverNominalTypeEnvironmentV1
  with a fresh ResolverCatalogBrandV1

semantic signature authority:
  SemanticInstanceDeclarationIssuerV1

non-authority:
  parser names/ordinals alone, FunctionOwnerId compilation brand,
  ReceiverPolicy, Builder catalog, ExactTrivial*Abi, MirType,
  FunctionSignature, EffectMask, Home/Query defaults, target/Recipe/MIR
```

The handoff brand is retained as provenance/membership evidence. It is not
reused as nominal type identity. The issuer must consume the handoff by value
and must not call `boxes()` to clone rows or issue partial receipts.

## Product boundary

The public semantic product is non-Clone and AST-free:

```text
VerifiedInstanceMethodDeclarationCatalogV1
  resolver catalog/type brand
  parser provenance brand
  exact nominal Box declaration/site
  exact method source site
  instance/static bit
  ordered semantic parameter/result signature
  optional typed CallableContractSyntaxV1 carriage only
```

The product does not issue `Handle`, `Trivial`, Query effect/control, or any
physical ABI fact. A missing `CallableContract(query)` is retained as absent
and is a later behavior `Declined`, not a declaration failure.

## Required negative matrix

```text
Rejected:
  foreign/stale/duplicate Box or method site
  static/instance mismatch
  generated/property/compatibility row
  mutated inventory ordinal treated as source identity
  duplicate nominal declaration

Unresolved / NoSafeSlice:
  unknown or missing nominal Box type authority
  missing semantic type issuer

Ownership:
  handoff reuse after consuming issue
  row clone/partial re-issuance path
```

## Guard and size rules

The implementation lives in a dedicated `src/mir/resolved_semantics/` module.
It must not import Builder, Recipe, CallSlot, target, provider, runtime,
`ExactTrivial*Abi`, `MirType`, `FunctionSignature`, or `EffectMask`.

Keep every new Rust source file below the 760-line split trigger and never
cross 800 lines. Split by owner/interface, not by arbitrary line count.

## Acceptance gates

1. Positive `TextLike.length(): i64` issues one declaration catalog with a
   fresh resolver brand, exact source sites, zero parameters, semantic `I64`,
   and typed Query carriage.
2. The issuer consumes the handoff by value and preserves parser provenance;
   no `boxes()`/row-clone path can issue the product.
3. Missing Query does not prevent declaration/signature issuance.
4. Foreign, duplicate, unused, unknown-type, and exact nominal/source mismatch
   cases fail at the declared boundary. Static/generated-only/compatibility
   rows are rejected or excluded by the parser handoff before this issuer;
   they are not reclassified here.
5. No Home ABI, Query behavior, target, Recipe/CallSlot, Builder/MIR, or
   physical ABI caller is added.
6. Focused tests, `src/mir/resolved_semantics/README.md`,
   `docs/reference/language/callable-contracts.md`, this card, and current
   pointers are updated in the same implementation closeout.

## Verification

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust instance_method_declaration
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

No production selection or runtime gate is opened by this row.

## Implementation receipt (2026-08-09)

Landed in the same implementation closeout:

```text
src/mir/resolved_semantics/instance_method_declaration.rs
src/mir/resolved_semantics/instance_method_declaration_tests.rs
src/mir/resolved_semantics/README.md
docs/reference/language/callable-contracts.md
```

The issuer consumes the handoff through `into_parts(self)`, creates a fresh
resolver catalog/type brand, validates exact nominal source coverage, and
returns one non-`Clone` declaration catalog. The focused matrix covers the
positive typed Query carriage, missing Query, unknown nominal Box, unsupported
semantic type, duplicate source identity/name, and unused nominal declaration.
Static/generated-only/compatibility rows are owned by the parser handoff
boundary and are intentionally not fabricated as issuer fixtures.

Verification receipt:

```text
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust instance_method_declaration  # 8 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust source_resolver_handoff      # 3 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust parser::source_seal         # 13 passed
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust                             # pass
```

The row deliberately does not claim Home ABI, Query behavior/conformance,
resolver targets, Recipe/CallSlot, Builder/MIR, physical ABI, provider/runtime,
fallback, or production activation. The next row is the Home ABI design stop.
