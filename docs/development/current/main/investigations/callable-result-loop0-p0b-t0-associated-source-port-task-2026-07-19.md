---
Status: T0-R0 D0-S0 closed; shared Parts dispatcher cutover next
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

#### T0-R0 worker-audit decision lock

Three read-only audits found no design-consultation blocker. R0 is one BoxShape
row with this internal implementation order:

```text
R0-V0
  bind one O0 representation to one located expression port

R0-C0
  extract one port-aware If-condition tail core

R0-D0
  parameterize the existing Parts recipe dispatcher with one source provider

R0-P0
  raw/associated parity, actual strict-carrier proof, and guard closeout
```

These are implementation checkpoints inside `LOOP0-P0b-T0-R0`, not new public
rows or independently landable semantic authorities.

##### R0-V0: bound borrowed view

The O0 representation remains non-Clone and keeps all fields private. It gains
one same-call binding entry equivalent to:

```text
VerifiedLocatedGenericLoopBodyRepresentationV1
  + LocatedLoopPlanExpressionPortV1
  -> BoundLocatedGenericLoopBodyViewV1
```

The constructor first requires the port to recognize the representation's
exact stored Loop root. Failure occurs before Builder mutation. The bound view
borrows both products and exposes only the already-sealed direct prefix,
cleanup, recipe block/item, and branch carriers needed by Parts. It does not
expose constructors that can independently pair a recipe, source carrier, or
step disposition.

The located port may add borrowed input forms for retained O0 carriers. This is
a lifetime adapter only:

```text
borrowed expression carrier
borrowed statement carrier
borrowed body carrier
```

It owns no site, target, ABI, ledger, or recipe policy.

##### R0-C0: If-condition associated-input seam

The audits found one required seam not visible in the earlier short summary:
existing Parts If owners pass a cloned/raw `CondBlockView` to
`lower_cond_branch`. A located route must not use that view as its syntax
authority.

R0 therefore extracts one neutral associated-input tail core from the existing
If-plan condition owner. The raw facade retains prelude handling and selects a
raw port. The core receives the exact condition input and preserves the current
`!`, `&&`, `||`, comparison, join, freshening, and branch semantics. Leaves use
the existing `PlanNormalizer::lower_value_input` and
`PlanNormalizer::lower_compare_input` owners.

```text
raw If condition:
  existing CondBlockView prelude
  -> raw port
  -> shared associated-input tail core

located If condition:
  exact O0 IfCondition carrier
  -> same shared tail core
```

PATH0 already owns `IfCondition`, `IfThen`, `IfElse`, `BlockExprPrelude`, and
`BlockExprTail`. R0 adds no child-role vocabulary and no condition dispatcher.

##### R0-V0/C0 closeout

V0 and C0 are closed as one behavior-neutral Refactor Series milestone. The O0
product now binds to the exact located port only after that port revalidates the
stored Loop root. The resulting non-Clone view borrows the exact condition,
direct prefix, cleanup, explicit If branches, and retained
`StmtWrappedJoinIf` singleton product; it rebuilds no recipe and owns no
Builder or ledger state.

The located port now has lifetime-only borrowed expression, statement, and
body carriers. One associated-input If-condition tail owner consumes those
carriers through the existing PATH0 child roles. The raw `CondBlockView`
facades retain prelude policy and delegate only the tail. A worker review found
and closed one compatibility seam before landing: the old raw owner admitted
arithmetic/value leaves through generic value lowering, so the new shared
condition-value entry preserves that fallback instead of narrowing all leaves
to the bool-expression subset.

Evidence:

```text
bound O0 view fixtures: 3/3
If-condition raw/associated + exact located-site fixtures: 3/3
arithmetic, comparison, Not, and joinless And/Or leaf parity: green
actual borrowed LoopCondition located call occurrences: 3, Unlocated: 0
generic_loop: 84/84
cargo check --all-targets: green
public expression-spine guard: green
production located execution callers: 0
ledger claims: 0
files at or above 800 lines: 0
```

