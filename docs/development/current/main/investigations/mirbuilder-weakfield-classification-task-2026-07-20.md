---
Status: Active D-prime task
Date: 2026-07-20
Scope: declaration-backed weak-field classification before ordinary FieldSet receipt migration
Parent: docs/development/current/main/investigations/mirbuilder-fact0-transaction-boundary-task-2026-07-20.md
Predecessor: FIELDSTORE-OBSERVE0-M0
---

# WEAKFIELD-CLASSIFY0: split weak classification from issuance

## Decision

`WEAKFIELD-CLASSIFY0-D0` selects Candidate A-prime. A private non-Clone,
single-use route product classifies an exact field assignment once, then the
weak issuer consumes its prepared route without re-querying declarations.

- `prepare_field_write_route_v1` returns either fully prepared `Ordinary` or
  fully prepared `KnownWeak`.
- `WEAKFIELD-CLASSIFY0-I0` changes only classification and issuance ownership.
- `FIELDSTORE-OBSERVE0-I0` alone moves the ordinary FieldSet access site behind
  a successful FieldSet receipt.

## Fixed order

1. `WEAKFIELD-CLASSIFY0-S0`
2. `WEAKFIELD-CLASSIFY0-P0`
3. `WEAKFIELD-CLASSIFY0-I0`
4. `WEAKFIELD-CLASSIFY0-G0`
5. `FIELDSTORE-OBSERVE0-P0`
6. `FIELDSTORE-OBSERVE0-I0`
7. `FIELDSTORE-OBSERVE0-G0`
8. `MIRBUILDER-FSESSION0-D0`

`WEAKFIELD-CLASSIFY0-I0` is the sole next code-facing row after S0/P0 proof.

### `WEAKFIELD-CLASSIFY0-S0` — closed (2026-07-20)

`src/mir/builder/weak_field_write_route.rs` now owns the disconnected pure
route vocabulary and classifier. It retains exact physical inputs in both
route variants; `KnownWeak` additionally seals declaration-order index and
schema fingerprint. It performs no Builder, MIR, metadata, site-ID,
type/origin, contract, or registry mutation. Production consumers remain zero.

Focused evidence:

```text
cargo fmt --check
cargo test -q --lib weak_field_write_route   # 3 passed
cargo check --all-targets
source file = 163 lines (<800)
```

### `WEAKFIELD-CLASSIFY0-P0` — closed (2026-07-20)

The pure matrix and existing timing boundaries are fixed by five focused
tests: three classifier cases plus weak success and weak FastMem issuance
cases. The existing ordinary FieldSet failure witness remains the pre-I0
baseline (`FieldSet=0`, access site count `1`). No production route consumer
or metadata timing changed.

Focused evidence:

```text
cargo fmt --check
cargo test -q --lib weak_field_write_route   # 3 passed
cargo test -q --lib weak_field_write         # 5 passed
cargo test -q --lib mir::builder::fields::tests::ordinary_fieldset_failure_currently_leaves_only_the_pre_emission_site -- --exact
cargo check --all-targets
```

`WEAKFIELD-CLASSIFY0-I0` is now the sole next row. It may change only route
classification/issuance ownership; ordinary access-site timing remains
owned by the later FieldStore row.

### `WEAKFIELD-CLASSIFY0-I0` — closed (2026-07-20)

The field assignment path now classifies once through the prepared route
product. `KnownWeak` is consumed by a prepared issuer; the issuer performs the
existing FastMem check, allocates its physical site ID, and emits the same
`WeakFieldWrite`. The declaration registry is no longer re-read by issuance.
Ordinary, typed-array, and FastMem route timing remains unchanged.

Focused evidence:

```text
cargo fmt --check
cargo test -q --lib weak_field_write_route
cargo test -q --lib weak_field_write
cargo test -q --lib mir::builder::fields::tests::ordinary_field_access_records_site_metadata -- --exact
cargo check --all-targets
source files: weak_field_write_route=173, weak_field_write=141, fields=399
```

