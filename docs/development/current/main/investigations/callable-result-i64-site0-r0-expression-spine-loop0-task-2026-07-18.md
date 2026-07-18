---
Status: Candidate A-prime-plus selected; LOOP0-P0b-O0-S0 next
Date: 2026-07-19
Code baseline: 2a87a3bbe91318f52154b97ff5fadc8ee24d5dec
Decision-stop baseline: 4f9b84138a
Decision: uniform call provenance plus one non-Clone final-plan seal
Parent: callable-result-i64-site0-r0-expression-spine-task-2026-07-18.md
Supersedes frontier: callable-result-i64-site0-r0-expression-spine-loop0-design-stop-2026-07-18.md
---

# Callable-result SITE0-R0 expression-spine LOOP0 task

## Decision

LOOP0 selects B-prime:

```text
all call-bearing CoreEffectPlan variants
  carry one uniform source-provenance field

one final selected CorePlan
  is co-sealed in a non-Clone located-loop wrapper

canonical activation rows
  remain outside CorePlan
  and are committed by one ledger-owned source-order batch
```

Plan construction order, MIR evaluation/emission order, and source coverage
claim order are independent. The actual GenericLoop-v1 route may continue to
construct body plans before condition handoff. The activation ledger still
claims condition rows 6-8 before body rows 9-14, and outer row 13 before its
nested argument row 14. Plan emission may consume row 14 before row 13 because
argument evaluation precedes the outer call.

`LOOP0-P0b-F0` is closed and Candidate A-prime-plus selects the two-mode O0
body law. `LOOP0-P0b-O0-S0` is the sole next code-facing row. Production
located consumers, ledger claims, grammar, runtime, backend, and ownership
remain unchanged through O0.

## Repository-ground corrections

The decision is accepted with four mechanical corrections from three
read-only worker audits.

### Loop root identity is a statement site

The exact Loop owner is a statement child. The final wrapper stores a
`SourceStmtSiteV1` root. Expression descendants remain `SourceExprSiteV1`.
Fabricating an expression root for the Loop statement is forbidden.

### Raw fallback parity remains unchanged

The current raw router may continue to the next ordered route after selected
verify/lower/shadow failures. Removing that behavior in a claimed
behavior-neutral I0 would be a semantic change.

```text
raw existing ordered fallback:
  preserve exactly

new raw fallback:
  0

located fallback/retry:
  0
```

Raw fallback retirement, if ever selected, is a separate design row.

### Transformed bodies are outside the first profile

GenericLoop facts may clone and flatten scope bodies. A source site cannot be
recovered afterward from cloned AST equality, effect order, or target spelling.
The first located profile therefore requires a direct ordinal LoopBody layout
whose statements correspond one-to-one with the existing located body
carrier. ScopeBox flattening, transformed bodies, and nested Loop reject before
compose and before ledger effects.

### Claim preparation and claim commitment are separate products

A product containing `ClaimedCallableResultActivationSiteV1` is already
post-commit. The durable split is:

```text
VerifiedCallableResultLoopClaimScheduleV1
  preclaim, branded, canonical row references in source order

VerifiedCallableResultCallerLedgerV1::claim_loop_batch
  full prevalidation, then one non-fallible state commit

ClaimedCallableResultLoopBatchV1
  post-claim, non-Clone, site-keyed plan-order consumption
```

No generic site-only claim API is authorized.

## Exact live inventory

The four current call-bearing `CoreEffectPlan` variants are exhaustive:

```text
MethodCall
GlobalCall
ValueCall
ExternCall
```

The live compiled tree currently has 34 constructors across 12 files:

```text
MethodCall  14
GlobalCall   6
ValueCall    1
ExternCall  13
```

Historical consult copies under `apps/selfhost-runtime/consult/**` are evidence
artifacts and are excluded from live schema counts.

A source `MethodCall` may normalize to MethodCall, GlobalCall, or ExternCall
depending on its receiver. Conversely, Array/Map literal construction, index
assignment, Print, and console operations synthesize call effects that do not
represent one source MethodCall. Therefore every variant receives the same
field, but only an exact AST MethodCall terminal may produce a located source.
Blanket-stamping every effect returned from located lowering is forbidden.

## `CoreCallSourceV1`

Add one plan-layer vocabulary:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CoreCallSourceV1 {
    Unlocated,
    LocatedMethodCall(SourceExprSiteV1),
}
```

Each call-bearing variant has exactly one field:

```text
source: CoreCallSourceV1
```

The provenance means only:

```text
this final call effect came from this exact source MethodCall site
```

It owns no canonical target, ABI, required argument contract, result
representation, effect, selection disposition, or claim state. Raw and
synthetic constructors explicitly use `Unlocated`; implicit `Option::None` is
not used.

`CoreCallSourceV1` is Clone-safe immutable provenance. Existing mechanical
CorePlan cloning and ValueId remapping preserve it byte-for-byte. Remappers
modify only ValueId fields and never rewrite a source site.

## Layer ownership

`CorePlan` is visible only inside `crate::mir::builder`, so the products are
physically split:

```text
src/mir/builder/control_flow/plan/
  call_source.rs
  located_loop.rs
  located_loop_error.rs

