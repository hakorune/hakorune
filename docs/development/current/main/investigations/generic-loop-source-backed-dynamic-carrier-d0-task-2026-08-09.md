---
Status: S0/P0 and L0 R0/S0/P0 closed; bounded Dynamic operation/rebind P1 canary closed; PHI temporal-order D0 accepted; Enter handoff R0 next
Date: 2026-08-09
Row: `GENERIC-LOOP-SOURCE-BACKED-DYNAMIC-CARRIER-D0`
Blocks: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`
Mode: BoxShape / dynamic representation authority
---

# GENERIC-LOOP-SOURCE-BACKED-DYNAMIC-CARRIER-D0

## Decision

The current parser R1 source is valid. The first executable failure is:

```text
function = ParserScanLoopBox.skip_while/4
source   = local i = pos; loop(i < end) { ... i = i + 1 }
failure  = GenericLoop carrier i has Unknown representation
```

The compiler must accept a source-backed dynamic carrier. It must not repair
the source with an `i64` annotation and must not reinterpret missing type
evidence as Integer.

The durable distinction is:

```text
VerifiedDynamic
  = source deliberately declares an untyped/dynamic value and the exact
    parameter/local/carrier relation is sealed

Unknown
  = representation evidence is missing or was lost
```

Only the first may authorize a dynamic-wire Loop carrier and PHI. Raw
`MirType::Unknown`, `None`, or a failed lookup remains a typed failure.

## Why entry-I64 promotion is rejected

The existing callable result row for `skip_while/4` is:

```text
ExactI64 { required_i64_arguments = [1] }
```

This means that ordinal 1 must be exact I64 when a caller consumes the result
as exact I64. It does not declare a universal callee-entry type. The
counterexample is:

```text
id(x) { return x }
```

Its conditional result proof may require argument 0, but `x` is not thereby a
declared Integer parameter. `skip_while` also uses ordinal 2 (`end`) in the
comparison, while the result proof contains only ordinal 1. Result demand is
therefore neither the correct owner nor complete body-entry evidence.

Do not create a callable-entry Integer contract from this row. The existing
`ParameterEntryContract` remains valid for source-declared exact numeric
contracts; it is not a license to fabricate `declared_type_name = i64` for an
untyped parameter.

## Source authority

```text
exact untyped ParamDecl
+ resolver-issued formal BindingRef
+ exact local initializer relation (formal pos -> local i)
+ exact Loop carrier membership
+ verified body rebind relation
        -> private parameter/local/carrier observations
        -> VerifiedSourceBackedDynamicCallableV1
```

The canonical semantic issuer is:

```text
SourceBackedDynamicCallableIssuerV1
```

It lives in the source-only `normal_callable_dynamic_source.rs` sibling of the
existing normal-callable semantic seal, not under GenericLoop or a physical
Builder service. It is called only while one exact function root, its
`VerifiedSourceProjectionV1`, and the matching resolver forest/ledger are
simultaneously borrowed. This is the only point where the source header still
proves that a parameter is untyped, the projection proves the exact AST owner,
and the resolver forest proves each `BindingRefV1` and Loop membership.

The issuer returns one non-`Clone`, AST-free aggregate:

```text
VerifiedSourceBackedDynamicCallableV1
  callable owner
  complete untyped formal rows
    parameter ordinal + exact formal BindingRef
  exact local-initialization rows
    dynamic formal -> local BindingRef
    declaration + initializer + lexical-ref sites
  exact Loop-carrier rows
    local BindingRef -> exact Loop source/frame/scope-region
    condition read + body rebind source relations
```

The formal catalog is complete for the selected callable, not just for
parameters that later become Loop carriers. This is required because
`skip_while/4` also reads untyped `end` directly in the comparison. Private
parameter/local/carrier DTOs may exist inside the issuer, but callers receive
only the aggregate and cannot freely re-pair them.

Names and unbranded raw ordinals are diagnostic only. The source-branded
parameter ordinal remains part of the exact declaration coordinate. The
issuer consumes the exact callable syntax view and existing resolved callable
source ledger and produces an AST-free product.
It does not read Builder state, `ValueId`, `MirType`, result requirements, a
method name, or a route label as semantic authority.

For the first fixture the dynamic authority is needed for `pos -> i`. `end`
remains a separate dynamic operand at the comparison boundary; the design
must not silently classify it as exact I64. The first physical canary may
close only when the existing dynamic comparison/call emission path can consume
that operand without inventing an exact representation.

## Physical representation

Do not immediately add `MirType::Dynamic` as a second runtime type algebra.
The VM already executes values on the dynamic lane and existing MIR surfaces
carry `MirType::Unknown`. The first design should keep that wire encoding but
require a distinct authorization receipt:

```text
PreparedLoopCarrierRepresentationV1
  Exact(MirType)
  Dynamic(source-backed callable + exact carrier row,
          wire = MirType::Unknown)