`WEAKFIELD-CLASSIFY0-G0` is now the sole next row. It must freeze one
classifier, one route consumer, one prepared issuer, and zero registry
re-queries before FieldStore P0 begins.

### `WEAKFIELD-CLASSIFY0-G0` — closed (2026-07-20)

The existing row-guard manifest now owns one dedicated
`mirbuilder-weakfield-classification-authority` guard. It freezes one
classifier, one production route consumer, one prepared issuer, zero old bool
emitters, zero issuer declaration-registry queries, zero `weak_fields_by_box`
classifier reads, and zero prepared-product Builder/metadata/site-ID fields.

Evidence:

```text
tools/checks/run_row_guard.sh --only mirbuilder-weakfield-classification-authority
python3 tools/checks/guard_manifest_inventory.py --root .
```

The guard reports `classifier=1 consumer=1 issuer=1 registry_requery=0` and
the manifest inventory remains green. `FIELDSTORE-OBSERVE0-P0` is now the sole
next code-facing row.

## Authority

The classifier reads only the existing base origin
`type_ctx.value_origin_newbox[base]` and the exact field declaration in
`comp_ctx.user_box_field_decls[owner]`.

| Current input | Route |
| --- | --- |
| base origin absent | Ordinary |
| owner declaration absent | Ordinary |
| exact field absent | Ordinary |
| declared non-weak field | Ordinary |
| declared weak field | KnownWeak |

`weak_fields_by_box`, typed-array contract identity, FastMem region, runtime
tags, MIR/source names, final metadata, route success, fallback, and retry are
not classification authorities.

Both route variants retain the exact `region`, `base`, `field`, and `value`
inputs. `KnownWeak` additionally retains exact owner and field spelling,
declaration-order index, and schema fingerprint. It derives the existing weak
contract ID from fingerprint plus index. It owns no Builder, metadata, site ID,
type/origin fact, typed-array identity, instruction, or mutable registry.

## Timing law

`region = Some` never changes classification: a weak field is still
`KnownWeak`. The preserved weak sequence is classification, existing
pre-emission site append, then prepared weak issuance; FastMem rejection remains
at the final step with its current error and site timing.

After `WEAKFIELD-CLASSIFY0-I0`, all current route timings remain unchanged:

- KnownWeak: pre-emission site then weak issuance.
- Ordinary FastMem: pre-emission site then existing FastMem FieldStore.
- Ordinary typed-array: pre-emission site then existing contract/FieldSet path.
- Ordinary no-FastMem/no-contract: pre-emission site then FieldSet.

Only the final lane changes later in `FIELDSTORE-OBSERVE0-I0` to FieldSet
receipt followed by one site commit.

## Row contracts

`WEAKFIELD-CLASSIFY0-S0` adds only private route/product vocabulary, typed
errors, and pure tests. It has zero production consumers and zero Builder,
emission, site, contract, type, or origin writes.

`WEAKFIELD-CLASSIFY0-P0` proves the classifier/product matrix and old-path
parity for weak success, weak emission failure, weak FastMem, ordinary failure,
ordinary typed-array, and ordinary FastMem.

`WEAKFIELD-CLASSIFY0-I0` has behavior delta zero. It replaces the bool-returning
classify-and-emit helper with one classifier and one consuming prepared weak
issuer. The field route classifies once; the issuer does no registry query.
Ordinary access-site timing remains pre-emission.

`WEAKFIELD-CLASSIFY0-G0` guards one classifier, one route consumer, one
prepared issuer consumer, zero old bool emitters, zero issuer registry queries,
zero `weak_fields_by_box` classifier reads, and zero Clone/Builder/metadata/site
ID fields in a prepared product.

`FIELDSTORE-OBSERVE0-P0` then freezes the classified route matrix.
`FIELDSTORE-OBSERVE0-I0` alone admits Ordinary + `region=None` + no declared
field contract. It changes failed FieldSet from site one to site zero, with no
origin delta or retry. Weak, FastMem, and typed-array timing remains unchanged.

