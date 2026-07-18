---
Status: ASN0-S0 closed; ASN0-I0 is next
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

  -> STMT0-LCL0-S0
  -> LCL0-I0
  -> LCL0-P0
  -> LCL0-L0

  -> STMT0-ASN0-S0
  -> ASN0-I0
  -> ASN0-P0
  -> ASN0-L0

  -> STMT0-RET0-S0
  -> RET0-I0
  -> RET0-P0
  -> RET0-L0

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
SITE0-R0-EXPR0-SPINE0-STMT0-ASN0-I0
```

`ASN0-S0` is closed. One disconnected exact Variable-target Assignment driver
preserves the existing declared-binding preflight before one associated RHS
descent and delegates completion to the existing from-value owner. ASN0-I0 may
only select that raw driver through the existing `build_assignment` facade;
field/index/compound selectors, Return, If, Loop, production located root
callers, and callable-result publication remain unchanged.

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
actual statement-surface rows:           10
  Local initializer:                      5
  Assignment RHS (including arguments):   4
  Return value:                           1
direct expression statement rows:         0
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

The ledger currently exposes an exact inactive-prefix proof and a typed
`RowsUnderPrefix` rejection, but no positive active/inactive classifier.
Therefore BIN0-L0 must not call `prove_expr_inactive`, catch
`RowsUnderPrefix`, and use that error to select the located route. That would
be input probing and would create an implicit retry boundary.

Instead, every ordinary Binary encountered by the disconnected located
session goes through the located Binary driver. Its left and right associated
inputs independently use the existing ledger law:

```text
child prefix contains no rows:
  prove the exact child inactive
  -> existing raw whole-child delegation

child prefix contains rows:
  continue located descent
  -> exact terminal claim
```

This admits no new source shape and needs no new ledger API. `And` and `Or`
still reject before child effects and remain owned by SC0. If whole-Binary raw
delegation becomes a required invariant, stop BIN0-L0 and design a separate
positive prefix-classification product rather than branching on an error.
Production located callers and callable-result publication remain zero.

Required BIN0-L0 fixtures:

```text
pass:
  active MethodCall in lhs
  active MethodCall in rhs
  active rows on both sides, exact lhs-before-rhs claim order
  nested ordinary Binary depth two through four
  actual If-condition Eq shapes for rows 2 and 11
  inactive child raw delegation with an exact child-prefix proof
  failure poisons the session; a fresh session remains independent

reject/no-effect:
  And and Or
  foreign or unlocated Binary input
  wrong PATH0 child role
  unsupported active non-Binary prefix
  location reconstruction or error-based route probing
```

Post-row counters:

```text
ordinary Binary associated-input drivers = 1
raw Binary implementations/selectors = 1 / 1
located Binary implementations = 1
Binary child-role policy owners = 1 (PATH0)
error-based active-prefix selectors = 0
production located root callers = 0
callable-result publishers = 0
Builder plan/view/ledger fields = 0
```

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

### BIN0-I0 closeout

BIN0-I0 is closed. One owned `RawLegacyBinaryInputV1` and one raw port
implementation select only the existing non-logical `build_binary_op` branch
through the generic driver. `And` / `Or` remain selected first by the existing
short-circuit owner. The adapter adds no recursion guard, operator classifier,
location, ledger, route, result, type, CFG, or PHI authority.

Six raw fixtures prove source left-before-right materialization, MethodCall on
both sides, nested depth and reuse, left/right failure stopping, existing
short-circuit selection, and parent/child depth restoration. Generic BIN0
6/6, raw BIN0 6/6, recursive child 7/7, short-circuit 1/1, callable-result
48/48, every existing EXPR0 guard, cargo check, quick 66/66, and the release
build are green. The SPINE0 guard now proves `raw selector = 1`, `raw impl = 1`,
`located impl = 0`, and preserved short-circuit ownership. BIN0-P0 is next.

### BIN0-P0 closeout

BIN0-P0 is closed. One `#[cfg(test)]` pre-I0 reference preserves the retired
three-step orchestration only as parity evidence. Selected and reference paths
start from fresh identical Builders and compare output/error, ordered block
instructions and terminators, transient types, value kinds, origins, next
ValueId, and recursion depth.

Four focused fixtures cover the exact 16-operator ordinary matrix, MethodCall
on each side, nested Binary depth two through four, lhs/rhs failures, and
post-failure reuse. All snapshots are exact-equal. The SPINE0 guard proves the
reference exists only in the test module and remains absent from every
production source. Production code, grammar, MIR, runtime, located callers,
ledger consumers, and result authority are unchanged. BIN0-L0 is next.

### BIN0-L0 closeout

