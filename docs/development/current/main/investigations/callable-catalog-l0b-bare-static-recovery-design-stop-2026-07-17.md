---
Status: design consultation stop
Date: 2026-07-17
Baseline: 31bab44f65
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Scope: declaration-catalog L0b versus declaration-order-dependent bare-static recovery
---

# Callable catalog L0b bare-static recovery design stop

## Stop result

Two worker audits agree that one catalog should contain exactly the union of
the two legacy declaration stores:

```text
StaticBoxMethod:
  every FunctionDeclaration in a non-sync, non-record static box

InstanceBoxMethod:
  every non-static method in a non-sync, non-record instance box
```

Constructors, top-level functions, ordinary-box static methods, records, and
sync boxes remain outside this catalog. Static candidate lookup must filter to
`StaticBoxMethod`; future exact-i64 result analysis must also consume static
rows only.

That namespace correction is local and behavior-neutral. The production
cutover is not.

## Exact observed behavior delta

The old `static_method_index` is populated once during declaration indexing,
then populated again after each static method is lowered in
`module_lifecycle.rs` or `exprs.rs`. Therefore a provider already lowered in
the current static box can appear twice, while a provider not yet lowered
appears once.

Two HMI-independent probes were compared against baseline `31bab44f65` and the
catalog-cutover WIP.

### Provider lowered first

```hako
static box Helpers {
    seed(x) { return x + 1 }
    use(x) { return seed(x) }
}

print(Helpers.use(1))
```

```text
baseline:
  compile reject
  Unresolved function: 'seed'

catalog WIP:
  compile succeeds
  reaches the unavailable-reference-backend boundary
```

### Caller lowered first

```hako
static box Helpers {
    a_use(x) { return z_seed(x) }
    z_seed(x) { return x + 1 }
}

print(Helpers.a_use(1))
```

```text
baseline:
  compile succeeds

catalog WIP:
  compile succeeds
```

Exact process results without `vm-reference` were:

```text
baseline provider-first: rc=1, unresolved seed
catalog  provider-first: rc=2, reference backend unavailable after compile
baseline caller-first:   rc=2, reference backend unavailable after compile
catalog  caller-first:   rc=2, reference backend unavailable after compile
```

The catalog removes accidental duplicate rows and therefore makes unique
bare-static recovery declaration-order independent. Calling this
`production behavior delta: 0` would be false.

## What remains proven

The WIP established the following implementation feasibility before stopping:

```text
one catalog with StaticBoxMethod + InstanceBoxMethod
structured owner/method/arity body queries
no physical-symbol reverse parsing
static candidates exclude instance rows
record-helper consumer-local snapshots only
old body/index stores can be physically removed
catalog tests: 5/5
record-helper tests: 3/3
cargo check: green
historical M0 inventory: reference green after catalog-shape update
all touched source/check files: below 800 lines
```

The existing phase-296x helper guard cannot currently run because its first
required historical document,
`296x-154-POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH.md`, is absent. That
failure predates and is independent of this cutover.

The implementation is preserved only as evidence in:

```text
wip/callable-catalog-l0b (bare-static recovery behavior delta)
```

It must not be applied or restored wholesale. After the decision, only the
authorized structure is reimplemented or selectively recovered on a clean
tree.

## Candidate A — authorize canonical unique recovery (recommended)

Define bare-static recovery by the complete immutable declaration catalog:

```text
method + arity has exactly one StaticBoxMethod key:
  resolve to that canonical key

zero or multiple keys:
  no recovery
```

This intentionally accepts provider-first and caller-first forms equally.
Lowering order and duplicate registration cease to be semantic inputs.

Suggested task order:

```text
R0-CALLABLE-CATALOG-L0B-S0
  disconnected Static/Instance namespace and normalized parity

R0-BARE-STATIC-RECOVERY0-P0
  baseline split plus canonical unique/ambiguous/app/script fixtures

R0-CALLABLE-CATALOG-L0B-CUT0
  one pre-body catalog producer
  two static-resolution consumers
  structured record-helper body consumers
  old static_method_index/lowered_method_asts stores and writes = 0

R0-CALLABLE-CATALOG-L0B-G0
  authority/caller/reverse-parse/order-dependence guards

then:
  R0-CALLABLE-RESULT-I64-CATALOG0-S0
```

`CUT0` is one explicit semantic row. It may claim declaration-order-
independent unique bare-static recovery, but no broader callable resolution.

## Candidate B — retire bare-static recovery

Remove both old unique-recovery consumers. Require explicit qualified static
calls such as `Helpers.seed(x)`.

This is structurally smaller but rejects currently accepted caller-first bare
calls and may require a source migration census. It must not silently rewrite
or qualify source calls.

## Candidate C — emulate lowering-order multiplicity

Rejected.

Preserving the old split requires a mutable lowering-progress/multiplicity
authority beside the immutable catalog. It would retain declaration-order
semantics, duplicate callable truth, and the exact BoxShape problem L0 was
selected to remove.

## Shared invariants

Whichever accepted candidate is selected:

```text
complete catalog seal occurs before declaration-index side effects
catalog install count per root = 1
StaticBoxMethod and InstanceBoxMethod stay namespace-disjoint
instance rows in static candidate lookup = 0
instance rows in result-contract solver = 0
physical-symbol reverse parsing = 0
second persistent AST/body map = 0
callee-first/re-lowering/retry/fallback = 0
source/check files >= 800 lines = 0
```

## Consultation question

> Should L0b explicitly authorize Candidate A, making unique bare-static
> recovery declaration-order independent from the complete immutable catalog
> and accepting the previously rejected provider-first form? Or should
> Candidate B retire bare-static recovery and require qualified static calls,
> despite rejecting the currently accepted caller-first form? Candidate A is
> recommended because it removes accidental lowering-order authority while
> preserving every previously successful call and adds only the missing
> symmetric case.

## Stop law

No production catalog consumer, old-store deletion, source migration, or
result-contract implementation may land until this semantic choice is locked.
