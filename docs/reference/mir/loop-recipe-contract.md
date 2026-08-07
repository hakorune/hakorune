# Portable Loop Recipe Contract

Decision: accepted — `LOOP-RECIPE-PRODUCER-ID-S0` (building on
`JOINIR-LOOP-TRUE-REFERENCE-CLOSEOUT0-M7-S3-S3`).

Status: caller-zero logical reference. This page documents the portable
Recipe/JoinSig contract and the landed LoopTrue S2 producer; it does not
activate a production Loop route.

Primary design authority:
`docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`

Implementation receipt (`CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, 2026-08-07):
the canonical V2 function lowerers now enter one typed finish terminal before
DraftSeal. The portable Recipe/JoinSig algebra and its caller-zero status are
unchanged; this receipt adds no physicalizer or production Loop authority.
The bounded prepare-design correction is closed. The current boundary is
caller-zero `LOOP-PHYSICAL-PREPARE-P0`; physical/production activation remains
closed.

Executable authority:
`src/mir/loop_recipe_contract/`

## Count and shape invariant

The legacy scheduler currently exposes 19 ingress rows. That number is a
migration-coverage count, not a portable Recipe-kind count. Every accepted
row must normalize into the same recursive `LoopRecipeV1` algebra:

```text
LoopNode(condition = Always | Predicate)
Item = Operation | If | Loop | Exit
```

Nested loops use the same `Loop` item recursively. `break`, `continue`, and
in-loop `return` use the common `Exit` item. While/true/conditional-loop,
scan/accum, and Generic labels remain source-policy or legacy-adapter
identities. `IfPhiJoin` names a shared If/join obligation, not another Loop
kind. M7 establishes the shared algebra and representative adapter cohorts;
M8 closes the remaining legacy-ingress coverage. Neither milestone may add a
route-specific verifier, CFG/PHI owner, or physicalizer.

Reference receipt — `LOOP-JOINSIG-MODULE-SPLIT-R0` (2026-08-06): the former
flat `join_sig.rs` is retired. `join_sig/mod.rs` remains the stable facade;
`join_sig/model.rs`, `join_sig/port.rs`, `join_sig/visibility.rs`, and
`join_sig/flow.rs` own the logical model, port projection, visible payloads,
and recursive dataflow/elaborator respectively. The existing
`join_sig_branch.rs` keeps direct branch-row helpers, while exit-edge
projection has one owner in `join_sig/port.rs`. This is a behavior-neutral
module split: the public-in-crate API and Recipe/JoinSig goldens are unchanged,
and no selector, physical lowering, or production caller is added.

Reference receipt — `LOOP-RECIPE-PRODUCER-ID-S0` (2026-08-06): portable
provenance now uses `producer_id: LoopRecipeProducerIdV1`; the old
`producer_route` wire key is rejected instead of being accepted as a V1 alias.
The current portable profiles use `direct_accum_v1`,
`loop_true_break_continue_v1`, and `nested_predicate_v1`; `generic_g0` is
reserved for the later canonical Generic producer. `LoopRouteId` remains a
legacy scheduler/policy/registry identity and is not imported by the portable
schema or producers. Test-only `LegacyRouteParityReceiptV1` records the three
profile mappings and marks legacy Generic V0/V1 as `legacy_only`. No selector,
registry, route-order, verifier dispatch, physicalizer dispatch, or production
caller changed.

Reference receipt — `LOOP-JOINSIG-NESTED-SHADOW-S0` (2026-08-06): visible
carrier projection now walks the verified Recipe parent chain from the target
loop toward the root, keeps the first `LoopBindingKeyV1` for each binding, and
emits one payload row per binding in binding-key order. The nearest recurrence
carrier therefore shadows an ancestor carrier; three or more nested duplicates
follow the same rule. Sibling carriers are isolated by ancestry. Unknown loop
owners and same-owner duplicate carriers remain `LoopRecipeVerifierV1`
rejects, while source owner/frame/`BindingRefV1` negatives remain deferred to
the source-bound core row. This is common JoinSig behavior with no Generic,
After, PHI, physical-ID, selector, schema, producer, or production-caller
change. Focused evidence is in `join_sig_nested_shadow_tests.rs`.

Reference receipt — `LOOP-JOINSIG-AFTER-BINDING-S0` (2026-08-06): JoinSig now
publishes deterministic `LoopJoinPortBindingV1` rows for every logical
`Header`/`After` port. All incoming edges for one port must have the same
duplicate-free binding set and consistent classes; edge values are not part of
the identity. `VerifiedLoopJoinSigV1::require_after_binding` is the sole issuer
of an opaque, non-`Clone` `VerifiedLoopAfterBindingV1`. No incoming After edge
is valid but yields no capability. Wrong owner/binding, expected-class
mismatch, duplicate payload, incoming-set mismatch, and class mismatch are
typed rejects. No source `BindingRef`, Return, PHI, physical ID, Generic
selection, schema, producer, or production-caller authority is added.

Reference receipt — `LOOP-RECIPE-SOURCE-BOUND-CORE-S0` (2026-08-06): the
caller-zero source-bound core now co-seals one already verified Recipe artifact,
one verified JoinSig, an opaque structural source claim, and resolver-issued
binding/effect relations into move-only `VerifiedLoopCoreProductV1`. The issuer
owns exact Recipe-key coverage, one-to-one source `BindingRefV1` owner/class
checks, source-only declaration provenance, typed read/write/derived roles,
typed loop-statement plus Recipe-carrier anchors, and Recipe/JoinSig pairing.
Foreign, duplicate, uncovered, synthetic, wrong-class, wrong-role, and wrong
carrier relations reject before publication. This row adds no Generic key
issuance, selector, AST inspection, Builder/MIR, physical ID, retry, or
production caller. Real Generic relation instances remain an S4 responsibility.

## Callable single-loop co-seal design (2026-08-07)

Decision: accepted design r1 — `RECIPE-COSEAL-D0`.

This is a caller-zero design boundary for the selected
`StringHelpers.int_to_str/1` profile. It does not activate a Recipe producer,
physical route, or production selection. The existing nested Generic G0 S4
producer remains a separate, closed caller-zero profile; its
`VerifiedGenericAfterEffectG0` and exact-trivial ABI are not a common callable
After authority.

The callable path reuses the common chain:

```text
MAP-S1 source map
  -> common LoopRecipe/JoinSig/Core
  -> operation-source + input-source relations
  -> profile-neutral Loop continuation contract
  -> separate callable Prelude/Tail source contracts
  -> CanonicalSsaFunctionSessionV2 (later sole ValueId/CFG/PHI owner)
  -> VerifiedFunctionCompletionV1 / DraftSeal (later sole terminal owner)
```

The bounded logical mapping is:

| Source role | Portable product | Boundary |
| --- | --- | --- |
| `InitialCarrier` | `LoopRecipeCarrier(entry_value)` + `InputSourceRelation` | Preserve the `i = 0` preheader source; do not hide it as a loop-body constant. |
| `ConditionRead` / `StepRead` | `ReadBinding` operations | Exact operation/item/value/source-site relation; same carrier binding. |
| `ConditionBound` / `StepDelta` | `ConstI64(1)` values | Exact admitted literal only. |
| `ConditionOperator` | `CompareI64(Less)` | Logical compare result only. |
| `StepOperator` | `BinaryI64(Add)` | Logical arithmetic result only. |
| `StepWrite` | one `WriteBinding` | Exact target/lhs rebind; no second carrier. |
| `PrefixBoundary` | outer callable-prelude receipt | Optional direct target is preserved; absence is explicit and not repaired by name. |
| `TailReturnRead` | `VerifiedCallableTailV1` | The tail reads prefix `value`, not the loop-carrier After binding. |
| logical Loop After | `VerifiedLoopContinuationContractV1` | Common continuation only; it carries no callable Tail, ABI, or Completion. |
| loop source/frame | `SemanticContext` | Resolver/MAP retain owner/origin/source-kind, frame, Scope/Region. |

The common design names the move-only aggregate
`VerifiedLoopRecipeCoSealV1`: existing verified Core plus the typed
operation-source, input-source, semantic-context, and Loop-continuation
capabilities. `VerifiedCallablePreludeV1` and `VerifiedCallableTailV1` remain
disjoint sibling source contracts. This row does not issue an exact return ABI
or `VerifiedFunctionCompletionV1`; their existing issuers are consumed only by
the later prepared physicalization product. It must not create a second Core,
JoinSig, BindingSSA, PHI, or completion owner.
Every source row is consumed exactly once by `(site, role, target-kind)`; missing,
duplicate, foreign, unconsumed, cross-owner, or second-owner evidence rejects
before any physical effect. If a future profile cannot satisfy this common
shape, it is `NoSafeSlice`, not an invitation to add a callable-specific
Recipe kind or physicalizer. The callable row is one more instance of the
single recursive algebra; it is not a twentieth Recipe variant.

`VerifiedLoopAfterTailEnvelopeV1` is not part of the contract. The
implementation row must update this reference page and
`docs/reference/mir/generic-loop-stage-matrix.md` in the same commit. Until
fresh-session, atomic rollback, backend parity, and caller-zero gates close,
this section remains a design receipt rather than a production claim.

## Callable single-loop co-seal implementation receipt (2026-08-07)

`RECIPE-COSEAL-I0-R0` is now closed as caller-zero evidence. The test-only
`callable_single_loop_recipe_coseal.rs` consumes the sealed callable source map
once and delegates Recipe verification, JoinSig elaboration, and source-bound
Core sealing to their existing owners. It emits one common recursive
`LoopRecipeV1` with one carrier, one explicit preheader input, seven logical
operations, and one verified Loop After binding. The callable Prelude and Tail
remain separate sibling contracts; the Tail is the exact terminal statement
site and binding from the resolver/MAP product, not a reconstructed path.

The producer id `callable_single_loop_v1` is test-only provenance for this
caller-zero profile and is not a legacy route alias. The product has no
Builder/MIR/ValueId/BasicBlockId, ABI/Completion, physicalizer, selector,
retry, fallback, or production-publication authority. Focused tests cover the
positive co-seal, source-view lifetime independence, Prefix/Tail mismatch, and
Tail/Loop-After fusion rejection. The source files remain below the 800-line
lane limit. Physical preparation, function-terminal completion, production
selection, and legacy deletion remain closed.

## Caller-zero physical prepare boundary (P0)

`LOOP-PHYSICAL-PREPARE-P0` adds only typed, pre-effect contracts in the
test-only `loop_physical_prepare` module. The boundary is:

```text
exact resolved input + callable index/header
  -> callable input brand
  -> LoopRecipeCoSeal move-only demand
  -> prelude target/receiver/arity/result capability
  -> Tail/ABI/Completion compatibility receipt
  -> PreparedCallableLoopPhysicalizationV1
```

The prepared input borrows the resolved source view; the Loop demand and
compatibility receipts own only AST-free sealed products. Completion is moved
into the prepared product exactly once. No Builder, CFG, PHI, ValueId,
physicalizer, selector, publication, retry, or fallback is involved.

The existing callable fixture uses `helper.to_i64(n)` as a MethodCall. Its
resolver source ledger consequently leaves `direct_callable` absent, and the
prepare boundary must reject it as `NoSafeSlice::MissingPreludeTarget`. This
negative is intentional: injecting a free-static catalog target into that
MethodCall would not prove source resolution. A positive Prepared fixture
requires a separately verified static-call source profile with an exact
receiver/target relation; it is not silently fabricated in P0.

This section is a caller-zero implementation receipt, not a production Loop
claim. G0 must reuse the same terminal compatibility relation later, and the
physical selector remains closed until the common physicalization and parity
rows are complete.

## Contract boundary

`LoopRecipeV1` is a Builder-free semantic wire. It owns canonical recipe-local
arenas for loops, blocks, items, values, carriers, and exits. It does not own
AST lookup, route choice, physical `BasicBlockId`/`ValueId`, MIR mutation,
runtime behavior, or backend lowering.

`LoopRecipeVerifierV1` checks the closed semantic shape. It cannot inspect
source ownership, select a route, retry a failed route, or mutate a Builder.
`LoopJoinSigElaboratorV1` consumes only `VerifiedLoopRecipeV1` and emits a
deterministic logical signature. The LoopTrue S2 producer is caller-zero: it
consumes the sealed policy demand, retains the policy receipt, verifies the
source-bound artifact, and returns the verified Recipe plus JoinSig without
touching a Builder.

## LoopTrue S2 envelope

The accepted `LoopTrueBreakContinue` producer emits exactly one `Always` loop
with three blocks (`body`, `then`, `else`), one I64 binding/carrier, four
values, six items, and two exits. Its body reads the binding, compares it with
the sealed branch bound using `Equal`, and publishes one explicit-else `If`:
the then arm exits with owner-targeted `Break`, while the else arm exits with
owner-targeted `Continue`. The resulting logical edge roles are
`Enter`, `BodyEntry`, `Break`, and `Continue`; there is no `Backedge` for this
shape. The producer preserves the policy frame receipt and uses the existing
Recipe verifier and JoinSig elaborator as the only downstream authorities.

Decision: accepted for caller-zero logical parity only. This is a source-bound
structural claim from the sealed projection; it is not AST re-inspection,
route activation, physical CFG/PHI construction, runtime execution, or
backend lowering.

## JoinSig products

The verified `LoopJoinSigV1` contains the existing logical loop rows plus
caller-zero `branches` rows:

```text
LoopJoinBranchV1
  owner_loop
  if_item
  condition
  then_exit: LoopJoinBranchExitV1
  else_exit: LoopJoinBranchExitV1

LoopJoinBranchExitV1
  exit_item
  role: Break | Continue
  target_loop
  payload
```

M7-S2-A admits exactly this branch shape inside an `Always` Loop:

```text
sole body item = explicit-else If
then block     = one direct Break targeting the owner Loop
else block     = one direct Continue targeting the owner Loop
```

The branch row is ordered by owner and If item. The Loop row receives the two
logical Body edges (`Break` to `After`, `Continue` to `Header`) and receives no
natural `Backedge` for this shape. Payloads are the already-visible logical
carrier rows; no hidden ownership operation is inserted.

### Visible carrier payloads

For a target loop, the logical visible payload snapshot is defined by the
Recipe ancestry alone:

```text
target -> parent -> ... -> root
  first carrier for each binding wins
  current binding value is projected
  output rows are sorted by binding key
```

The resulting vector contains no duplicate binding. A sibling's carrier is not
visible, and the JoinSig layer does not inspect source names or manufacture
physical `ValueId`/PHI identities. Structural owner errors are rejected before
the projection owner runs.

### Header/After identity

The logical identity table is ordered by `(loop_key, port, binding)`:

```text
LoopJoinPortBindingV1(loop_key, Header|After, binding, class)
```

The table names only the binding identity and class. A later physical owner
may map it to Binding SSA and PHI, but this contract never chooses a value or
creates a physical identity. A later source-bound/Generic product requests the
opaque After capability for its exact loop and binding.

## Rejection boundary

The following remain typed rejects at this stage:

- implicit else or one-arm fallthrough;
- any branch binding write or divergent branch state;
- nested control inside either direct branch arm;
- Return or any non-owner exit in the branch pair;
- a branch block containing more than its one direct exit;
- calls, effects, physical CFG construction, PHI materialization, scheduler
  selection, retry, and legacy-route fallback.

`BranchMergeMismatch` is the logical rejection for a branch that is not the
accepted direct pair. Existing `UnreachableItem`, `UnsupportedExit`, and
carrier/value availability errors remain owned by their existing JoinSig
checks.

## Non-claims and next slice

This row claims only projection from the already sealed source shape into the
source-bound Recipe artifact. It does not claim fresh AST-to-Recipe discovery,
route activation, physical CFG/PHI parity, runtime execution, or deletion of
the located legacy Loop handoff. Binding merge and implicit-fallthrough
products require a separate design/implementation row after this logical
closure. The required README and reference updates are landed in this
closeout.