BIN0-L0 is closed. `LocatedLegacyLoweringSessionV1` now implements the one
ordinary Binary associated-input port. Every ordinary Binary reaches the
closed generic driver once, and the existing PATH0 source view alone produces
its `BinaryLeft` and `BinaryRight` inputs. Each child independently proves an
inactive prefix for raw whole-child delegation or continues located descent to
an exact MethodCall claim. No error is used as a route selector.

Logical `And` / `Or` still reject before child effects with the stable SC0
boundary. One located-session recursion guard owns the outer Binary; child
expressions retain their existing guards, failures poison only the current
session, and a fresh session remains independent. The port adds no operator,
ledger, result, type, CFG, PHI, effect, or Builder-state authority.

Four new focused fixtures prove a row under an ordinary Binary, exact
lhs-before-rhs claims through nested depth, the actual If-condition equality
shape, and fail-closed logical/unlocated inputs. Located lowering 8/8, all
BIN0 16/16, recursive child 7/7, callable-result 51/51, the staged SPINE0
guard, current pointer, quick 66/66, release build, formatting, diff check, and
line caps are green. Production located root callers and callable-result
publishers remain zero. SC0-S0 is next.

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

### SC0-S0 closeout

SC0-S0 is closed. One private `ShortCircuitExpressionDescentPortV1` and one
generic driver admit only `And` and `Or`, descend the lhs first, and hand one
deferred rhs descent closure to the existing short-circuit CFG owner. The rhs
input is requested and lowered only after that owner enters the eval-RHS
block. Ordinary Binary rejects before child effects.

The existing `build_logical_shortcircuit` raw facade now delegates its
already-lowered lhs to one extracted after-lhs core. That core remains the
sole CFG, PHI, constant-result, result-type, variable-map, and diagnostic
owner. The disconnected driver adds no raw or located adapter, production
selector, location, ledger, result, type, or control-flow authority.

Six focused fixtures prove exact lhs/rhs timing, shared And/Or completion,
ordinary-operator rejection, syntax/lhs failure before CFG effects, rhs
failure only after entering the eval block, and fresh-driver reuse. SC0 6/6,
BIN0 16/16, located lowering 8/8, callable-result 51/51, the staged SPINE0
guard, current pointer, quick 66/66, release build, formatting, diff check,
and line caps are green. Raw and located SC0 implementations and production
callers remain zero. SC0-I0 is next.

### SC0-I0 closeout

SC0-I0 is closed. One owned `RawLegacyShortCircuitInputV1` and one raw port
implementation now carry the existing `And` / `Or` source entry through the
generic SC0 driver. `MirBuilder::build_binary_op` retains the sole selector:
logical operators choose SC0 once, while every ordinary operator remains on
BIN0. The adapter adds no operator, recursion, CFG, PHI, result, type,
location, ledger, or fallback authority.

Six raw fixtures prove And/Or completion, rhs materialization outside the
entry block, lhs failure before CFG effects, rhs failure after entering the
eval block, ordinary-Binary separation, recursion restoration, and fresh
Builder independence. Combined SC0 12/12, BIN0 16/16, located lowering 8/8,
callable-result 51/51, the SPINE0 guard, current pointer, quick 66/66, release
build, formatting, diff check, and line caps are green. The old raw facade is
no longer production-selected and remains only until SC0-P0 fixes exact
parity. Located SC0 adapters and callable-result publication remain zero.
SC0-P0 is next.

### SC0-P0 closeout

SC0-P0 is closed. The retired pre-I0 raw orchestration now exists only as one
`#[cfg(test)]` reference. Fresh selected and reference Builders compare exact
result or error, ordered block instructions and terminators, transient types,
value kinds, origins, variable and pin maps, current block, next ValueId, and
recursion depth.

Four fixtures cover the full And/Or Bool matrix, nested And/Or/comparison
trees, MethodCall children, lhs/rhs failures, and post-failure reuse. Every
snapshot is exact-equal. Combined SC0 16/16, BIN0 16/16, located lowering 8/8,
callable-result 51/51, the SPINE0 guard, quick 66/66, release build, formatting,
diff check, and line caps are green. The old production raw facade is absent;
the reference is not a selector or fallback. Located SC0 adapters and
callable-result publication remain zero. SC0-L0 is next.

### SC0-L0 closeout

SC0-L0 is closed. `LocatedLegacyLoweringSessionV1` implements the short-circuit
port once and selects it only for `And` / `Or`. PATH0 remains the sole source
of `BinaryLeft` and `BinaryRight`. The lhs is located and claimed first; the
rhs location is not requested until the existing CFG owner enters eval-RHS.
Each child independently proves an inactive prefix or continues located
descent to an exact claim. No site reconstruction or ledger-error probing is
used.

