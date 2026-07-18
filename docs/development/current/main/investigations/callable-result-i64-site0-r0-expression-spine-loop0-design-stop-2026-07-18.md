---
Status: External design consultation required
Date: 2026-07-18
Baseline: 2a87a3bbe91318f52154b97ff5fadc8ee24d5dec
Decision: no unique existing Loop child-demand authority
Parent: callable-result-i64-site0-r0-expression-spine-task-2026-07-18.md
Scope: read-only LOOP0-D0 authority audit; implementation delta zero
---

# Callable-result SITE0-R0 expression-spine LOOP0 design stop

## Result

Three read-only worker audits agree on one result:

```text
LOOP0-D0 classification:
  AMBIGUOUS

locally selected S0 owner:
  none

implementation authorization:
  none
```

The current Loop path has one raw ingress, but no single owner that can carry
the located condition/body children through route selection, CorePlan
construction, and selected emission without losing source-site identity.
LOOP0 therefore stops at an external authority consultation. Earlier
expression-spine rows remain landed behavior-neutral prerequisites.

## Exact blocker

The selected source is
`ParserBox.static_const_parse_add/2`. Nine of its fifteen direct-call rows are
inside one Loop:

```text
6  Body(4).LoopCondition.Lhs.Rhs              text.length
7  Body(4).LoopCondition.Rhs.Lhs.Lhs          text.substring
8  Body(4).LoopCondition.Rhs.Rhs.Lhs          text.substring
9  Body(4).LoopBody(0).Initializer(0)         text.substring
10 Body(4).LoopBody(1).Initializer(0)         me.static_const_parse_mul
11 Body(4).LoopBody(2).IfCondition.Lhs        me.static_const_eval_is_error
12 Body(4).LoopBody(3).Initializer(0)         me.static_const_eval_value
13 Body(4).LoopBody(5).Value                  ParserStringUtilsBox.skip_ws
14 Body(4).LoopBody(5).Value.Argument(1)      me.static_const_eval_pos
```

The exact call-row law requires source coverage order, including row 13 before
its nested argument row 14. The currently selected `generic_loop_v1` pipeline
orchestrates body work before condition work, while the source ledger requires
condition rows 6-8 before body rows 9-14. Route order is therefore not source
claim order.

The raw production path is:

```text
block_stmt::build_statement
  -> build_expression(ASTNode::Loop)
  -> MirBuilder::cf_loop
  -> try_cf_loop_joinir
  -> LoopRouteContext {
       condition: &ASTNode,
       body: &[ASTNode],
     }
  -> route / recipe / PlanNormalizer
  -> CoreEffectPlan::{MethodCall, GlobalCall { func, args }, ...}
  -> PlanLowerer::emit_effect
```

`PlanNormalizer::lower_value_ast` recursively consumes raw syntax. The
resulting `CoreEffectPlan` retains ValueIds, target spelling, and arguments,
but not the exact `SourceExprSiteV1`, canonical callable row, profile coverage
identity, or ledger order. At `PlanLowerer`, attaching a site is already too
late; reconstructing one from AST equality, span, name, argument shape, or
emission order is forbidden.

## Why the existing owners are insufficient

| Candidate existing owner | Blocking fact |
| --- | --- |
| `MirBuilder::cf_loop` | one raw ingress, not a child-demand owner |
| `LoopRouteContext` | owns raw AST only; route/facts consumers re-observe it |
| `LegacyBlockDescentPortV1` | suffix/statement boundary only; active Loop body is outside its authority |
| `RecursiveChildLoweringPortV1` | ordinary located lowering only; does not own JoinIR Loop planning |
| `PlanNormalizer::lower_value_ast` | central value lowering, but mutates Builder and erases exact source-site identity |
| `CoreEffectPlan` | durable plan vocabulary, but current call rows are source-site-free and Clone/remap-capable |
| `PlanLowerer::emit_effect` | selected mutation owner, but exact site and source claim order have already been erased |

Facts/planning may inspect multiple candidates before one plan is selected.
Claiming the exact ledger during this phase can poison the session when a
later verifier/lowerer step fails. Waiting until MIR emission avoids
speculative claims but loses the identity and required outer-before-argument
order. This transaction law has no current owner.

## Authority and non-authority

| Concern | Current authority | Must remain non-authority |
| --- | --- | --- |
| exact source call site | sealed located caller plan | AST equality, span, target name, MIR order |
| canonical target/ABI/effect row | verified direct-call profile | `CoreEffectPlan::GlobalCall.func` |
| route/recipe selection | existing Loop router/planner | located ledger |
| CFG/PHI/loop-stack mutation | existing PlanLowerer/Loop owners | new located adapter |
| exact-once claim state | caller ledger | Builder fields, speculative facts/planner |
| source claim order | canonical coverage order | body-first route orchestration |
| suffix routing | closed SUFFIX0 authority | LOOP0 changes |

## Competing durable shapes

### Candidate A — located child-demand port through PlanNormalizer

Thread one associated located input through `LoopRouteContext`, facts/recipe
selection, and `PlanNormalizer::lower_value_ast`.

Open questions:

- whether every route and shadow path can share one port without duplicating
  route policy;
- whether planner observation of borrowed syntax is permitted or facts must be
  sealed before Builder effects;
- how exact call rows survive CorePlan Clone/freshening/remap;
- how source-order claims remain separate from body-first orchestration.

### Candidate B — site-bearing `CoreEffectPlan`

Make call effects retain an opaque source-site or prepared direct-call
identity.

Open questions:

- whether this makes CorePlan a second call-target/ABI/effect authority;
- whether a non-Clone sealed row can lawfully cross Clone/remap operations;
- whether all current CoreEffectPlan constructors and consumers must widen;
- where exact-once claim occurs after a plan is selected.

