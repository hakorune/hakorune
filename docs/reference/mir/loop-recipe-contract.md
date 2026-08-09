# Portable Loop Recipe Contract

## Explicit typed Recipe V2 wire — `LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0`

Decision: accepted — V2 is an explicit schema boundary; V1 is not silently
widened. This receipt covers only the profile-neutral typed wire and its
structural verifier. It does not issue resolver instance-call targets or
source-bound Home/effect/ABI relations.

The V2 logical vocabulary preserves `I64`, `Bool`, and `Unit`, and adds the
logical `Text` class. It adds two operations:

```text
CallSlot {
  receiver: Option<ValueKey>
  args: [ValueKey, ...]
  result: Option<ValueKey>
}
TextEq { left: ValueKey, right: ValueKey, result: ValueKey }
```

`CallSlot` is a recipe-local slot only. Method/Box names, resolver
capabilities, Home/effect/ABI contracts, MIR/physical IDs, and runtime lookup
strings are source-bound concerns and are not on the V2 wire. The first
callable cohort may later require a receiver and result, but that admission
policy belongs to the resolver/source relation row.

`LoopRecipeVerifierV2` rejects unsupported schema versions, non-canonical
keys, unknown references, duplicate value definitions, invalid numeric
domains, and non-`Text` `TextEq` operands or non-`Bool` results. The landed
operand-definition guard also requires every CallSlot, numeric, TextEq,
WriteBinding, If, and Return operand to have a verified prior definition; a
declared value key alone is insufficient. It does not
claim source existence, target resolution, input-source relations,
ScanWithInit, Loop/Tail/Completion, Builder/MIR/physicalization, fallback, or
production activation. The focused receipt is
`typed_schema_v2_tests.rs` (twelve tests, including independent duplicate,
forward-use, Return-key, and wrong-result-class negatives); all touched Rust files remain below the
760-line design trigger and 800-line hard boundary.

The next independent rows are resolver instance-target issuance, source-bound
call relation verification, typed parameter/initializer input relations, and
only then the S6C ScanWithInit observer/producer. No guessed scan counts or
legacy deletion belongs to this receipt.

## Accepted Dynamic value boundary — V2 only

Decision: accepted and schema I0 landed — the source-backed Dynamic invocation
result is represented honestly as `LoopValueClassV2::Dynamic`. It is a logical Recipe class, not
`Unknown`, `Any`, a runtime tag, provider identity, Home state, or physical
representation. V1 remains unchanged.

The class and the source relation are complementary authorities:

```text
LoopValueClassV2::Dynamic
  = Recipe-local type consistency

item/source Dynamic call relation
  = exact source call + resolver target + indivisible invocation envelope
```

The V2 wire stays target-free. A later item-keyed sibling co-seal borrows the
complete Dynamic envelope catalog and relates one exact CallSlot to one exact
source call without copying selector, Home, effect, suspension, Fault, or
provider facts. Dynamic may appear in type-consistent V2 value/input/binding/
carrier/CallSlot-result/Return positions, but semantic-program publication
requires complete source relations. Loop/If predicates remain Bool; existing
I64 and Text operations do not accept Dynamic. Dynamic JoinSig/After requires
a separate V2 authority and is not claimed by the current V1 JoinSig.

Fault is outside Recipe lexical/value control. A Dynamic CallSlot result is
defined only on Normal completion; Fault is not a value, resultless CallSlot,
Loop exit, JoinSig edge, Return, Completion, or DraftSeal result.

The unchanged source module owns seven Dynamic envelopes. Exact membership is
`3` Loop-owned calls and `4` valid non-Loop calls; the first `skip_while/4`
Recipe relation selects its exact `2` Loop calls while retaining all `7`
catalog rows. Fixture reduction, selector-based filtering, source rewriting,
and fallback are forbidden. The ordered implementation starts with the
behavior-neutral V2 operand-definition guard, then Dynamic value schema,
source-value relations, and exact CallSlot/envelope co-seal.

The atomic source/Recipe/envelope co-seal is now landed. One consuming issuer
validates the complete six-binding and twenty-eight-source role tables, then
binds I6 and I7 through exact resolver owner plus exact call site against a
borrowed complete envelope catalog. Candidate and catalog must derive from the
same branded semantic-source authority; an equal-looking foreign resolver
owner is rejected. The issuer does not require the callable owner to have only
two Dynamic rows: additional valid rows remain unselected in the complete
catalog. The current seven/two/five counts are regression evidence only, not a
language acceptance limit.

The caller-zero Dynamic semantic program now also retains one private complete
Fault cut-point catalog. It derives four rows from the verified V2 operation
contract and two rows from the exact Dynamic call-relation seal:

```text
I1  DynamicLess       -> V5
I5  DynamicAdd        -> V9
I6  DynamicInvocation -> V10
I7  DynamicInvocation -> V11
I9  DynamicLess       -> V13
I15 DynamicAdd        -> V17
```

The order is verified Recipe item order. A row means only that the operation
may produce Fault before its named normal result is published. The catalog is
an internal field of the non-Clone semantic program and exposes only a
borrow-scoped view; callers cannot supply or split a Recipe, call item, Fault
family, or expected schedule. It does not create a Fault value, Recipe Exit,
JoinSig edge, Home/cleanup obligation, Completion claim, or physical route.
The complete exit transaction remains closed until source-backed Dynamic local
Home Flow and multi-Return Completion consumption are available.

The next consuming wrapper now co-seals the two `DynamicAdd` Normal-result
lifecycle rows without accepting caller-supplied Recipe keys, source sites,
operator contracts, or JoinSig data:

```text
I5 -> V9:
  exact I6 argument 1
  BorrowedNoEscapeForInvocation
  end after the I6 Normal-or-Fault outcome

I15 -> V17:
  exact I16 WriteBinding(B0,V17)
  exact Backedge(B0=V17)
  forward at the later rebind commit
```

Only borrow-scoped views leave the non-Clone wrapper. This receipt does not
end the displaced B0 carrier, install V17, infer Home, execute cleanup, or
claim CFG/MIR/Completion authority.

The landed caller-zero V2 golden covers one consistent Dynamic input, binding,
ReadBinding result, carrier, CallSlot normal result, WriteBinding, and Return.
Eighteen focused schema tests reject forward/undefined/duplicate values,
Dynamic predicates, I64/Text domain confusion, mixed carrier classes, and V1
decode. The unchanged envelope regression retains all seven source rows. No
source-value relation or semantic-program publication is claimed yet.

Top-down audit found that a standalone source-value relation I0 would be
partial truth: no complete production V2 Recipe producer exists yet, and the
unchanged source still needs logical Dynamic Add/Less operations. The accepted
order is therefore Dynamic operation vocabulary, complete full-body producer,
then one atomic source/Recipe/CallSlot/envelope co-seal. No durable intermediate
`Verified*` key relation is published. The module-wide seven-row envelope
catalog is borrowed, never consumed by one Recipe.

The Dynamic operation Decision is accepted. V2 uses exact
`DynamicAdd` and `DynamicLess` variants instead of widening `BinaryI64` /
`CompareI64` or adding an unconstrained Dynamic operator family. The first
domain table is:

```text
DynamicAdd:
  Dynamic x I64 -> Dynamic

DynamicLess:
  Dynamic x Dynamic -> Bool
  Dynamic x I64     -> Bool
```

The unchanged `skip_while/4` source requires four exact rows: root `i < end`,
substring-end `i + 1`, inner `indexOf(ch) < 0`, and step `i + 1`. The two
equal literal `1` sites and the literal `0` site remain three separate
`ConstI64` operations/source anchors. The prior P1/S0 condition-plus-step
proof is migration evidence, not complete four-row authority.

These operations follow the language operator contract in
`docs/reference/language/dynamic-operators.md`: normal Add publishes one Dynamic value,
normal Less publishes one Bool, and unsupported runtime operand kinds produce
TypeError/Fault with no result. Fault is not a Recipe value, resultless
operation, false predicate, Exit, JoinSig edge, Return, or Completion. The
schema I0 is logical only; physical/provider/runtime execution remains closed
until a route proves the same result/Fault behavior without retry or fallback.

Implementation receipt (`LOOP-V2-DYNAMIC-OPERATION-I0`, 2026-08-10): V2 now
contains the two exact variants and verifies all three admitted domains. A
dedicated four-operation golden preserves two Add, two Less, and three distinct
`ConstI64` rows (`1`, `0`, `1`). Eight focused Dynamic-operation tests and the
complete twenty-six-test V2 schema suite pass. V1 decoding remains unchanged.
No source relation, full producer, physical writer, or production activation
is introduced by this receipt.

The profile-neutral operator issuer is now also live. Beside the V2 operation
schema, `LoopOperationExecutionClassV2` exhaustively classifies every current
variant as non-Faulting, Fault-before-normal-result, or externally bound
outcome. The bounded Fault catalog consumes that projection and has no wildcard
skip, so a future operation variant must update the canonical classification.
This adds no Fault value/edge, source relation, Home, or physical behavior.

Complete-producer preflight (`LOOP-V2-DYNAMIC-FULL-PRODUCER-D0`, 2026-08-10):
the unchanged `skip_while/4` source is fully expressible with the existing V2
logical vocabulary:

```text
1 Loop / 3 blocks
1 Dynamic induction binding / 4 Dynamic inputs / 1 carrier
18 values / 17 items / 1 inner Return exit
2 CallSlots / 2 DynamicAdd / 2 DynamicLess / 3 distinct ConstI64
```

The iteration-local `ch` is a source-local relation over the substring result,
not a second Recipe binding or carrier. The inner Return is the only Recipe
Exit; the outer Return is Callable Tail and remains paired with the same
two-site Completion product.

The preflight found that V2 artifact verification is weaker than V1: it does
not yet prove exact root/block/If/Exit use and ownership, recursive preorder,
terminal Exit placement, or structural source-binding coverage/path shape.
The next row is therefore the behavior-neutral
`LOOP-V2-CONTROL-STRUCTURE-GUARD-R0`, followed by the complete private
producer. This is a compiler correction boundary; narrowing or rewriting the
accepted source is forbidden. No source/envelope co-seal, Home, JoinSigV2,
Completion consumption, physical route, or production activation is claimed
by this Decision.

Implementation receipt (`LOOP-V2-CONTROL-STRUCTURE-GUARD-R0`, 2026-08-10):
V2 now seals root/parent identity, condition/body/If block ownership, exact
block/Loop/Exit use, terminal Exit placement, Break/Continue ancestry, and
recursive Loop/Block/Item preorder in a neutral structure module. Artifact
verification also validates the shared Loop-source wire and retains a
non-Clone structural claim instead of raw source coordinates. The
resolver-issued non-Clone Loop root has a consuming V2 adapter, so no caller
may manufacture the Recipe root key. Six structure tests, thirty-seven V2
schema tests, and the focused adapter test are green. The row adds no new
source shape, operation, JoinSig, producer, physical route, or production
caller. This R0 is the prerequisite for the complete producer receipt below.

Implementation receipt (`LOOP-V2-DYNAMIC-FULL-PRODUCER-I0`, 2026-08-10):
the unchanged production `skip_while/4` source now issues the complete
1-Loop/3-block/17-item verified V2 artifact. It retains all six binding rows,
twenty-eight source rows, and the original two-site Completion beside the
artifact. The non-Clone resolver Loop token moves exactly once into the
artifact's structurally verified path claim; its frame and scope/region remain
in the retained source product, so no authority is copied or discarded. Exact
source-to-Recipe correspondence remains private unsealed candidate truth.
Private role-to-key claims cover the complete source boundary while `ch`
remains a V10 local-value relation and the outer Return remains Callable Tail.
Three focused tests are green. No envelope co-seal, JoinSigV2, physical route,
fallback, or production caller is introduced. The next row is the atomic
source-bound Recipe/envelope co-seal design.

Implementation receipt (`LOOP-RECIPE-V2-JOINSIG-DYNAMIC-I0`, 2026-08-10): V1
and V2 Recipe wires now feed one private common JoinSig flow engine through
exact borrowed views. The V2 adapter exhaustively projects all current V2
operation def/use forms without a V2-to-V1 conversion, and the typed V2 seal
preserves `Dynamic`. Its branch target is exactly
`LoopJoinBranchExitTargetV2::{Loop, FunctionExit}`; Return-to-Loop and
Break/Continue-to-FunctionExit are rejected by the target/role contract while
the V1 Return-arm rejection remains unchanged.

The complete unchanged Recipe produces Enter, PredicateTrue,
PredicateFalse, inner Return, and Backedge edges with carrier payload values
`V1/V1/V1/V1/V17` for `B0`, one I10/V13 Return/fallthrough branch, and only
Header/After `B0:Dynamic` port bindings. Exact tests prove that `V10/ch` and
Return operand `V14` never enter payloads or ports, and the V2 verifier rejects
either value when substituted as the root carrier entry. Fifteen focused
Dynamic full-body tests, all 31 focused JoinSig regressions, and `cargo check
--lib` are green; the largest touched source file is 757 lines. The outer source
Return remains Callable Tail/Completion and Dynamic Fault remains outside
Recipe/JoinSig. Source co-seal, After/Continuation issuance, Completion
consumption, Home/Fault handling, physical lowering, production selection,
retry, and fallback remain closed. The next row is the atomic V2 semantic
program co-seal.

Implementation receipt (`LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0`, 2026-08-10):
the exact Dynamic source/Recipe/envelope is now the sole input to one atomic
semantic-program issuer. The neutral V2 owner derives the verified Recipe root
and its exactly-one root carrier, privately elaborates JoinSig, privately
requires the matching After, and returns one non-`Clone`, non-splittable
`VerifiedLoopJoinClosureV2`. No raw V2 After or V2 elaborator remains in the
production facade.

`VerifiedDynamicFullLoopSemanticProgramV2` moves the complete envelope and
that control closure together. It lends exact `L0/B0/Dynamic` After and the
existing V10/I6/I7 iteration-local relation; V10 and V14 remain absent from
all payloads, ports, and After identity. Its issuer accepts no owner, root key,
Recipe, JoinSig, After, Continuation, or Completion input. The already-sealed
two-site Completion partition remains inside the envelope and is neither
rechecked nor consumed here. Dynamic Fault, Home, physical transfer/layout,
Builder/MIR/CFG/PHI, DraftSeal, collector, publication, retry, and fallback
remain closed.

