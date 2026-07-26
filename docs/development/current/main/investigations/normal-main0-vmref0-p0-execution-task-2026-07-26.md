---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-VMREF0-P0
Scope: prove canonical Main execution, process projection, diagnostics, and reuse through the sole neutral VM-reference terminal
ceremony_tier: T1 connected proof over an existing disconnected canonical source family
proof_inventory_before: canonical Main adapter fixture plus Raw S3 projection law
new_proofs: one canonical Main execution/status/diagnostic/reuse matrix
retired_or_merged_proofs: adapter-only execution assertions merge into this matrix
net_proof_delta: one bounded source-family parity proof
sunset_budget: adapter-only assertions may remain local; no new shell wrapper
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-main0-vmref-adapter0-i0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-tx0-i0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md
---

# NORMAL-MAIN0-VMREF0-P0

## Outcome

Prove the complete canonical Main reference chain without adding a runner:

```text
source Main.main/0
  -> canonical F1 plan
  -> atomic source Main + physical thunk candidate
  -> explicit publication
  -> neutral exact VM execution
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
  -> bounded status/diagnostic report
```

No new compiler, executor, process projection, diagnostic adapter, or source
classifier is allowed.

## Closeout

The canonical Main proof is green:

```text
canonical Main VM-reference tests            = 3/3
matrix rows                                  = 12
reuse sequence                               = 7 terminals
existing Raw execution matrix                = 18/18
normal module transaction fixtures           = 11/11
neutral execution/owner guards               = green
cargo check --lib --features vm-reference    = green
canonical Main runner/CLI caller             = 0
```

The matrix includes actual division-by-zero VM Fault, range faults, unsupported
Bool/Float process results, every admitted Unit origin, and later-success
Builder reuse. Status and diagnostic tags come only from the existing shared
projection/adapter.

## Required matrix

```text
empty body                    -> Unit(EmptyBody)           -> status 0
non-empty fallthrough         -> Unit(ImplicitFallthrough) -> status 0
bare return evidence          -> Unit(BareReturn)          -> status 0
return void                   -> Unit(ExplicitVoid)        -> status 0
return null                   -> Unit(ExplicitNull)        -> status 0

return Integer(0)             -> status 0
return Integer(255)           -> status 255
return Integer(-1)            -> status 70 + range diagnostic
return Integer(256)           -> status 70 + range diagnostic

return Bool                   -> status 70 + unsupported-result diagnostic
return Float                  -> status 70 + unsupported-result diagnostic
```

If the admitted Main grammar can express an actual VM division-by-zero while
remaining inside the current call-free F1 profile, add:

```text
VM division-by-zero           -> source Fault
                              -> status 70
                              -> stable source-fault diagnostic
```

If it cannot, record a typed fixture exclusion. Do not widen grammar or Main
capability inside this proof row.

String/object results remain pre-execution capability rejection and are not
converted into an executed process Fault here.

## Diagnostic law

Use the existing bounded terminal projection:

```text
ProcessExitProjectionV1               = status authority
VmReferenceProcessDiagnosticAdapterV1 = formatting authority
```

Required stable tags:

```text
[process/exit-code-out-of-range]
[process/unsupported-result]
[process/source-fault] when a VM Fault fixture is admitted
```

The proof must not inspect `VMValue`, rebuild status 70, or parse diagnostic
strings to choose semantics.

## Reuse law

Use one reusable canonical builder/compiler owner where the current API makes
that owner explicit:

```text
success Integer(7)
-> process Fault Integer(256)
-> success Unit

success Unit
-> unsupported Bool process result
-> success Integer(1)

VM Fault when admitted
-> later success
```

Every executed program result, including process Fault, is a normal terminal
completion. It must not poison the next unpublished Main transaction.

## Failure boundary

The following remain invocation/capability rejection rather than program
Fault:

```text
source-plan rejection
Main preflight rejection
unsupported result carrier
TX0 preparation/verification rejection
publication/adapter rejection
```

No rejected owner retries a different source family, Raw profile, entry, or
Legacy route.

## Implementation order

```text
P0-A MATRIX0
  table-driven source/result/status/diagnostic fixture

P0-B REUSE0
  success/process-Fault/VM-Fault -> later success

P0-C CENSUS0
  sole neutral executor/projection/diagnostic
  canonical runner caller zero

P0-D CLOSEOUT
  update current pointer to NORMAL-CALLABLE-SOURCE0-S0
```

Prefer extending:

```text
src/mir/compiler/source_entry_vm_normal_main_adapter.rs
```

If the test body would push the file near 800 lines, split only the
`#[cfg(test)]` module into:

```text
source_entry_vm_normal_main_adapter_tests.rs
```

Production and test files must each remain below 800 lines.

## Structural gate

```text
canonical Main exact neutral executor              = 1
ProcessExitProjectionV1 status authority           = 1
diagnostic adapter authority                       = 1

Main status reconstruction                         = 0
VMValue/source-result reinference                  = 0
module/entry scan                                  = 0
fallback/retry                                     = 0

canonical Main runner/CLI production caller        = 0
Raw production behavior delta                      = 0
default/product route delta                        = 0

success/process-Fault -> later success              = green
all modified/new source/check files                < 800 lines
```

Extend the existing neutral execution guard. Do not add a per-row shell
wrapper.

## Immediate continuation

```text
NORMAL-MAIN0-VMREF0-P0
-> NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
```

## Non-claims

```text
canonical Main runner/CLI activation
helper/direct-call support
ordinary callable catalog generalization
String/object/dynamic function result
imports/using
default/product backend cutover
JSON/LLVM/native
cleanup
Legacy or Raw retirement
```