Four dedicated fixtures prove left/deferred-right block separation, nested
And/Or/comparison descent, the actual Loop-condition surface, failure poisoning,
and fresh-session independence. The previous logical-rejection fixture now
proves located logical acceptance while preserving unlocated ordinary-Binary
rejection. SC0 16/16, BIN0 16/16, located 12/12, callable-result 55/55, the
SPINE0 guard, quick 66/66, release build, formatting, diff check, and line caps
are green. Production located root callers and callable-result publication
remain zero. LCL0-S0 is next.

### LCL0-S0 closeout

LCL0-S0 is closed with one disconnected Local statement descent driver. It
obtains the Local syntax view and completes the existing whole-declaration
exact-numeric preflight before any initializer effect. Initializers are then
requested in declaration order. Ordinary expressions descend through E0;
untyped missing initializers publish the existing Null value without a child
demand; typed-array and record-constructor initializers remain explicit hooks
for their existing specialized preflight and lowering owners. The driver calls
the existing from-values completion exactly once, only after every initializer
value has been produced.

Eight fixtures prove preflight-before-effects, including later exact-numeric
and typed-array declaration failures, initializer order, missing-value
handling, syntax/input/child failure boundaries, typed-array and record hook
ordering, preclaim transport, and binding publication only at completion. Local
8/8, variable-statement 3/3, recursive child lowering 7/7, callable-result
55/55, the split-but-single-entry SPINE0 guard, quick 66/66, release build,
formatting, diff check, and line caps are green. Raw and located Local adapters,
production located root callers, and callable-result publication remain zero.
LCL0-I0 is next.

### LCL0-I0 closeout

LCL0-I0 is closed. The existing
`variable_stmt::build_local_statement` facade is the one production selector
for an owned raw Local input and the shared statement driver. The old raw
initializer loop is physically retired. The driver keeps one existing whole-
declaration preflight and its preflight-success debug observation before
initializer effects, requests ordinary expressions through the existing raw
recursive port, retains typed-array claim/preclaim and record-constructor
owners through thin hooks, and invokes the existing from-values completion
once after all values exist.

Seven raw fixtures cover declaration order and binding completion, whole-
declaration failure before effects, child failure before later initializers or
bindings, ordinary-Binary plus short-circuit initializer descent, typed-array
claim-before-append, record publication, and Null sugar. Combined Local 15/15,
existing variable-statement 3/3, recursive child lowering 7/7, callable-result
55/55, cargo check, the SPINE0 guard, quick 66/66, release build, formatting,
diff check, and line caps are green. Located Local implementations/selectors,
production located root callers, and callable-result publication remain zero.
LCL0-P0 is next.

### LCL0-P0 closeout

LCL0-P0 is closed with one `cfg(test)` pre-I0 Local orchestration reference.
It does not call the selected driver or either descent port. Six fixtures
compare exact selected/reference result or error plus normalized blocks,
instructions, terminators, transient types/kinds/origins, literal/map facts,
bindings and lexical frames, slot registry, local/array/record contracts,
record-local state, SSA/materialization caches, allocator counters, current
span/block, and recursion depth.

Ordinary, exact-numeric, Null, typed-array, record, Binary, and short-circuit
initializers are exact-equal. Whole-preflight, left/right child, unsupported
typed-array, record-arity, and completion-redeclaration failures retain exact
partial state and same-Builder recovery parity. Combined Local 21/21,
variable-statement 3/3, recursive child lowering 7/7, callable-result 55/55,
cargo check, the SPINE0 guard, quick 66/66, release build,
formatting, diff check, Python compile, and line caps are green. The pre-I0
reference remains test-only; located Local implementations/selectors and
callable-result publication remain zero. LCL0-L0 is next.

### LCL0-L0 closeout

LCL0-L0 is closed with one disconnected located Local selector and one located
port implementation. Local selection is syntax-owned and never chosen by
catching a ledger error. Each ordinary initializer is obtained from the
existing source view with the exact `LocalInitializer(index)` role, then uses
the already-closed expression spine and ledger claim order. Typed-array and
record-constructor hooks share one exact initializer-prefix inactivity proof;
an active MethodCall under an array element or constructor argument rejects
before specialized Builder effects or binding publication.

Six dedicated fixtures prove direct and nested ordinary initializer order,
short-circuit deferred RHS descent, inactive typed-array and record hooks,
active specialized subtrees failing closed, wrong statement order, session
poisoning, and fresh-session independence. Existing Local 21/21 and
callable-result 55/55 tests, the SPINE0 guard, cargo check, quick 66/66 in 94s,
release build, formatting, diff check, and line caps are green. Production located root callers and
callable-result publication remain zero. ASN0-S0 is next.