src/mir/callable_result_representation/
  loop_claim_schedule.rs
  loop_claim_batch.rs
  loop_claim_batch_error.rs
```

Do not widen CorePlan visibility. The ledger never inspects CorePlan, and the
plan layer never mutates the ledger.

## Final located-plan seal

`VerifiedLocatedCoreLoopPlanV1` is non-Clone and owns the completed CorePlan.
It is constructed only after selected composition, all mechanical Clone and
freshening/remap, and PlanVerifier success.

Conceptual shape:

```rust
pub(in crate::mir::builder) struct VerifiedLocatedCoreLoopPlanV1<'plan> {
    plan: CorePlan,
    loop_root: SourceStmtSiteV1,
    schedule: VerifiedCallableResultLoopClaimScheduleV1<'plan>,
    _seal: LocatedCoreLoopPlanSealV1,
}
```

The constructor consumes a non-Clone caller/activation-branded located Loop
carrier. A caller key plus relative source paths plus `PhantomData` is not
sufficient: identical relative paths may exist in another caller.

The seal traverses the final plan once and validates:

```text
every located call occurrence lies under the exact Loop statement domain
missing activation-row site = 0
extra plan site = 0
duplicate plan occurrence = 0
active located source call emitted as Unlocated = 0
foreign activation plan/caller = 0
```

It does not derive source order from CorePlan traversal. It borrows the exact
order already owned by `activation_plan.rows_for(caller)`. Plan occurrences may
be indexed for exact membership, but sorting that index must not become the
claim schedule.

The one call-effect visitor covers `CorePlan::Seq`, If branches, Loop body and
block effects, BranchN arms/else, direct Effect, and nested `IfEffect` effect
lists. A second visitor or route-specific site table is forbidden.

## Atomic ledger batch

`VerifiedCallableResultLoopClaimScheduleV1` is non-Clone, plan/caller branded,
and contains exact activation-row references in canonical source order. Loop
domain selection comes from the existing located/PATH0 authority; the ledger
must not add a second LoopCondition/LoopBody path matcher.

The ledger-owned API performs:

```text
1. exact activation-plan brand check
2. exact canonical caller check
3. compare the complete requested schedule with the next unclaimed
   consecutive caller-row slice
4. detect request duplicates using temporary state
5. on any error, claimed-set delta = 0
6. only after all rows pass, insert the complete schedule
7. return one non-Clone ClaimedCallableResultLoopBatchV1
```

The schedule is not assumed to be the tail of the caller rows. For the actual
fixture, human rows 1-5 (Rust indices 0-4) are claimed first and the Loop batch
is human rows 6-14 (Rust indices 5-13). Human row 15 (Rust index 14) remains an
ordinary claim after the batch.

The post-claim batch uses a `BTreeMap<SourceExprSiteV1, Claimed...>` only for
plan-order lookup/removal. It rejects double removal and rejects finish while
any claim remains. ValueId, block ID, effect ordinal, target spelling, span,
AST pointer, and emission order are never site identity.

## Planning and transaction law

Facts and route predicates may inspect borrowed syntax, but may not create
source sites, read the ledger, claim rows, or store location in Builder.

The eventual located route has two phases:

```text
Phase A: observation and final plan
  exact located Loop carrier
  -> pure structural route selection
  -> selected GenericLoop-v1 composition only
  -> complete Clone/freshening/remap
  -> PlanVerifier
  -> non-Clone final-plan seal
  ledger claims = 0

Phase B: source transaction and emission
  schedule full prevalidation
  -> atomic source-order claim commit
  -> existing plan-order emission
  -> exact site removal per call effect
  -> batch consumption finish
```

Selected compose/seal failure claims zero rows. Claim-batch failure changes no
ledger state and emits no MIR. Selected emission failure leaves claims inside a
poisoned located session; ledger finish, fallback, retry, and same-session reuse
are forbidden. Fresh Builder/compiler/session reuse remains required.

This row does not claim whole-Builder rollback for pre-existing composition
effects. P0 fixes exact parity with the raw route's current partial-effect
boundary.

## Emission law

The eventual stack-scoped emission port receives either no claimed batch or
one non-Clone claimed batch. It is threaded through all recursive CorePlan
lowering owners; it is never stored in `MirBuilder`.

```text
Unlocated:
  exact existing raw emission

LocatedMethodCall(site):
  remove the exact claim from the batch

