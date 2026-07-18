---
Status: BIN0-S0 closed; BIN0-I0 is next
Date: 2026-07-18
Decision: expand located lowering by one structural child family per row
Parent: callable-result-i64-catalog0-i0-activation-design-stop-2026-07-17.md
Scope: behavior-neutral associated-input descent before EXPR0-C0
---

# Callable-result SITE0-R0 expression-spine task

## Decision

The actual A0 caller proves that `SITE0-R0-EXPR0-C0` cannot honestly connect
the closed located session directly to BLK0. The next work expands the
**located lowering structural acceptance boundary**, not the source language,
MIR instruction vocabulary, callable-result solver, or runtime semantics.

The selected order is:

```text
SITE0-R0-EXPR0-SPINE0-BIN0-S0
  -> BIN0-I0
  -> BIN0-P0
  -> BIN0-L0

  -> SPINE0-SC0-S0
  -> SC0-I0
  -> SC0-P0
  -> SC0-L0

  -> STMT0-S0
  -> STMT0-I0
  -> STMT0-P0
  -> STMT0-L0

  -> IF0-S0
  -> IF0-I0
  -> IF0-P0
  -> IF0-L0

  -> SUFFIX0-S0
  -> SUFFIX0-P0
  -> SUFFIX0-I0

  -> LOOP0-D0
  -> [mechanically selected Loop rows]

  -> EXPR0-C0
  -> EXPR0-P0
```

The sole next code-facing row is:

```text
SITE0-R0-EXPR0-SPINE0-BIN0-S0
```

`docs_only_closeout = forbidden` for BIN0. It must add the disconnected
associated-input Binary substrate plus focused executable fixtures.

## Why C0 is held

The actual caller is:

```text
ParserBox.static_const_parse_add/2
```

Its A0 plan owns 15 MethodCall rows in exact source order:

```text
 1 Body(0).Initializer(0)                         me.parse_mul
 2 Body(1).IfCondition.Lhs                       me.is_error
 3 Body(2).Initializer(0)                        me.value
 4 Body(3).Value                                 selected skip_ws
 5 Body(3).Value.Argument(1)                     me.pos
 6 Body(4).LoopCondition.Lhs.Rhs                 text.length
 7 Body(4).LoopCondition.Rhs.Lhs.Lhs             text.substring
 8 Body(4).LoopCondition.Rhs.Rhs.Lhs             text.substring
 9 Body(4).LoopBody(0).Initializer(0)            text.substring
10 Body(4).LoopBody(1).Initializer(0)            me.parse_mul
11 Body(4).LoopBody(2).IfCondition.Lhs           me.is_error
12 Body(4).LoopBody(3).Initializer(0)            me.value
13 Body(4).LoopBody(5).Value                     selected skip_ws
14 Body(4).LoopBody(5).Value.Argument(1)         me.pos
15 Body(5).Value                                 me.pair
```

Normalized shape counts:

```text
direct statement-child MethodCall roots: 8
nested outer-MethodCall arguments:       2
non-MethodCall Binary ancestors:         5
active receiver nesting:                 0
active Unary/Array/Index/BlockExpr:       0
```

The closed L0 session can already claim direct MethodCalls and nested
MethodCall arguments. It deliberately rejects row 2 with `RowsUnderPrefix`
because the active MethodCall is below a Binary expression. Therefore this is
an observed boundary, not a speculative widening.

## Authority split

| Concern | Authority |
| --- | --- |
| child structural location | existing PATH0 roles and located source view |
| exact call-site consumption | existing caller ledger |
| ordinary Binary evaluation order | BIN0 associated-input driver |
| `&&` / `||` conditional RHS | SC0 short-circuit CFG owner |
| value-bearing statement preflight/completion | STMT0 statement port |
| If condition and branch timing | IF0 associated-input If owner |
| raw suffix admission | SUFFIX0 exact inactive-suffix proof |
| Loop/CorePlan site carriage | LOOP0, after a dedicated design audit |
| final block order/scope/termination | existing BLK0 driver |
| target/result/effect/type publication | existing route and terminal owners |

None of these rows may store the activation plan, source view, ledger, or a
mutable current site in `MirBuilder`.

## BIN0 — ordinary non-short-circuit Binary

### S0: disconnected substrate

