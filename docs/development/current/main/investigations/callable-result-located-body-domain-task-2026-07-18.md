---
Status: active prerequisite
Date: 2026-07-18
Decision: canonical located-body domain before IF0-L0
Parent: callable-result-i64-site0-r0-expression-spine-task-2026-07-18.md
Scope: behavior-neutral located carrier and ledger repair
---

# Callable-result located body-domain task

## Finding

IF0-L0 exposed a correctness false-green before production connection.
Canonical activation rows flatten body items beside their semantic body root:

```text
row:     Body(0).IfThen(0).Initializer(0)
carrier: Body(0).IfThenBody
```

`LegacyBodyInputV1` currently appends `IfThen(0)` to the carrier root and the
caller ledger applies literal `starts_with`. The selected row is therefore
outside the literal root prefix: `prove_body_inactive` succeeds and raw branch
lowering can run even though an activation row exists.

The same mismatch exists for `IfElseBody` / `IfElse(_)` and `LoopBodyRoot` /
`LoopBody(_)`. A focused pre-connection fixture observed the located If return
`Ok` for active then and else rows. The implementation WIP is evidence-only in
stash `wip/if0-l0 body-prefix vocabulary mismatch (fails focused gate)` and
must not be restored wholesale.

## Decision

Select one typed canonical body-domain owner:

```text
SITE0-R0-LDG0-BODYDOMAIN0-S0
  -> BODYDOMAIN0-I0
  -> resume IF0-L0
```

The activation-row producer remains unchanged. `IfThenBody`, `IfElseBody`,
and `LoopBodyRoot` are semantic body/scope identities, not literal ancestors
of canonical statement sites.

The located body carrier retains:

```text
parent statement site
SourceBodyKindV1
body statements
plan/caller identity
```

`body_stmt(index)` emits the canonical item site:

```text
parent + kind.item_segment(index)
```

The ledger proves a body inactive only when no row belongs to its typed domain:

```text
row starts with parent
next segment is exactly the item family owned by body kind
all remaining descendant segments stay inside that item
```

Function-root bodies retain `Body(_)` membership. Statement and expression
inactive proofs retain their existing literal-prefix law.

## Authority and non-authority

| Concern | Authority |
| --- | --- |
| canonical statement/expression sites | existing activation/source-path producers |
| body scope identity | existing `SourceBodyKindV1` root vocabulary |
| body item family | existing `SourceBodyKindV1::item_segment` vocabulary |
| located body membership | new immutable typed body-domain parts |
| exact row consumption/order | existing caller ledger |

This row does not own or change:

```text
activation row paths
source target/result catalogs
statement/expression prefix semantics
If/Loop selection or lowering
raw subtree admission
Builder state
runtime/backend behavior
```

## S0 — disconnected body-domain product

Add one typed body-domain carrier/parts shape and exact membership decision.
It must distinguish body kinds and must not skip arbitrary path segments.

Required fixtures:

```text
IfThen body catches Body(0).IfThen(0).Value
IfElse body catches Body(0).IfElse(0).Value
Loop body catches Body(0).LoopBody(0).Value
nested descendants remain inside the direct item domain
then and else do not cover one another
IfCondition is outside both branch domains
different parent statements do not overlap
empty body is inactive
function root covers Body(_)
foreign and unlocated carriers reject
```

## I0 — caller-ledger connection

Connect `prove_body_inactive` to the typed body-domain decision exactly once.
Keep `prove_stmt_inactive` and `prove_expr_inactive` on literal prefix proof.
Make `body_stmt` publish canonical parent-plus-item sites and prove they
round-trip through the existing source view/projection.

Integration fixtures must show active then, else, and loop body rows fail with
exact `RowsUnderPrefix` witnesses before raw Call/Return effects. The located
session becomes poisoned; a fresh independent session remains usable.

## Guards

```text
typed body-domain owners = 1
caller-ledger body-domain consumers = 1
stmt/expr literal-prefix owners = 1
root-plus-item located paths = 0
activation producer path delta = 0
source projection/shadow path delta = 0
If/Loop selector callers = 0
raw body effects after active-row proof failure = 0
fallback/retry/path probing = 0
Builder-stored body domains = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop if any implementation requires:

1. adding body-root segments to activation rows;
2. deleting semantic body-root identity from source-path vocabulary;
3. treating body root as a global segment-equivalence rule;
4. parent-only prefix matching without exact body-kind item classification;
5. changing statement/expression prefix law;
6. AST rewalk, name/span heuristics, or reconstructed row order;
7. activating If branches, Loop bodies, suffix routing, or a production root;
8. retry, fallback, or raw delegation after proof failure;
9. a persistent Builder path/domain table;
10. a source/check file reaching 800 lines.

## Closeout

BODYDOMAIN0 is behavior-neutral and production callers remain zero. Once its
focused tests, callable-result suite, structural guard, check, release, and
line caps are green, resume IF0-L0 from a clean reimplementation. Do not apply
the failed IF0-L0 stash wholesale.