```

This keeps wire compatibility while preventing:

```text
missing type fact
  -> raw Unknown
  -> accidental dynamic acceptance
```

The sole `setup_function_params` owner continues to allocate/publish formal
`ValueId`s. It must additionally publish or hand off the verified dynamic
origin for an untyped parameter; local Copy propagation must preserve that
origin to the exact final carrier value. GenericLoop only verifies and
consumes the prepared representation. It does not infer dynamic meaning.

PHI materialization must accept Unknown wire type only when the same
source-backed dynamic lineage authorizes every incoming carrier relation.
The entry origin alone is insufficient: an exact Dynamic operation-result
receipt must authorize the body result before an atomic rebind can become the
backedge input. Mixed exact/dynamic, missing lineage, foreign owner, or
partially covered incoming rows reject. No fact refinement from raw Unknown
is added.

## Backend boundary

The MIR interpreter is the first supported consumer because it already
executes dynamic values. The compiler and `NormalCompileRequestV1` are
currently backend-neutral; the runner selects a backend only after MIR module
construction. Therefore two distinct fail-fast points are normative:

```text
Dynamic semantic/PHI preflight failure
  -> before Loop block allocation

unsupported selected backend
  -> completed MirModule shared backend preflight
  -> before backend serialization, lowering, execution, or fallback
```

Do not thread a backend string, environment lookup, or runner policy into the
Builder merely to move the second check earlier. The Dynamic Loop capability
is installed as passive function/module metadata and registered in the shared
`mir::backend_capability` gate. Unsupported backends return one stable error
with `silent_fallback_allowed=false`.

If a future product requires target-specific rejection before MIR
construction, it needs a separate target-aware compile-request Decision. It
is not part of L0.

## Reachability is not this repair

`VerifiedSelectedNormalCallableSourceInventoryV1` contains every selected
non-Main method; it is not a reachability proof. The whole-source static call
inventory does not seal external, runtime/provider, bare-function, or every
opaque source ingress, and it records bounded observation unavailability.
Moreover, the VM can invoke a module function by symbol.

Therefore these shortcuts are forbidden:

```text
zero observed calls -> unreachable
skip imported method -> fixed
selected source inventory -> closed-world call graph
```

Reachability pruning would require its own artifact/root/visibility and
complete-ingress Decision. It is not part of this blocker.

## Task order

### S0 — source-backed dynamic parameter/local carrier

Row: `GENERIC-LOOP-DYNAMIC-SOURCE-S0`

Change:
  Add `SourceBackedDynamicCallableIssuerV1` at the normal callable source
  co-seal. From one exact syntax/projection/forest relation, issue one
  non-`Clone`, AST-free
  `VerifiedSourceBackedDynamicCallableV1` containing complete untyped-formal
  coverage and exact formal-to-local-to-Loop-carrier relations. Old authority:
  none.

Contract:
  `ParamDecl::declared_type_name == None` is source syntax evidence only while
  the matching resolver ledger supplies every `BindingRef`, initializer read,
  Loop membership, condition read, and body rebind. The aggregate contains no
  `MirType`, `ValueId`, route label, method-name policy, result requirement, or
  Builder state. Typed formals are never relabeled Dynamic; unrelated body
  reads are allowed and do not weaken exact carrier membership.

Done:
  The unmodified `skip_while/4` source issues dynamic formal rows for `pos`
  and `end`, plus the exact `pos -> i -> Loop carrier` row, with zero Builder
  effect. Focused negatives reject foreign owner/site, typed-formal relabel,
  missing or duplicate initializer/rebind relations, arbitrary construction,
  and raw-`Unknown` issuance. Update `src/mir/resolved_semantics/README.md` and
  `docs/reference/mir/generic-loop-stage-matrix.md` in the implementation
  commit.

Stop:
  Return to design if the row needs a post-resolver AST rescan, a Builder
  lookup, source-name matching, a body-cardinality assumption, or any inferred
  exact numeric type. Do not open entry propagation or GenericLoop in S0.

Closeout:
  Closed in the S0 implementation slice. Six focused tests prove complete
  untyped-formal coverage, exact `pos -> i -> Loop` membership, typed-formal
  non-promotion, multi-Loop nearest-membership separation, foreign
  projection/forest rejection, and names-only compatibility rejection. The
  prerequisite body-shape correction records a binding assignment target as
  an lvalue shape rather than fabricating a lexical read. The aggregate remains
  disconnected from entry publication and GenericLoop.

### P0 — function-entry and local propagation

Row: `GENERIC-LOOP-DYNAMIC-ORIGIN-P0-I0`

Decision:
  Connect the S0 product only to the existing entry snapshot and local
  completion terminal. `setup_function_params` remains unchanged and remains
  the sole formal `ValueId`/type publisher.

Canonical flow:

```text
VerifiedNormalCallableSourceIngressReceiptV1
  -> ResolvedFunctionLoweringInputV1
  -> SourceBackedDynamicCallableIssuerV1
  -> CallableDynamicOriginLoweringStateV1