Add one private associated-input boundary, suggested shape:

```rust
trait BinaryExpressionDescentPortV1: RecursiveChildLoweringPortV1 {
    type BinaryInput;

    fn binary_syntax(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<BinarySyntaxViewV1<'_>, String>;

    fn binary_left_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String>;

    fn binary_right_input(
        &self,
        input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String>;
}
```

The driver owns exactly:

```text
operator preflight
left descent
right descent
existing build_binary_op_from_values completion
```

`And` and `Or` must fail closed at this boundary; they belong to SC0.

### I0: raw behavior-neutral cutover

Route the existing ordinary Binary source entry through one raw port. Preserve
left-before-right effects, error priority, recursion-depth accounting, value
materialization, MIR, types, diagnostics, and Builder reuse.

### P0: parity

Required fixtures:

```text
arithmetic and comparison operators
MethodCall in lhs
MethodCall in rhs
nested Binary two through four levels
lhs failure -> rhs/terminal effects 0
rhs failure -> terminal effects 0
raw normalized MIR parity
recursion-depth restoration and Builder reuse
```

### L0: disconnected located acceptance

The located session accepts an active ordinary Binary only by constructing
`BinaryLeft` and `BinaryRight` children through the existing source view.
Inactive Binary remains eligible for exact inactive-prefix raw delegation.
Production located callers and callable-result publication remain zero.

### BIN0-S0 closeout

BIN0-S0 is closed. One private `BinaryExpressionDescentPortV1` and one generic
driver own only borrowed operator observation, child-effect-free `And` / `Or`
rejection, exact left-then-right E0 descent, and one completion through the
existing `build_binary_op_from_values` owner. Operator conversion,
arithmetic/comparison policy, LocalSSA, destination allocation, types,
recursion depth, and short-circuit CFG/PHI remain with their prior owners.

Six focused fixtures prove arithmetic and comparison completion, the exact
16 ordinary versus 2 logical operator boundary, syntax/input/child failure
order, terminal failure after both children, no retry, and fresh-driver reuse.
The reusable SPINE0 guard proves one driver, two E0 descents, zero raw or
located implementations, zero production callers, and no located/ledger/
result authority. Callable-result 48/48, recursive child 7/7, existing
short-circuit 1/1, every prior EXPR0 guard, quick, release build, formatting,
and line caps are green. BIN0-I0 is next.

## SC0 — short-circuit Binary

SC0 keeps `And` and `Or` separate from ordinary Binary because eager RHS
evaluation would change semantics.

The associated-input short-circuit owner must preserve:

```text
lhs descent before branch construction
rhs descent only inside the eval-RHS block
existing short-circuit CFG, PHI, result type, and diagnostics
no eager both-operands lowering
```

Required fixtures include actual nested Loop-condition shape, false-AND and
true-OR RHS suppression, RHS failure only on the evaluated edge, nested
AND/OR/comparison trees, normalized CFG/MIR parity, and fresh Builder reuse.

SC0 adds no result, type, PHI, or control-flow policy. It only lets the
existing short-circuit owner request associated child inputs.

## STMT0 — value-bearing statements

STMT0 extracts one associated-input statement boundary for the actual direct
surfaces:

```text
Local initializer
Assignment RHS
Return value
direct expression statement
```

It must preserve each existing source preflight and then reuse existing
from-value completion helpers. It may not call those helpers while skipping
or duplicating source-level policy. If one shared statement contract cannot
express all four without branching policy duplication, split the row before
implementation.

IF and Loop are not STMT0 variants.

## IF0 — associated-input If control

IF0 lets the existing If lowering request:

```text
IfCondition
IfThen body
optional IfElse body
```

through one associated-input port. It retains the existing IfForm CFG/PHI,
variable snapshots, branch analysis syntax view, termination, and diagnostics.
Condition failure descends neither branch. Located branch bodies never fall
back to raw lowering when their prefix is active.

## SUFFIX0 — exact raw suffix boundary

The current BLK0 `suffix_route_input` exposes raw `&[ASTNode]` without a proof
carrier. A located block must never hand an active suffix to the JoinIR suffix
router.

SUFFIX0 must select and prove one exact law:

```text
raw suffix is exposed only after the caller ledger proves that every row lies
outside body[index..]
```

