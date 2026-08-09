---
Status: S0 closed; P0 accepted and ready
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
source-backed dynamic receipt authorizes every incoming carrier relation.
Mixed exact/dynamic, missing origin, foreign owner, or partially covered
incoming rows reject. No fact refinement from raw Unknown is added.

## Backend boundary

The MIR interpreter is the first supported consumer because it already
executes dynamic values. Backends that require a concrete carrier type must
reject before Loop block allocation or module publication with a stable
capability error. They must not erase the receipt, choose Integer, retry a
legacy Loop route, or defer failure until code generation.

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

### L0 — GenericLoop/PHI canary

1. Extend the prepared carrier representation with the explicit Dynamic arm.
2. Authorize Unknown-wire PHI only from complete dynamic carrier coverage.
3. Prove exact and dynamic lanes remain distinct and mixed inputs reject.
4. Run the unmodified `skip_while/4` canary on the MIR interpreter.
5. Verify unsupported backends fail before physical Loop effects.

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
  unsupported backend rejects pre-effect
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