setup_function_params
  -> CallableEntryShapeV1::prepare_values
  -> PreparedCallableEntryValuesV1
  -> exact dynamic formal origin installation

existing local terminal
  -> CompletedLocalBindingV1 {
       ordinal,
       initializer,
       local,
     }
  -> exact formal-to-local origin propagation
```

Ownership:

- `VerifiedNormalCallableSemanticLoanV1::into_parts` returns lineage plus the
  exact source ingress; it no longer creates a temporarily origin-less
  lowering state.
- `NormalCallableSemanticLoanPortV1::with_callable_source_scope` constructs
  `CallableSemanticLoweringState` once from that ingress and the non-`Clone`
  S0 product.
- private `CallableDynamicOriginLoweringStateV1` owns the source product,
  formal/local indexes, consumption state, and physical `ValueId` to exact
  source-formal origin relation.
- `CompletedLocalStatementV1` carries ordinal-keyed initializer/local pairs;
  it does not require an instruction scan or a second Copy owner.
- rebind removes the prior current-value authorization. Dynamic operation and
  PHI continuation remain L0 responsibilities.

Fail-fast:

- source owner, input/forest, formal ordinal/cardinality, or entry coverage
  mismatch rejects before body effects;
- wrong initializer/local `ValueId`, ordinal drift, duplicate local completion,
  or missing completion poisons the unpublished function and uses the existing
  whole-session discard; there is no repair or retry.

Non-authority:

```text
MirType::Unknown
TypeContext
variable_map / names
result demand
post-emission instruction scan
GenericLoop / PHI
```

Done:

1. untyped `pos` maps to its already-published formal `ValueId`, then the exact
   `local i = pos` Copy receipt preserves the same origin;
2. all untyped formals, including `end`, receive entry origins while typed
   formals on an Unknown wire receive none;
3. unrelated locals and Copies cannot acquire Dynamic origin;
4. foreign owner, missing/duplicate entry, arity/ordinal mismatch, wrong
   initializer/local `ValueId`, duplicate/missing local completion, and stale
   rebind origin all reject;
5. the production callable source scope consumes the exact ingress, all
   focused tests and structural guards are green, and every source file stays
   below 800 lines;
6. Builder/stmts README, this card, the MIR reference receipt, and current
   pointers are updated in the implementation commit.

Stop:
  Return to design if implementation needs Builder-global origin storage,
  `TypeContext` mutation, `MirType::Unknown` inference, instruction scanning,
  name matching, a second parameter publisher, or any GenericLoop/PHI change.
  L0 remains a separate BoxCount row.

Closeout:
  Closed in the P0 implementation slice. The normal callable scope now builds
  its lowering state atomically from the exact source ingress, installs all
  source-backed dynamic formal origins from the existing entry receipt, and
  propagates an origin only through an ordinal- and ValueId-exact local
  completion pair. Rebind invalidates current authorization. Seven origin,
  six source, twelve callable-semantic, twenty-two local-statement, nine
  body-shape, and seven Query-body focused tests are green. The regression
  pass fixed the compiler's neutral static-current-owner `me` representation;
  it also made the callable completion ledger consume the co-located read
  receipt of a compound-rebind target. No source workaround or new fallback
  was added.

### L0 — GenericLoop/PHI canary

Status: `accepted`; implementation is split below

#### Decision

The compiler acceptance is too narrow and must be repaired. The current
`VerifiedCallableSemanticLoopBindingScheduleV1` fixes total role counts to:

```text
condition reads = 1
body reads      = 1
body rebinds    = 1
```

That is a fixture shape, not a language or GenericLoop contract. The exact
`skip_while/4` source contains a redefined carrier plus read-only Dynamic
operands. Raising the constants would only create another narrow shape.

The durable source classification is:

```text
carrier:
  exact entry seed
  exact Loop-local rebind
  backedge payload / PHI required

read-only operand:
  exact source reads inside the Loop
  no rebind by this Loop
  no carrier PHI

iteration-local result:
  exact operation result inside the Loop
  no entry seed
  no header carrier PHI
```

All resolver-issued reads and rebinds in the supported Loop source window are
exact-source-site-keyed and completely covered. The later operation-source row
is the sole owner of the source-site-to-operation-role relation. Total read
count is not semantic authority. The first cohort may still support one
carrier and one canonical rebind topology, but it must say so with typed
relations rather than counts.

#### Three separate owners

```text
1. Dynamic carrier ingress                     // before Builder effect
   complete source coverage
   + exact carrier relation
   + P0 current origin
      -> PreparedSourceBackedDynamicLoopIngressV1

