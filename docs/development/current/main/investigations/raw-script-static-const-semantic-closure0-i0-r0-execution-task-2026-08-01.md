# RAW-SCRIPT-STATIC-CONST-SEMANTIC-CLOSURE0-I0-R0 — execution task

## Decision

```text
Decision:
  Candidate A-prime — accepted with worker corrections

Ceremony:
  T2, one atomic I0/R0 commit

Selected family:
  StaticConstTable
  + DirectStaticConstRuntimeCompletion

Grammar / metadata / MIR policy delta:
  0
```

This row does not reopen the already-closed StaticConst lowering owner. It
removes one selected-normal reachability from the shared Deferred Script root
by admitting an existing zero-child metadata/runtime-completion boundary into
the Script Complete source product.

## Production edge

```text
Named caller:
  ModuleBuilderInvocationSessionV1::
    complete_normal_default_program_root_catalog_lifecycle

Current:
  StaticConstTable
  + DirectStaticConstRuntimeCompletion
  -> UnsafeRuntimeStatement
  -> Deferred
  -> RawInvocationSourceTransportV1::script_root(())

Target:
  exact pair
  -> Script semantic closure Complete
  -> VerifiedScriptSemanticSourceV1
  -> script_semantic_root
  -> exact ProgramBody(original ordinal) source loan
  -> existing StaticConst runtime completion owner
```

Unrelated Deferred, raw, and reference Script routes remain unchanged.

## Exact accepted closure

The request is Complete only when every selected runtime row is either:

```text
existing Complete closure:
  Literal
  prior-Local-backed Variable
  admitted Local
  Print
  Unary except Weak
  Binary including And/Or
  Await
  CheckExpr

or:
  ASTNode::StaticConstTable
  paired exactly with
  DirectStaticConstRuntimeCompletion
```

StaticConst has zero expression/body/call/owner children. A request containing
any remaining unsupported family is wholly Deferred; per-item mixed routing is
forbidden.

## Admission product

Generalize the current internal admission vocabulary without adding a file:

```rust
struct ScriptSemanticClosureFactsV1 {
    lexical: ScriptLexicalFactsV1,
    static_const_completion_source_indices: Box<[usize]>,
}

enum ScriptSemanticClosureAdmissionV1 {
    Complete(ScriptSemanticClosureFactsV1),
    Deferred(ScriptLexicalDeferredReasonV1),
}

enum ScriptSemanticAdmissionInvariantErrorV1 {
    SourceAdmissionMismatch,
    SourceOrdinalMissing,
    DuplicateCompletionSite,
    RuntimeCoverageMismatch,
}
```

The one existing source traversal returns:

```rust
Result<
    ScriptSemanticClosureAdmissionV1,
    ScriptSemanticAdmissionInvariantErrorV1,
>
```

The exact pair is admitted once. A `DirectStaticConstRuntimeCompletion` row
whose source is not `StaticConstTable` is a `ScriptSemanticSeal` invariant
rejection, never Deferred. An ordinary unsupported source remains the existing
Deferred result.

Do not rescan Program or reinterpret table payloads in semantic admission.

## Verified source product

Add a bounded receipt in the existing semantic-source file:

```rust
struct VerifiedScriptStaticConstCompletionV1 {
    site: SourceStmtSiteV1,
}
```

Seal requirements:

```text
site = ProgramBody(original ordinal)
Program statement at ordinal = exactly StaticConstTable
completion sites = unique
all selected runtime rows = covered exactly once
runtime_source_indices = lexical roots union StaticConst completion sites
```

The receipt owns only source transfer/coverage. It must not own:

```text
table name or element type authority
value-range validation
StaticTableContractSpec / StaticDataPlan
backend symbol
ValueId / MirType / ABI / slot
Binding / Scope / Region / Exit facts
```

## Exact source transport correction

Installing a prepared statement source is not sufficient today:
`StaticConstTable` currently falls through the raw source context to
`UnlocatedCompatibility(CallObject)`.

Add a dedicated zero-child runtime-completion predicate in
`raw_invocation_source_transport.rs`, and admit only `StaticConstTable` through
it in both body-statement location checks. Do not widen the existing scalar,
control, call/object, or Lambda predicates.

Then change the existing runtime branch to:

```text
DirectStaticConstRuntimeCompletion
-> prepare_body_statement_source_v1(statement, original ordinal)
-> with_prepared_child_source_v1
-> lower_direct_static_const_runtime_completion_v1
```

The existing terminal remains kind-check -> span -> Void.

## Existing semantic owners

Keep these owners unchanged:

```text
metadata:
  PreparedNormalProgramStaticTableMetadataV1::prepare(...).commit()

runtime completion:
  lower_direct_static_const_runtime_completion_v1
```

Element type, value range, contract specs, and static data plans remain in
RootLower. They are not moved to ScriptSemanticSeal.

