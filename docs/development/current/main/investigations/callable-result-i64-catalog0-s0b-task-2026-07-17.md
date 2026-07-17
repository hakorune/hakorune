---
Status: closed
Date: 2026-07-17
Baseline: aedbef98417cbcc78cd912642c6a3ef56dadb764
Parent: callable-result-i64-catalog0-task-2026-07-17.md
Decision: Candidate B-prime, post-CUT0 clean rewrite
Scope: disconnected exact-site callable-result composition
---

# Callable result exact-i64 catalog S0b task

## Audited closeout task

Three independent post-implementation worker audits classify the remaining
work as a local closeout, not a new design consultation:

```text
DPRIME-CALLABLE-RESULT-I64-CATALOG0-S0B-CLOSEOUT-001
```

The implemented authority shape is accepted. The result catalog retains the
exact declaration and target catalogs by lifetime, target evidence borrows the
CUT0 row, Pending remains construction-only, and production producers and
consumers remain zero. No stop condition has fired.

Closeout order is fixed:

1. Replace the unreachable `rows_by_key.len() > static_count` exhaustion check
   with an explicit bounded-loop completion/stall result. A budget exhaustion
   must be the only producer of `ResultWorklistDidNotConverge`.
2. Remove the unused `require_target_catalog_brand` wrapper. The target
   catalog's read-only brand query remains the single identity decision.
3. Add exact-target direct and mutual recursion fixtures that close to
   `Unavailable(RecursiveDependency)` without SCC inference.
4. Add an equal-key foreign declaration/target catalog fixture that rejects
   with `SourceTargetCatalogBrandMismatch`.
5. Add a two-forwarding-wrapper chain and verify declaration reorder parity.
6. Add result-layer Core fixtures for `length` / `len` / `size`, unsupported
   receiver, wrong spelling/arity, and reachable String non-i64 results. Keep
   unreachable Dynamic/non-String domains parked rather than synthesizing a
   fake authority.
7. Add missing/Unknown required-argument substitution and heterogeneous
   untyped-return fixtures. Missing sealed arguments may be tested at the pure
   substitution boundary when source construction makes the state unreachable.
8. Rewrite the module README from the obsolete S0a contract to the S0b branded
   catalog, exact-site row, bounded String/Core composition, deterministic
   solver, bare-FunctionCall-unavailable, and production-zero contract.
9. Run the complete required gate list, record exact counts here, and only then
   close S0b and advance the pointer to
   `DPRIME-CALLABLE-RESULT-I64-CATALOG0-P0-001`.

Private malformed states such as duplicate call-site evidence and stable-final
drift need a fixture only when a narrow private draft seam already exists.
Do not add a production constructor merely to make those states injectable;
retain them as typed invariant errors plus structural guards otherwise.

The callable-result Python guard is already 765 lines. P0 must split or add a
separate P0 checker before extending it; exceeding or weakening the 800-line
limit is forbidden.

## Closeout evidence

S0b closed without opening any stop condition. The monotone solver now has an
explicit completion/stall state and a typed budget-exhaustion branch; the
redundant brand wrapper is removed. Exact-target direct and mutual recursion,
foreign catalog identity, two forwarding wrappers, Core aliases and negative
rows, required-argument missing/Unknown, and heterogeneous returns are fixed by
the disconnected suite. The module README now describes the S0b authority.

```text
callable-result focused tests = 23/23
source-target focused tests = 42/42
Core result-kind focused tests = 5/5
Core manifest malformed tests = 9/9
quick gate = 66/66
callable-result structural guard = green
Core manifest guard = green
cargo check = green
current-state pointer guard = green
git diff --check = green
modified source/check files >= 800 lines = 0
```

Production producers and consumers remain zero. P0 is next; it may normalize
the closed proof matrix but may not activate Builder/MIR/runtime publication.

## Selected next row

```text
R0-CALLABLE-RESULT-I64-CATALOG0-S0b
```

This is the sole next code-facing row. Three read-only worker audits and a
local source review agree that no external design consultation is required.
The CUT0 source-target authority is sufficient when S0b preserves its exact
declaration-catalog identity instead of joining equal-looking keys.

```text
production behavior delta = 0
production producers = 0
production consumers = 0
Builder / MIR / runtime / backend delta = 0
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

The saved stash remains evidence only:

```text
stash@{0} = wip/callable-result-s0b (blocked by source-call AST/site co-seal)
apply / pop / restore / wholesale copy = forbidden
```

## Durable composition

```text
exact declaration catalog
  + exact branded source-target catalog
  + local exact-i64 result rows
  + generated Core result-kind row
  + bounded source String receiver fact
    -> one site-indexed callable-result catalog