SelectedExactI64:
  consume the existing claimed canonical target and required-argument/result
  disposition through one later canonical emission terminal

Unselected:
  preserve existing raw CoreEffectPlan payload emission
```

The current claim token directly owns site plus disposition, including the
selected canonical target and required i64 argument ordinals. It does not own
a general effect or full ABI product. No selected-claim MIR emission terminal
exists yet: the current located legacy terminal carries `_claim` without
consuming its disposition. I0b/L0 must add exactly one canonical disposition
consumer and may not invent broader ABI/effect authority.

## First accepted located profile

```text
root:
  exact located Loop statement

route:
  GenericLoopV1 only

body correspondence:
  direct ordinal LoopBody layout only
  no ScopeBox flattening or transformed body

condition:
  existing ordinary Binary and short-circuit shapes

body:
  existing Local, Assignment, If, Return, and exact suffix laws

active source call kind:
  MethodCall inventory only

nested argument:
  admitted; outer row 13 precedes argument row 14 in claims

source ASTNode::FunctionCall / ASTNode::Call activation:
  rejected

explicit FunctionCall("externcall") source form:
  rejected

CoreEffectPlan::ExternCall carrying LocatedMethodCall(site):
  structurally permitted only when it represents a source MethodCall
  selected exact-i64 disposition remains outside the first profile

nested Loop:
  rejected

normalized shadow:
  pre-effect typed rejection; no located fallthrough
```

The actual ParserBox function is the first structural fixture, not a by-name
special case.

## Task order

```text
LOOP0-S0a (closed)
  -> LOOP0-S0b (closed)
  -> LOOP0-S0c (closed)
  -> LOOP0-P0a (closed)
  -> LOOP0-P0b-F0 (closed)
  -> LOOP0-P0b-O0-S0
  -> LOOP0-P0b-O0-R0
  -> LOOP0-P0b-O0-P0
  -> LOOP0-P0b-O0-G0
  -> LOOP0-P0b-T0
  -> LOOP0-P0b-P0
  -> LOOP0-P0c
  -> LOOP0-I0a
  -> LOOP0-I0b
  -> LOOP0-L0
  -> EXPR0-C0
```

Consult reconciliation (2026-07-19): the later B-prime packet does not reopen
`LOOP0-S0`. Its uniform provenance, non-Clone located-plan seal, canonical
schedule, and atomic claimed batch are the already-closed S0a/S0b/S0c
authorities. The packet's tentative `PreparedCallableResultLoopClaimsV1` must
not be added beside the landed schedule -> ledger commit -> claimed-batch
split. The B-prime packet is therefore recorded as an already-closed
prerequisite. Candidate A-prime-plus now selects the P0b-O0 two-mode body law;
no S0 authority is reopened. O0-S0 is next and T0 remains forbidden until
O0-G0 is green.

### LOOP0-S0a — uniform provenance schema

Production behavior delta: 0. Located consumers: 0. Ledger claims: 0.

```text
new plan/call_source.rs
four call-bearing variants receive source
all 34 live constructors explicitly initialize Unlocated
all exhaustive destructure owners bind or ignore source explicitly:
  effect_emission, remapper, effect_validators, and exact test destructures