Reference receipt — `LOOP-JOINSIG-NEUTRAL-ENGINE-R0` (2026-08-10): the
verified V1 Recipe now enters one private borrowed V1 view and one common
class-generic JoinSig flow engine. The view exhaustively projects every V1
operation/item/exit/carrier/class and is neither stored nor serialized. The
public-in-crate V1 facade and aliases remain stable; all 31 focused JoinSig
tests preserve their existing exact loop rows, branch rows, payload order,
port bindings, After capabilities, rejection matrix, and accepted shapes.
`cargo check --lib` is green and the largest touched source file is 634 lines.
This receipt imports no V2 schema or Dynamic class and adds no one-arm Return,
FunctionExit branch target, semantic-program co-seal, Continuation, physical
effect, production caller, retry, or fallback. The separately named V2 I0
above is the only later widening of that neutral engine.

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
The bounded prepare-design correction, Callable full physical canary, and G0
Builder-free exact ingress are closed. The private Recipe-derived
segment/resume plan and Callable segment-block adapter are closed. A worker
premise audit found that R2 maps onto the old fixed Header/Body/Step/After
topology rather than allocating one block per R1 segment. The selected
Callable path now has the R3-I0 implementation receipt below: exact R1
segment allocation, neutral recursive After sealing, and the existing
Tail/Completion/DraftSeal handoff are live in the caller-zero canary. G0
physical and production activation remain closed.

Implementation receipt — `CALLABLE-LOOP-AFTER-CLOSURE-P0` (2026-08-07): the
caller-zero continuation canary now consumes a real Prelude materialization
receipt and the complete seven-operation Callable schedule (`Pure=4`,
`Read=2`, `Write=1`) before issuing the fixed CFG edges. Preheader, body, step,
header, and After are sealed through the canonical CFG/identity owners, then a
single session-local `ReadyLoopAfterContinuationV1` is issued. Unsealed PHI
`Unknown` is published only from the verified Recipe value class; concrete or
missing type facts reject as `ResultTypeMismatch`. The successor Tail slice
now consumes that receipt, reads the exact Tail binding through canonical
identity, validates the declared `i64` ABI, and claims Completion/return
coverage once. The After receipt forwards a non-Clone profile-close receipt
for the sealed After predecessor and exact callable coverage (`7 = Pure4 +
Read2 + Write1`, including the Bool condition); later finish must consume it
through a non-no-op closure. Finish/DraftSeal, production selection,
retry/fallback, and legacy retirement remain closed. The bounded DraftSeal
success canary consumes that receipt through the typed function-finish
terminal and existing DraftSeal prepare/commit, producing one unpublished
`CompletedFunctionDraftV1` without collector or module publication. The next
boundary is the docs-only named production-edge census; no production switch
or selector is active.

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

## Callable source/facts issuer S0

Decision: accepted implementation slice —
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0` (2026-08-08).

The production source/facts boundary has one resolver site issuer and one
owner-branded navigation seam:

```text
CallableSemanticSourceLedgerView::only_loop_site()
  -> exactly one resolver-sealed Loop membership
  -> zero or multiple memberships = typed rejection

FunctionSourceViewV1::stmt_at(membership)
  -> exact statement only when present in the sealed owner inventory
```

This slice is source/facts transport only. It does not activate Recipe,
JoinSig, Prepared physicalization, Builder/MIR emission, a selector, retry,
fallback, Generic G0 substitution, or a production caller switch. Source
observation may inspect verified `root_body()` contents for totality, but raw
AST/path/name/ordinal reconstruction is not a source authority. The S0
bounded negative matrix and exact resolver-identity parity are closed; issuer
construction callers remain test-only and no external diagnostic mapping is
claimed. SourceMap branch-level duplicate-evidence coverage is deferred to the
later Recipe/JoinSig parity row.

`VerifiedLoopAfterTailEnvelopeV1` is not part of the contract. Fresh-session,
atomic rollback, backend parity, and physical caller-zero gates belong to later
physicalization rows; this source/facts receipt does not claim them.

## Callable logical issuer D0/S0

Decision: accepted design —
`CALLABLE-LOOP-PRODUCTION-LOGICAL-ISSUER-D0` (2026-08-08).

The bounded implementation is closed. It reuses `LoopRecipeVerifierV1`,
`LoopJoinSigElaboratorV1`, `VerifiedLoopJoinSigV1::require_after_binding`, and
the source-bound Core co-seal. The seven source roles map to the canonical
recipe items `I0..I6`/values `V0..V6` exactly as recorded in the D0 design
task; source roles are consumed once and `CallableSingleLoopV1` is
diagnostics-only provenance. The production profile shape is owned by
`callable_single_loop_recipe.rs`; `callable_recipe()` remains a test-only
parity fixture wrapper.

The issuer is production-scoped but caller-zero: it has no selector, physical
consumer, or production caller. Focused logical-issuer, SyntaxFacts, and
SourceMap tests are green; the next step is a fresh design stop for the
prepared ingress, not physical lowering.

This row does not authorize Prepared/ABI/Completion physicalization, CFG/SSA/
PHI/Builder/MIR, selector/admission, a production caller switch, Generic G0,
retry/fallback, legacy retirement, runtime/backend behavior, or user-facing
diagnostic mapping.

## Callable single-loop co-seal implementation receipt (2026-08-07)

`RECIPE-COSEAL-I0-R0` is now closed as caller-zero evidence. Its historical
test-only implementation in `callable_single_loop_recipe_coseal.rs` consumed
the sealed callable source map once and delegated Recipe verification,
JoinSig elaboration, and source-bound Core sealing to their existing owners. It
emits one common recursive
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

## Callable Prelude argument receipt P0 (2026-08-07)

Decision: accepted as caller-zero pre-effect evidence.

The callable Prelude boundary now owns one move-only,
AST-free `VerifiedCallablePreludeArgumentListV1`. Its rows preserve the exact
argument ordinal, source site, resolver-issued `BindingRefV1`, and the first
profile's exact `i64` representation. The issuer accepts only direct local
parameter variables in the genuine resolver-backed `FreeStatic` fixture.
Literals, nested expressions, upvars, foreign bindings, arity mismatches, and
unsupported ABI shapes remain typed `NoSafeSlice` outcomes; no name lookup or
arity-only reconstruction is permitted.

The prepared Prelude owns and transfers this list exactly once. The future
outer materializer may consume it to install canonical session entry bindings
and issue a private `ReadyLoopEntryV1`; the common recursive physicalizer never
receives the argument list, AST, Tail, ABI, Completion, or source input view.
The focused prepare suite verifies the positive ordinal, owner, and ABI
receipt. No Builder/MIR effect, selector, retry, fallback, or production
selection is opened by this row. The next bounded row is
`LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`.

## Callable Prelude materialization receipt P0 (2026-08-07)

The first caller-zero physical Prelude adapter is now landed as a test-only
receipt. It consumes the prepared resolver-backed Prelude capability and
publishes all values through the existing canonical session owners:

```text
resolver parameter/argument BindingRef rows
  -> canonical identity reads
  -> resolver-issued static call header
  -> Prelude result ValueId + result-local declaration