Silently disabling the suffix router for every located body is not accepted
without normalized behavior evidence. Returning the raw suffix without the
proof is forbidden.

## LOOP0 design boundary

The actual second selected `skip_ws` row is inside a Loop body, and three
standard calls occur in its short-circuit condition. Loop lowering crosses
JoinIR/CorePlan planning, suffix routing, CFG/PHI construction, and recursive
body descent. It is intentionally not mixed into BIN0, SC0, STMT0, IF0, or
C0.

At LOOP0-D0, run a fresh read-only worker audit. Local selection is allowed
only if exactly one existing Loop child-demand owner can carry the located
condition/body inputs without:

```text
AST rewalk or site reconstruction
raw active-subtree descent
plan/ledger storage in Builder
route/recipe duplication
fallback or retry
```

If the selector is not unique, LOOP0-D0 emits the external design
consultation. Earlier rows remain landed behavior-neutral prerequisites.

## C0 completion law

After every required spine/control row is green, C0 may add one stack-local
located BLK0 port and one root connector.

```text
construct session from exact plan + caller
obtain located root body
drive BLK0 once
on body error:
  session is poisoned
  return primary error
  finish = 0
  retry/fallback = 0
on body success:
  consume ledger finish exactly once
  return body value only after finish succeeds
```

C0 acceptance requires the actual caller to claim all 15 rows in exact order,
including 2 selected and 13 unselected rows. Each outer `skip_ws` claim must
precede its nested argument claim.

## Counters and guards

```text
Binary child-role policy owners = existing PATH0 only
ordinary Binary associated-input drivers = 1
short-circuit associated-input drivers = 1
eager RHS evaluations for And/Or = 0

active-prefix raw delegations = 0
active-suffix raw router inputs = 0
AST rewalk/site reconstruction = 0
plan/view/ledger Builder fields = 0
fallback/retry/input probing = 0

actual A0 rows = 15
actual selected rows = 2
actual unselected rows = 13
actual Binary-ancestor rows = 5

production located callers before EXPR0-C0 = 0
callable-result publication before later SITE0-C0/CUT0 = 0
new language grammar/opcodes/backend/runtime behavior = 0
source/check files >= 800 lines = 0
```

Prefer one SPINE0 structural guard reused by BIN0 and SC0, and one later C0
guard. Do not add one shell wrapper per subrow.

## Implementation may claim

```text
located lowering can preserve exact source sites through the accepted child
families landed by each row

ordinary Binary preserves lhs-before-rhs evaluation

logical Binary preserves conditional RHS evaluation

unsupported active expression/control shapes continue to fail closed
```

## Implementation must not claim

```text
new source syntax or broader language semantics
general AST located lowering
general Loop/JoinIR site carriage before LOOP0
general expression-spine coverage beyond the admitted families
callable-result production publication
runtime/backend/ownership widening
fallback, retry, or result inference from names/symbols
```

## Stop conditions

Stop the current row if any of the following becomes necessary:

1. A child location is reconstructed from AST equality, span, name, or order.
2. An active prefix or suffix is delegated to raw lowering.
3. `And` or `Or` eagerly lowers both operands.
4. Existing route, CFG, PHI, type, effect, or result authority is duplicated.
5. The activation plan, source view, ledger, or mutable site enters Builder.
6. A statement completion bypasses its existing source preflight.
7. If or Loop conditions/bodies use raw descent after located selection.
8. Loop requires a second CorePlan/JoinIR recipe authority.
9. Failure calls finish, retries another route, or publishes a partial result.
10. A source/check file reaches 800 lines.

## Final lock

> The acceptance boundary should now expand, but only as a sequence of
> structural associated-input rows. The actual A0 caller proves five active
> MethodCalls under Binary ancestors, so direct EXPR0-C0 wiring is held.
> Ordinary Binary is the sole next shape and begins at
> `SITE0-R0-EXPR0-SPINE0-BIN0-S0`; short-circuit Binary, value-bearing
> statements, If, raw suffix admission, and Loop site carriage remain separate
> owners. Every prerequisite keeps production located callers and
> callable-result publication at zero. C0 may resume only after the actual
> 15-row caller completes one exact located traversal and ledger finish with no
> raw active-subtree escape, no retry, and no duplicated route/control/result
> authority.
