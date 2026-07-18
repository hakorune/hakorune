---
Status: T0-B0 closed; T0-R0 next
Date: 2026-07-19
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Prior decision: callable-result-i64-site0-r0-expression-spine-loop0-p0b-o0-design-stop-2026-07-19.md
Decision: one T0 semantic row, four-commit Refactor Series
---

# Callable-result LOOP0-P0b-T0 associated-source port task

## Decision

Three independent repository audits select one implementation order without an
external design consultation:

```text
LOOP0-P0b-T0-C0
  condition raw facade + shared expression-port core

LOOP0-P0b-T0-B0
  DirectRecipeOnly body/cleanup port threading

LOOP0-P0b-T0-R0
  neutral Parts associated-source recipe entry

LOOP0-P0b-T0-L0
  same-call disconnected located composer

then
  LOOP0-P0b-P0 parity
```

This is one BoxShape objective split under Refactor Series Mode. C0 through R0
must not add a located production root or ledger claim. L0 remains disconnected
from the production route registry.

## Why the raw body lowerer is not the located owner

The current raw GenericLoop body path still:

```text
reads environment/body policy
reads facts.body/body_no_exit
scans source statements with matches_loop_increment
builds NoExit or ExitAllowed recipes
lowers cloned/raw AST through CondBlockView and Parts
```

Passing the O0 product directly to that entry would reconstruct decisions and
erase the exact PATH0 carriers already sealed by O0. T0 therefore threads one
borrowed `LoopPlanExpressionPortV1` through the existing shared normalizer and
Parts owners. It does not add a GenericLoop-local statement or If dispatcher.

## Authority boundary

```text
source identity:
  O0 exact PATH0 carriers

body mode and cleanup:
  O0 VerifiedLocatedGenericLoopRepresentationV1

expression/statement descent:
  one borrowed LoopPlanExpressionPortV1

recipe semantics:
  existing shared Parts owners

call target/ABI/claim:
  existing activation plan and caller ledger, still disconnected in T0
```

Non-authorities:

```text
facts.body clone
recipe arena cloned AST
AST equality, spans, names, ValueIds, or effect order
environment variables in the located path
body_no_exit presence in the located path
matches_loop_increment after O0 sealing
```

The same port instance must reach Loop condition, direct body statements,
nested If condition/branches, and cleanup. A side map from RecipeItem to source
syntax is forbidden.

## T0-C0 acceptance

C0 is behavior-neutral and owns only the condition seam:

```text
raw lower_loop_header_cond
  keeps existing CondBlockView prelude lowering
  selects RawLoopPlanExpressionPortV1 once
  delegates the tail expression to one shared associated-input core
```

The shared core owns CFG-shaped `!`, `&&`, and `||` descent. Comparison operands
are requested through existing PATH0 BinaryLeft/BinaryRight roles. Every value
leaf is lowered through `PlanNormalizer::lower_value_input`.

Required C0 proof:

```text
raw facade/core normalized effects and branches are equal
short-circuit intermediate blocks remain lazy
existing GenericLoop tests remain green
production located condition callers = 0
ledger claims = 0
files at or above 800 lines = 0
```

### C0 closeout

The raw facade now retains only `CondBlockView` prelude lowering and one raw
port selection. One shared port core owns lazy CFG descent. The existing
`PlanNormalizer` compare owner was made port-aware instead of duplicating its
operator table; its raw API is a thin facade and parent comparison span
restoration remains exact.

Evidence:

```text
focused raw facade/port core fixture: 1/1
generic_loop filtered library tests: 77/77
cond_lowering filtered library tests: 4/4
cargo check --all-targets: green
public expression-spine guard: green
current pointer, format, diff, and line guards: green
production located condition callers: 0
ledger claims: 0
```

The fixture fixes the exact lazy shape for `A && (B || C)`: `A` true enters
the And RHS, `A` false exits, `B` true enters the body, `B` false alone reaches
the Or RHS, and only the final comparison returns to body/exit. It also fixes
operand ordering, comparison ownership, and final parent span restoration.

## T0-B0 closeout

B0 keeps all body-mode, environment, `body_no_exit`, and progression-step
selection in the existing raw GenericLoop facade. One associated-input sequence
owner now consumes only an already-selected direct statement prefix and stops
after terminal plans. One associated cleanup owner consumes only the selected
cleanup expression after the existing terminality check.

The existing statement semantics were not copied into GenericLoop. Assignment,
Local initializer, MethodCall, FunctionCall, and Return now have neutral
associated-input primitives under the normalizer; their raw entries select one
`RawLoopPlanExpressionPortV1` and delegate. Nested expression children use the
existing PATH0 roles and `PlanNormalizer::lower_value_input`. Six source-derived
call constructors therefore moved from explicit `Unlocated` construction to
the existing call-source port, while production located producers remain zero.

Evidence:

```text
associated statement parity fixtures: 3/3
direct statement-sequence fixtures: 3/3
cleanup fixtures: 4/4
generic_loop filtered library tests: 81/81
cargo check --all-targets: green
public expression-spine guard: green
current pointer, format, and diff guards: green
production located body callers: 0
ledger claims: 0
source/check files at or above 800 lines: 0
```

The consolidated T0 guard fixes the C0/B0 facade/core boundaries, PATH0 child
roles, raw-only step filtering, terminal-before-cleanup order, focused fixtures,
forbidden authority vocabulary, zero premature located consumers, and line
caps. T0-R0 is the next row and owns only the neutral Parts associated-source
recipe entry.

## Remaining rows

### T0-R0

Add one neutral Parts-associated-source entry. ExitAllowed items use their O0
co-sealed exact carriers. `StmtWrappedJoinIf` consumes the retained singleton
NoExit recipe plus exact condition/then/else carriers; it never rebuilds or
reclassifies the recipe.

### T0-L0

Consume the non-Clone O0 representation in one same-call disconnected located
composer. Condition, body, nested branches, and cleanup share the same port.
Production located root callers, ledger claims, PlanLowerer activation, route
registry changes, and normalized shadow remain zero.

## Guard plan

T0 closeout adds one private helper:

```text
tools/checks/lib/
  callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_t0.py
```

It is imported once by the existing public expression-spine guard. No new
shell, manifest, or Cargo runner is added. The final helper fixes:

```text
one raw facade and one associated-input owner per seam
one same-call located owner with zero production callers
same borrowed port across condition/body/branches/cleanup
zero reclassification, recipe rebuild, side maps, fallback, or retry
focused parity fixtures
source/check files below 800 lines
```

## Stop conditions

Stop and request a new design only if implementation requires any of:

1. a second recipe-item or statement/If dispatcher;
2. a RecipeItem-to-source side map or AST equality pairing;
3. cloned recipe AST as located syntax authority;
4. condition prelude syntax not represented by the existing PATH0 carrier;
5. located environment, facts.body, body_no_exit, or step reclassification;
6. recipe rebuilding for StmtWrappedJoinIf;
7. production located root, ledger, PlanLowerer, registry, or shadow changes;
8. fallback or retry;
9. a source/check file reaching 800 lines.

## Non-claims

```text
general located Loop support
non-GenericLoopV1 route support
normalized-shadow located support
production callable-result activation
ledger transaction completion
Builder rollback
grammar/runtime/backend/ownership widening
```