remapper preserves source unchanged and owns no provenance policy
one exhaustive call-effect visitor
Clone/remap preservation tests
live guard excludes historical consult copies
```

Closeout (2026-07-18): closed. One `CoreCallSourceV1` vocabulary now covers all
four call-bearing variants. Every 34 live raw constructor explicitly publishes
`Unlocated`; production located producers, located consumers, and ledger claims
remain zero. The exhaustive plan/effect visitor covers Loop block effects and
nested If effects, while Clone and ValueId remap preserve the exact source
carrier. The public expression-spine guard inventories the full live plan tree,
rejects premature S0b/ledger authorities, and excludes historical consult
copies. Focused tests, `cargo check --all-targets`, quick 66/66 in 110s, format,
diff, pointer, and line-cap checks are green. Three worker reviews are GO after
the repository-wide guard correction.

### LOOP0-S0b — disconnected final-plan seal

Production behavior delta: 0. Located consumers: 0.

```text
new plan/located_loop.rs and located_loop_error.rs
new callable_result_representation/loop_claim_schedule.rs
non-Clone final-plan wrapper
statement-root and branded caller/plan pairing
missing/extra/duplicate/unlocated/foreign rejection
canonical schedule borrowed from activation rows
remap after seal unavailable by construction
```

Closeout (2026-07-18): closed. One non-Clone
`VerifiedCallableResultLoopClaimScheduleV1` retains the exact activation-plan
brand, canonical caller, located `SourceStmtSiteV1` Loop root, and borrowed
activation rows in their existing source order. Loop membership reuses the
PATH0 `LoopCondition` and `LoopBody` roles; it is never derived from CorePlan
traversal. One non-Clone `VerifiedLocatedCoreLoopPlanV1` consumes a completed
Core Loop, runs `PlanVerifier` before location sealing, and owns only the
private plan plus schedule. Its sole call-source visitor rejects missing
scheduled occurrences (including an active site lost as `Unlocated`),
duplicate occurrences, and extra located occurrences. Unrelated synthetic/raw
`Unlocated` effects remain outside source-occurrence truth. Foreign
plan/caller, wrong statement/plan kind, empty Loop domain, malformed plan, and
same-caller sibling-Loop leakage reject in focused fixtures. Production
located consumers, ledger claims, claim batches, Builder storage, CorePlan
escape, and remap-after-seal APIs remain zero. Focused tests are 9/9; the
public expression-spine structural guard, all-target check, release build,
format/diff/line guards, and quick 66/66 in 78.13s are green. Three worker
reviews are GO after PlanVerifier and structural-guard hardening. `LOOP0-S0c`
is next.

### LOOP0-S0c — disconnected atomic claim batch

Production behavior delta: 0. Production claims: 0.

```text
new callable_result_representation/loop_claim_batch.rs
ledger-owned full prevalidation and one commit
non-Clone post-claim site-removal batch
unused/double-removal typed rejection
actual 15-row exact-order fixture
```

Closeout (2026-07-18): closed. The non-Clone preclaim schedule is consumed by
one ledger extension entry. The ledger reuses the ordinary
Duplicate/WrongOrder/Unexpected policy, stages the complete canonical row
slice without mutation, pointer-checks every scheduled row against the exact
activation-plan row, constructs the claimed tokens, and performs one
non-fallible `claimed.extend` commit. It returns one non-Clone batch branded by
the real activation plan, canonical caller, and Loop statement root. The batch
retains source order only for deterministic unused diagnostics; exact-site
slots support plan-order removal and distinguish unexpected, already-consumed,
and unconsumed claims without copying target or ABI authority.

Six focused fixtures include the actual 15-row ParserBox caller. They prove
human rows 1-5 ordinary, 6-14 atomic Loop batch, and 15 ordinary; source order
keeps outer row 13 before nested row 14 while simulated plan emission consumes
14 before 13. Wrong-order, foreign-plan, and foreign-caller failures leave the
ledger reusable; double removal, unknown removal, and unfinished batches are
typed failures. Production `claim_loop_batch` callers, located Loop consumers,
Builder storage, grammar, runtime, backend, and ownership deltas remain zero.
Callable-result 75/75, located-plan 9/9, the public expression-spine guard,
all-target check, release build, formatting, diff, line-cap checks, and quick
66/66 in 82s are green. `LOOP0-P0` is next.

### LOOP0-P0 — actual GenericLoop plan proof

The initial worker GO was conditional on a recursively complete direct-ordinal
body representation. O0 disproved that premise for both actual modes.
Candidate A-prime-plus now replaces it with one retained successful step
disposition and one non-Clone two-mode seal. P0a remains closed; O0-S0 is next,
while P0b-T0 and later proof slices remain forbidden until O0-G0 is green.

#### LOOP0-P0a — one expression-demand port vocabulary

Production behavior delta: 0. Production located consumers: 0.

Add one stack-scoped `LoopPlanExpressionPortV1` at the
`compose_generic_loop_v1_recipe -> apply_generic_loop_v1_pipeline` boundary.
The raw implementation borrows the existing AST/body and always returns
`CoreCallSourceV1::Unlocated`. The `cfg(test)` located implementation owns no
path table: it uses only `VerifiedCallableResultLegacySourceViewV1`,
`LegacyStmtInputV1` / `LegacyExprInputV1` / `LegacyBodyInputV1`, and the
existing `ExprChildRoleV1` / `BodyChildRoleV1` vocabulary.

`PlanNormalizer::lower_value_ast` remains the normalizer SSOT. Its existing
raw entry becomes a thin facade over one port-driven internal lowering entry;
there is no second expression normalizer, AST walker, or target resolver.

P0a fixtures:

```text
raw leaf/child demands preserve the existing CorePlan and Builder snapshot
raw call source is always Unlocated
located child demands return the exact existing PATH0 child carrier
located MethodCall stamps exactly its activation_site
synthetic/canonical helper expressions without an exact carrier stay Unlocated
port construction, inspection, and failure claim zero ledger rows
```

P0a is closed. One sealed GAT port now delegates every admitted child demand
to the existing PATH0 roles. The raw facade retains exact nested-call and
nested pure value-If behavior, while the test-only located port co-validates
the source-view brand before stamping a MethodCall site. Qualified MethodCalls
preserve distinct inner/outer sites even though effect order is inner-first;
synthetic Array calls stay Unlocated. Nine focused tests cover all four call
families, exact Builder/type/effect snapshots, foreign-carrier rejection, and
zero ledger mutation. One structural guard fixes the single raw/test-located
port owners, production located producers/consumers zero, no GenericLoop
threading, no second path/identity owner, and all touched source/check files
below 800 lines. Normalizer, located-plan, callable-result, umbrella guard,
all-target check, release build, formatting, pointer, and worker re-review are
green; quick is 66/66 in 80 seconds. Boolean/short-circuit condition descent,
BlockExpr prelude, and explicit externcall remain outside the first located
profile and are carried explicitly
to P0b rather than silently entering raw helpers.

#### LOOP0-P0b — GenericLoopV1-only port threading

Production behavior delta: 0. Production located consumers: 0.

The pre-audit conditional GO is superseded by Candidate A-prime-plus. The
actual route has two distinct body representations, not one universal
RecipeBlock: default RecipeOnly may own no NoExit recipe, while strict
ExitAllowed keeps a join-bearing If opaque as a Stmt item. O0-S0 is authorized;
T0 remains forbidden until the four O0 rows are green.

P0b is split into the following mechanically ordered rows:

```text
LOOP0-P0b-F0
  shared actual 15-row fixture and one complete environment lock