2. Dynamic operation result / atomic rebind    // body emission terminal
   current Dynamic operand
   + exact operation-source relation
   + emitted result ValueId
      -> CurrentDynamicBindingReceiptV1

3. Dynamic PHI authorization                   // CFG-ready, before PHI emit
   JoinSig/expected predecessor roles
   + actual VerifiedPredecessors
   + every incoming Dynamic lineage
      -> PreparedDynamicPhiAuthorizationV1
```

The first owner must reject before `LoopBlocksStandard5::allocate`. The third
owner necessarily runs after blocks/body values exist; its failure poisons
the unpublished function and uses whole-session discard. It must not be
misdescribed as a zero-Builder-effect rejection.

The PHI writer remains the canonical Binding SSA / `PhiTxn` path. L0 adds an
authorization receipt, not a second PHI writer. Pending-PHI cleanup remains
best-effort diagnostic hygiene; whole unpublished session discard owns
atomicity.

#### Representation boundary

```text
PreparedGenericLoopCarrierRepresentationV1
  Exact {
    init,
    exact_type,
  }
  Dynamic {
    init,
    authorization,
    wire = MirType::Unknown,
  }
```

Only the Dynamic arm can project to an Unknown wire. `MirType::Unknown` can
never construct that arm. Exact and Dynamic inputs are mutually exclusive;
dual evidence, mixed PHI inputs, raw Unknown, missing origin, foreign origin,
and stale current values reject.

The prepared representation is passed into the skeleton. The skeleton must
not rediscover it from a variable name, `variable_map`, or `TypeContext`.
Additional route-local carriers currently defaulted with
`unwrap_or(MirType::Unknown)` are not admitted by this row.

#### Operation and Recipe boundary

P0 invalidates an origin on rebind but does not yet issue the result of
`i = i + 1`. L0 must add a profile-neutral source-backed Dynamic operation
contract. For the first cohort:

```text
Dynamic operand + exact literal Add
  -> Dynamic result with the same lineage

Dynamic operand + Dynamic read-only bound Compare
  -> exact Bool result