co-sealed Loop input initializer source site
  -> exact source-view literal
  -> entry ValueId + Loop input declaration
  -> ReadyLoopEntryV1
```

The Prelude result binding and the Loop input binding are distinct resolver
bindings. The adapter rejects missing or mismatched input declarations and
unsupported initializer shapes; it does not infer one binding from the other,
search the AST by name, or pass the Prelude result as the Loop entry value.
The focused canary emits exactly one static call and one entry initializer,
then discards the unpublished function session. The topology After receipt is
still open allocation evidence and is not readable by Tail. The next bounded
row is `CALLABLE-LOOP-AFTER-CLOSURE-P0`, which must emit the fixed CFG edges,
seal CFG/identity, and issue one session-local `ReadyLoopAfterContinuationV1`.
Only that sealed receipt may feed Tail. Tail, Completion,
`finish_for_draft_seal`, DraftSeal, operation `emit_all`, and production
selection remain closed.

## Recursive physicalizer P0 scope (2026-08-07)

This closed historical row is a caller-zero topology/After probe. It consumes
the topology-only compatibility `VerifiedLoopPhysicalDemandV1` and one session-local
`ReadyLoopEntryV1`, borrows the existing canonical session services, and may
construct only the recursive logical child/header/body/step/After topology.
It does not physically emit `ReadBinding`, `WriteBinding`, constants,
comparisons, or arithmetic. Those operations remain typed `NoSafeSlice` until
the later `LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0` design boundary issues a
neutral item-keyed, exact-source-anchor effect projection. Name, ordinal, or
profile-based matching is forbidden. No Return, DraftSeal, publication,
selector, retry, fallback, or legacy deletion is opened by this row.

That compatibility demand is not the current full operation input and cannot
feed operation emission. The canonical target is the complete
`VerifiedLoopOperationPhysicalDemandV1` described below.

## Recursive physicalizer P0 implementation receipt (2026-08-07)

The caller-zero canary now has a test-only common boundary. A move-only
`VerifiedLoopPhysicalBoundaryV1` carries the existing Core plus the issued
logical After capability; `ReadyLoopEntryV1` carries the exact entry keys,
resolver bindings, and preheader for one fresh function test session. The
common `physicalize_topology_v1` allocates only the recursive
header/body/step/After block skeleton and returns one
`LoopAfterContinuationReceiptV1`. It validates owner, preheader, exact input
coverage, binding ownership, parent topology, and root After identity before
the first block allocation. Unknown parents reject; they never fall back to
the root preheader.

Focused canary coverage proves a two-level Generic G0 Recipe produces child
and root After continuations, child preheader placement is preserved, and an
incomplete entry is rejected without allocating blocks. The module is
`cfg(test)` and has no production caller, selector, MIR operation emission,
Return, DraftSeal, publication, retry, fallback, or legacy deletion. The
operation boundary is now the passive
`LOOP-RECIPE-OPERATION-EFFECT-S0` product below; physical operation emission
remains closed.

## Operation/effect product S0 (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-S0` is closed as a caller-zero, passive
item-keyed verifier. `VerifiedLoopOperationEffectProductV1` moves one sealed
`VerifiedLoopCoreProductV1` exactly once and owns only the profile-issued
source-evidence ledger. Recipe operations, operands, `BindingRefV1`, and
binding-effect rows remain views into the moved Core; a second operation or
effect catalog is not created.

The verifier requires one evidence row for every Recipe `Operation` item,
exact Recipe block/loop placement, owner-branded source anchors, and exact
`SourceRead`/`SourceWrite` Core relations for binding operations. Pure
constant, binary, and comparison operations reject fabricated binding
evidence. Duplicate, missing, foreign, wrong-owner, wrong-placement, invalid
source-loop, missing-effect, role-mismatch, and class-mismatch cases reject
before any physical effect. Tail/After reads and structural carrier rows stay
outside this product by contract.

Focused tests cover nested positive coverage (19 operation items), duplicate
and missing evidence, foreign owner, wrong placement, and fabricated binding
evidence on a pure operation. This row emits no operation MIR, ValueId,
BasicBlockId, Return, DraftSeal, selector, retry, fallback, or production
route. The Callable source-evidence adapter is now a separate closed receipt;
the Callable source-evidence adapter and Generic G0 anchor ledger are separate
closed receipts; cross-profile parity is now the current next row and
operation physicalization remains closed.

## Callable operation/effect adapter S0 (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-CALLABLE-ADAPTER-S0` is closed as caller-zero
evidence. The adapter consumes the existing callable co-seal once, compares
each transient operation view with the sealed Recipe item, derives exact
block/loop placement from the Core, and matches binding operations to the
existing Core `SourceRead`/`SourceWrite` relation. It then issues the neutral
`VerifiedLoopOperationEffectProductV1` while retaining the callable Prelude,
Tail, input, semantic context, and continuation in one profile wrapper.

No operation view, operand, binding relation, effect catalog, or second Core
is copied. The adapter has no Builder/MIR, selector, retry, fallback, Return,
DraftSeal, publication, or production authority. The Generic G0 anchor row is
also closed: its producer issues the explicit 15-row item-to-anchor ledger
before source facts leave the producer boundary, with item 3 as the existing
child-entry `DerivedCarrierEntry` for carrier 2. Item 4, C0/C1 carriers, and
Generic tail reads remain outside the operation product. Cross-profile parity
and reviewed Decision B are closed as caller-zero boundaries. Decision B keeps
the complete operation demand/preflight separate from private leaf emission;
the next implementation is the Builder-free full-demand P0.

## Cross-profile operation/effect parity receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EFFECT-CROSS-PROFILE-PARITY-S0` is closed as caller-zero
diagnostic evidence. Both profile adapters issue the same neutral
`VerifiedLoopOperationEffectProductV1`; the parity receipt validates the
shared schema and owner-branded evidence without comparing profile item counts
or source order. Callable has seven rows, Generic G0 has fifteen, and Generic
item 3 remains the profile-specific `DerivedCarrierEntry` for child carrier 2.
Prelude/Tail and After/tail reads remain separate.

The existing common product verifier remains the sole authority for exact
Recipe operation equality, item placement, Core effect matching, and the
duplicate/missing/foreign/wrong-placement/pure-binding rejection family. The
parity receipt adds no second operation/effect catalog and does not select by
count, ordinal, profile label, or source preorder. Focused parity evidence is
green (8 operation/effect tests and 43 Generic G0 tests).

No operation MIR, physicalizer, selector, retry/fallback, publication, or
legacy deletion is opened by this receipt. Reviewed Decision B fixes one
private move-only `VerifiedLoopOperationPhysicalDemandV1` as the complete
full-program input: it bundles the moved resolver semantic context,
operation/effect evidence, common continuation, and a key-only private index
and exposes no single-operation extraction API.
The Builder-free `prepare_all` and behavior-neutral physicalizer module split
rows are now closed. The next implementation row is the canonical physical
block receipt; the Const leaf-emitter row is now closed, and the Generic item-3
bridge remains a later row.