LOOP0-P0b-O0-S0
  canonical successful extraction product and raw facts facade parity

LOOP0-P0b-O0-R0
  non-Clone DirectRecipeOnly / ExitAllowedRecipe seal

LOOP0-P0b-O0-P0
  actual default/strict and StmtWrappedJoinIf bridge proof

LOOP0-P0b-O0-G0
  structural guard and negative matrix

LOOP0-P0b-T0
  GenericLoopV1-only same-port threading

LOOP0-P0b-P0
  default RecipeOnly and strict/planner-required ExitAllowed direct-compose parity
```

`P0b-F0` extracts the existing actual ParserBox fixture instead of creating a
third source copy. One mutex owns save/set/restore for both legacy and current
DEV, STRICT, PLANNER_REQUIRED, and DEBUG variables. Default and strict modes
run sequentially under that same lock; separate environment locks are
forbidden.

F0 closeout (2026-07-19): closed. One cfg(test)-only fixture now owns the
actual ParserBox extraction, exact two selected sites, sealed 15-row plan, and
caller. The prior activation and Loop-batch tests borrow it instead of keeping
local copies. One crate-wide process-state guard owns all six current/legacy
DEV, STRICT, PLANNER_REQUIRED, and DEBUG keys across the complete default then
strict pair; direct literal writers for those keys are zero. Production
GenericLoop, located producer/consumer, route, claim, grammar, runtime,
backend, and ownership deltas remain zero. Focused mode, GenericLoop extract,
and callable-result tests, both structural guards, all-target check, release
build, quick 66/66 in 70 seconds, formatting, diff, pointer, and line-cap
checks are green. Two final worker reviews are GO. Candidate A-prime-plus is
selected and O0-S0 is the sole next code-facing row.

`P0b-O0` is the hard authorization gate. Its audit stop fired before code: the
canonical step
placement is discarded before facts publication; default RecipeOnly has no
NoExit RecipeBlock because the actual body contains an early Return; and
strict ExitAllowed intentionally represents the source join If at ordinal 4
as opaque `Stmt(4)`. The previous universal RecipeBlock law is therefore not
implementable. Candidate A-prime-plus is selected; its successful-disposition,
same-call PATH0 extraction, policy-owned mode selection, and O0-owned
StmtWrappedJoinIf laws are fixed in
`callable-result-i64-site0-r0-expression-spine-loop0-p0b-o0-design-stop-2026-07-19.md`.
O0-S0 is next; T0 remains forbidden until O0-G0 is green.

O0 row contracts are fixed as follows:

```text
O0-S0:
  one primary successful extraction product
  StepResolution owns the final disposition
  raw facts and hint remain thin behavior-identical facades

O0-R0:
  passive production-visible located port + non-Clone constructor
  exact Located root and LoopCondition retained
  DirectRecipeOnly / ExitAllowedRecipe chosen only by sealed policy
  extraction owns the ExitAllowed recipe exactly once
  root/execution consumers = 0

O0-P0:
  actual default/strict proof under the existing F0 lock
  explicit If condition/branches and StmtWrappedJoinIf bridge closed
  Builder/composer/skeleton/ledger delta = 0

O0-G0:
  one private helper imported by the public LOOP0 guard
  focused Rust fixtures plus structural negative matrix