## Failure and precedence

```text
Parser
< RootExpansion
< PrepareModule
< CatalogSeal
< ScriptSemanticSeal      # invariant/coverage only
< CatalogInstall
< RootLower               # StaticConst semantics remain here
< FinalizeModule
```

Required behavior:

```text
AST/admission mismatch          -> ScriptSemanticSeal hard rejection
missing/duplicate/partial site  -> ScriptSemanticSeal hard rejection
unsupported element/value      -> existing RootLower diagnostic
mixed unsupported family       -> whole request Deferred, owner issue = 0
Complete seal failure           -> never downgrade to Deferred
failure                         -> candidate discarded
reuse                           -> fresh request/session only
fallback / retry                -> 0
```

Catalog failure must precede invalid StaticConst. Invalid StaticConst metadata
must precede a later runtime undefined-variable diagnostic, preserving the
current Program lowering order.

## Sunset and ratchet correction

Reuse:

```text
SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
```

The current manifest still says `complete_baseline = 0` and all selected
Script requests Deferred. That is stale after Literal/Local/Unary/Print/
Binary/Await/Check/AndOr landed.

In this same I0/R0:

```text
replace stale baselines with named Complete fixture identities
add script_static_const_u16_completion as Complete
retain exact named Deferred reasons for unaffected fixtures
require old Complete identities to remain present
```

Do not claim a percentage-only ratchet. Add only a compact assertion to the
existing shared guard; it is currently 794 lines and must remain below 800.
If the identity ratchet cannot be consumed within that budget, stop the row
rather than adding inert JSON or a new per-row guard.

## Focused evidence

Use existing test modules and no new Rust/check file.

```text
1. exact StaticConst/admission pair -> Complete
2. wrong AST/admission pair -> typed invariant rejection
3. Program ordinals 0 and 2 remain exact across a transferred declaration
4. parser-backed positive source:
     static const TABLE: u16[] = [1, 2, 3]
     print(1)
5. normal/legacy parity:
     MirPrinter
     verification_result
     static_table_contract_specs
     static_data_plans
     function set
6. invalid element type remains RootLower and retains exact diagnostic
7. failed candidate publishes no table metadata
8. same compiler accepts a fresh valid request afterward
9. StaticConst plus unsupported family -> whole request Deferred
10. exact located ProgramBody source context, no CallObject portal
```

The inline positive source is a parser-backed production fixture, not a
tracked real-file fixture. Do not run the broad mimalloc proof application or
any corpus/benchmark harness in this row.

## Exact files

Production edits may touch only:

```text
src/mir/builder/normal_script_lexical_binding.rs
src/mir/builder/normal_script_runtime_work.rs
  symbol/type wiring only; current 758-line file must not grow materially
src/mir/builder/normal_script_semantic_source.rs
src/mir/builder/normal_default_root_catalog_lifecycle.rs
src/mir/builder/normal_script_runtime_block_port.rs
src/mir/builder/raw_invocation_source_transport.rs
```

Evidence edits:

```text
tools/checks/manifests/raw_public_cutover_caller_manifest_v1.json
tools/checks/lib/
  cut0_i0_root0_raw_source0_lower_root_post0_public_ingress0_guard.py
existing module-local tests only
```

No new source/test/check file. Every source/check file must remain below 800
lines. In particular:

```text
normal_script_runtime_work.rs                 = 758 before
raw_invocation_source_transport.rs            = 761 before
shared cut0 guard                              = 794 before
```

## Atomic deletion

After the commit:

```text
StaticConst exact pair
-> UnsafeRuntimeStatement
-> Deferred
-> script_root(())
reachability = 0

other Deferred -> script_root(()) = retained exactly once
raw/reference behavior            = unchanged
```

## Hard stops

```text
StaticConst table-load expression
Array / Map / Record
Call / MethodCall / Field / New
If / Loop / QMark / Match
Weak / Lambda / Box

table type/value semantics copied into semantic source
StaticDataPlan or contract spec copied into semantic source
StaticConst diagnostic moved to ScriptSemanticSeal
fake Binding/Scope/Region/Exit facts

partial request Complete
Complete-to-Deferred downgrade
second resolver / second forest
fallback / retry

shared Deferred branch deletion
raw/reference script_root change
collector drain reopen
closed StaticConst runtime-completion row reopen

new source/test/check file
inert manifest keys without a consuming check
any source/check file reaching 800 lines
```

## Done

```text
StaticConst positive parser fixture = Complete
exact ProgramBody source receipt = located
shared forest/projection = unchanged
metadata/MIR/function parity = green
invalid table remains RootLower
failure discard and fresh reuse = green
mixed unsupported request = Deferred as a whole
fixture identity ratchet = consumed and monotonic
fallback/retry = 0
all source/check files < 800
```

