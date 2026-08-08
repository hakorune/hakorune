---
Status: active — bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-abi0-s0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
---

# OWN-HOME-ABI0-S0 implementation

## Objective

Implement the exact design-stop product with one resolver Home ABI issuer.
The slice is AST-free after declaration issuance, non-`Clone`, and caller-zero
for Home Flow/Builder/Recipe/runtime.

## Allowed surface

```text
src/mir/resolved_semantics/home_abi.rs
src/mir/resolved_semantics/home_abi_tests.rs
src/mir/resolved_semantics/mod.rs
```

The implementation may read the existing declaration catalog and passive Home
relation vocabulary. It must not import or call Builder, MIR ownership SSA,
Recipe, target, runtime, plugin, or physical ABI modules.

## Required products

```text
ResolverHomeCapabilityEnvironmentV1
  resolver catalog brand
  explicit I64UnitTrivial schema
  relation-batch brand

VerifiedHomeAbiV1
  resolver brand
  nominal Box type and exact Box/method source site
  relation-batch brand as provenance only
  receiver demand
  ordered parameter demands
  result relation

VerifiedDeclaredInstanceMethodHomeCatalogV1
  owned declaration catalog
  one Home ABI row per declaration, in declaration order
```

Only `CallableHomeAbiIssuerV1::issue(catalog, environment)` may issue the
Home catalog. No public partial row constructor, axis accessor that returns a
standalone semantic receipt, test-only forged constructor, clone/reissue path,
or relation-ordinal-as-declaration-key is allowed.

## Bounded mapping

```text
ordinary instance receiver -> Handle
semantic I64/Unit parameter -> Trivial
semantic Unit result -> Unit
semantic I64 result -> Trivial
```

The current declaration issuer rejects unsupported types before this row. Do
not add `Unknown`, default-to-`Trivial`, or body/MIR/physical inference.
Query syntax is intentionally ignored by this issuer.

## Acceptance

Positive:

```text
length(): i64       -> Handle, [], Trivial
reset(): Unit       -> Handle, [], Unit
read(i64): i64      -> Handle, [Trivial], Trivial
```

Negative/guarded:

```text
catalog/environment resolver-brand mismatch
foreign nominal Box or source site
foreign relation-batch brand
existing declaration issuer rejects static/unsupported types
Query present versus absent has identical Home output
fresh catalog has a fresh relation-batch brand
```

The focused tests must also show that the output catalog retains declaration
identity and that the separate relation brand is never equal to or used as the
resolver catalog brand. Impossible forged-row states are protected by private
fields and non-`Clone` products rather than test constructors.

## Non-claims

```text
No Home Flow / Ownership SSA / transfer failure
No grammar (`take`, `share`, `release`)
No Query behavior or body conformance
No target/CallSlot/Recipe/Builder/MIR/physical ABI/runtime/provider
No fallback/retry or production caller
```

## Verification

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust home_abi
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
python3 - <<'PY'
import tomllib
from pathlib import Path
tomllib.loads(Path('docs/development/current/main/CURRENT_STATE.toml').read_text())
PY
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The implementation commit must update `src/mir/resolved_semantics/README.md`,
`docs/reference/language/ownership.md`,
`docs/reference/language/callable-contracts.md`, the parent Home taskboard,
and current pointers in the same slice. Keep each Rust file below the 760-line
split trigger and hard-stop at 800.