```

All recursively paired PATH0 bodies reject Program, ScopeBox, and nested Loop.
The raw extraction retains `usize` body length so S0 adds no overflow failure;
checked ordinal projection belongs to the disconnected located seal.

`P0b-T0` adds paired thin raw facades and port-aware GenericLoopV1 owners only:

```text
RecipeComposer::compose_generic_loop_v1_recipe
  -> apply_generic_loop_v1_pipeline
  -> orchestrate_generic_loop_v1_carriers
  -> lower_generic_loop_v1_body

apply_generic_loop_v1_pipeline
  -> apply_generic_loop_v1_condition_step_handoff
  -> apply_generic_loop_condition
  -> lower_loop_header_cond
```

The same borrowed port reaches condition leaves, body leaves, nested If child
bodies, and the exact final cleanup-step carrier. Existing Parts RecipeBlock
dispatch may gain a neutral port-aware entry only after O0 succeeds; a
GenericLoop-local duplicate dispatcher is forbidden. V0, non-Generic routes,
registry selection, normalized shadow, nested-loop adoption, PlanLowerer,
ledger claims, and production located roots remain untouched. Large existing
normalizer files near the 800-line cap are not extended; new proof/adapter/test
owners stay in small dedicated files.

`P0b-P0` uses independently seeded raw and test-located Builders and invokes
the direct composer, never the registry or PlanLowerer. Each environment mode
proves raw/located structural plan and complete post-compose Builder-state
parity; the only located delta is exact source provenance. Child-demand trace
must cover ordinary Binary, short-circuit `&&`/`||`, MethodCall terminals,
body statements, nested If bodies, and cleanup step. Body-first composition
and condition-later handoff are preserved and must not be confused with source
claim order. Ledger claims remain zero.

Thread the same borrowed port through exactly the GenericLoopV1 composer,
carrier orchestration/body lowering, and condition/step handoff. Both body and
condition leaves must reach the same port-driven value lowering entry.
`CondBlockView`, `RecipeBody`, flattened facts, or cloned AST may remain raw
planning data, but none may become located source identity authority.

The old assumption that both modes provide a recursively complete RecipeBlock
is superseded by Candidate A-prime-plus. T0 must consume the selected two-mode
seal and may not reclassify steps or Stmt-wrapped If nodes, rebuild a NoExit
recipe, or infer location from RecipeBlock indices, cloned AST, or a new
mapping table.

Only the located test path performs this preflight before
`alloc_generic_loop_v0_skeleton`. It must consume the selected O0 two-mode
seal: DirectRecipeOnly supplies the exact source prefix plus cleanup carrier;
ExitAllowed supplies exact root RecipeItem refs plus the closed
StmtWrappedJoinIf source-to-singleton-Join bridge. `ScopeBox`, `Program`,
flattened/transformed body, nested Loop, or a required child with no existing
PATH0 role rejects before skeleton allocation. Therefore the rejection proves:

```text
composer invocations = 0
Builder ValueId/block/type/binding/loop-state delta = 0
ledger claims = 0
retry/fallback = 0
```

The raw port continues to consume the existing cloned/flattened facts exactly
as before. The shared port unifies lowering mechanics, not syntax authority;
located admission must not narrow existing raw production behavior.

This row does not select a route. Tests may construct `LoopRouteContext`, use
the existing pure facts outcome, assert GenericLoopV1, and invoke
`RecipeComposer::compose_generic_loop_v1_recipe` directly. Registry routing,
`route_generic_loop_v1`, `PlanLowerer`, normalized shadow, and non-Generic
routes are outside P0.

#### LOOP0-P0c — actual 15-row disconnected parity proof

Production behavior delta: 0. Production located consumers: 0. Production
claims: 0.

Use separate, identically seeded raw and test-located Builders. Compose the
actual GenericLoopV1 plan directly, complete all existing ValueId remapping,
run `PlanVerifier`, and only then seal `VerifiedLocatedCoreLoopPlanV1`. Run
this proof separately under the default environment and under the exact
strict/dev + `planner_required` lock. A default-only green result cannot close
P0 or authorize C0.

The normalized snapshot must compare structure rather than `Debug` text:

```text
CorePlan:
  Loop block roles, body, block effects, phis, frag, final values, step mode

Builder post-compose state:
  current block
  blocks/instructions/terminators
  value_types/value_kinds/origins
  variable and binding maps
  next block/value/core cursors
  loop/context depth

Located-only evidence:
  exact child-demand trace
  exact nine Loop SourceExprSiteV1 occurrences
  remap preserves every site without changing semantic plan structure
  canonical claim schedule remains condition 6-8 then body 9-14
