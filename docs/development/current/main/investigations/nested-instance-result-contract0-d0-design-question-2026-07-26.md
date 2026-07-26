---
Status: accepted design / execution handoff
Date: 2026-07-26
Decision: NESTED-INSTANCE-RESULT-CONTRACT0-prime-r1
Closes: NESTED-INSTANCE-RESULT-CONTRACT0-D0
Classification: BoxCount / T2 source result-contract authority
Choice: A-prime — bounded unannotated current-owner instance body proof
First executable row: NESTED-INSTANCE-RESULT-CONTRACT0-S0 (closed)
Blocked umbrella: CALLABLE-RESULT-NESTED-REP0-S0
Related:
  - docs/development/current/main/investigations/stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/callable_result_representation/README.md
  - src/mir/callable_result_representation/tests/actual_parser_add_fixture.rs
  - tools/checks/generic_loop_progression_role_v0_guard.sh
---

# Bounded nested instance-result contract

## Decision

Select Option A with one structural refinement:

```text
A-prime =
  one route-disjoint source-only instance target owner
  + one bounded contract owner
  + the existing callable body-result proof spine
```

The existing static result catalog remains static-only.  The new owner does
not add instance rows to it and does not create a second expression/body
walker.

```text
existing static catalog widening = 0
new source-only contract owner    = 1
GenericLoop type production       = 0
MIR/type_ctx publication          = 0
```

This decision closes the design stop.  The next row is executable, but its
first commit is a behavior-neutral reference-fixture refresh required by the
currently red reusable lane guard.

## Observed source relation

The exact source call is:

```hako
me.static_const_eval_pos(ret)
```

The source sites are already locatable without names or MIR recovery:

```text
caller =
  ParserBox.static_const_parse_add/2
  namespace = InstanceBoxMethod

pre-loop site =
  Body(3).Value.Argument(1)

loop-refresh site =
  Body(4).LoopBody(5).Value.Argument(1)

receiver =
  canonical me
  shadow receiver = CurrentOwner

target =
  same owner
  namespace = InstanceBoxMethod
  method/arity derived from the verified MethodCall
  static_const_eval_pos/1 in the current fixture
```

The first production-shaped proof producer is the pre-loop site.  The
loop-refresh site is a second exact-site fixture using the same contract; it
does not create a second semantic owner.

## Why A-prime

The current static result proof already has the finite body/expression grammar
needed by the target:

```text
"" + ret
  -> exact String receiver fact

String.lastIndexOf/1
String.substring/2
String.length/0
  -> existing generated Core String contracts

StringHelpers.to_i64/1
  -> existing same-module static target/result row

early return 0
final return StringHelpers.to_i64(...)
  -> exact Integer when required argument set is empty
```

`function_proof::prove_function` already accepts a verified callable
declaration.  The static-only boundary is introduced by `solver.rs` iterating
`static_declarations()`, not by the proof grammar itself.

Therefore:

```text
new expression walker = forbidden
new body walker       = forbidden
existing proof spine  = reused
```

### Why not widen the static catalog

`VerifiedSameModuleCallableResultCatalogV1` has an established law:

```text
row source =
  static declarations only
```

Adding instance rows would alter its cardinality and every consumer's
authority boundary.  The existing static current-owner target route also
deliberately rejects instance callers.

The new route is physically and semantically disjoint.

### Why not retain Unselected

Typed Unselected would be safe, but it would park the exact Stage-B source
shape even though all required source contracts are already available.  The
bounded proof advances one source shape without claiming general instance-call
support.

## Authority boundary

### New durable owner

Use one neutral sibling module:

```text
src/mir/source_instance_result_contract/
  README.md
  mod.rs
  target.rs
  contract.rs
  rejection.rs
  tests/
    mod.rs
    contract.rs
    rejection.rs
```

Responsibilities:

