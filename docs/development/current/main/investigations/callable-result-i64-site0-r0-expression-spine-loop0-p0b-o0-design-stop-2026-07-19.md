---
Status: O0-S0 closed; O0-R0 is the sole next code-facing row
Date: 2026-07-19
Decision baseline: eef7c61d54085e8344608a7859420bd9e9a74887
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Decision: Candidate A-prime-plus
---

# Callable-result LOOP0-P0b-O0 A-prime-plus decision and task

## Conclusion

The original O0 direct-ordinal proof is not implementable under its locked
identity law. Candidate A-prime-plus is selected as its replacement. O0-S0 may
now implement one canonical successful extraction product; T0 remains
forbidden until O0-S0, O0-R0, O0-P0, and O0-G0 are all green.

The selected architecture is A-prime-plus:

```text
one canonical GenericLoopV1 extraction result
  retains its already-computed successful step disposition

one non-Clone located body seal
  DirectRecipeOnly
  OR ExitAllowedRecipe

source identity
  existing PATH0 carriers only
```

The plus lock adds four drift barriers:

```text
1. retain successful step disposition, not raw StepPlacementDecision
2. located seal invokes canonical extraction once from the exact PATH0 Loop
   carrier; independently supplied facts/extraction/ordinals are impossible
3. body mode comes only from extraction.body_lowering_policy
4. O0 owns the complete StmtWrappedJoinIf singleton recipe and exact
   condition/then/else carriers; T0 never rebuilds or reclassifies it
```

Repository audit adds four mechanical corrections without changing the
selected semantics:

```text
1. canonical_body_len is usize in the raw extraction product; checked u32
   projection belongs only to the disconnected located seal, so S0 adds no raw
   overflow rejection
2. the outer located wrapper retains the exact Loop root and LoopCondition
   carriers as well as extraction and body mode; T0 never re-derives them
3. the extraction remains the sole ExitAllowed recipe owner; the mode owns only
   the verified located item/source tree and cleanup, never a cloned recipe
4. R0 promotes the located expression port and co-seal constructor as passive
   production-visible schema with exactly zero production root/execution call
   sites
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

## Candidate A-prime-plus — retained witness plus two-mode seal (selected)

### Canonical extraction product

Refactor one internal GenericLoopV1 extraction result to retain the
already-computed step disposition. The raw facts-only API remains a thin
facade over that same owner.

```text
GenericLoopV1ExtractionV1
  facts
  successful GenericLoopV1StepDispositionV1

GenericLoopV1StepDispositionV1
  NumericProgression {
    placement: StepPlacement,
    canonical_body_len: usize,
  }
  OR BodyManagedState
```

`StepPlacementDecision` remains candidate-classification state inside the
extractor. It is not retained. The successful disposition is co-validated with
`facts.carrier_role`; `canonical_body_len` is the exact flattened-body length.
Checked u32/source-ordinal projection occurs only in the disconnected located
seal. Product fields and constructors are private, and facts plus witness
cannot be reconstructed independently.

`StepResolution` carries the final successful disposition directly. It does
not retain a second `use_body_managed_step` truth. If existing validation
changes a candidate to BodyManagedState, the returned disposition is
BodyManagedState; numeric candidates retain their post-validation placement.

The primary entry is one `try_extract_generic_loop_v1` returning the extraction
product. Existing `try_extract_generic_loop_v1_facts` consumes that product
through `into_facts`, and the recipe hint observes the same primary entry with
`is_some()`. Reject/log/freeze order remains unchanged.

The located constructor receives only the located expression port and exact
PATH0 Loop statement carrier. Inside one constructor it requires an exact
Located carrier (never Unlocated or Synthetic), validates the Loop, obtains
exact LoopCondition and LoopBody carriers, and invokes the canonical extractor
exactly once. It accepts no externally constructed facts, extraction,
placement, or ordinal list, so foreign pairing is unavailable by construction.

The first profile accepts only:

```text
carrier role = NumericProgression
placement = Last
source body length = one or more
direct body ordinals = 0..n-2
cleanup ordinal = n-1
```

No later step reclassification is permitted.

Every recursively paired exact PATH0 body must contain no ScopeBox, Program,
flattening, transformation, or nested Loop. Root-only rejection is
insufficient because canonical flattening traverses nested branches. The root
direct length must equal the retained canonical body length. For the first profile only
NumericProgression plus `StepPlacement::Last` is admitted; `n - 1` is a checked
projection of that sealed result, not a new classification.

### Located body representation

```text
VerifiedLocatedGenericLoopRepresentationV1
  exact located Loop root carrier
  exact LoopCondition carrier
  owns the extraction product privately
  owns one VerifiedLocatedGenericLoopBodyModeV1

  DirectRecipeOnly
    exact PATH0 body prefix ordinals
    exact final cleanup carrier

  ExitAllowedRecipe
    recipe remains owned once by extraction.facts.body_exit_allowed
    verified located root item/source tree
    root RecipeItem ordinal co-seal
    exact final cleanup carrier