```

The short-circuit proof is CFG-shaped: RHS call effects must remain in their
separate evaluation block. A call count or final MIR-only comparison is not
sufficient to prove laziness.

Required positive fixtures:

```text
actual static_const_parse_add 15-row carrier
O0-sealed two-mode exact PATH0 Loop body
condition rows 6,7,8 and body rows 9..14
outer row 13 before nested row 14 in source schedule
nested row 14 before outer row 13 in plan/evaluation traversal
CorePlan clone/remap/site preservation
PlanVerifier green
raw/located plan, CFG, type, binding, carrier, and Builder-state parity
default RecipeOnly and planner-required ExitAllowed parity
short-circuit RHS separate-block laziness
```

Required negative fixtures:

```text
ScopeBox or flattened body rejects before composer invocation
Program wrapper rejects before composer invocation
nested Loop rejects
missing PATH0 child role rejects
located site reconstructed by AST equality/span/target/ValueId/order rejects
raw active condition/body handed to located port rejects
```

False-green guards:

```text
Debug-string snapshot authority = 0
site-count-only assertions = 0
schedule-only assertions = 0
final-MIR-only parity = 0
source order == plan order assumptions = 0
facts clone == source authority assumptions = 0
route registry/fallback participation = 0
```

Hard stop and new consultation are required if the selected two-mode seal
cannot prove and later consume the exact PATH0 carrier, including the
StmtWrappedJoinIf bridge, without AST equality or a second path table. The same
stop applies to normalized shadow, nested Loop, a child demand with no existing
PATH0 role, or any second location/path authority. Successful O0, T0, P0, and
P0c advance to LOOP0-I0a; they do not activate located Loop production.

Parked DX follow-up: `dev-gate-quick-latency-task-2026-07-18.md` owns the
measured 42-134 second quick-gate latency, parallelization inventory, and
conservative changed-path selector. Parallel execution remains parked behind
its PAR0 row, and a second stable ultra-quick profile is rejected. It is not a
LOOP0 prerequisite.

### LOOP0-I0a — shared route selection owner

Mechanically split ordered selection from selected execution. Raw preserves
all existing route/fallback behavior. Located selects GenericLoopV1 exactly
once and propagates every later failure without fallback. Production located
roots remain zero.

### LOOP0-I0b — shared effect emission port

Thread one stack-scoped emission port through core/body/block/loop lowering.
The raw facade preserves exact behavior and all existing callers. No plan,
view, site table, ledger, or claim batch becomes a Builder field.

The disconnected located wrapper intentionally exposes no owned plan escape.
Before L0, I0b must add exactly one consuming handoff that moves the sealed
CorePlan and its schedule into the claim-aware execution bundle. Clone,
borrowed CorePlan access, remap after seal, and independently movable plan/site
sidecars remain forbidden.

I0b/L0 also owns the first and only selected-claim emission terminal. It must
consume `ClaimedCallableResultActivationSiteV1` disposition directly; raw
`func`/`method` spelling is diagnostic or unselected payload only. Treating the
current ignored `_claim` parameter as an already-complete terminal is
forbidden.

### LOOP0-L0 — disconnected located GenericLoop acceptance

Connect exact located carrier to pure selection, located composition,
verification, final seal, atomic claims, and plan-order emission. Normalized
shadow and every non-Generic route reject before effects. Production source
root callers remain zero.

### EXPR0-C0 — one production connector

Connect one located root traversal. The actual caller claims all 15 rows in
exact order, consumes every Loop batch claim, and finishes the ledger exactly
once. Failure poisons the selected session; no finish or retry occurs.

## Required S0 fixtures

```text
four variants carry the uniform source field
all 34 raw/synthetic constructors use Unlocated
source MethodCall may stamp MethodCall, GlobalCall, or ExternCall terminal
synthetic Array/Map/Print calls remain Unlocated
site-preserving CorePlan Clone
site-preserving ValueId remap
before/after normalized remap proves exact source preservation
missing scheduled occurrence, duplicate/extra located occurrence, and
foreign-pair seal rejection
required active site lost as Unlocated -> missing; unrelated synthetic
Unlocated remains accepted
actual caller exact 15-row sequence
rows 0-5 followed by atomic rows 6-14
row 13 before row 14 in schedule
row 14 before row 13 in simulated emission removal
batch duplicate, wrong-order, unknown, and extra rejection
batch failure leaves ledger delta zero
double removal and unused finish rejection
ordinary claims may precede/follow the batch as canonical order allows
final ledger finish
```

## Later parity and failure fixtures

```text
actual static_const_parse_add all 15 rows
condition rows 6 -> 7 -> 8
body rows 9 -> 10 -> 11 -> 12 -> 13 -> 14
short-circuit RHS laziness
raw/located route, recipe, plan, CFG, type, binding, and loop-stack parity
all current raw LoopRouteId behavior and fallback parity
candidate predicates claim zero rows
compose/seal failure claims zero rows
batch failure emits zero MIR
selected emission failure poisons only the located session
fresh Builder/compiler/session reuse
debug/release and environment restoration parity
normalized-shadow located rejection before effects
```

## Counters and guards

Use one reusable subchecker
`tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_loop0.py`, invoked by
the existing expression-spine guard. Do not add a new shell/manifest family.

```text
uniform call-source vocabulary owners = 1
call-bearing variants with source = 4/4
live call constructors with explicit source = 34/34
ad-hoc Option/site fields = 0
call-effect visitor owners = 1

