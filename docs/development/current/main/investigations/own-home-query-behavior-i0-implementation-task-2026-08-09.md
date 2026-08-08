---
Status: active — bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-query-behavior-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# RESOLVER-DECLARED-QUERY-BEHAVIOR-I0 implementation

## Objective

Implement the accepted Query D0 as one resolver-only, non-`Clone` behavior
catalog. The issuer consumes the existing declaration catalog by reference,
reads only typed `CallableContractSyntaxV1::Query`, and emits a non-empty
catalog for the exact Query subset. It must not issue or copy Home relations.

## Allowed surface

```text
src/mir/resolved_semantics/query_behavior.rs
src/mir/resolved_semantics/query_behavior_tests.rs
src/mir/resolved_semantics/mod.rs
src/mir/resolved_semantics/README.md
docs/reference/language/callable-contracts.md
```

The implementation may borrow the landed resolver declaration catalog and its
typed declaration/source-site getters. It must not import or call Home Flow,
Builder, MIR, Recipe, target, runtime, provider, or physical ABI modules.

## Required products

```text
DeclaredQueryBehaviorV1
  semantic behavior = ReceiverDirectReadNoEffects

VerifiedDeclaredQueryBehaviorV1
  resolver catalog brand
  exact nominal Box identity
  exact Box statement ordinal
  exact method member ordinal
  typed behavior
  optional rune ordinal for diagnostics only

VerifiedDeclaredQueryBehaviorCatalogV1
  resolver catalog brand
  non-empty Query subset rows in declaration order
```

The public issuer is the only construction authority:

```text
DeclaredQueryBehaviorIssuerV1::issue(
    &VerifiedInstanceMethodDeclarationCatalogV1,
) -> Result<VerifiedDeclaredQueryBehaviorCatalogV1, QueryBehaviorIssueV1>
```

No public partial-row constructor, empty verified catalog, clone/reissue path,
raw-rune parser, or test-only forged verified constructor is allowed. The
catalog is non-`Clone`; the declaration catalog remains the resolver identity
owner and is only borrowed here.

## Bounded semantics

```text
allowed:
  exact receiver direct-state reads
  ordinary return

forbidden:
  receiver/global writes
  Home transfer/share/end/escape
  allocation
  IO/FFI
  Fault/throw/non-local failure propagation
  suspension
  non-local control transfer
```

The product stores the behavioral obligation only. It does not store
`HomeDemandV1`, `HomeResultRelationV1`, parameter/result signature classes,
`HomeRelationBrandV1`, body read footprints, MIR `EffectMask`, or physical ABI.
The optional rune ordinal is diagnostic provenance, never semantic identity.

## Selection and dispositions

The issuer scans the resolver catalog in declaration order and emits only
declarations carrying typed `CallableContractSyntaxV1::Query`. Non-Query rows
are outside this behavior family; they are not defaulted into Query. A
zero-row result is never represented as a verified catalog.

```text
Candidate:
  exact non-empty typed Query subset, same resolver brand/site

Declined:
  no Query declaration, or caller requests a strict all-row Query cohort with
  non-Query declarations (`MixedQueryCohort`)

Unresolved:
  typed source/site is unavailable (reserved for future source gaps)

Rejected:
  foreign brand/site, duplicate behavior, conflicting metadata, or stale
  declaration identity

NoSafeSlice:
  issuer/design is not implemented (development state, not a source result)
```

The first row may emit a Query subset from a larger declaration catalog, but a
later Home/Query aggregate must pass that same selected subset and verify exact
one-to-one declaration/Home/behavior coverage.

## Acceptance tests

```text
positive:
  length(): i64 with CallableContract(query) -> one Query row
  row identity matches declaration brand/nominal Box/source sites
  behavior is ReceiverDirectReadNoEffects
  rune ordinal is diagnostic only

negative/guards:
  missing Query -> typed Declined; no empty verified catalog
  mixed Query/non-Query -> only exact Query subset; strict all-row policy is
    explicitly declined, never silently completed
  Query-present vs Query-absent Home ABI output is unchanged
  foreign/stale identity cannot be represented or is rejected by issuer guard
  multiple Query rows preserve declaration order
```

## Explicit non-claims

```text
No Home ABI issuance or Home Flow
No body conformance or transitive read-footprint proof
No Pure/MIR EffectMask projection
No target/source-bound Call relation
No Recipe/CallSlot/Builder/MIR/physical ABI/runtime/provider
No fallback/retry/production activation
```

## Same-slice documentation and verification

The implementation commit must update `src/mir/resolved_semantics/README.md`
and `docs/reference/language/callable-contracts.md` to record the landed
issuer and its non-claims. Keep both Rust files below the 760-line split
trigger and hard-stop at 800.

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust query_behavior
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

After this row closes, stop at the parked
`own-home-declared-query-home-aggregate-d0-design-task-2026-08-09.md`; do not
start target, Recipe/CallSlot, body lowering, or production work.