## ConstI64 leaf-emitter S0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` is now closed as a private,
test-only leaf-emitter canary. A prepared `ConstI64` operation is checked
against the exact owner, preheader, logical Loop/Block, physical role, and
un-terminated destination block in `LoopPhysicalBlockReceiptV1`. The emitter
delegates to the existing canonical Builder Const/type-fact path and returns
one physical `ValueId` receipt; it does not infer placement from
`current_block`.

Focused evidence proves exact one-instruction placement, typed owner/placement
rejects before emission, a harness-only post-emission failure with whole
unpublished-session discard, and semantic repeat in a fresh session. This row
does not extract one item from a full demand and does not open full Loop
physicalization, continuation, BindingSSA/PHI, Return, Completion, DraftSeal,
production selection, retry/fallback, or legacy deletion. The next operation
kind must be opened as a separate design/implementation row.

## ReadBinding leaf-emitter I0 receipt (2026-08-07)

`LOOP-RECIPE-OPERATION-EMITTER-READ-I0` is closed as a bounded, private
test-only leaf receipt. The complete `PreparedLoopOperationProgramV1` now
projects every `ReadBinding` row and checks the Recipe operation, source
`Expr(OwnedExprSiteV1)`, `SourceRead` effect relation, binding, owner, and
logical placement together. Ordinary expression leaf projection excludes
`DerivedCarrierEntry`; the common I0 receipt below handles that anchor through
a separate full-program carrier-seed row. No single-operation demand
extraction exists, and no Generic-specific physicalizer was added.

The leaf borrows the canonical BindingSSA/PHI owners through
`CanonicalBindingReadServicesV1`, claims the exact source site, and receives
one `CanonicalBindingReadReceiptV1`. Its immutable
`ReadBindingEmissionReceiptV1` keeps logical and physical blocks distinct.
Entry requirements are explicit: `PreheaderSeed` requires an exact entry row,
while `CanonicalLive` uses canonical SSA availability. Pre-claim rejects are
typed; claim/read/type/receipt failures are terminal to the unpublished
function session and must be discarded as one transaction.

This receipt opens no other operation, carrier seed, full Loop physicalizer,
continuation/Tail, Return/Completion, DraftSeal, selector, retry/fallback,
production route, legacy deletion, or performance claim. The production
replacement row remains open until a named caller switch and old-edge
retirement are landed.

## Full operation demand P0 receipt (2026-08-07)

The Builder-free demand and `prepare_all` are now landed. Callable's seven
operation rows and Generic G0's fifteen rows are scheduled from Recipe
Loop/Block/Item structure with zero Builder/MIR effect. The neutral context and
continuation wrappers move existing resolver/JoinSig evidence exactly once.
This receipt does not open physical block mapping, operation emission,
function completion, selection, fallback retirement, or legacy deletion.

## Physicalizer module split R0 receipt (2026-08-07)

The test-only topology physicalizer now lives behind one directory facade:
`loop_recipe_physicalizer/mod.rs` re-exports the unchanged topology API,
`topology.rs` owns the recursive block skeleton, and `tests.rs` owns the two
focused topology canaries. The former flat `loop_recipe_physicalizer.rs` file
is retired. This is a behavior-neutral BoxShape split: nested Generic G0
topology and pre-allocation entry rejection remain unchanged, and no operation
shape, physical block receipt, emitter, session, selector, or legacy route is
opened.

## Physical block receipt P0 (2026-08-07)

The topology canary now owns one private `LoopPhysicalBlockReceiptV1` issued
from the existing canonical CFG allocation. It records the owner, root
preheader, and exact `Preheader`/`Header`/`Body`/`Step`/`After` rows for each
logical Loop. Existing topology/After queries use this receipt as their sole
physical mapping; no second CFG/SSA/PHI owner or implicit `current_block`
placement is introduced. The focused receipt checks preserve nested Generic G0
allocation and reject incomplete entry before allocation. Operation emission,
session/Completion, selection, fallback, and legacy deletion remain closed.

## Callable source-shape split receipt (2026-08-07)

`CALLABLE-SOURCE-SHAPE-THIN0` is closed as a behavior-neutral BoxShape slice.
The observer's neutral syntax vocabulary now lives in the small
`callable_single_loop_source_shapes.rs` module. The neutral SyntaxFacts and
SourceMap issuers compile in production scope; fixture constructors,
mutation helpers, and syntax-observer/source-map tests remain test-only
siblings. The issuer entry uses resolver `only_loop_site()` and owner-branded
`stmt_at(membership)`; multiple sites reject before AST projection. The
SourceMap retains resolver Loop/frame/Scope-Region identity and is checked
against SyntaxFacts without introducing a Bridge or Recipe owner.
`SourceCallKindV1::Method` and
`SourceCallKindV1::FreeStatic` are explicit shape labels; neither label is a
resolver target or a proof of a callable ABI. The existing `helper.to_i64(n)`
MethodCall remains the typed `MissingPreludeTarget` negative.

This split changes no resolver, Recipe, JoinSig, physical, Builder, selector,
retry, fallback, publication, or production behavior. The next bounded row is
`CALLABLE-STATIC-PREFIX-S0`, which may add only the exact resolver-backed
`FreeStatic` fixture and its observer evidence. Same-compilation different-owner
target validation and declaration-derived ABI remain later P0 boundaries.

## Callable static-prefix observer receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-S0` is closed as a caller-zero source-observation
cell. A separate top-level catalog fixture now resolves
`int_to_str(n: i64): i64` calling `to_i64(n: i64): i64` through the existing
callable index and direct-call ledger. The observer records
`SourceCallKindV1::FreeStatic` with the exact arity and retains the resolver
source site; it does not inject a target or infer an ABI. The existing
`helper.to_i64(n)` `MethodCall` remains a typed `Method` negative with no
direct callable target.

The positive fixture proves only source-shape and resolver target evidence.
Same-compilation different-owner source-map acceptance is the next bounded
`CALLABLE-STATIC-PREFIX-MAP-S1` task; declaration-derived ABI and a positive
Prepared product remain later P0 work. No Recipe, JoinSig, physicalizer,
Builder/MIR, selector, retry, fallback, publication, or production route is
opened by this receipt.

## Callable static-prefix source-map receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-MAP-S1` is closed as a caller-zero source-map
relation. The resolver-issued `to_i64` target is retained when its function
owner differs from the caller but its compilation brand matches. Independently
sealed catalogs provide the foreign-brand negative, which rejects as typed
`ForeignOwner` before any map product is issued. The MethodCall fixture stays
a typed negative with no direct callable target.

This product still owns no ABI, Prepared relation, Recipe, JoinSig, physical
ID, Builder/MIR effect, selector, retry, fallback, publication, or production
caller. The next bounded cell is `CALLABLE-STATIC-PREFIX-P0` for
declaration-derived ABI and Prepared evidence.

## Callable static-prefix Prepared receipt (2026-08-07)

`CALLABLE-STATIC-PREFIX-P0` is closed as a caller-zero pre-effect relation.
The caller result ABI is derived from the sealed completion declaration and
exact callable header; the callee result ABI is derived from the resolver
target header. The resolver-backed `FreeStatic` fixture therefore produces one
positive `PreparedCallableLoopPhysicalizationV1`, while the MethodCall fixture
continues to reject with typed `MissingPreludeTarget`.