```

This is not `BinaryI64`/`CompareI64` inference. `LoopRecipeV1` currently has
only I64/Bool/Unit value classes and I64 arithmetic/comparison operations.
The bounded legacy GenericLoop canary may borrow the neutral Dynamic operation
receipt, but it may not become its semantic owner. Production selection stays
closed until a later Recipe vocabulary Decision represents Dynamic
operations honestly and the legacy adapter has a same-commit retirement
edge.

#### Retry boundary

The current release Generic handler converts some lowering errors into
`PostEffectRetryDebt` and continues the route schedule. A selected
source-backed Dynamic failure must be a typed terminal rejection. It may not
be converted to retry debt, run a suffix route, use the LLVM mock fallback,
or reuse the poisoned function session.

#### Ordered implementation tasks

##### L0-R0 — `CALLABLE-LOOP-SOURCE-COVERAGE-R0`

Change:
  Replace the fixed `(1,1,1)` schedule and count-only consumed receipt with
  one exact-source-site-keyed complete relation set. Co-seal the one selected carrier,
  its exact reads/rebind, read-only operands, and iteration-local bindings. No
  Builder/MIR effect.

Done:
  The unmodified `skip_while/4` source is accepted with `i` as the only
  carrier and additional exact reads retained as operands. Foreign,
  duplicate, missing, unconsumed, cross-Loop, and cross-binding rows reject.
  Existing simple one-carrier fixtures remain green. Update the owning Builder
  README and this reference receipt in the same implementation commit.

Stop:
  Do not widen by total counts, names, source-position guesses, or a default
  catch-all row. Do not open representation, operation emission, PHI, route
  selection, or fallback.

Closeout:
  Closed in the R0 implementation slice. The count-only receipt is replaced
  by grouped exact relation rows, and the actual production `skip_while/4`
  source is covered without annotation or copying. `i` is the sole carrier;
  `end/src/pred_chars` are read-only operands and `ch` is iteration-local.
  Variable read cardinality is no longer policy. Focused handoff, semantic
  source, raw child-entry, Dynamic source, and Dynamic origin tests are green;
  the owner file remains below 800 lines. No Builder/MIR effect or later L0
  authority was opened.

##### L0-S0 — `DYNAMIC-LOOP-OPERATION-SOURCE-S0`

Change:
  Issue one source-only neutral Dynamic carrier-lineage operation relation set
  for the exact Add/rebind and comparison rows needed by the bounded source
  window.

Done:
  Every carrier-lineage operand/result/source role is covered exactly once;
  Dynamic Add and exact Bool Compare semantics are explicit. Existing method
  calls, local `ch`, early Return, and Tail remain separately owned and are
  not reissued by this product. No `MirType`, ValueId, Builder, Recipe-I64
  relabel, or method-name inference. Update the resolved-semantics README and
  MIR reference in the same commit.

Stop:
  Missing call/method result authority is `NoSafeSlice`; do not infer it from
  raw Unknown or emitted opcodes.

Closeout:
  Closed in the S0 implementation slice. A dedicated source-only sibling
  module co-seals the exact production `skip_while/4` condition
  `i < end` as Dynamic operands with an exact Bool result, and the exact
  `i = i + 1` update as current Dynamic carrier plus exact I64 literal with a
  Dynamic result and same-binding rebind. The issuer consumes existing
  resolver/dynamic/schedule authority and preserves exact source sites; it
  contains no `MirType`, `ValueId`, Builder mutation, Recipe relabel, name
  inference, or fallback. Typed `end`, subtraction, and reversed Add reject.
  Existing method calls, `ch`, early Return, and Tail remain outside this
  product.

##### L0-P0 — `DYNAMIC-LOOP-PREPARE-P0`

Change:
  Co-seal the complete source coverage, P0 current origins, carrier and
  read-only operand rows, operation relations, and expected Enter/Backedge
  roles into one non-`Clone` prepared program before Loop allocation. Add the
  closed Exact/Dynamic carrier representation enum.

Done:
  missing/dual/mixed/foreign/stale evidence rejects with block, ValueId, and
  instruction snapshots unchanged. Raw Unknown remains rejected. The skeleton
  accepts only the prepared representation. Update GenericLoop README and the
  MIR reference in the same commit.

Stop:
  No Builder-global origin map, backend string, name lookup, TypeContext
  repair, PHI emission, retry, or fallback.

Closeout:
  Closed in the P0 implementation slice. One dedicated pre-effect issuer
  consumes the R0 schedule and S0 operation set while borrowing the existing
  current-origin owner. It retains all exact source rows, prepares the sole
  carrier plus every read-only entry operand, skips iteration locals until
  their own materialization owner, and seals the carrier's Enter/Backedge
  expectations. The externally visible representation receipt is opaque; its
  private closed family contains Exact and source-backed Dynamic only, with
  no raw Unknown arm. The actual `skip_while/4` fixture prepares four entry
  bindings, and a stale carrier rejects with block and value-allocation
  snapshots unchanged. The issuer accepts no Builder/CFG handle and opens no
  operation emission, PHI, backend, retry, fallback, or route activation.

##### L0-P1 — `DYNAMIC-LOOP-REBIND-P1`

Change:
  First repair the common compiler rule that currently promotes
  `Unknown + Integer` to `Integer`: an unknown physical fact is not proof of
  an exact integer result, either at ordinary BinOp completion or during
  function type re-propagation. Keep the production source unchanged.

  Then consume the whole prepared ingress in one private operation-execution
  state. Emit the authorized Add through the existing
  `MirBuilder::emit_instruction_at` writer and the comparison through the
  existing `compare::emit_to_at` writer. Bind the exact emitted Add result to
  the prepared Dynamic relation and atomically replace both current-value
  projections. A late failure discards the whole unpublished session.

Done:
  `Unknown + Integer` remains unknown without source authority; exact
  `Integer + Integer` remains Integer. No invalidate/register gap exists;
  unrelated values and stale targets cannot acquire the lineage. Compare
  publishes exact Bool only. The completed move-only handoff retains Enter,
  Backedge, exact assignment source, definition block, one lineage, and
  expected roles for P2. Update the operation owner README and MIR reference
  in the same commit.

Stop:
  No source rewrite, narrower fixture, raw-Unknown admission, second operation
  emitter, same-session repair, or PHI claim. Do not pass a raw predecessor
  vector, Phi token, or caller-constructed PHI destination to P2.

Authority correction:
  When a valid production source is outside a legacy inference shortcut, fix
  the compiler authority that made the shortcut too narrow. Do not annotate,
  copy, simplify, or otherwise rewrite the `.hako` source to fit it. Here the
  narrow owner is the common `Unknown + Integer -> Integer` inference, not the
  `skip_while/4` fixture.

P1 output (bounded canary; not a canonical Loop-current handoff):

```text
PreparedSourceBackedDynamicLoopIngressV1
  -> private exact-once operation execution
  -> ReadySourceBackedDynamicLoopCarrierForPhiV1
       owner / exact Loop site
       carrier BindingRef / one Dynamic origin
       Enter ValueId
       Backedge Add-result ValueId
       exact assignment source site
       Add definition block
       expected [Enter, Backedge]