### `FIELDSTORE-OBSERVE0-P0` — closed (2026-07-20)

The classified route matrix is frozen without changing production timing.
Focused route tests cover ordinary declaration-backed writes with and without
FastMem regions, weak writes before the existing FastMem error boundary, and
exact preservation of base/value/field/region inputs. Existing Builder tests
continue to cover weak success, weak FastMem failure, ordinary FieldSet
success-site observation, and the pre-I0 ordinary FieldSet failure witness
(`FieldSet=0`, site count `1`).

Focused evidence:

```text
cargo fmt --check
cargo test -q --lib weak_field_write_route   # 5 passed
cargo test -q --lib weak_field_write         # 5 passed
cargo test -q --lib mir::builder::fields::tests::ordinary_fieldset_failure_currently_leaves_only_the_pre_emission_site -- --exact
cargo check --all-targets
```

The matrix is behavior-neutral: weak, FastMem, typed-array, and ordinary
no-contract routes still append their access site before physical emission.
`FIELDSTORE-OBSERVE0-I0` remains the sole owner of moving only the ordinary
no-FastMem/no-contract site append behind successful FieldSet receipt.

### `FIELDSTORE-OBSERVE0-I0` — closed (2026-07-20)

Only the ordinary, no-FastMem, no-declared-contract lane now prepares its
existing access-site descriptor and commits it after successful `FieldSet`.
Weak, FastMem, and typed-array lanes retain their pre-emission site timing.
Failed ordinary `FieldSet` therefore publishes neither the instruction nor the
access-site metadata, with no retry or origin change.

Focused evidence:

```text
cargo fmt --check
cargo test -q --lib mir::builder::fields::tests::ordinary_field_access_records_site_metadata -- --exact
cargo test -q --lib mir::builder::fields::tests::ordinary_fieldset_failure_leaves_no_access_site_after_receipt_cutover -- --exact
cargo test -q --lib weak_field_write               # 7 passed
cargo check --all-targets
```

`PreparedOrdinaryFieldStoreAccessSiteV1::commit` is the sole ordinary receipt
consumer. It owns no rollback or ValueId allocation; it only appends the
already-resolved site after the physical `FieldSet` succeeds.

### `FIELDSTORE-OBSERVE0-G0` — closed (2026-07-20)

The existing row manifest now guards one ordinary receipt product, one commit
owner, and one production consumer, plus the post-cutover failure witness.
The guard does not inspect weak, FastMem, or typed-array timing, so those lanes
remain owned by their existing producers.

Evidence:

```text
tools/checks/run_row_guard.sh --only mirbuilder-fieldstore-observe-authority
```

`MIRBUILDER-FSESSION0-D0` is now the next design row; no broader field
transaction or whole-Builder rollback was introduced.

## Required fixtures and stops

Fixtures cover Ordinary missing-origin/owner/field/nonweak cases; typed and
untyped KnownWeak cases at index zero and nonzero; declaration reorder; weak
FastMem classification plus current issuer error; weak success (`site=1`,
`WeakFieldWrite=1`, `FieldSet=0`); weak emission failure; and ordinary failed
FieldSet before (`site=1`) and after (`site=0`) FIELDSTORE I0.

Stop for a new consultation if implementation needs an issuer registry re-query,
a fields.rs reclassification, `weak_fields_by_box` as a second authority,
Builder/metadata/site ID in a product, provisional site cancellation, weak
post-success timing, classification-time FastMem failure, typed-array timing
change, ordinary receipt movement in WEAKFIELD I0, generic field transactions,
persistent route/ValueId maps, fallback/retry, or a source/check file of 800+
lines.

## Decision lock

> **WEAKFIELD-CLASSIFY0 selects Candidate A-prime. One non-Clone prepared route
> product classifies from base origin plus the declaration registry; KnownWeak
> issuance consumes it without re-querying declarations and preserves weak-site
> and FastMem timing. WEAKFIELD I0 is behavior-neutral. FIELDSTORE I0 alone
> owns ordinary non-FastMem, no-contract FieldSet receipt migration.**