No Builder session, physical ID, physicalizer, selector, retry, fallback,
publication, or production caller follows from this receipt. The next step is
the design-only common physicalizer/session boundary stop.

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
  then_arm: LoopJoinBranchArmV1
  else_arm: LoopJoinBranchArmV1

LoopJoinBranchArmV1
  Exit(LoopJoinBranchExitV1)
  Fallthrough { payload }

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
natural `Backedge` for this explicit-exit shape. Payloads are the
already-visible logical carrier rows; no hidden ownership operation is
inserted.

Reference receipt — `LOOP-JOINSIG-MIXED-FALLTHROUGH-D0` (2026-08-08): the
bounded shared JoinSig contract now records one-sided terminal/fallthrough
branches without changing `LoopRecipeV1`. An omitted source `else` is a
logical `Fallthrough` arm, not a synthesized AST node. The terminal arm keeps
its own payload, while the normal arm continues with its own state; the loop
row receives the terminal `Break`/`Continue` edge and a normal `Backedge`.
Two normal arms must agree on binding/value state. This is a caller-zero
logical contract only: source observation, physical CFG/PHI, Builder/MIR,
selector, retry/fallback, production activation, and legacy deletion remain
closed.

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

- divergent normal-arm binding/value state;
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

This row claims only logical projection from an already sealed Recipe shape
into JoinSig branch-arm/edge relations. It does not claim fresh AST-to-Recipe
discovery, route activation, physical CFG/PHI parity, runtime execution, or
deletion of the located legacy Loop handoff. Source observation and Recipe
production remain the next S6B row; physical transfer and PHI obligations are
still closed. The required README and reference updates are landed in this
closeout.

## Callable operation-emitter preparation receipt (2026-08-07)

The caller-zero preparation slice now has bounded, test-only physical seams
for the next callable canary. The complete operation demand still remains a
move-only full-program product; no operation is extracted from it. A private
Prepared-product handoff moves its six capability parts without cloning
Completion, and the operation contract exposes a complete WriteBinding
projection alongside the existing ReadBinding projection.

The private leaf boundary now has typed bridges for `ConstI64`, `BinaryI64`
(`Add`/`Sub`), and `CompareI64` (`Less`/`LessEqual`/`Equal`). These emit only
through the existing Builder/type owners and retain a schedule-local
`LoopValueKey -> ValueId` transport; BindingSSA, CFG, PHI, Completion, and
DraftSeal remain owned by their existing sessions. A focused fixture proves
the pure Const -> Binary -> Compare chain and the full WriteBinding projection
proves source/effect/placement retention.

This receipt is not the full callable physicalizer. The Builder-free demand
now prepares the complete Recipe-order schedule, and a private dispatcher
joins prepared rows across all five operation families with an opaque typed
value ledger. The physical dispatcher now issues one exact
logical-to-physical target receipt per row, validates all target blocks before
the first leaf effect, and distinguishes semantic preflight from post-claim
physical failure. Exact Prelude materialization and the bounded
Tail-to-ValueId/Completion handoff are closed caller-zero seams. The fresh
function session remains the sole discard boundary; `finish_for_draft_seal`
and DraftSeal integration are later bounded work.
No production caller,
selector, retry/fallback, Generic G0 parity, module publication, or legacy
deletion is opened by this slice.

## Callable full-demand preflight receipt (2026-08-08)

`CALLABLE-LOOP-PRODUCTION-FULL-DEMAND-PREFLIGHT-S2` is closed as a Builder-free
full-demand boundary. `PreparedCallableLoopIngressV1::prepare_full_demand` is
the single callable entry: it consumes the source-plus-logical ingress once,
issues the existing neutral operation/effect demand, checks source/context,
input, Prelude, and Tail owner identity, and calls `prepare_all` for the
complete seven-row Recipe-order schedule. The resulting thin profile product
retains only source/input/Prelude/Tail transport; the common demand remains the
sole owner of Recipe/JoinSig, operation/effect coverage, semantic context, and
Loop continuation.

This receipt creates no `ValueId`, `BasicBlockId`, CFG/SSA/PHI state, function
session, ABI/Completion claim, selector, retry, fallback, publication, or
legacy behavior. There is no first/select/filter operation extraction API, and
the existing test wrapper remains `cfg(test)`; production exposes only the
one-shot adapter parts required by the assembler. The caller-zero full
physical canary was the next bounded row and is now closed by the receipt
below; fresh-session discard and reuse remain required before any production
switch.

## Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is closed as caller-zero evidence. The
test-only bridge starts from the exact resolved callable-module input and its
existing resolver ledger, then consumes the S2 full-demand product exactly
once. The complete seven-row Recipe-order schedule reaches the existing
Prelude/`ReadyLoopEntryV1`, canonical topology and block receipts, the common
Read/Const/Compare/Binary/Write dispatcher, a sealed Loop After continuation,
the distinct callable Tail/Completion handoff, `finish_for_draft_seal`, and
DraftSeal prepare/commit.

The failure witness pre-seeds the Recipe-derived Compare result key, emits
earlier rows, observes typed `ValueAlreadyPublished`, discards the complete
unpublished function, and reruns the same semantic fixture in a fresh
session. The bridge adds no resolver owner, AST clone, selector, retry,
fallback, Generic G0 parity, module publication, or legacy authority. G0 D0 is
now accepted: exact resolver input is paired with neutral S4 by a compiler-side
composite ingress. The Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` fifteen-row `prepare_all` receipt is now
closed. Top-down I1 review fixes the next physical contract: nested control
may split one logical block into pre-child and parent-resume segments. R1 is
now closed as the Builder-free
`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` receipt. The current follow-up is
segment-aware canonical block cutover R2. Recipe/JoinSig remain the sole
logical authority; the private layout records only mechanically derived order,
item placement, and nested After -> resume.

## Generic G0 parity D0/I0 boundary (2026-08-08)

The neutral `VerifiedGenericRecipeProductG0` intentionally does not retain the
compiler `ResolvedFunctionLoweringInputV1`. The accepted boundary is a thin
compiler-side composite that pairs the exact input/ledger/entry receipt with
that neutral product. It validates owner/origin/kind/site/frame/scope, forest,
two G0 input bindings, and `L0.After/b1` before Builder effect. Missing,
foreign, stale, duplicate, or inferred capability is typed `NoSafeSlice`; no
re-resolve, AST clone, or profile relabel is permitted.

`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` issued this composite and proved the full
fifteen-row common `prepare_all` without Builder effect. Physical root/child
emission, G0 After-to-tail read, Completion, DraftSeal, selector, retry/
fallback, and legacy deletion remain closed. R1, R2, and Callable R3-I0 are
now closed. The next common row is the D1 per-transfer-Predicate/carrier-seed
contract; only after it may G0 I1-R0 open.

## Generic G0 I1 D1 boundary (2026-08-08)

The worker-reviewed D1 design fixes two common, not Generic-specific,
contracts before the caller-zero G0 canary:

```text
Predicate transfer
  -> its own completed Bool ValueId receipt

DerivedCarrierEntry
  -> prepared carrier-seed operation
  -> canonical identity.read_entry_receipt