```

The mode never clones or independently owns `ExitAllowedBlockRecipe`.
Wrapper-only consuming access exposes the recipe and verified source proof
together to T0. Recipe owner count remains exactly one. The outer wrapper is
non-Clone and retains the exact LoopCondition carrier so condition rows 6--8
cannot be re-derived or paired from a foreign root by T0.

`Stmt(k)` and `Exit { stmt: k }` are opaque exact ordinal carriers, except for
a closed `StmtWrappedJoinIf` disposition below. An explicit
`IfV2 { if_stmt: k }` requires recursive RecipeBlock pairing through PATH0
`IfThen` / `IfElse`.

For strict source ordinal 4, `Stmt(4)` is sealed as a closed
`StmtWrappedJoinIf` disposition. O0 itself must verify the complete bridge:

```text
exact PATH0 LoopBody(4) source If + root Recipe Stmt(4)
  -> existing try_build_no_exit_block_recipe([exact source If], allow_extended=true)
  -> singleton root IfV2 { if_stmt: 0, contract: Join }
  -> exact PATH0 IfThen / IfElse child-local carriers
```

T0 may consume that proof and re-enter the exact source carrier through the
shared port. It must never use the cloned recipe AST as location authority or
reclassify the Stmt-wrapped If independently.

The closed product owns all inputs needed by T0:

```text
VerifiedStmtWrappedJoinIfV1
  exact source If statement carrier
  exact IfCondition carrier
  exact IfThen carrier
  exact optional IfElse carrier
  singleton NoExit recipe
  recursively sealed singleton located root
```

Seal order is exact:

```text
1. outer ExitAllowed item is Stmt(k)
2. exact PATH0 LoopBody(k) source is If
3. obtain PATH0 IfCondition
4. obtain PATH0 IfThen
5. obtain exact IfElse presence/carrier
6. pass only that exact source If to the existing NoExit builder
7. singleton root item count is one
8. singleton root item is IfV2
9. singleton if_stmt is local ordinal zero
10. singleton contract is Join
11. recipe branch presence equals exact source branch presence
12. recursively co-seal child blocks using PATH0 child-local ordinals
```

An ExitAllowed `Stmt(k)` whose exact source is If cannot be admitted as an
ordinary opaque statement. Explicit root `IfV2` and Stmt-wrapped If both retain
IfCondition as well as then/else carriers. This law applies at every
recursively sealed RecipeBlock, not only the actual root ordinal 4. Cloned
`RecipeBodies` AST and `CondBlockView` remain planning data and are never
located source-identity authorities.

The wrapper is non-Clone, owns no source-site map, receives no Builder, and
has no composer, ledger, or production root/execution call site through O0.

Body mode is selected only from the extraction product's sealed
`body_lowering_policy`:

```text
RecipeOnly -> DirectRecipeOnly
ExitAllowed -> ExitAllowedRecipe
```

The seal must not reread environment variables, strict/dev flags,
planner-required, method names, or `body_no_exit` presence as route policy.
Actual default `RecipeOnly + body_no_exit=None` is one required proof, not a
claim about every RecipeOnly loop.

## Candidate B — widen the shared RecipeBlock producer (parked)

Change ExitAllowed so every join-bearing If becomes `IfV2::Join`, and provide
a recipe for the default path. This creates a more uniform tree, but changes a
shared production recipe/lowering contract and mixes a broad BoxShape change
with the narrow O0 authorization. Select it only if A-prime-plus cannot seal the
existing Stmt-wrapped lowering path.

## Candidate C — post-hoc ordinal/site sidecar (rejected)

Reconstruct the removed step or build a RecipeItem-to-source-site table using
AST equality, span, names, target spelling, ValueId, or recipe order. This is a
second source identity authority and violates the existing O0 stop law.

## Decision lock

Candidate A-prime-plus is accepted with the following exact laws:

```text
1. retain one successful step disposition containing the already-computed
   canonical StepPlacement in one same-call GenericLoopV1 extraction product;

