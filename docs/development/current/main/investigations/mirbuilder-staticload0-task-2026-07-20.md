---
Status: Active D′ architecture task
Date: 2026-07-20
Scope: One exact `StaticDataLoad` result type producer after CONST0
Parent: docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Predecessor: docs/development/current/main/investigations/mirbuilder-exact0-const-task-2026-07-20.md
---

# STATICLOAD0: sealed u16 static-data result publication

## Decision

`STATICLOAD0-D0` is closed. The next code-facing row is disconnected:

```text
STATICLOAD0-S0
  -> STATICLOAD0-M0
  -> STATICLOAD0-P0
  -> STATICLOAD0-I0
  -> STATICLOAD0-G0
```

The only admitted producer is the existing successful
`MirInstruction::StaticDataLoad` emitted from
`MirBuilder::build_index_expression` for an exact static `u16` data plan.

```text
sealed StaticTableContractSpec::U16
  -> static_data_plan_from_spec
  -> exact StaticDataLoad plan lookup
  -> successful index lowering
  -> successful StaticDataLoad emission
  -> exact Integer transient fact
```

The static table contract is the sole representation authority. The variable
spelling only selects an existing exact `StaticDataPlan`; index AST shape,
index `ValueId`, runtime table contents, runtime class, method name, and JSON
output do not decide the result type.

## Exact boundary

The existing source rejects a non-`u16` plan before lowering the index or
allocating the load destination. It lowers the index, allocates a fresh
destination, emits `StaticDataLoad`, then writes `Integer` to both current
transient type facts and `MirFunction.metadata.value_types`.

Only the former is lowering-time type authority. Finalization later snapshots
the current transient map into finalized metadata. The early metadata write is
therefore a candidate legacy publication, not authority to preserve by default.

`STATICLOAD0` does not remove it in S0. M0 must first prove that no legal
lowering-time consumer requires the entry. If that proof fails, I0 stops; it
does not make finalized metadata a second live authority.

## Selected I0 law, conditional on M0/P0

After M0 proves timing parity, I0 may use the existing `TypeFactDecisionV1`:

```text
fresh dst + candidate Integer
  -> prepare before StaticDataLoad emission
  -> successful emission only
  -> commit Publish(Integer) to current type_ctx
```

The early metadata insert is removed only in I0. `Idempotent` preserves an
exact existing entry; a synthetic concrete conflict rejects before the load.
`Unknown` is a non-fact, not a proposal.

```text
failed index lowering:
  no dst and no type fact

failed StaticDataLoad emission:
  no transient type fact
  no finalized metadata fact

successful load:
  transient Integer immediately
  finalized Integer only through normal finalization snapshot
```

The fresh ValueId cursor is not rolled back. No origin, string, map, record,
receiver, or ownership fact is written.

## Required proof matrix

M0/P0 must independently show:

```text
sealed u16 table -> StaticDataLoad -> transient Integer
normal finalization snapshot -> finalized Integer
post-I0 candidate: metadata entry absent before finalization
unsupported element rejects before destination allocation/load
index lowering failure publishes no load type
load-emission failure publishes no transient/metadata type
synthetic conflicting exact destination rejects before load emission
no origin publication
MIR JSON/VM parity, including runtime out-of-bounds failure
```

The pre-finalization metadata assertion is the hard timing gate. No test may
read final metadata to reconstruct a lowering-time type.

## Scope and exclusions

Included:

```text
one exact static u16 data-plan lookup
one StaticDataLoad instruction family
one Integer result fact
```

Excluded:

```text
ordinary Array/Map indexing
other static element types
index result/type inference
static table value/range policy
runtime bounds semantics
FieldGet / Call / Select / Compare
origin and Unknown retirement
finalization repair or metadata propagation
new static-data grammar, backend, runtime, or ownership behavior
```

## Stop conditions

Stop and open a timing/metadata consultation if any is required:

1. an early metadata read is semantically required before finalization;
2. result type inference from index expression, table name, or runtime value;
3. `Unknown` as an exact proposal or type backfill after failed load;
4. retry, raw fallback, metadata fallback, or a persistent ValueId map;
5. source grammar, runtime bounds, backend, ownership, Array/Map indexing, or
   another static element type change; or
6. a source/check file at or above 800 lines.

## Completion claim

After G0, the compiler may claim only that a successfully emitted static `u16`
load obtains `Integer` from sealed static-data representation through one
post-emission transient-fact owner, and finalized metadata receives that fact
only through normal finalization snapshot. It may not claim general indexing
type inference or metadata/finalization cleanup.
