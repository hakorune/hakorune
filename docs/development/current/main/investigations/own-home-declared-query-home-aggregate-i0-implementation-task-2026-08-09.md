---
Status: active — bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-declared-query-home-aggregate-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# DECLARED-QUERY-HOME-AGGREGATE-I0 implementation

## Objective

Implement the sole resolver-only co-seal for the landed declaration/Home and
Query catalogs. The Home catalog remains the only declaration owner. The new
aggregate owns the two input catalogs and only their relational compatibility.

## Allowed surface

```text
src/mir/resolved_semantics/declared_instance_contract.rs
src/mir/resolved_semantics/declared_instance_contract_tests.rs
src/mir/resolved_semantics/home_abi.rs       # resolver-brand getter only
src/mir/resolved_semantics/mod.rs
src/mir/resolved_semantics/README.md
docs/reference/language/callable-contracts.md
```

No target, CallSlot, Recipe, body conformance, Builder, MIR, physical ABI,
runtime, provider, fallback, or production caller is allowed.

## Required product and issuer

```text
DeclaredInstanceMethodContractIssuerV1::issue(
    home: VerifiedDeclaredInstanceMethodHomeCatalogV1,
    query: VerifiedDeclaredQueryBehaviorCatalogV1,
) -> Result<VerifiedDeclaredInstanceMethodContractCatalogV1,
            DeclaredInstanceMethodContractIssueV1>

VerifiedDeclaredInstanceMethodContractCatalogV1
  owns home catalog
  owns query catalog
  exposes borrowed declaration/Home/Query projections only
```

The aggregate must not accept or store a separate declaration catalog. It must
not expose standalone Home or Query receipts that can be recombined with a
different aggregate.

## Required coverage checks

```text
home catalog is non-empty
query catalog is non-empty
home declaration count == Home ABI row count
home/query resolver brands match
each Home ABI row matches its declaration identity
each Query row matches exactly one Home declaration and Home ABI row by:
  nominal Box type
  Box statement ordinal
  method member ordinal
Query rows are unique and strictly increasing in Home declaration order
selected_pairs count == query row count
```

The Query catalog may cover only a strict subset of Home declarations. No
default Query row is fabricated for non-Query declarations.

## Acceptance tests

```text
positive:
  one length(): i64 Query row + matching Home catalog -> aggregate
  mixed Query/non-Query declaration catalog -> exact Query subset co-seals
  multiple Query rows preserve declaration order

negative:
  Query catalog from a foreign declaration brand -> reject
  stale/foreign nominal Box or source site -> reject
  duplicate/misordered/empty coverage -> reject or unconstructible
  Home ABI row/declaration mismatch -> reject
  no Query declaration is rejected before aggregate issuance
```

## Explicit non-claims

```text
No body conformance or read-footprint verification
No target/source-bound call relation or CallSlot
No Recipe/Builder/MIR/physical ABI/runtime/provider
No function pointer, module publication, fallback, retry, or production
```

## Same-slice documentation and verification

The implementation commit must update `src/mir/resolved_semantics/README.md`,
`docs/reference/language/callable-contracts.md`, the current task map, and
`CURRENT_STATE.toml` in the same slice. Keep each Rust file below the 760-line
split trigger and hard-stop at 800.

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust declared_instance_contract
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

After this row closes, stop at body-conformance/catalog closure design. Do
not open resolver target or physical lowering from this implementation alone.