2. seal default RecipeOnly as exact PATH0 direct ordinals 0..4 plus cleanup 5,
   without requiring a NoExit RecipeBlock;

3. seal strict ExitAllowed RecipeItems by ordinal, treating Stmt/Exit as
   opaque direct carriers and recursively pairing only explicit IfV2;

4. classify source If + Stmt(4) as one StmtWrappedJoinIf disposition, and in
   O0 prove the exact-source singleton NoExit bridge
   IfV2 { if_stmt: 0, Join } plus PATH0 IfThen/IfElse children, so T0 only
   consumes that closed proof through the shared port.
```

The existing ExitAllowed producer remains unchanged. If the sealed PATH0
condition/branch carriers and existing singleton NoExit builder cannot close
the Stmt-wrapped bridge, stop O0 and reopen Candidate B as a separate shared
producer refactor consultation.

## Fixed task order for Candidate A-prime-plus

```text
LOOP0-P0b-O0-S0
  retain/co-seal the canonical step witness in one extraction owner
  raw facts facade parity
  production behavior delta = 0
  docs_only_closeout = forbidden
  code_or_artifact_delta_required = 1

LOOP0-P0b-O0-R0
  disconnected non-Clone DirectRecipeOnly / ExitAllowedRecipe seal
  StmtWrappedJoinIf product vocabulary
  production root/execution call sites = 0

LOOP0-P0b-O0-P0
  actual default + strict proof
  exact PATH0 child-demand inventory
  Builder/ledger delta = 0

LOOP0-P0b-O0-G0
  structural guard and negative matrix

then:
  LOOP0-P0b-T0
```

### O0-S0 contract

```text
files:
  generic_loop/README.md
  generic_loop/facts_types.rs
  generic_loop/facts/extract/v1.rs
  generic_loop/facts/extract/mod.rs

primary extraction owners = 1
existing facts API = one thin into_facts facade
recipe hint = same primary owner + is_some only
StepResolution truth = successful disposition only
raw facts/reject/log/freeze parity = exact
production behavior delta = 0
```

Closeout (2026-07-19): closed. One canonical
`GenericLoopV1ExtractionV1` now privately co-seals the existing facts with the
final successful disposition. `StepResolution` retains only the increment and
that disposition; NumericProgression keeps the validated `StepPlacement` plus
the flattened `usize` body length, while both the initial receiver-managed
route and the post-validation fallback retain BodyManagedState. The existing
facts API consumes the product through `into_facts`, and the recipe hint asks
the same primary owner only for `is_some()`. The primary has one passive
builder-visible re-export and zero external production callers.

Focused fixtures cover Last, InBody, ContinueIf, BreakElseIf, initial and
fallback BodyManagedState, flattened-body length, and facts-facade parity.
Focused extraction tests 22/22, the evolved carrier inventory, the consolidated
public expression-spine guard, all-target check, release build, formatting,
diff/line guards, and quick 66/66 in 236s are green. Worker reviews are GO.
Production behavior, located execution callers, Builder, composer, skeleton,
ledger, grammar, runtime, backend, ownership, fallback, and retry deltas are
zero. O0-R0 is next.

### O0-R0 contract

Use a small dedicated `generic_loop/located_representation/` family for the
outer product, mode, recipe seal, wrapped join If, errors, and tests. Promote
the existing located expression port/error/re-export and the co-seal
constructor from cfg(test) to passive production-visible schema.

```text
production located port schema owners = 1
production located representation constructors = 1
production root/execution call sites = 0