```text
target.rs
  exact current-owner MethodCall site
  exact same-catalog instance target
  receiver / caller namespace / target namespace checks

contract.rs
  same-brand dependency pairing
  bounded unannotated body-result proof request
  empty-required-argument closure
  final Integer contract from the existing ExactI64 proof vocabulary

rejection.rs
  typed stage/cause
  retained non-Clone failure owner

tests/
  borrow the sole existing actual-source fixture
  reorder/foreign-brand/unsupported negatives
  reuse and no-widening evidence
```

The new module owns no Builder, MIR, runtime, emission, or publication code.

### Existing proof adapter

The existing proof internals remain private.  Add at most one narrow method on
the existing result proof owner.  That method issues a neutral, non-Clone
proof product:

```text
sealed static dependency result catalog
  + one declaration borrowed from the same declaration catalog
  -> VerifiedUnannotatedCallableBodyResultProofV1
```

The method must:

```text
verify declaration-catalog identity
reject any return annotation
invoke the existing private function/body/expression proof
return the exact required-argument set
retain the declaration/catalog pairing in the proof product
never add an instance row to the static catalog
never expose rows_by_key or a brand-free result map
```

The method does not select instance routes.  The sibling owner proves
`InstanceBoxMethod`, current-owner receiver, target identity, and exact site,
then consumes only `VerifiedUnannotatedCallableBodyResultProofV1`.

The sibling production module must not name, retain, or accept
`VerifiedSameModuleCallableResultCatalogV1`.  That exact type remains
construction-only under its existing static-catalog guard.  In S0 the narrow
proof product is issued only by the catalog method and consumed by the sibling
co-seal; later production activation of the catalog remains outside this row.

This keeps the production sources under `callable_result_representation/`
free of instance-route policy and preserves both existing guard laws:

```text
InstanceBoxMethod inside callable-result production sources = 0
VerifiedSameModuleCallableResultCatalogV1 external
production consumers = 0
```

If the implementation cannot maintain those two counts, stop the row.  Do not
silently weaken `callable_result_i64_catalog_s0.py`.

## Exact owner chain

```text
resolved / merged Program AST
  ↓
VerifiedSameModuleCallableDeclarationCatalogV1
  - instance caller
  - instance target
  - static dependency declarations
  ↓
VerifiedSourceMethodCallSiteV1
  - exact caller
  - exact source path
  - exact MethodCall AST
  ↓ consuming target seal
VerifiedCurrentOwnerInstanceResultTargetV1
  - exact call site
  - exact same-owner instance target declaration
  - same declaration-catalog brand
  ↓
VerifiedSourceStaticCallTargetCatalogV1
  - remains static-only
  - exact target-body static dependency sites
  ↓
VerifiedSameModuleCallableResultCatalogV1
  - remains static-only
  - exact dependency result contracts
  ↓ one neutral body-proof issue
VerifiedUnannotatedCallableBodyResultProofV1
  - existing finite proof grammar
  - existing `ExactI64` disposition
  - required argument set = empty
  - declaration/catalog pairing
  ↓ sibling co-seals with exact instance target
SealedNestedInstanceResultContractV1
```

The declaration catalog allocation is the common brand authority.  Equal
owner/method spellings from a foreign catalog are not equivalent.

## Product shape

Conceptual types:

```rust
pub(in crate::mir) struct VerifiedUnannotatedCallableBodyResultProofV1<
    'catalog,
> {
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    disposition: VerifiedCallableResultDispositionV1,
    _seal: VerifiedUnannotatedCallableBodyResultProofSealV1,
}

pub(in crate::mir) struct VerifiedCurrentOwnerInstanceResultTargetV1<
    'site,
    'catalog,
> {
    call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
    target: &'catalog VerifiedSameModuleCallableDeclarationV1,
    _seal: VerifiedCurrentOwnerInstanceResultTargetSealV1,
}

pub(in crate::mir) struct SealedNestedInstanceResultContractV1<
    'site,
    'catalog,
> {
    target: VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog>,
    disposition: ExactNestedInstanceResultDispositionV1,
    _seal: SealedNestedInstanceResultContractSealV1,
}

pub(in crate::mir) enum ExactNestedInstanceResultDispositionV1 {
    Integer,
}
```