```

The product owns no `PhiToken`, PHI destination, raw incoming vector,
`MirType` inference, Builder handle, or SSA handle. A later audit proved that
it cannot be consumed as a post-hoc canonical PHI input program: Compare and
Add were already emitted from Enter. It remains useful as bounded evidence
for exact operation/source emission and atomic Dynamic rebind only.

Closeout:
  Closed in the P1 implementation slice. Ordinary Add completion and final
  BinOp re-propagation no longer promote `Unknown + Integer` to `Integer`;
  exact Integer pairs retain their old result. One private exact-once terminal
  consumes the whole P0 ingress and delegates comparison, constant, and Add
  insertion to existing writers. Its prepare/commit boundary updates callable
  current value and Dynamic lineage together, and stale, foreign, reused, or
  duplicate values reject without mutation. The production `skip_while/4`
  source remains unchanged. The comparison publishes exact Bool; the Add wire
  stays physically untyped while the source-backed relation authorizes its
  Dynamic lineage. The move-only P2 handoff contains only exact Enter,
  Backedge, definition/source identity, lineage, and expected roles. These
  roles are descriptive canary evidence, not canonical reaching-value or CFG
  authorization. A
  post-emission injected failure discards the complete unpublished function
  session. No PHI, second emitter, retry/fallback, or production route opened.

##### L0 PHI temporal-order correction

Row: `DYNAMIC-LOOP-PHI-ORDER-D0`

Decision: `accepted correction`; the former post-P1-only
`DYNAMIC-LOOP-PHI-P2` row is rejected.

The landed P1 emits both Compare and Add from `carrier.entry()`. Opening a PHI
afterward would create an unused value and would keep every iteration on the
entry value. Advancing callable-current directly to the Add result also does
not represent the zero-iteration path or the value reaching Loop After.

The only accepted temporal order is:

```text
same fresh CanonicalSsaFunctionSessionV2
  -> exact physical Loop roles created by the canonical CFG owner
  -> Enter definition already present in canonical Binding SSA
  -> unsealed Header read
       -> provisional PHI / canonical Header current
  -> Compare and Add consume that Header current
  -> exact source assignment defines the Add result in its definition block
  -> actual terminal Backedge path reaches Header
  -> CanonicalCfgSessionV1 seals Header
       -> VerifiedPredecessorsV1
  -> ResolvedSsaIdentityStateV2 seals Header
       -> existing BindingSsaBuilderV1 / PhiTxn patch the provisional PHI
  -> After reads the canonical merged reaching value
```

`CanonicalSsaFunctionSessionV2` is the only mutable CFG/Binding SSA/PhiTxn
bundle. The Dynamic adapter may authorize one source-backed lineage and drive
the existing APIs in order; it does not allocate or patch a PHI itself.

Physical-role mapping for this pre-Recipe L0 canary is one private,
move-only, bounded placement receipt issued while the exact blocks are created
through the canonical CFG session. It relates owner/exact Loop site to Enter,
Header, body path, terminal Backedge predecessor, and After. It is placement
only and retires when the common Recipe physical-layout receipt owns this
caller. Actual predecessor truth remains solely
`CanonicalCfgSessionV1::seal_block` / `VerifiedPredecessorsV1`.

The Add definition block is not assumed to equal the terminal Backedge
predecessor. The existing MIR Binding SSA adapter must verify that each value
definition dominates its selected terminal predecessor. Block-ID order,
predecessor-vector position, terminator rescans, or name inference never assign
Enter/Backedge roles.

Owner table:

```text
source-backed Dynamic lineage and expected carrier roles:
  P0/S0 semantic products

physical role -> block placement:
  private bounded canonical-CFG placement receipt

actual Header predecessor set:
  CanonicalCfgSessionV1 / VerifiedPredecessorsV1

reaching definitions and assignment source claim:
  ResolvedSsaIdentityStateV2

provisional PHI decision:
  BindingSsaBuilderV1

physical PHI define/patch/best-effort pending rollback:
  MirBindingSsaAdapterV1 borrowing the session PhiTxn

failure atomicity:
  whole unpublished CanonicalFunctionLoweringSessionV1 discard
```

Forbidden:

```text
post-P1 PHI insertion
rewriting already-emitted operands
raw predecessor Vec / PhiToken / caller-chosen PHI destination
second BindingSsaBuilderV1 or PhiTxn
route-local PHI writer
entry or Add result as Loop After current
definition block == terminal Backedge assumption
same-session repair/retry/fallback
source annotation or narrower .hako fixture
```

The valid production source remains the fixture. If the legacy route cannot
accept this sequence, repair or replace the compiler owner; never rewrite the
source to fit the narrower route.

The audit also found that current P0 has discarded one required relation. Its
ingress retains only carrier `BindingRef`, entry `ValueId`, and Dynamic origin;
canonical declaration adoption additionally requires the exact local
declaration/materialization relation. P2A therefore remains `NoSafeSlice`
until the following compiler handoff row lands.

##### L0-R0 — `DYNAMIC-LOOP-ENTER-HANDOFF-R0`

Change:
  Retain one private exact relation from the already completed local terminal:

```text
owner
+ exact local SourceBindingSiteV1
+ exact local BindingRefV1
+ CompletedLocalBindingV1 {
     initializer,
     local entry ValueId,
   }