exact Located root preflight = required
Unlocated / Synthetic / foreign root acceptance = 0
recipe clones = 0
recursive Program / ScopeBox / Loop acceptance = 0
Builder / composer / skeleton / ledger parameters = 0
```

The prior P0a guard expectation `production_located_ports=0` is deliberately
advanced to passive schema=1 with execution consumers still zero. Keeping the
constructor cfg(test)-only is forbidden because T0 must consume the same
durable authority rather than promote or recreate it later.

### O0-P0 contract

Use the existing F0 shared actual fixture and environment lock. Focused Rust
tests, not structural counts alone, prove default, strict, root condition,
ordinal 2 ExitOnly If, ordinal 4 wrapped Join If, and cleanup carriers.

### O0-G0 contract

The new private helper is imported by the existing public LOOP0 guard. It may
reuse base LOOP0 helper functions but must not import the public P0 guard back
into itself. It scans production-stripped source, structurally requires the
focused test module/owners, and enforces every O0 source/check file below 800
lines. Row verification runs the focused cargo test command separately; the
public structural guard must not become a cargo-test runner.

The O0 guard must live in a new helper imported by the existing public guard:

```text
tools/checks/lib/
  callable_result_i0_site0_r0_expr0_spine0_loop0_p0b_o0.py
```

## Required pass evidence

```text
canonical extraction parity:
  Last
  InBody
  InContinueIf
  InBreakElseIf
  BodyManagedState
  every existing accepted facts/reject/freeze outcome unchanged

located first profile:
  NumericProgression + Last
  body length 1 with empty prefix
  actual body length 6
  exact located Loop root retained
  exact LoopCondition carrier retained

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
  ordinal 2 explicit IfV2 retains exact IfCondition + IfThen + absent IfElse
  ordinal 4 StmtWrappedJoinIf retains exact IfCondition + IfThen + IfElse
  ordinal 4 proves exact source local 0 IfV2 Join bridge
  singleton IfV2 children co-seal through PATH0 IfThen / IfElse

all modes:
  Builder delta = 0
  ledger claims = 0
  production root/execution call sites = 0

recursive recipe proof:
  explicit IfV2 ExitAllowed
  nested acyclic IfV2
  nested join If inside an explicit or wrapped branch
  every Stmt(k) whose exact source is If becomes StmtWrappedJoinIf
```

## Required rejection evidence

```text
missing, multiple, conditional, body-managed, or non-final step
canonical/exact body length mismatch
foreign independently constructed facts/source pairing
ScopeBox / Program / nested Loop
ScopeBox / Program nested inside a recursively paired If body
flattened or transformed source input
Unlocated / Synthetic / foreign Loop root
recipe ref gap, duplicate, reorder, or out-of-range body
wrong IfV2.if_stmt
source/recipe kind mismatch
IfV2 source is not If
IfCondition role missing
missing or extra If child role/block
IfElse presence mismatch
Stmt-wrapped If not accepted by the existing NoExit builder
source If left as ordinary OpaqueStmt
singleton recipe has zero or multiple root items
Stmt-wrapped singleton root is not exact IfV2 { if_stmt: 0, Join }
Stmt-wrapped singleton child-local ordinal/role mismatch
AST/span/name/target/ValueId equality reconstruction
new recipe-index/source-site map
```

## Counters and guards

```text
canonical GenericLoopV1 extraction owners = 1
canonical step-placement decision owners = 1
retained successful step witnesses = 1 per extraction
raw facts facades = 1
post-facts step reclassification = 0
raw facts API = one thin facade over canonical extraction owner
post-facts classify_step_placement calls in O0 = 0
post-facts matches_loop_increment calls in O0 = 0