The neutral proof product exposes only bounded disposition inspection,
same-declaration pairing, and consuming discard.  It does not expose the
static result catalog or its row map.

The target constructor accepts the verified call product.  It does not accept
detached owner, method, arity, target key, or catalog arguments from a caller.

The contract means:

```text
on a successful return from this exact source target,
the source result representation is Integer
```

It does not prove:

```text
purity
termination
totality
absence of runtime Fault
physical Call success
MIR destination identity
```

## Proof law

### Annotation is forbidden authority

The current general proof can treat an exact return annotation as immediate
authority.  This route must reject annotated instance targets before entering
the body proof:

```text
unannotated target = required
annotated lookalike = DeclaredResultAuthorityForbidden
```

This keeps the selected result source-owned and body-proven.

### Required arguments must be empty

The existing proof may produce a conditional exact result with required
argument ordinals.  The first nested contract accepts only:

```text
ExactI64(required_i64_arguments = [])
```

Any non-empty set is:

```text
NonEmptyRequiredArguments
```

It is not promoted to an unconditional contract.

### Pending after sealing is an invariant error

Static dependencies are sealed before the instance proof.  A dependency that
is still `Pending` is not a reason to retry or run a second solver:

```text
sealed dependency exposes Pending
  -> structural invariant error
```

## Failure owner

```rust
pub(in crate::mir) struct RejectedNestedInstanceResultContractV1<...> {
    owner: RetainedNestedInstanceResultOwnerV1<...>,
    stage: NestedInstanceResultContractStageV1,
    cause: NestedInstanceResultContractErrorV1,
}
```

Stages:

```text
ExactSite
InstanceTarget
DependencyBranding
StaticDependency
CoreDependency
BodyProof
ResultClosure
CoSeal
```

Representative typed causes:

```text
CanonicalMeReceiverRequired
CallerNotInstanceBoxMethod
TargetOutsideCatalog
TargetDeclarationBrandMismatch
StaticTargetCatalogBrandMismatch
StaticResultCatalogBrandMismatch
DeclaredResultAuthorityForbidden
StaticDependencyTargetUnavailable
StaticDependencyResultUnavailable
CoreReceiverProofUnavailable
CoreContractUnavailable
UnsupportedStatementKind
UnsupportedExpressionKind
MissingReturn
KnownNonIntegerReturn
ConflictingReturnRepresentations
NonEmptyRequiredArguments
SealedDependencyPending
CallTargetContractMismatch
```

Public failure surface:

```text
stage()
cause()
discard(self)
```

Forbidden:

```text
Clone
into_owner()
retry()
resume()
try_static()
try_another_target()
legacy fallback
```

## Fixture law

The existing:

```text
src/mir/callable_result_representation/tests/actual_parser_add_fixture.rs
```

owns a deliberate 15-row all-Unselected baseline.  Its `source()` and `plan()`
semantics must not be changed.

Extend that same fixture file with a separate callback/helper for the contract
proof.  It must retain:

```text
actual ParserBox target body
actual pre-loop caller slice
actual loop-refresh caller slice
actual StringHelpers dependency body
one declaration catalog brand
```

The new sibling tests borrow it through the existing test-only re-export:

```text
crate::mir::callable_result_representation::actual_parser_add_fixture
```

Do not create `source_instance_result_contract/tests/actual_parser_fixture.rs`
or clone the actual source into a second policy owner.

## Acceptance matrix

