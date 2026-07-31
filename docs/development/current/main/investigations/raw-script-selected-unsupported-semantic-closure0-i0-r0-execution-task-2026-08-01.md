# RAW-SCRIPT-SELECTED-UNSUPPORTED-SEMANTIC-CLOSURE0-I0-R0 — execution task

## Decision

```text
Decision:
  Accept — existing selected-unsupported diagnostic family

Ceremony:
  T2, one atomic I0/R0 commit

Pack:
  DESCENT-SPINE0

Grammar / diagnostic / result policy delta:
  0
```

Census50 selected the existing `DirectSelectedUnsupportedStatement` family.
It did not select `UsingStatement`: Using currently has no dedicated
admission/terminal family, so selecting it here would create a new no-op
authority instead of replacing an existing one.

No additional external consultation is required. Three read-only worker
audits established the production family, edge deletion, and file budget.

## Named production edge

```text
caller:
  ModuleBuilderInvocationSessionV1::
    complete_normal_default_program_root_catalog_lifecycle

current:
  exact selected-unsupported source/admission pair
  -> UnsafeRuntimeStatement
  -> Deferred
  -> RawInvocationSourceTransportV1::script_root(())

target:
  exact pair
  -> typed existing-diagnostic boundary receipt
  -> VerifiedScriptSemanticSourceV1
  -> script_semantic_root
  -> exact ProgramBody(original ordinal) source loan
  -> existing lower_direct_selected_unsupported_statement_v1
```

The existing diagnostic terminal is retained and runs exactly once. Unrelated
Deferred requests and raw/reference routes remain unchanged.

## Exact family

The family is the existing exact admission:

```text
DirectSelectedUnsupportedStatement
```

paired with exactly one of:

```text
LoopRange
Break
Continue
ImportStatement
BuildGate
EnumDeclaration
BrandDeclaration
TypeAliasDeclaration
GlobalVar
```

All nine are zero-demand boundaries for this route:

```text
semantic child demand = 0
lexical facts         = 0
control resolution    = 0
owner topology        = 0
```

`Break` and `Continue` do not activate CONTROL0 here. At this production
edge they are existing unsupported diagnostics, not loop-exit operations.
Likewise, syntactic payloads under `LoopRange`, `BuildGate`, or `GlobalVar`
are not traversed because the existing terminal rejects the boundary itself.

Do not split this family into per-kind routes or copy diagnostic policy into
semantic admission.

## Admission and verified coverage

Extend the existing one-pass Script admission aggregate with a thin receipt:

```rust
struct ScriptExistingDiagnosticBoundaryFactV1 {
    source_statement_index: usize,
}

struct VerifiedScriptExistingDiagnosticBoundaryV1 {
    site: SourceStmtSiteV1,
}
```

Admission accepts only the exact AST/admission pair. A mismatch is the
existing `SourceAdmissionMismatch` ScriptSemanticSeal invariant rejection,
never Deferred.

Seal requirements:

```text
site = ProgramBody(original ordinal)
site is unique
AST kind is one of the exact nine
runtime coverage includes the site exactly once
```

The receipt owns only source coverage and transfer to the existing diagnostic
owner. It must not own diagnostic text, child semantics, Binding/Scope/Region,
Exit facts, ValueId, ABI, slot, or MIR type.

A request is Complete only if every other runtime row is already in the
current Complete closure. Any Call, If, allocation, Weak, Lambda, Box, Using,
or other residual sibling keeps the whole request Deferred. Per-item route
mixing is forbidden.

## Exact source handoff

Change the existing runtime branch to:

```text
DirectSelectedUnsupportedStatement
-> prepare_body_statement_source_v1(statement, original ordinal)
-> with_prepared_child_source_v1
-> lower_direct_selected_unsupported_statement_v1
```

The source transport must recognize the exact nine as one located diagnostic
terminal family. Prefer compact reuse/extraction because
`raw_invocation_source_transport.rs` is already near the line limit. Do not
widen scalar, call/object, Lambda, or general control predicates.

If this exact source transport cannot be expressed without reaching 800 lines,
stop rather than landing an unlocated compatibility path.

## Failure and precedence