### ASN0-S0 closeout

ASN0-S0 is closed with one disconnected exact Variable-target Assignment
driver and one raw port whose production callers remain zero. Its input carries
only an already-selected variable name and RHS, so field/index/compound target
syntax is structurally absent. The existing `AssignmentResolverBox` check runs
before RHS input or effects; one associated RHS is lowered through E0; and the
existing `build_assignment_from_value` owner retains its second declaration
check, typed contracts, `ReleaseStrong`, and variable publication.

Six fixtures prove success order and exactly-once completion, undeclared,
binding-missing, and synthetic-pin rejection before RHS effects, syntax/RHS
input failure, RHS failure without assignment publication, completion-time
recheck, same-Builder reuse, and raw Binary descent. Assignment 6/6, resolver
4/4, recursive child 7/7, callable-result 55/55, the split SPINE0 guard, quick
66/66 in 76s, release build, formatting, Python compile, diff check, and line
caps are green. Raw and located production selectors plus callable-result
publication remain zero. ASN0-I0 is next.

## STMT0 — value-bearing statement family

The worker audit found that one shared STMT0 driver would have to duplicate or
bypass distinct source preflights. STMT0 is therefore a family, not one
implementation row:

```text
LCL0:
  Local initializer descent only after the existing local-shape/type preflight
  actual call-site rows = 5

ASN0:
  exact Variable-target Assignment RHS only
  field/index target evaluation remains parked
  actual call-site rows = 4, including nested MethodCall arguments

RET0:
  Return value only after proving the existing match-return optimization
  inactive; existing cleanup/contract completion remains authoritative
  actual call-site rows = 1
```

Each subrow follows `S0 -> I0 -> P0 -> L0`, preserves its existing source
preflight, and then reuses the existing from-value completion helper. No
subrow may call a completion helper while skipping or duplicating source-level
policy. A future shared statement facade is permitted only after these three
contracts are proven identical enough to share without branching policy.
Direct expression statements have zero actual rows and remain parked until a
concrete caller requires that separate surface.

Minimum subrow proof:

```text
LCL0:
  preserve exact-numeric/typed-array/record-constructor preflight timing
  child failure publishes no binding and performs no later initializer

ASN0:
  undeclared Variable target rejects before RHS effects
  field/index/compound targets remain outside the admitted row
  RHS failure performs no assignment

RET0:
  cleanup prohibition and match-return probe precede child descent
  child failure emits no Return
  void Return remains outside the admitted row
```

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
actual short-circuit descendant rows = 3
actual statement-surface rows = 10
actual If-condition rows = 2
actual Loop-subtree rows = 9
actual A0 path inventory = the exact 15 ordered paths listed above

production located callers before EXPR0-C0 = 0
callable-result publication before later SITE0-C0/CUT0 = 0
new language grammar/opcodes/backend/runtime behavior = 0
source/check files >= 800 lines = 0
```

Extend the existing SPINE0 structural guard across BIN0 and SC0 staged counts,
then use one later C0 guard for LCL0/ASN0/RET0/IF0/SUFFIX0/LOOP0/C0. Keep the
closed PATH0/A0/SITE0-L0/LDG0/BLK0/E0/L0 guards independently green. Do not
add one shell wrapper per subrow.

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
> Ordinary Binary, short-circuit Binary, Local initializer descent, and the
> disconnected Variable-target Assignment driver are closed through ASN0-S0.
> The sole current shape is
> `SITE0-R0-EXPR0-SPINE0-STMT0-ASN0-I0`; later value-bearing statements, If, raw suffix
> admission, and Loop site carriage remain separate owners. BIN0-L0 always
> uses the located driver for an ordinary Binary and lets each associated child
> prove inactivity or continue to an exact claim; it never selects a route by
> catching `RowsUnderPrefix`. Every prerequisite keeps production located callers and
> callable-result publication at zero. C0 may resume only after the actual
> 15-row caller completes one exact located traversal and ledger finish with no
> raw active-subtree escape, no retry, and no duplicated route/control/result
> authority.

## Parked tools/DX task

Quick-gate latency is tracked separately in
[`dev-gate-quick-latency-task-2026-07-18.md`](./dev-gate-quick-latency-task-2026-07-18.md).
Its first row is `DEV-GATE-Q0-M0`; it does not change this card's active
`SITE0-R0-EXPR0-SPINE0-STMT0-ASN0-I0` blocker or share a commit with a
compiler semantic row.
