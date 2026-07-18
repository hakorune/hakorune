---
Status: design consultation required; no O0 code authorized
Date: 2026-07-19
Code baseline: 498c5d6cde
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Decision: pending
---

# Callable-result LOOP0-P0b-O0 design stop

## Conclusion

The current O0 direct-ordinal proof is not implementable under its locked
identity law. Three read-only worker audits agree that proceeding now would
require AST equality or a second source-path authority. P0b-T0 is forbidden
until this consultation selects a replacement body-representation law.

The recommended candidate is A-prime:

```text
one canonical GenericLoopV1 extraction result
  retains its already-computed step-placement witness

one non-Clone located body seal
  DirectRecipeOnly
  OR ExitAllowedRecipe

source identity
  existing PATH0 carriers only
```

## Reconciliation with the returned B-prime packet

The returned B-prime packet is accepted architecture, but it is not a new
answer to this O0 stop. Its durable slices are already landed:

```text
uniform CoreCallSourceV1
  LOOP0-S0a = closed

non-Clone final located Loop plan seal
  LOOP0-S0b = closed

source-order preclaim schedule plus atomic claimed batch
  LOOP0-S0c = closed

one PATH0-backed expression-demand port
  LOOP0-P0a = closed
```

The repository must not add the packet's tentative
`PreparedCallableResultLoopClaimsV1` beside the landed
schedule -> ledger commit -> claimed-batch authority. The packet correctly
separates source claim order from plan/evaluation order, but it does not decide
how the selected GenericLoopV1 composer preserves exact source identity across
the two body representations found by O0. Therefore it neither reopens
`LOOP0-S0` nor authorizes `LOOP0-P0b-T0`.

## Exact repository findings

The actual `static_const_parse_add` Loop is function `Body(4)`. Its exact
LoopBody has six direct statements:

```text
0 local op
1 local rhs
2 if error(rhs) { return rhs }
3 local rv
4 if op == "+" { value = value + rv } else { value = value - rv }
5 pos = ParserStringUtilsBox.skip_ws(...)
```

Ordinal 5 is the numeric-progression cleanup selected by the existing
GenericLoop classifier.

### Gap 1: the canonical step witness is discarded

`resolve_step_for_candidate` computes a `StepPlacementDecision`, but
`GenericLoopV1Facts` retains only the cloned increment/body and carrier role.
The exact placement is discarded before O0 can consume it. Re-running
`matches_loop_increment` later uses AST equality and is forbidden as a source
identity reconstruction.

### Gap 2: default has no NoExit RecipeBlock

Default mode selects `BodyLoweringPolicy::RecipeOnly`. The early Return under
source ordinal 2 makes `try_build_no_exit_block_recipe` reject, so
`body_no_exit == None`. The current default lowerer walks the source-order
facts body directly and skips the progression step. A universal RecipeBlock
seal therefore has no default product to consume.

### Gap 3: strict keeps a join If opaque

Strict/planner-required mode selects ExitAllowed. Its root recipe for source
prefix 0..4 is:

```text
Stmt(0)
Stmt(1)
IfV2(if_stmt=2, ExitOnly, then Exit(Return, 0))
Stmt(3)
Stmt(4)
```

Source ordinal 4 is a join-bearing If, but the shared ExitAllowed builder
intentionally stores it as opaque `Stmt(4)`. Its then/else RecipeBlocks are
created later by the return-prelude lowering path. Therefore the previous O0
claim that every source If already has child RecipeBlocks is false.

## Candidate A-prime — retained witness plus two-mode seal (recommended)

### Canonical extraction product

Refactor one internal GenericLoopV1 extraction result to retain the
already-computed step disposition. The raw facts-only API remains a thin
facade over that same owner.

```text
GenericLoopV1ExtractionV1
  facts
  canonical step disposition / placement
```

The located constructor receives the exact PATH0 LoopCondition and LoopBody
carriers and invokes this canonical extractor exactly once. It does not accept
an independently constructed facts product, so foreign source/facts pairing
is unavailable by construction.

The first profile accepts only:

```text
carrier role = NumericProgression
placement = Last
source body length = one or more
direct body ordinals = 0..n-2
cleanup ordinal = n-1
```

No later step reclassification is permitted.

### Located body representation

```text
VerifiedLocatedGenericLoopBodyRepresentationV1
  DirectRecipeOnly
    exact PATH0 body prefix ordinals
    exact final cleanup carrier

  ExitAllowedRecipe
    exact PATH0 body prefix ordinals
    root RecipeItem ordinal co-seal
    exact final cleanup carrier
```

`Stmt(k)` and `Exit { stmt: k }` are opaque exact ordinal carriers, except for
the one closed `StmtWrappedJoinIf` disposition below. An explicit
`IfV2 { if_stmt: k }` requires recursive RecipeBlock pairing through PATH0
`IfThen` / `IfElse`.

For strict source ordinal 4, `Stmt(4)` is sealed as a closed
`StmtWrappedJoinIf` disposition. O0 itself must verify the complete bridge:

```text
exact PATH0 LoopBody(4) source If + root Recipe Stmt(4)
  -> existing try_build_no_exit_block_recipe([exact source If])
  -> singleton root IfV2 { if_stmt: 0, contract: Join }
  -> exact PATH0 IfThen / IfElse child-local carriers
```

T0 may consume that proof and re-enter the exact source carrier through the
shared port. It must never use the cloned recipe AST as location authority or
reclassify the Stmt-wrapped If independently.

The wrapper is non-Clone, owns no source-site map, receives no Builder, and
has no composer, ledger, or production consumer through O0.

## Candidate B — widen the shared RecipeBlock producer (parked)