| Case | Expected |
| --- | --- |
| pre-loop exact site | `ExactI64([])` -> Integer contract |
| loop-refresh exact site | same target/body contract |
| declaration reorder | same normalized target/result |
| foreign equal-looking catalog | brand mismatch |
| receiver other than canonical `me` | typed reject |
| static caller presented to route | typed reject |
| missing or wrong-arity target | typed reject |
| annotated lookalike target | typed reject |
| same-spelled non-Integer body | typed unavailable |
| missing static dependency target | typed unavailable |
| missing static dependency result | typed unavailable |
| missing Core String contract | typed unavailable |
| target result depends on unproved argument | non-empty requirements reject |
| sealed dependency remains Pending | invariant error |
| failed proof followed by fresh proof | fresh verifier succeeds |
| existing static result catalog | instance rows remain zero |
| existing static current-owner route | instance caller remains rejected |
| existing 15-row actual fixture | all rows remain Unselected |

## First executable row

```text
NESTED-INSTANCE-RESULT-CONTRACT0-S0
```

This is one BoxCount semantic slice.  It has two commits after this docs
decision.

### Commit 1 — required Ghost reference refresh

```text
commit intent =
  refresh already-landed GenericLoop carrier inventory expectations

semantic delta =
  0

source code delta =
  0
```

Current live inventory is:

```text
direct skeleton callers = 4
v1 header PHI rows      = 3
```

The stored fixture still expects 3 and 2, so:

```bash
bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

currently ends with:

```text
[generic-loop-carrier-type-inventory] reference drift
```

Before adding the new contract owner:

1. verify the two added live rows belong to already-landed
   `generic_loop_located_composer` / carrier work;
2. update only
   `tools/checks/fixtures/generic_loop_carrier_type_m0_inventory_v1.json`;
3. rerun the existing guard to green;
4. commit the refresh separately.

Do not create a Ghost row, a new design card, or a new guard.

### Commit 2 — bounded instance result contract

Implement:

```text
VerifiedCurrentOwnerInstanceResultTargetV1
SealedNestedInstanceResultContractV1
typed retained rejection owner
narrow catalog-issued body-proof product
sole actual-source fixture callback + sibling positive/negative tests
reuse/no-widening tests
unchanged callable-result static-catalog guard
extended reusable GenericLoop lane-guard assertion
```

If the reusable GenericLoop guard gains the new structural assertion, update
its existing entry in `docs/tools/check-scripts-index.md` in the same commit.
Do not add a row-specific guard.

The BoxCount commit admits exactly one result representation:

```text
exact current-owner instance target
  + unannotated bounded body proof
  + empty required arguments
  -> Integer
```

No other source shape is admitted.

## Verification

## S0 closeout

`NESTED-INSTANCE-RESULT-CONTRACT0-S0` closed in two commits:

```text
b3c48482b9  refreshes only the stale GenericLoop inventory fixture
7085a31cd2  seals the bounded source-only nested instance Integer contract
```

The actual ParserBox pre-loop and loop-refresh sites both seal through the
same current-owner target and opaque unannotated body-proof path. The static
catalog remains static-only; no ValueId, MirType, type_ctx, MIR emission, or
production caller was added. The next required work is the separate
`NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-D0` design stop.

## Verification

Minimum gates:

```bash
cargo build --release --bin hakorune
cargo test -q callable_result_representation
cargo test -q source_instance_result_contract
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/generic_loop_progression_role_v0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The first reusable guard must be green before the semantic commit begins and
remain green after it.

## Structural gate

```text
source instance-result contract producer              = 1
current-owner instance target producer                = 1
existing callable body-result proof consumer          = 1
new expression/body walker                            = 0

static result catalog row source                      = static-only
static result catalog instance rows                   = 0
static result catalog external production consumers   = 0
static current-owner target route widening            = 0
brand-free dependency result map                      = 0

target annotation authority                           = 0
non-empty required-argument promotion                 = 0
Pending retry                                         = 0

ValueId                                               = 0
MirType                                               = 0
MirBuilder                                            = 0
type_ctx                                              = 0
MIR metadata/runtime inference                        = 0
GenericLoop result production                         = 0

source AST clone/rewrite                              = 0
name/ParserBox policy                                 = 0
fallback/retry/profile reselection                    = 0
persistent source-site -> result/ValueId map          = 0

existing actual 15-row fixture behavior delta         = 0
new per-row shell guard                               = 0
all modified/new source/check files                   < 800 lines
```