```

The result catalog becomes lifetime-bound and non-Clone. It borrows the exact
declaration and source-target catalogs used by the proof. Construction must
co-validate pointer identity through a narrow target-catalog brand query;
equal canonical keys from a foreign declaration catalog are not sufficient.

Suggested product shape:

```rust
pub(crate) struct VerifiedSameModuleCallableResultCatalogV1<
    'targets,
    'catalog,
> {
    declarations:
        &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    targets:
        &'targets VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    rows_by_key:
        BTreeMap<CanonicalSameModuleCallableKeyV1,
                 VerifiedCallableResultDispositionV1>,
    call_rows_by_site:
        BTreeMap<(CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
                 VerifiedCallableResultCallSiteV1<'targets>>,
}
```

The source-target layer may add only these neutral read-only queries:

```text
VerifiedSourceStaticCallTargetV1::target()
VerifiedSourceStaticCallTargetCatalogV1::is_branded_by(declarations)
```

It must not expose mutable rows, raw route inputs, or a second declaration
lookup path.

## Call-result row

One accepted site row co-seals the exact evidence needed by that site.

```rust
pub(crate) struct VerifiedCallableResultCallSiteV1<'targets> {
    evidence: VerifiedCallableResultEvidenceV1<'targets>,
    required_i64_arguments: Box<[u32]>,
}

enum VerifiedCallableResultEvidenceV1<'targets> {
    SameModuleStatic {
        source_target: &'targets VerifiedSourceStaticCallTargetV1,
        callee_required_i64_arguments: Box<[u32]>,
    },
    CoreStringMethod {
        receiver_fact: SourceCoreReceiverFactV1,
        contract: &'static CoreMethodContractResultRowV1,
    },
}
```

The same-module row borrows the CUT0 target row. It must not clone a target
row into a second target authority. The Core row retains only the bounded
source fact and the exact generated contract row; method spelling alone is
never result authority.

## Exact expression law

The existing source-body proof gains canonical `SourcePathV1` threading. It
does not add another AST projector or call inventory.

```text
MethodCall:
  prove arguments once in source order
  -> lookup exact (caller key, SourceExprSiteV1) target
  -> if same-module target exists, consume its current result disposition
  -> otherwise require bounded ExactStringOnSuccess receiver fact
     plus one generated Core receiver/spelling/arity row

FunctionCall:
  arguments are still observed once
  result remains StaticCallTargetAuthorityUnavailable
```

Nested call arguments are proven child-before-parent. A local initialized from
a bounded String receiver expression may retain `SourceCoreReceiverFactV1`
inside the construction-only expression environment. This does not add String
to `I64ExpressionFactV1`.

## Required-argument substitution

For a callee row `ExactI64 { required_i64_arguments }`, substitute only those
callee ordinals through the caller's ordered argument facts.

```text
required argument exact:
  union its caller-parameter requirements

required argument missing / Unknown / KnownNonI64:
  Unavailable(RequiredArgumentRepresentationUnavailable)

non-required argument non-i64:
  does not invalidate the result row
```

The substituted caller requirements and the callee's original required
ordinals are both retained in the call-result row.

## Deterministic solver law

Use one construction-local monotone solver in canonical-key order.

```text
initial:
  every static declaration has a private Pending state

local/declared/Core exact proof:
  may promote Pending -> ExactI64

call dependency:
  may promote only after the callee row is exact

permanent unsupported source:
  closes to its exact Unavailable reason

stable unresolved cycle/dependency:
  closes to Unavailable(RecursiveDependency)

final pass:
  re-prove stable rows and seal call_rows_by_site exactly once
```

No public result row is used as mutable Pending vocabulary. The solver may
not depend on declaration order, callee-first lowering, MIR, runtime state,
retry, or SCC inference. Loop re-analysis must roll back draft call rows and
return observations between construction iterations.

## Clean implementation order

One S0b commit owns this complete disconnected slice:

1. Add unavailable/error vocabulary and the pure required-argument
   substitution helper.
2. Add the lifetime-bound call-result row/evidence product.
3. Add exact-site call proof over the post-CUT0 target catalog and generated
   Core result rows.
4. Thread canonical source paths and the separate Core receiver environment
   through expression/function proof.
5. Add the deterministic monotone solver and stable final call-row seal.
6. Rebuild test helpers from exact S0/L0/R0 source-call products; do not use
   removed raw candidates or manually paired AST expressions.
7. Add synthetic substitution/nested/reorder tests and the actual-source
   matrix.
8. Extend the existing callable-result and Core manifest guards compactly.

Expected source structure:

```text
src/mir/callable_result_representation/
  call_substitution.rs
  call_row.rs
  call_proof.rs
  expression_proof.rs
  function_proof.rs
  solver.rs
  tests/call_substitution.rs
  tests/actual_sources.rs
