# RAW public cutover decision and task map

Decision: `RAW-PUBLIC-CUTOVER-prime-r1`

Status: selected. This card closes `RAW-PUBLIC-CUTOVER-CONSULT0` and fixes the
execution order through bounded parity and old Raw-chain retirement.

## Decision lock

```text
Q1 = A
  compile_with_source remains Legacy.
  compile_raw_with_source remains the sole explicit NarrowV1 Raw API.
  A normal-entry switch is not authorized by bounded first-slice parity.

Q2 = A
  Require bounded success/failure parity for the exact sealed Raw grammar,
  normalized MIR/verification relation, observable VM behavior, and compiler
  reuse. Unsupported shapes remain typed rejection with no fallback.

Q3 = A+
  Every Raw failure must leave the live Builder unchanged and permit reuse.
  Import disposition is request-owned; ambient imports are not Raw authority.

Q4 = A
  Retire the old Raw finalizer/run_raw/ledger-root evidence only in one
  dedicated caller-zero row after its two unique fixtures and guards migrate.

Q5 = A
  JSON, executor, selfhost, fastmem, and CUT0 remain separate authorities.

Q6 = A
  Use a production/cfg-test-aware route census plus
  RAW-PUBLICATION-SUNSET-001. Repository-wide lexical zero is not required.
```

## Worker inventory corrections

Four read-only worker audits fixed the following facts.

```text
compile_raw_with_source definition                 = 1
compile_raw_with_source non-test caller            = 0

compile_with_source normal-route leaf callers      = 14
  source-hint/no-import leaves                     = 6
  source-hint/explicit-import leaves               = 6
  MirCompiler::compile adapter leaves              = 2

direct build_module non-test callers               = 2
  legacy compiler internal                         = 1
  runtime AST-JSON bridge                          = 1

host-provider AST-JSON build_module                = cfg(test)-only
old compiler Raw run_raw non-test caller           = 0
old Raw calls                                      = old cfg(test) fixtures only
```

The earlier consultation census incorrectly treated the host-provider
AST-JSON module as production. `src/host_providers/mir_builder/lowering.rs`
registers it under `#[cfg(test)]`; the production AST-JSON bridge is
`src/runtime/mirbuilder_emit.rs`.

## Why normal cutover is not the next row

### Import authority is not sealed

The current explicit Raw entry snapshots the live Builder configuration.
Therefore a reused compiler can carry stale `using_import_boxes` into a bare
Raw request. Conversely, the legacy import-aware entry mutates live Builder
imports before compilation, which cannot satisfy mutation-free Raw failure.

The first public Raw policy must own this exact disposition:

```text
RawPublicImportDispositionV1::None
  -> candidate imports are exact empty
  -> live Builder imports are not mutated
  -> failure retains the original live Builder
```

An explicit-import Raw request is a later capability row. It must not reuse an
ambient Builder snapshot as source authority.

### The helper grammar is not bounded

The root body is sealed as `LinearScalar0`, but App helpers currently validate
only a narrow header and then pass arbitrary body/metadata to the legacy child
lowerer. `NarrowV1` therefore cannot yet make a truthful bounded-grammar claim.

The first helper profile is fixed as:

```text
StaticHelper0
  static
  non-override
  arity = 0
  params / param_decls = empty
  return / uses / attrs / contracts = empty
  body = empty
```

This is a correctness narrowing with zero production callers. A later
`HelperLinear0` row may widen the profile; it must not be smuggled into parity.

### Normal callers are two different request families

`compile_with_source` and `compile_with_source_and_imports` feed different
runner lanes. Switching only the former would make no-import Stage1/bench/MIR
routes Raw while import-aware VM/LLVM/WASM/vm-hako routes remain Legacy. That
is a split compiler authority, not a clean cutover.

## Fixed execution order

No new design consultation is required for the following rows.