Change ExitAllowed so every join-bearing If becomes `IfV2::Join`, and provide
a recipe for the default path. This creates a more uniform tree, but changes a
shared production recipe/lowering contract and mixes a broad BoxShape change
with the narrow O0 authorization. Select it only if A-prime cannot seal the
existing Stmt-wrapped lowering path.

## Candidate C — post-hoc ordinal/site sidecar (rejected)

Reconstruct the removed step or build a RecipeItem-to-source-site table using
AST equality, span, names, target spelling, ValueId, or recipe order. This is a
second source identity authority and violates the existing O0 stop law.

## Decision question

May O0 select Candidate A-prime?

```text
1. retain the already-computed canonical StepPlacement in one same-call
   GenericLoopV1 extraction product;

2. seal default RecipeOnly as exact PATH0 direct ordinals 0..4 plus cleanup 5,
   without requiring a NoExit RecipeBlock;

3. seal strict ExitAllowed RecipeItems by ordinal, treating Stmt/Exit as
   opaque direct carriers and recursively pairing only explicit IfV2;

4. classify source If + Stmt(4) as one StmtWrappedJoinIf disposition, and in
   O0 prove the exact-source singleton NoExit bridge
   IfV2 { if_stmt: 0, Join } plus PATH0 IfThen/IfElse children, so T0 only
   consumes that closed proof through the shared port?
```

If every source If must instead own an existing child RecipeBlock before T0,
the current ExitAllowed producer is incompatible and Candidate B requires a
separate refactor consultation.

## Contingent task order for Candidate A-prime

```text
LOOP0-P0b-O0-S0
  retain/co-seal the canonical step witness in one extraction owner
  raw facts facade parity
  production behavior delta = 0
  docs_only_closeout = forbidden
  code_or_artifact_delta_required = 1

LOOP0-P0b-O0-R0
  disconnected non-Clone DirectRecipeOnly / ExitAllowedRecipe seal
  production consumers = 0

LOOP0-P0b-O0-P0
  actual default + strict proof
  exact PATH0 child-demand inventory
  Builder/ledger delta = 0

LOOP0-P0b-O0-G0
  structural guard and negative matrix

then:
  LOOP0-P0b-T0
```

The O0 guard must live in a new helper imported by the existing public guard:

```text
tools/checks/lib/
  callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0.py
```

## Required pass evidence

```text
actual Loop root = Body(4)
actual LoopBody length = 6
retained placement = Last
body ordinals = 0..4
cleanup ordinal = 5
cleanup expression site = existing F0 selected site

default:
  RecipeOnly
  body_no_exit = None is accepted
  DirectRecipeOnly representation

strict:
  ExitAllowedRecipe representation
  root refs = 0,1,2,3,4
  ordinal 2 explicit IfV2 recurses through PATH0 IfThen
  ordinal 4 StmtWrappedJoinIf proves exact source local 0 IfV2 Join bridge
  singleton IfV2 children co-seal through PATH0 IfThen / IfElse

all modes:
  Builder delta = 0
  ledger claims = 0
  production consumers = 0
```

## Required rejection evidence

```text
missing, multiple, conditional, body-managed, or non-final step
foreign independently constructed facts/source pairing
ScopeBox / Program / nested Loop
flattened or transformed source input
recipe ref gap, duplicate, reorder, or out-of-range body
wrong IfV2.if_stmt
missing or extra If child role/block
Stmt-wrapped If not accepted by the existing return-prelude/NoExit classifier
Stmt-wrapped singleton root is not exact IfV2 { if_stmt: 0, Join }
Stmt-wrapped singleton child-local ordinal/role mismatch
AST/span/name/target/ValueId equality reconstruction
new recipe-index/source-site map
```

## Counters and guards

```text
canonical step-placement decision owners = 1
retained canonical step witnesses = 1
post-facts step reclassification = 0
raw facts API = one thin facade over canonical extraction owner
post-facts classify_step_placement calls in O0 = 0
post-facts matches_loop_increment calls in O0 = 0

located body representation variants = 2
default actual representation = DirectRecipeOnly
strict actual representation = ExitAllowedRecipe
strict actual root shape = Stmt0,Stmt1,IfV2(2),Stmt3,Stmt4
StmtWrappedJoinIf proof-only dispositions = 1
StmtWrappedJoinIf O0 lowering consumers = 0

source/recipe side maps = 0
new PATH0 vocabularies = 0
AST equality/span/name reconstruction = 0
duplicated recipe builders = 0

O0 MirBuilder parameters = 0
O0 composer consumers = 0
O0 ledger claims = 0
production located consumers = 0
production constructors/consumers in cfg(test)-stripped O0 sources = 0

O0 guard helper imports from public LOOP0 guard = 1
O0 guard helper files >= 800 lines = 0

grammar/runtime/backend/ownership delta = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop before T0 if any of the following is required:

1. re-scan the body with `matches_loop_increment` after extraction;
2. compare cloned recipe AST with source AST;
3. add a RecipeItem-to-source-site or source-ordinal side table;
4. accept a foreign facts/source pairing;
5. change shared ExitAllowed semantics inside O0;
6. lower strict `Stmt(4)` from cloned recipe AST;
7. add a GenericLoop-local duplicate statement dispatcher;
8. admit ScopeBox, Program, nested Loop, or non-final progression;
9. allocate a skeleton, mutate Builder, claim a row, retry, or fall back;
10. grow a source/check file to 800 lines.

## Implementation claim boundary

Before a decision, O0 may claim only that the previous proof shape is
insufficient. It must not claim that retained StepPlacement alone closes O0,
that default owns a NoExit recipe, or that every strict source If is IfV2.
