---
Status: LOOP0-S0b closed; LOOP0-S0c next
Date: 2026-07-18
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

The next code-facing row is `LOOP0-S0`, implemented as the short Refactor
Series `S0a -> S0b -> S0c`. S0 changes the internal plan schema explicitly but
has no production execution, located-consumer, ledger-claim, grammar, runtime,
backend, or ownership delta.

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
fixture, ordinary rows 0-5 are claimed first and the Loop batch is the next
consecutive slice 6-14.

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
  use the existing claimed canonical target and required-argument/result
  terminal authority

Unselected:
  preserve existing raw CoreEffectPlan payload emission
```

The current claim token directly owns site plus disposition, including the
selected canonical target and required i64 argument ordinals. It does not own
a general effect or full ABI product. LOOP0 must not claim broader ABI/effect
authority until the later emission row names the existing exact terminal owner.

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
LOOP0-S0a
  -> LOOP0-S0b
  -> LOOP0-S0c
  -> LOOP0-P0
  -> LOOP0-I0a
  -> LOOP0-I0b
  -> LOOP0-L0
  -> EXPR0-C0
```

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

### LOOP0-P0 — actual GenericLoop plan proof

Add one shared raw/test-located expression port without duplicating the
normalizer. Invoke the GenericLoopV1 composer directly and prove the actual
15-row child-demand trace, CorePlan, remap, PlanVerifier result, CFG plan,
types/bindings, loop carrier, short-circuit shape, and exact nine-site Loop
inventory. P0 does not own route selection or non-Generic route rejection;
those become executable only in I0a/L0. Direct-ordinal body is accepted, while
ScopeBox/flattened body rejects before composer invocation with Builder delta
zero and claims zero. Production located consumers remain zero.

### LOOP0-I0a — shared route selection owner

Mechanically split ordered selection from selected execution. Raw preserves
all existing route/fallback behavior. Located selects GenericLoopV1 exactly
once and propagates every later failure without fallback. Production located
roots remain zero.

### LOOP0-I0b — shared effect emission port

Thread one stack-scoped emission port through core/body/block/loop lowering.
The raw facade preserves exact behavior and all existing callers. No plan,
view, site table, ledger, or claim batch becomes a Builder field.

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
ClaimedCallableResultLoopBatchV1 definitions = 1  # pending S0c
wrapper/schedule Clone implementations = 0
batch Clone implementations = 0  # pending S0c

speculative planner claims = 0
compose-before-seal claims = 0
production located consumers before L0 = 0
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
production located Loop consumers and claims remain zero
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
> normalized shadow before effects. `LOOP0-S0a` and `LOOP0-S0b` are closed;
> `LOOP0-S0c` is next, and production located consumers and ledger claims remain
> zero through S0.