```

No source/check file may reach 800 lines. Split tests or helpers before that
limit; do not weaken the existing guard to make room.

## Pass matrix

```text
StringHelpers.skip_ws/2
  = ExactI64 {1}

ParserStringUtilsBox.skip_ws/2
  = ExactI64 {1}

StringHelpers._digit_value/1
  = ExactI64 {}
  through one exact current-owner target row

StringHelpers.to_i64/1
  = ExactI64 {}
  through current-owner _digit_value and String.length/0

qualified imported-alias call
direct canonical-owner call
one forwarding wrapper
two forwarding wrappers
nested call argument
callee-required-argument substitution
String.length / len / size exact generated-row parity
provider/caller declaration reorder parity
```

## Unavailable matrix

```text
bare FunctionCall
missing exact source-target row
unsupported Core receiver
wrong Core spelling or arity
Dynamic / non-i64 Core result
required argument missing, Unknown, or non-i64
heterogeneous untyped returns
direct recursive dependency
mutual recursive dependency
unsupported source expression/control shape
```

These remain valid source with no exact result row. They do not become new
compile errors.

## Typed structural errors

```text
declaration / target catalog identity mismatch
source-target caller outside the static result catalog
source target key outside the exact result catalog
required argument ordinal outside target arity
duplicate call-result site with different evidence
result row/cardinality drift
bounded worklist non-convergence
stable final-pass result drift
```

## Required gates

```bash
cargo test -q callable_result_representation
cargo test -q source_call_target
cargo test -q core_method_result_kind
python3 tools/checks/lib/callable_result_i64_catalog_s0.py .
bash tools/checks/core_method_contract_manifest_guard.sh
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

Guard invariants:

```text
result catalog definitions = 1
result catalog Clone implementations = 0
result catalog production producers/consumers = 0
source-target catalog production consumers = 0
same-module target/result co-seal owners = 1
call substitution owners = 1
Core result lookup consumers = exactly one disconnected call proof
bounded String receiver consumers = exactly one result-proof family
bare call activation = 0
physical-symbol parsing = 0
Builder MirType / final metadata / runtime-tag reads = 0
retry / re-lowering / callee-first ordering = 0
new persistent ValueId type/owner maps = 0
source/check files >= 800 lines = 0
```

## Stash rewrite classification

Reusable as algorithm evidence:

```text
required-argument substitution
exact-site call proof
SourcePath threading
separate Core receiver environment
flow merge and loop draft rollback
bounded fixed point plus stable call-row pass
synthetic and actual-source expected matrices
```

Stale and forbidden to restore:

```text
QualifiedStaticCallCandidateV1
CurrentOwnerStaticCallCandidateV1
raw caller/site/AST/fact constructors
old seal_qualified(declarations, imports, candidates)
old extend_current_owner(declarations, candidates)
manual AST scan paired to a current-owner candidate
pre-CUT0 source-target guard sections
stashed CURRENT_STATE / design-card edits
owned cloned source-target evidence
```

## Implementation may claim

```text
one exact declaration/target/result catalog identity chain
site-exact qualified and current-owner result composition
exact required-argument substitution
bounded String receiver plus generated Core result composition
declaration-order-independent acyclic wrapper composition
actual skip_ws wrapper and to_i64 exact-i64 rows
production consumers and behavior delta remain zero
```

## Implementation must not claim

```text
general callable result typing
bare-call final authority
general String abstract interpretation
general non-i64 result domain
call totality, purity, or termination
recursive/SCC result inference
Builder/MIR/runtime result inference
production publication or HMI execution
fallback, retry, or re-lowering
```

## Stop conditions

Stop before implementation broadening if any of these is required:

1. Declaration/target identity can be joined only by canonical-key equality.
2. A target row must be cloned and used as an independent target authority.
3. A second callable catalog, AST traversal, result map, or call inventory is
   required.
4. Raw names, physical symbols, Builder state, final MIR metadata, runtime
   tags, or HMI-specific spelling must enter the proof.
5. Bare FunctionCall target guessing is required.
6. String or another non-i64 representation must enter
   `I64ExpressionFactV1`.
7. Callee-first lowering, provisional lowering, re-lowering, fallback, or
   retry is required.
8. Recursive SCC inference is required for the selected actual-source chain.
9. Fixed-point draft call rows leak across failed or repeated iterations.
10. A production consumer must be added before P0/I0.
11. Stash restoration or a removed raw candidate API is required.
12. A source/check file reaches 800 lines.

If none occurs, implement locally without another consultation.