```text
0. PUBLIC-INGRESS0-CLOSEOUT-REPAIR0-S0
   repair the stale closed-row guard and task status
   freeze a cfg-test-aware caller manifest
   add the missing public-chain reuse/failure closeout fixtures
   route/grammar/config behavior delta = 0

1. PUBLIC-INGRESS-CONFIG0-S0
   add request-owned RawPublicImportDispositionV1::None
   candidate imports = exact empty
   live Builder mutation before publication = 0
   stale-import and failure/reuse fixtures

2. PUBLIC-CUTOVER-COVERAGE0-S0
   seal exact StaticHelper0 before physical effects
   arbitrary helper body/metadata legacy descent = 0
   place new coverage code in a small sibling module

3. PUBLIC-CUTOVER-PARITY0-S0
   bounded Legacy-vs-Raw success/failure/reuse proof
   normalized MIR + verification + VM-observable relation
   fallback = 0
   normal-entry consumer = 0

4. OLD-RAW-RETIRE0
   R0a proof migration
   R0b old source/variant deletion
   G0 exact caller/source zero closeout
```

`OLD-RAW-RETIRE0` is one semantic BoxShape row using Refactor Series Mode
(two or three buildable commits). It is not combined with normal-entry
activation.

After these rows, a controlled production consumer or broader grammar row may
be selected from measured evidence. `PUBLIC-NORMAL-CUTOVER0` is deliberately
not executable while the normal request surface exceeds NarrowV1.

## Closeout repair contract

The first row repairs evidence, not behavior.

```text
PUBLIC-INGRESS0 execution card status            = closed
PUBLIC-INGRESS0 guard usable after later pointers = yes
guard counts cfg(test) items correctly            = yes

compile_raw_with_source producer                  = 1
compile_raw_with_source non-test caller           = 0
compile_with_source route delta                   = 0
compile_with_source_and_imports route delta       = 0
runtime AST-JSON route delta                      = 0
Program(JSON v0) route delta                      = 0
old Raw non-test caller                           = 0

focused public fixtures:
  empty Script success
  REPL pre-binding rejection
  repeated Raw success
  Raw failure -> Raw success reuse
  stable stage prefix
  live Builder unchanged on failure
```

Import contamination belongs to `PUBLIC-INGRESS-CONFIG0`; helper coverage
belongs to `PUBLIC-CUTOVER-COVERAGE0`; the closeout repair must not pre-solve
either contract.

## Bounded parity matrix

Success:

```text
empty Script
all seven literal variants
three admitted unary operators
ordinary binary operators; And/Or excluded
Expr / Print / Local / Assignment / CompoundAssignment
App empty/scalar main
App exact-empty StaticHelper0
optimize on/off
source-file hint
Raw -> Raw
Raw failure -> Raw
Raw -> Legacy
Legacy -> Raw
```

Compare:

```text
sorted function set
signature / arity / return / effects
normalized CFG / value / op / constant relation
backend-required metadata
verification disposition
VM-observable result where the locked grammar permits it
```

Failure:

```text
REPL / non-Program root
Script declaration / non-Main App
If / Loop / LoopRange / Return / Break / Continue / ScopeBox
And / Or / weak unary
typed local / cardinality drift / invalid assignment target
App metadata or arity drift
undefined variable
helper outside StaticHelper0
dirty publication target
```

Every failure proves stable stage/code, unchanged live Builder, no result, no
fallback, and a succeeding reuse compile. Natural POST0 fault production is
not added merely for this matrix; existing lower-level typed fixtures remain
the evidence for optimizer/contract-refresh failures.

## Old Raw retirement boundary

Delete:

```text
src/mir/builder/raw_physical_finalization.rs
src/mir/compiler/raw_finalization.rs
old ModulePostprocessInputV1::Raw / run_raw / Raw evidence arms
old external-commit ledger/root-only Raw evidence arm
old registrations and re-exports
```

Retain:

```text
RawCompleteInvocationV1::into_parts used by new DRAIN0
ModuleVerificationEvidenceV1::Raw
ModulePostprocessScheduleV1::for_family(Raw)
shared run_postprocess_stages
new run_raw_ready
raw_finalization_contract.rs
raw_root_finalization.rs
canonical publication authority
```

Before deletion, migrate:

```text
new DRAIN0 PublishedShell rejection fixture
new FINAL0 BuilderReadiness rejection/owner-retention fixture
historical guards that directly require old source files
```

## Hard non-claims

```text
compile_with_source cutover
compile_with_source_and_imports cutover
runner source-hint rewiring
REPL
runtime/mirbuilder_emit
AST JSON
Program(JSON v0)
JSON import-bundle compile_legacy
core executor
Stage1 / vm-hako / selfhost
LLVM / WASM / fastmem
fallback
CUT0 activation
```

## First executable row

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLIC-INGRESS0-CLOSEOUT-REPAIR0-S0
```