located body representation variants = 2
default actual representation = DirectRecipeOnly
strict actual representation = ExitAllowedRecipe
strict actual root shape = Stmt0,Stmt1,IfV2(2),Stmt3,Stmt4
actual StmtWrappedJoinIf count = 1
StmtWrappedJoinIf O0 lowering consumers = 0
T0 StmtWrappedJoinIf reclassification = 0
T0 NoExit recipe rebuilding = 0

source/recipe side maps = 0
new PATH0 vocabularies = 0
AST equality/span/name reconstruction = 0
duplicated recipe builders = 0
ExitAllowed recipe clones = 0
environment reads in R0 seal = 0

sealed exact Loop root carriers = 1 per representation
sealed exact LoopCondition carriers = 1 per representation
T0 LoopCondition re-derivations = 0

O0 MirBuilder parameters = 0
O0 composer consumers = 0
O0 ledger claims = 0
production located root/execution call sites = 0
production located port schema owners = 1 after R0
production located representation constructor owners = 1 after R0
production root/execution call sites in cfg(test)-stripped sources = 0

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
10. re-derive the LoopCondition carrier in T0 or accept a separately supplied
    condition;
11. clone or duplicate the ExitAllowed recipe between extraction and mode;
12. keep the durable located constructor cfg(test)-only;
13. accept ScopeBox or Program at any recursively paired body depth;
14. grow a source/check file to 800 lines.

## Implementation may claim after O0-G0

```text
GenericLoopV1 extraction retains its already-selected successful step
disposition

one exact located Loop source and its extraction result are paired in one
same-call non-Clone product

NumericProgression + Last yields one exact direct source prefix and cleanup

default RecipeOnly is represented without inventing a NoExit recipe

strict ExitAllowed root items are paired by exact ordinal with PATH0 carriers

explicit IfV2 retains exact condition, then, and optional else carriers

a Stmt-wrapped exact no-exit join If is sealed once through the existing
NoExit builder with its exact PATH0 condition and branch carriers

T0 can consume the representation without source reconstruction, step
reclassification, or recipe rebuilding
```

## Implementation must not claim

```text
all RecipeOnly bodies have no NoExit recipe
all source If nodes are explicit IfV2 in the ExitAllowed root
all opaque Stmt nodes may be recursively lowered
general located GenericLoop support
non-final or conditional progression support
BodyManagedState support
ScopeBox / Program / nested Loop support
shared ExitAllowed semantics changed
new recipe vocabulary
production located Loop activation
ledger claims
Builder transaction or rollback
```

## Final decision lock

> Candidate A-prime-plus is selected. O0-S0 introduces one canonical
> GenericLoopV1 extraction product that retains the already-computed successful
> step disposition while preserving the existing facts API as a thin
> behavior-identical facade. The located O0 constructor accepts only one exact
> PATH0 Loop statement, derives its condition and body carriers, and invokes
> that extractor once; independently supplied facts, placement, and ordinals
> are unavailable. The first located profile admits only NumericProgression
> with `StepPlacement::Last`, exact direct body ordinals `0..n-2`, and cleanup
> `n-1`. One non-Clone wrapper seals DirectRecipeOnly without inventing a
> RecipeBlock or ExitAllowedRecipe with source-order recipe/PATH0 pairing. Each
> explicit IfV2 owns exact condition and branch carriers. Each root Stmt whose
> exact source is If must seal as StmtWrappedJoinIf through one singleton
> existing NoExit `IfV2 { if_stmt: 0, contract: Join }`; T0 consumes that
> recipe and its carriers without rebuilding or reclassification. The fixed
> order is O0-S0 -> O0-R0 -> O0-P0 -> O0-G0 -> T0. O0 owns no Builder,
> composer, skeleton, ledger, production located root/execution call site,
> fallback, retry, or
> shared ExitAllowed behavior change.