+ source-backed Dynamic formal origin
+ exact Loop carrier membership
  -> PreparedDynamicLoopEnterDefinitionV1
```

  The relation is co-sealed while the completed local receipt, resolved
  function ledger, Dynamic origin state, and Loop schedule are simultaneously
  available. The prepared Loop ingress retains it instead of reducing it to a
  raw `(BindingRef, ValueId)` pair.

Done:
  The unmodified `skip_while/4` source produces exactly one carrier Enter row
  for local `i`. Its declaration site resolves to the same BindingRef, the
  completed local ValueId is the retained entry, its initializer is the exact
  source-backed Dynamic formal value, and the origin/Loop membership all share
  one owner. Foreign, missing, duplicate, stale, non-local, mismatched
  initializer, or unrelated completed-local evidence rejects before Builder
  mutation.

  No new ValueId or declaration is published. P2A later consumes the row and
  calls the existing canonical identity declaration publisher with the
  resolved record's exact kind/name. A private exact-binding helper may hide
  those record fields, but a raw `adopt(BindingRef, ValueId)` API is forbidden.

Stop:
  No block allocation, canonical declaration adoption, Header read, PHI,
  operation emission, callable-current update, backend, retry, fallback, or
  source rewrite. Do not use `MirType::Unknown` as Dynamic authority.

Tests:

```text
positive:
  exact declaration site / local completion / origin / Loop carrier co-seal

negative:
  foreign owner or declaration site
  declaration -> different BindingRef
  completed local -> different local ValueId or initializer
  stale Dynamic origin
  non-carrier local
  duplicate Enter row
  raw BindingRef + ValueId construction unavailable

effect:
  block / instruction / ValueId snapshots unchanged