```

The neutral After receipt has no single condition key and no Callable
operation-count proof. G0 I1 remains a cfg(test) canary with two parameter
entries, five R1 segments plus root After, fifteen exact Recipe rows (item 3
carrier seed; item 4 structural nested Loop), distinct G0 `b1` Tail/Completion,
typed finish/DraftSeal, whole-session discard, and fresh-session rerun. No
production selection, M8/M9, M10b/M11/M12, fallback/retry, or broad legacy
retirement is claimed.

## Generic G0 exact-ingress I0 receipt (2026-08-08)

The bounded `cfg(test)` ingress now pairs the exact resolver-issued
`ResolvedFunctionLoweringInputV1` with neutral S4, validates the source/
owner/frame/forest/entry/tail relation, and consumes G0 After once into the
common continuation plus `VerifiedGenericG0TailCapabilityV1`. Common
`VerifiedLoopOperationPhysicalDemandV1::prepare_all` proves the complete
fifteen-item Recipe membership without Builder effect. Positive,
missing-input, foreign-input, and tail-separation tests are green, with
duplicate/missing evidence covered by the existing demand/producer suite.
No physical G0 emission, Completion/DraftSeal, selector, retry/fallback, or
legacy deletion is claimed.

## Recursive segment plan R1 receipt (2026-08-08)

`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` is implemented as a Builder-free
derived contract. Common `prepare_all` follows verified recursive Recipe
preorder; it does not flatten logical blocks or expose a first/select/filter
operation API. `PreparedLoopPhysicalLayoutV1` consumes the complete prepared
program and records exact operation placement plus nested child-entry and
parent-resume segments. Recipe/JoinSig remain the only logical authorities.

The closeout fixtures prove:

```text
Callable: seven operation rows in Recipe preorder
Generic G0: [0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]
Generic G0: 16 items / 15 operations / 5 derived segments
```

This receipt creates no physical IDs, CFG/SSA/PHI mutation, function session,
After writer, Tail/Completion path, selector, retry/fallback, or production
caller. Segment-aware canonical block cutover is the next R2 task; physical
G0 and legacy retirement remain closed.

## Segment block cutover R2 receipt (2026-08-08)

`LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is closed for the selected Callable
canary. The private `LoopPhysicalSegmentBlockReceiptV1` adapts the existing
canonical topology blocks to the exact R1 segment keys and rejects foreign,
duplicate, missing, or aliased segment placements. The segment receipt is
owner- and preheader-branded; it is not a second CFG/SSA/PHI owner.

The segment-aware dispatcher consumes the complete `PreparedLoopPhysicalLayoutV1`,
builds one item-to-segment index from that layout, and resolves every operation
target through the exact segment key. The selected canary no longer calls the
logical-block-only operation target path. Callable parity remains seven rows
(`Pure=4`, `Read=2`, `Write=1`), with typed owner/missing/duplicate negatives,
late-failure whole-session discard, and fresh-session reuse evidence.

This is a bounded adapter cutover only. The current topology adapter rejects
segment aliasing rather than sharing a block; it is not the R1 segment
allocator. Generic G0 physical emission, recursive After, Tail/Completion
changes, selector, retry/fallback retirement, collector/publication, and
legacy deletion remain closed. The R3-I0 implementation receipt below replaces
this adapter on the selected Callable path; the investigation records both
the correction and its closeout.

## Recursive After R3-I0 implementation receipt (2026-08-08; Decision: accepted)

The R2 receipt cannot be the input to a neutral recursive edge writer: it maps
R1 segments onto the old fixed `Header/Body/Step/After` topology, while R1's
verified transfers are segment-based. A new writer on that adapter would leave
the synthetic `Step` outside the transfer graph and would retain the old fixed
edge authority.

The corrected R3 boundary is now implemented for the selected Callable
caller-zero canary:

```text
PreparedLoopPhysicalLayoutV1 + ReadyLoopEntryV1
  -> one physical block per R1 segment + one root After (no Step)
  -> retain layout + segment receipt + completed operation receipts
  -> preflight entry and every R1 Jump/Predicate/OpenNestedLoop transfer
  -> emit through the existing canonical CFG/identity/PhiTxn owners
  -> one neutral ReadyLoopAfterContinuationV1
```

`PreparedLoopPhysicalLayoutV1` now exposes an explicit sealed entry segment;
position zero is not an authority. `segment_allocator` allocates exactly one
block per R1 segment plus one root After block and does not allocate a Step
block. `CompletedLoopSegmentProgramV1` retains the layout, entry, segment
receipt, completed operation receipts, and value ledger so the recursive After
stage does not reconstruct or re-resolve them. Predicate conditions come from
the completed operation receipt. The R3 closure preflights the entry edge and
every R1 Jump/Predicate/OpenNestedLoop transfer, emits each once through the
canonical CFG/identity/PhiTxn owners, seals the segment blocks and root After,
then returns one neutral `ReadyLoopAfterContinuationV1`. Callable keeps its
`Pure=4 + Read=2 + Write=1` check in a thin wrapper while Tail/Completion and
DraftSeal semantics remain unchanged.

The selected canary asserts exact segment coverage and a distinct root After,
and covers the late duplicate failure, whole unpublished-session discard, and
fresh-session rerun. The former fixed Callable close helper and
`from_callable_layout` adapter are removed from the selected path. G0 receives
no physical allocation or operation emission; production selection,
retry/fallback retirement, collector/publication changes, and broad legacy
deletion remain closed.

## Common Predicate/carrier I0 receipt (2026-08-08; Decision: accepted)

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` supersedes the earlier temporary
`CarrierSeedUnavailable` read-only boundary for the bounded common contract.
The ordinary expression `ReadBinding` projection still admits only expression
anchors. A `DerivedCarrierEntry` read is projected by the separate full-program
`PreparedLoopDerivedCarrierSeedRowV1` and is dispatched as a profile-neutral
`CarrierSeed` operation; no expression site is fabricated and no G0-specific
physicalizer or SSA owner is introduced. The leaf uses canonical identity's
`read_entry_receipt` and publishes the same immutable operation value receipt
as the other common producers.

The neutral `ReadyLoopAfterContinuationV1` now contains only common owner,
root-After, and predecessor facts. For every
`LoopPhysicalTransferV1::Predicate`, the recursive writer resolves the
transfer's own completed Bool value, verifies its owner, type, and physical
source segment, and emits that value for that edge. Callable's `7 = 4 + 2 + 1`
coverage and condition-key proof stay in the outer profile close. The focused
Callable suite is green (25/25), Generic demand identifies exactly one item-3
derived carrier row, and no G0 physical allocation, production selection,
fallback/retry retirement, publication, or legacy deletion is claimed.

This section is an implementation receipt only. The next row is
`LOOP-CALLER-ZERO-PARITY-G0-I1-R0`; the reference must be updated again after
that canary and again at production cutover.

## Generic G0 I1 implementation receipt (2026-08-08; Decision: accepted)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` closes the bounded common-physical canary
for Generic G0. The test-only ingress moves the exact resolver-backed input,
the complete fifteen-operation prepared program, the G0 tail capability, and
the numeric target into one fresh function session. The canary publishes the
receiver and explicit parameters through canonical identity (including the
non-static `me` receiver), allocates exactly five R1 segment blocks plus a
distinct root After, and dispatches all fifteen rows exactly once through the
common operation dispatcher.