### Candidate C — co-sealed located CorePlan wrapper

Keep CorePlan unchanged and pair one completed plan with an exact non-Clone
source-site/effect correspondence product.

Open questions:

- which owner proves the correspondence before source identity is erased;
- whether the wrapper becomes a second CorePlan truth;
- how remapped ValueIds and nested argument order remain exact;
- whether selected emission can consume it transactionally without storing it
  in `MirBuilder`.

### Rejected local shortcuts

```text
bypass CorePlan and lower located Loop children through a separate CFG route
AST rewrite or placeholder calls
site reconstruction from names, spans, syntax equality, or MIR order
ledger claim during speculative planner analysis
plan/site/ledger storage in MirBuilder
raw located fallback or route retry
GenericLoop-v1 source-name special case
changing the closed suffix router
```

## Consultation questions

The external decision must lock all of the following together:

1. Which durable product owns source-site/canonical-call-row identity across
   CorePlan: a generic located port, a site-bearing effect, or a co-sealed
   located-plan wrapper?
2. May planner/facts inspect borrowed located syntax, or must all location facts
   be sealed before Builder mutation begins?
3. Where does the exact-once ledger claim occur so candidate inspection is
   observation-only and selected failure cannot poison a reusable session?
4. Must the Loop pipeline become two-phase so pure analysis preserves canonical
   condition-then-body claim order independently of body-first lowering?
5. What are the Clone, lifetime, ValueId-remap, and transaction laws for the
   selected product?
6. Must normalized-shadow use the same carrier, or must the located route
   reject it before effects with no fallthrough/retry?
7. What is the exact first code-facing row and its production consumer count?

## Post-decision task skeleton

This order is tentative; the consultation must name the S0 owner before it is
actionable.

```text
LOOP0-S0
  disconnected selected carrier/driver
  production consumers = 0

LOOP0-P0
  actual generic_loop_v1 plus alternate-route parity and failure proof
  production consumers = 0

LOOP0-I0
  exactly one production selection
  no raw retry/fallback

LOOP0-L0
  located condition/body adapter using the selected authority

EXPR0-C0
  one root connector and final exact-ledger completion
```

## Required proof matrix after selection

```text
actual static_const_parse_add with all 15 rows
Loop condition rows 6-8 and body rows 9-14
outer row 13 claimed before nested argument row 14
short-circuit RHS remains lazy
route kind + recipe + child-demand trace parity
full MIR/CFG/type/binding/loop-stack normalized parity
condition, route, body, and selector failure boundaries
pre-existing partial-effect parity where the raw route already mutates
speculative candidate observation claims zero rows
selected emission claims each row exactly once
session poison on selected failure; no finish and no retry
fresh valid compiler/Builder reuse
debug/release and environment lock/restore parity
normalized-shadow exact admission or pre-effect rejection
```

Final-MIR-only equality, a simple Loop without short-circuit calls, or an
always-disabled located port are false-green tests.

## Counters and guards

```text
production located Loop consumers before selected I0 = 0
speculative planner ledger claims = 0
selected emission exact claims = expected rows exactly once
AST rewalk/site reconstruction = 0
raw Lower FunctionCall.name authority on selected Loop rows = 0
CorePlan target spelling as callable-row authority = 0
plan/view/ledger fields added to MirBuilder = 0
route/recipe owner duplication = 0
located/raw fallback or retry = 0
suffix-router behavior delta = 0
grammar/runtime/backend/ownership delta = 0
source/check files >= 800 lines = 0
```

## Implementation may claim now

```text
one raw Loop ingress and the complete route/effect pipeline are inventoried
the actual nine Loop call rows and exact source order are fixed
source-site identity is erased before the current common effect lowerer
no existing child-demand owner uniquely satisfies the located ledger law
LOOP0 requires an authority and transaction decision before implementation
```

## Implementation must not claim now

```text
a selected Loop associated-input owner
general located Loop lowering
site-bearing CorePlan authority
transaction-safe exact-once claims through speculative planning
condition/body source-order lowering
normalized-shadow support
callable-result production activation
new grammar, runtime, backend, or ownership support
```

## Stop conditions

Stop any follow-up implementation if it requires:

1. passing raw active condition/body through a nominally located adapter;
2. raw PlanNormalizer descent over an active located subtree;
3. reconstructing a site after CorePlan creation;
4. storing the plan, view, site table, or ledger in `MirBuilder`;
5. adding a site/proof field ad hoc to one CoreEffectPlan constructor;
6. cloning a non-Clone proof or losing identity during ValueId remap;
7. claiming before complete plan selection or after source identity loss;
8. duplicating route, recipe, normalizer, CFG, or PHI policy;
9. changing SUFFIX0 or lowering an active body through its inactive proof;
10. fallback, retry, input probing, or source-name special cases;
11. treating a CorePlan schema widening as behavior-neutral;
12. using final MIR equality without route/demand/claim evidence.

## Final lock

> LOOP0-D0 finds no unique existing child-demand authority. The current raw
> Loop path converts located source calls into source-site-free CoreEffectPlan
> rows before selected MIR emission, while speculative planning is too early
> for exact-once ledger claims and the selected generic route's body-first
> orchestration differs from canonical condition-then-body claim order.
> Therefore no LOOP0 implementation owner is locally selected. External design
> must choose a generic located PlanNormalizer port, a site-bearing CoreEffect
> product, or a non-Clone co-sealed located CorePlan wrapper, and must lock its
> lifetime/remap/transaction/source-order laws. Until that decision, production
> located Loop consumers, ledger claims, route changes, and behavior deltas
> remain zero.