```

Same-slice docs:
  Update `src/mir/builder/README.md` and
  `docs/reference/mir/generic-loop-stage-matrix.md`. Keep every source file
  below 800 lines.

##### L0-P2A — `DYNAMIC-LOOP-PHI-OPEN-P2A`

Change:
  Borrow the one canonical function session, co-seal the exact Enter/Header
  placement with the prepared ingress, confirm the existing Enter definition,
  and read the unsealed Header through
  `ResolvedSsaIdentityStateV2::read_entry_receipt`. Return one opaque,
  move-only Header-current product. Do not expose its PHI token or construct a
  second SSA owner.

Done:
  The provisional PHI exists before operation emission; its current belongs
  to the exact owner/Loop/binding/Header and the same Dynamic lineage. Foreign,
  stale, missing-entry, already-sealed Header, duplicate-open, or role-alias
  inputs reject. All failures are post-effect and require whole-session
  discard.

Stop:
  Do not emit Compare/Add, define Backedge, patch PHI inputs, or claim Loop
  After.

##### L0-P1R — `DYNAMIC-LOOP-HEADER-REBIND-P1R`

Change:
  Correct the private P1 terminal to consume the opaque canonical Header
  current. Compare and Add use that value as their left operand; the Add
  result becomes one source-backed Dynamic Backedge definition receipt.
  Canonical Binding SSA remains the reaching-value authority; the legacy
  callable-current projection is not final Loop-current truth.

Done:
  Focused MIR evidence proves both operation lhs values equal the provisional
  Header PHI destination and differ from Enter. The Add stays physically
  unknown while its exact source relation preserves one Dynamic lineage.

Stop:
  No operand rewrite after emission, raw-Unknown admission, After publication,
  or second current-value owner.

##### L0-P2B — `DYNAMIC-LOOP-PHI-CLOSE-P2B`

Change:
  Define the exact source assignment through canonical Binding SSA, finish the
  actual terminal Backedge path, seal Header through canonical CFG, verify the
  witness is exactly Enter plus Backedge, then seal the canonical identity so
  existing Binding SSA / `PhiTxn` patches the already-open PHI.

Done:
  The PHI inputs are exactly `(Enter, entry)` and `(Backedge, Add result)`;
  each value dominates its selected predecessor. Missing, duplicate, phantom,
  extra, foreign, role-aliased, or different-lineage evidence rejects. The
  first cohort has one Enter and one Backedge; Continue and multiple Backedges
  remain closed. `identity.finish`, `phis.commit`, and `cfg.finish` can succeed
  only after this close.

Stop:
  No route-local PHI writer, raw-Unknown admission, inferred Dynamic type,
  legacy variable-map synchronization, or production claim. Mixed
  Exact/Dynamic remains structurally unavailable until an exact-arm issuer
  exists; do not forge it solely for a negative test.

##### L0-P2C — `DYNAMIC-LOOP-PHI-DISCARD-P2C`

Change:
  Prove whole-session atomicity at the open, operation, definition, and
  post-patch failure points.

Done:
  Every failure consumes the unpublished function session and restores the
  caller exactly once. Pending PHI rollback is best-effort diagnostic hygiene,
  never the correctness owner. A fresh session can repeat the same semantic
  program; tests compare roles and instruction shape rather than numeric IDs.

Stop:
  No same-session retry, MIR repair, suffix route, or fallback.

Implementation order after this D0 closes:

```text
R0 exact Enter-definition handoff
-> P2A Header-current open
-> P1R operation correction
-> P2B predecessor close / PHI patch
-> P2C discard canary
-> L0-I0 bounded ownership switch or caller-zero-only status
```

The implementation slice must update
`src/mir/builder/resolved_lowering/canonical_cfg/README.md`,
`src/mir/builder/ssa/binding/README.md`, `src/mir/builder/README.md`, and
`docs/reference/mir/generic-loop-stage-matrix.md` with landed facts. All new
source files remain below 800 lines.

##### L0-I0 — `GENERIC-LOOP-DYNAMIC-VM-CANARY-I0`

Change:
  Wire the bounded legacy GenericLoop adapter to consume only the prepared
  neutral receipts. Install passive Dynamic-Loop backend capability metadata,
  register it in the shared backend gate, and add a small tracked Main wrapper
  that imports and executes the production `skip_while/4` unchanged.

Done:
  MIR interpreter succeeds. Unsupported backends reject at the shared module
  backend preflight before backend effects with a stable tag and no retry,
  suffix route, mock fallback, or compatibility fallback. Source files remain
  below 800 lines. Update GenericLoop README, backend capability reference,
  this card, and `docs/reference/mir/generic-loop-stage-matrix.md` in the same
  commit.

Stop:
  This is a bounded VM canary, not production selection or Dynamic Recipe
  parity. The production `skip_while` source must not be copied, annotated, or
  rewritten.

##### Post-L0 — `LOOP-RECIPE-DYNAMIC-SEMANTICS-D0/I0`

Before production activation, add an explicit Recipe Decision for Dynamic
value class, Add/Compare semantics, source-operation coverage, and JoinSig
carrier compatibility. Then switch one named caller and delete its legacy
carrier inference, route-local PHI, retry/fallback, and fixed-read schedule in
the same cutover series.

### R1 resume

Resume the parser expression-product fixture only after S0/P0/L0 are green.
Observe the next first failure; do not pre-open instance-call/Box-result work.

## Acceptance matrix

```text
positive:
  untyped formal pos -> local i -> Loop carrier has VerifiedDynamic origin
  local Copy preserves the exact origin
  dynamic-wire PHI is authorized by complete incoming coverage
  VM executes the existing dynamic comparison/update path

negative:
  id(x) result requirement does not issue dynamic carrier authority
  raw Unknown without source receipt rejects
  typed exact parameter is not relabeled Dynamic
  foreign formal/local/Loop BindingRef rejects
  missing or duplicate initializer relation rejects
  mixed exact/dynamic PHI inputs reject in the first cohort
  unsupported backend rejects at shared module preflight before backend effect
  selected Dynamic failure never becomes PostEffectRetryDebt
  consumed/missing selected handoff never falls back
```

## Nonclaims

```text
general dynamic type inference
general Unknown acceptance
exact-I64 parameter inference
parameter-entry numeric contract expansion
reachability/dead-method elimination
instance/provider/external ABI
dynamic Box representation redesign
all Loop profiles or all PHIs
source grammar/annotation changes
retry/fallback
```

## Stop condition

The D0 stop is closed. The exact source authority is the co-present function
root plus `VerifiedSourceProjectionV1` and matching resolver forest/ledger;
the canonical issuer is
`SourceBackedDynamicCallableIssuerV1`, and its only public semantic output is
`VerifiedSourceBackedDynamicCallableV1`. S0 has landed that disconnected
issuer with zero Builder effect. P0 is accepted: the existing positional entry
snapshot and an ordinal-keyed local completion receipt are the only physical
handoffs, and `CallableDynamicOriginLoweringStateV1` is the private projection
owner. If implementation cannot preserve the origin through those owners
without a second publisher or raw-`Unknown` inference, return to `NoSafeSlice`
rather than widening GenericLoop.
