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

`WEAKFIELD-CLASSIFY0-S0` is the sole next code-facing row.

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

`WEAKFIELD-CLASSIFY0-P0` is now the sole next row. It must compare the pure
classifier/product matrix with the current weak and ordinary route timing
before the I0 authority cutover.

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