R0-D0 is next. It must parameterize the existing Parts recipe dispatcher; it
must not add a GenericLoop-local second statement/If dispatcher.

##### R0-D0: one recipe dispatcher, one source provider

The existing `RecipeItem` semantic match in Parts remains the sole dispatcher.
It is parameterized by one neutral source-provider boundary. The raw provider
keeps the current `RecipeBody::get_ref` and `CondBlockView` behavior. The
located provider reads only the bound O0 view.

The provider supplies these already-associated inputs:

```text
OpaqueStmt:
  exact statement input

OpaqueExit:
  exact statement input + retained ExitKind

ExplicitIfV2:
  exact condition, then body, optional else body, contract, and child blocks

StmtWrappedJoinIf:
  retained singleton NoExit recipe/root
  + exact condition, then body, and optional else body
```

###### R0-D0-S0 closeout

The disconnected association vocabulary is closed before the semantic
dispatcher cutover. One Parts-private sealed provider contract has exactly two
implementations. Raw blocks retain their issuing `RecipeBodies`; located
providers retain the exact expression port borrowed by the O0 lowering view.
Every published item keeps that port and its source carriers together in one
private product. Foreign raw arena/block and located port/block pairings reject
before either cardinality or item publication.

Raw body inputs are `&[ASTNode]`, matching the existing raw expression-port
contract. The located provider projects the actual strict five-item root and
retains the existing wrapped-Join singleton product without rebuilding it.
The product constructor is private, the provider trait is sealed, and no
production consumer can split or fabricate the pair.

Evidence:

```text
raw projection: green
actual strict located projection: green
foreign raw pairing rejection: green
foreign located pairing rejection: green
focused fixtures: 4/4
public expression-spine guard: green
Builder/lowering/production consumers: 0
located execution callers: 0
ledger claims: 0
files at or above 800 lines: 0
```

This is not full R0-D0 closeout. The next slice consumes the verified item
by value in one shared semantic dispatcher, replaces the three existing Parts
`RecipeItem` semantic matches with thin raw facades, and adds ExitOnly,
ExitAllowed, and NoExit/Join lowering parity. Port and item must not gain
independent production accessors.

Statement leaves reuse the B0 associated-input primitives. Return values use
the exact expression child. Existing Parts owners retain exit state, join
snapshot/payload, binding publication, and terminality semantics.

`StmtWrappedJoinIf` never calls `try_build_no_exit_block_recipe`, never
reclassifies the source If, and never treats the singleton recipe's cloned AST
or `CondBlockView` as located syntax. The retained singleton product is the
already-selected Join proof; its exact O0 branch carriers are the only source
inputs used by the shared Parts semantics.

##### R0-P0: required proof

Focused fixtures fix:

```text
raw parity:
  ExitOnly
  ExitAllowed
  NoExit / Join

associated items:
  OpaqueStmt Local and Assignment
  OpaqueExit Return
  ExplicitIfV2 with exact condition/then/else presence
  StmtWrappedJoinIf with retained singleton Join product

actual strict root:
  item 0 Local
  item 1 Local
  item 2 IfCondition + IfThen(ReturnValue)
  item 3 Local
  item 4 wrapped IfCondition + IfThen(AssignmentValue)
         + IfElse(AssignmentValue)

failure:
  foreign or mismatched port/product pairing rejects before Builder effects
  no later statement trace or plan is produced
```

The guard extends the existing private T0 helper. No shell, manifest, or public
guard family is added. It fixes one bound-view constructor, one associated
condition owner, one parameterized recipe dispatcher, zero recipe rebuilds,
zero side maps, zero production located callers, zero ledger claims, and the
800-line cap.

Recommended physical split:

```text
parts/associated_source.rs
parts/associated_source_tests.rs
normalizer/cond_lowering_if_plan_port.rs
generic_loop/located_representation/parts_associated_source_tests.rs
```

`parts/stmt.rs`, `parts/dispatch/block.rs`, and `parts/wiring_tests.rs` should
not receive new test bodies. Any necessary edits there are thin delegation or
generic threading only.

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