The receipt proves one derived-carrier seed with canonical provisional typing,
two distinct Bool predicate values at distinct physical placements, recursive
After predecessor count one, an exact I64 G0 Tail/Completion contract, and the
existing `finish_for_draft_seal`/DraftSeal path. A late duplicate publication
fails after the physical emission boundary, discards the whole unpublished
function session, and a fresh session reproduces the same shape-neutral
receipt. ValueId/BasicBlockId allocation numbers are deliberately not part of
the parity receipt.

This is a caller-zero physical canary only. It does not claim M8/M9 coverage,
production selection, M10b/M11/M12 cutover, retry/fallback deletion, module
publication policy, backend parity, or broad legacy retirement. The S6A design
is now accepted, its common initialized-local input-set prerequisite is closed,
and the bounded resolver-backed source observer slice is landed. Generic
parameter inputs remain a separate contract. This reference must be updated
after each implementation and again after production cutover.

## M8 S6A design decision (2026-08-08)

Decision: accepted; `LOOP-INPUT-SOURCE-RELATION-SET-R0` is closed and the
resolver-backed S6A implementation row is now the next caller-zero step.

The inspected
`apps/tests/loop_simple_while_inline_explicit_step_min.hako` fixture has
`acc = acc + i` followed by `i = i + 1`; current `LoopSimpleWhileFacts` and
`DirectAccumFacts` both Decline it, and it is not Generic G0. Its fast-gate
`LoopSimpleWhile` label is migration evidence only. The accepted S6A boundary
uses private input/condition/update/step/coverage observations and exposes one
move-only atomic `VerifiedVariableAccumRecurrenceFactsV1`. Its deterministic
provenance-only producer maps into the existing algebra with:

```text
external input-source relations = 2
Recipe binding relations        = 2
Core binding-effect relations   = 8
item-source relations           = 11
carriers                        = 2
in-loop exits                   = 0
```

The condition normalizes as `Const -> Read -> Compare`; accumulator and step
normalize as `Read -> Read/Const -> Add -> Write`. Initializers remain external
input relations, while `print(acc)` and `return 0` remain an unclaimed callable
tail. `NoSafeSlice` is a development state, not a fifth source disposition.

The source-to-Facts boundary is one atomic product even when the compiler
projection uses private partial observations. Resolver-issued owner/site/frame
and `BindingRef` identity stay with the non-Clone capability until the Facts
aggregate is sealed; the producer does not re-read AST or reclassify the family.
Source anchors are exact: variable references for reads, literal expressions
for constants, whole binary expressions for Compare/Add, assignment targets for
writes, Loop statement plus carrier key for carrier entries, and declaration
plus initializer expression for inputs. The four relation families are
cardinality checks over one source-role map: 2 input, 2 binding, 8 Core-effect,
and 11 item-source relations. Producer success is one terminal move-only
source-bound aggregate; partial input/Core/operation products are not published,
and any duplicate, foreign, missing, or inconsistent row fails with published
product count zero. Disposition precedence is Rejected for identity conflict,
Unresolved for unavailable evidence, Declined for a fully observed non-family
shape, and Candidate only for the exact complete shape.

The bounded ingress now also proves the exact program-owned `Main.main`
resolver path and the typed C/D/U/R envelope: Candidate for the complete
fixture, Declined for a fully observed non-`Less` condition, Unresolved for
incomplete coverage, and Rejected for a foreign owner, duplicate binding role,
or incoherent source site. Source identity remains a resolver-owned rejection;
`NoSafeSlice` remains a development status only.

R0 first replaces callable's singular initialized-local input relation with
one common move-only exact-coverage initialized-local set without changing
accepted source shapes. The S6A resolver-backed observer and provenance-only
`VariableAccumRecurrenceV1` producer are now landed as a caller-zero
implementation slice. The S6A identity/coherence negative matrix is closed;
the Facts source-site validator remains below 800 lines. No route relabel,
DirectAccum widening,
new Recipe kind, selector, physicalizer, Builder/MIR owner, M9 parity,
production selection, retry/fallback retirement, or legacy deletion is open.
The R0 and S6A implementation commits must each update this reference and the
module README; M10b cutover requires a final reference update.

## M8 S6B variable-accum break/fallthrough implementation receipt (2026-08-08)

Decision: accepted; `JOINIR-LOOP-M8-LOOPV0-EXITS-JOINS-S6B` is closed as a
caller-zero source-to-Recipe slice.

The resolver-backed observer consumes the natural
`apps/tests/loop_break_plan_subset_min.hako` source shape and seals one atomic
`VerifiedVariableAccumBreakFactsV1`. It keeps condition, branch, terminal
update, normal update, induction step, two initializer relations, exact
resolver sites, and complete coverage together. It does not mint Recipe keys,
JoinSig edges, physical IDs, or re-read syntax in the producer.

The deterministic producer uses the existing recursive algebra and the shared
mixed-fallthrough JoinSig contract:

```text
bindings                         = 2
external input relations        = 2
logical blocks                   = 3
Recipe item rows                 = 20
  operation rows                 = 18
  control rows                   = If(1) + Break(1)
values                           = 17
carriers                         = 2
break exits                      = 1
Core binding-effect relations   = 10
item-source operation rows      = 18
```

The branch row is `If(I6)`: its terminal arm is `Exit(I11)` with the visible
terminal payload, while the omitted source `else` is the independent logical
`Fallthrough` arm. The normal body then updates `sum` and `i` before the
existing logical backedge. The control statement anchors are retained in
`VariableAccumBreakControlSourceReceiptV1`; they are not incorrectly counted
as operation-source evidence. The producer receipt is
`variable_accum_break_v1` and is not a route selector or physical dispatch
identity.

The focused six-test matrix proves the exact candidate and normalized
Recipe/JoinSig/Core product, deterministic counts and arm preservation,
incomplete evidence as `Unresolved`, fully observed unsupported/explicit-else
shapes as `Declined`, and foreign owner as `Rejected`. No AST rewrite, name
lookup, Builder/MIR/CFG/PHI effect, completion/tail claim, selector,
retry/fallback, production caller, or legacy deletion is opened. This
reference and both module READMEs must be updated again for any later physical
or production activation.

## Initialized-local input relation set R0 implementation receipt (2026-08-08)

R0 is closed as a behavior-preserving common-contract migration. The former
callable-local `VerifiedLoopInputRelationV1` was removed. Its initialized-local
row is now issued as `LoopInitializedLocalInputSourceRelationV1` and sealed by
`VerifiedLoopInitializedLocalInputSourceSetV1` in the neutral
`loop_recipe_contract::input_source` module. Callable remains exactly one row;
Generic parameter inputs are not folded into this set.

The set is Builder-free and move-only. It checks complete Recipe input
coverage, owner, value class, unique carrier, Core binding relation, and the
exact `SourceBindingSiteV1::Local` declaration (whose sealed statement site is
the sole statement authority). The Prelude materializer consumes the complete
set and uses the sealed declaration site directly; it no longer reconstructs a
local ordinal. Focused callable
Recipe, operation/effect, prepared-ingress, Prelude, and physical-canary tests
remain green. No S6A observer, new producer, Recipe kind, selector, physical
route, or production caller was opened by this row.