VerifiedLocatedCoreLoopPlanV1 definitions = 1
VerifiedCallableResultLoopClaimScheduleV1 definitions = 1
ClaimedCallableResultLoopBatchV1 definitions = 1
wrapper/schedule Clone implementations = 0
batch Clone implementations = 0

speculative planner claims = 0
compose-before-seal claims = 0
production located root/execution call sites before L0 = 0
production located roots before C0 = 0

site mutation during remap = 0
remap after located-plan seal = 0
ValueId/effect-index/span/AST/name site reconstruction = 0
Builder plan/view/site/ledger/batch fields = 0
route/recipe/PATH0 duplicates = 0

located fallback/retry = 0
new raw fallback = 0
existing raw fallback parity = exact
located transformed-body admission = 0
located normalized-shadow fallthrough = 0

suffix-router behavior delta = 0
grammar/runtime/backend/ownership delta = 0
source/check files >= 800 lines = 0
```

## Implementation may claim after S0

```text
all live CorePlan call effects carry one uniform source-provenance vocabulary
raw and synthetic calls are explicitly Unlocated
source provenance survives mechanical Clone and ValueId remap
one non-Clone wrapper can seal final plan occurrences against branded rows
one branded source-order schedule can be committed atomically by the ledger
one non-Clone claimed batch supports exact plan-order site consumption
production located Loop root/execution call sites and claims remain zero
```

## Implementation must not claim after S0

```text
located GenericLoop production lowering
CorePlan target, ABI, result, effect, or claim authority
general located PlanNormalizer support
general Loop route or transformed-body support
normalized-shadow located support
condition-first CorePlan construction
Builder-wide rollback
source FunctionCall/Call activation or selected exact-i64 extern disposition
nested Loop support
new grammar, runtime, backend, or ownership support
```

## Stop conditions

Stop the current row if it requires:

1. adding provenance to fewer than all four call-bearing variants;
2. stamping a synthetic call merely because it is inside located lowering;
3. copying canonical target/ABI/effect into CorePlan;
4. an arbitrary unbranded site-only ledger claim API;
5. deriving the schedule from CorePlan traversal or BTreeMap sort order;
6. pairing caller-relative sites using only a caller key and PhantomData;
7. reconstructing sites from ValueId, effect index, span, AST equality, name,
   or emission order;
8. cloning the wrapper, schedule, claimed token, or batch;
9. remapping after final-plan seal or remapping a source site;
10. storing plan/view/site/ledger/batch authority in `MirBuilder`;
11. duplicating route, recipe, PATH0, CFG, PHI, or suffix policy;
12. removing existing raw fallback in a behavior-neutral row;
13. located fallback/retry after selection;
14. admitting flattened/transformed body, nested Loop, normalized shadow, or
    non-Generic route in the first profile;
15. widening grammar, runtime, backend, or ownership;
16. growing a source/check file to 800 lines.

## Final lock

> LOOP0 selects B-prime. Every live call-bearing CoreEffectPlan variant carries
> one uniform Clone-safe, ValueId-remap-invariant `CoreCallSourceV1`, while only
> an exact source MethodCall terminal may create `LocatedMethodCall(site)` and
> every raw or synthetic constructor explicitly creates `Unlocated`. After one
> GenericLoop-v1 plan finishes composition, cloning, remap, and verification, a
> non-Clone statement-rooted wrapper co-seals its located call occurrences with
> an activation-plan/caller-branded schedule borrowed in existing canonical row
> order. The caller ledger alone prevalidates and atomically commits that
> schedule, returning a non-Clone claimed batch consumed by exact site in plan
> emission order. Thus claims remain condition 6-8 then body 9-14 and outer 13
> before argument 14 even when planning is body-first and emission evaluates 14
> before 13. Raw fallback behavior remains unchanged; located selection never
> falls back. The first profile admits only direct-ordinal GenericLoop-v1
> MethodCall inventory and rejects transformed bodies, nested Loop, and
> normalized shadow before effects. `LOOP0-S0a` through `LOOP0-S0c` and
> `LOOP0-P0a` and `LOOP0-P0b-F0` are closed. Candidate A-prime-plus selects one
> retained successful extraction result and one non-Clone two-mode located
> body seal. O0-S0 is the sole next code-facing row; T0 and later rows remain
> forbidden until O0-G0 is green. Production located consumers and ledger
> claims remain zero.