```text
CatalogSeal
< ScriptSemanticSeal   # source/admission/coverage invariants only
< CatalogInstall
< RootLower            # existing unsupported diagnostic
< FinalizeModule
```

Required behavior:

```text
exact family source error       -> existing RootLower diagnostic
source/admission mismatch       -> ScriptSemanticSeal hard rejection
missing/duplicate coverage      -> ScriptSemanticSeal hard rejection
unrelated residual sibling      -> whole request Deferred
Complete seal failure           -> never downgrade to Deferred
failure                         -> candidate discarded
fresh request                   -> compiler reusable
fallback / retry                -> 0
```

Diagnostic text, span, kind spelling, stage, and first-error ordering remain
unchanged.

## Atomic old-edge deletion

After the commit:

```text
exact nine-kind family
-> UnsafeRuntimeStatement
-> Deferred
-> script_root(())
reachability = 0

other Deferred -> script_root(()) = retained exactly once
raw/reference behavior            = unchanged
```

This is a production authority replacement, not a new accepted language
family and not a proof-only owner.

## Sunset and ratchet

Reuse:

```text
SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001
```

The shared guard is at 797 lines. Before adding the new identity, replace the
StaticConst-only fixture-ID assertion with a compact table-driven mapping:

```text
fixture ID -> existing test anchor
```

Then add one family identity for this promotion. The guard must consume the
identity and remain at most 799 lines. Do not add a new guard.

Ratchet law:

```text
all previous Complete IDs remain Complete
unrelated Deferred IDs keep their reason
selected-unsupported family moves Deferred -> Complete
```

If the generic ratchet consumer cannot fit, stop. Do not add inert manifest
metadata or claim a percentage-only ratchet.

## Focused evidence

Use existing module-local test homes only.

```text
1. table test: all nine exact AST/admission pairs -> Complete
2. wrong AST/admission pairs -> SourceAdmissionMismatch
3. exact ProgramBody ordinal across a transferred declaration
4. representative parser-backed normal/legacy diagnostic parity
5. unsupported-before-undefined preserves the existing first diagnostic
6. undefined-before-unsupported preserves the existing first diagnostic
7. unrelated Call/If/etc sibling -> whole request Deferred
8. Complete source uses located diagnostic context, never CallObject
9. failed request publishes nothing; fresh request succeeds
10. previous Complete fixture identities remain green
```

Keep focused tests in `normal_script_lexical_binding.rs` and
`normal_script_semantic_source.rs`. Do not grow the 796-line legacy integration
test or the 793-line source-transport test. Do not run corpus, benchmark, or
parallel proof harnesses.

## File budget

Relevant pre-row measurements:

```text
normal_script_lexical_binding.rs       588
normal_script_runtime_work.rs          764   # wiring only
normal_script_semantic_source.rs       720
normal_script_runtime_block_port.rs    159
raw_invocation_source_transport.rs     789
shared cut0 guard                      797
legacy_candidate_session_tests.rs      796   # frozen
raw source transport tests             793   # frozen
```

No new source/test/check file. Every source/check file must remain below 800
lines. Behavior-neutral compaction or a bounded responsibility extraction is
allowed only where required to preserve this limit.

## Hard stops

```text
diagnostic text, span, stage, or first-error order changes
any child syntax must be semantically traversed
Break/Continue requires loop target or control facts
another family must be admitted simultaneously
request-local Complete/Deferred mixing

Using no-op authority
If / QMark / Match / Call / Object / Allocation
Weak / Lambda / Box / Outbox

new diagnostic owner or policy copy
Binding / Scope / Region / Exit facts
ValueId / ABI / slot / MIR type

second resolver / second forest
Complete-to-Deferred downgrade
fallback / retry

new per-row guard
inert fixture ID
any source/check file reaching 800 lines
```

## Done

```text
exact nine-kind family = Complete source coverage
exact ProgramBody source = located at existing terminal
existing diagnostic text/stage/order = unchanged
mixed residual request = Deferred as a whole
failure discard and fresh reuse = green
fixture identity ratchet = generic and consumed
old exact-family Deferred reachability = 0
fallback/retry = 0
all source/check files < 800
```