## Proof budget and retirement

```text
ceremony_tier =
  T2 new source-result authority

proof_inventory_before =
  one static result catalog
  + one 15-row actual all-Unselected fixture

new_proofs =
  one bounded nested instance result contract

retired_or_merged_proofs =
  0

net_proof_delta =
  +1

sunset_id =
  CALLABLE-RESULT-NESTED-REP0-PROOF-SUNSET-001

sunset_budget =
  +1 temporary bounded proof

sunset_row =
  CALLABLE-RESULT-NESTED-REP0-RETIRE0-S0

retirement owner =
  CALLABLE-RESULT-NESTED-REP0-RETIRE0

retire_when =
  generalized canonical callable-result authority covers the exact
  current-owner instance target and source/effect pairing
  + nested-specific production constructors/consumers = 0

budget_repayment_evidence =
  generalized callable-result parity is green for this exact target
  + SealedNestedInstanceResultContractV1 constructors/consumers = 0
  + the nested-only guard assertions are retired or merged
```

Do not issue a second sunset for the same proof.

## Downstream task order

The source contract does not yet prove that the actual Stage-B lowering path
retains the exact source site through successful physical Call emission.
Current evidence shows a site-aware legacy terminal that discards its input,
while the failing Stage-B probe may use a separate raw-prefix harness.

Therefore the next stop is:

```text
NESTED-INSTANCE-RESULT-CONTRACT0-S0
  ↓
NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-D0
```

The handoff decision must select one exact one-shot associated-source input
terminal.  It must reject:

```text
Builder-stored source maps
persistent source-site -> ValueId maps
generic emitter widening
result publication before physical Call success
```

If the S0 implementation evidence proves an already-existing exact one-shot
seam, D0 may close by selecting that seam without adding another owner.

After that decision:

```text
CALLABLE-RESULT-NESTED-REP0-P0
  exact source contract
  + exact associated-source lowering input
  -> non-Clone prepared emission receipt
  ValueId/type_ctx writes = 0

CALLABLE-RESULT-NESTED-REP0-I0
  successful physical Call
  + single-use prepared receipt
  -> publish destination MirType::Integer once

CALLABLE-RESULT-NESTED-REP0-P0b
  actual Stage-B source/Call/result parity
  failure writes = 0
  fresh compiler reuse

CALLABLE-RESULT-NESTED-REP0-G0
  producer/consumer = 1
  fallback = 0
  GenericLoop remains read-only

OWN-GRAM-REJECT0-HAKO0-S0
  resume only when the same Stage-B guard reaches the ownership syntax
  boundary
```

## Required closeout

```text
Decision:
  NESTED-INSTANCE-RESULT-CONTRACT0-prime-r1

Status:
  accepted

Choice:
  A-prime

source authority:
  exact verified current-owner MethodCall site
  + exact same-catalog instance target
  + existing static/Core dependency contracts
  + existing bounded body-result proof spine

admission:
  unannotated target
  + ExactI64 with an empty required-i64 argument set
  + empty required argument set

static result catalog:
  unchanged
  instance rows = 0

first executable row:
  NESTED-INSTANCE-RESULT-CONTRACT0-S0

first implementation action:
  refresh the stale GenericLoop inventory fixture and restore its existing
  guard to green in a separate behavior-neutral commit

immediate follow-up:
  NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-D0
```

## Non-claims

```text
general instance-call result inference
static result catalog widening
annotation-based result admission
instance method catalog activation
target purity / totality / termination

physical Call emission
ValueId / MirType / type_ctx publication
GenericLoop type production or repair
source-site -> ValueId persistent mapping

parser/Hako source edit
ownership grammar activation
VM/backend/PHI/finalization change
fallback/retry/another-route recovery
```
