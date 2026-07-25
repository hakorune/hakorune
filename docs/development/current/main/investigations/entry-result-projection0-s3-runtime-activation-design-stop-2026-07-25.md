# ENTRY-RESULT-PROJECTION0-S3 runtime activation design stop

Decision: `ENTRY-RESULT-PROJECTION0-S3-RAW-VM-ACTIVATION-prime-r1`
Status: accepted; implementation is authorized only through the staged
explicit Raw VM-reference route below

## Closed input

S2 now provides one compiler-internal, disconnected owner chain:

```text
ProjectedSourceEntryV1
  -> consume_vm_reference(self)
  -> VmReferenceProcessOutcomeV1
       complete projected owner retained
       normalized status
       exact typed fault
       discard-only terminal
```

Every canonical fault carries reserved status 70 from the projection owner.
The adapter does not inspect source results, module symbols, object handles,
or legacy status helpers. Focused fixtures cover exact 0/255 success,
out-of-range integers, unsupported values, source faults, and Script/App
evidence retention. Production consumers remain zero.

## Why execution stops here

The next change would select a real execution/publication authority. Current
runtime routes do not share one status law:

```text
VM / MIR runner       legacy scalar and fallback mappings
LLVM/native wrapper   backend-specific return normalization
public Raw ingress    compatibility result contract
normal compiler entry legacy route still selected
```

Connecting any one of them changes observable production behavior and cannot
be inferred from S2’s disconnected semantic proof.

## Accepted decisions

### Q1 — first production owner

Q1 selects exactly one:

```text
A (recommended): explicit Raw VM-reference production entry
B: existing general VM/MIR runner widening
C: LLVM/native ny_main activation
D: normal compile_with_source cutover
```

Decision: A. A named Raw entry selects the canonical projection
without silently changing legacy callers. B mixes compatibility status laws,
C requires ABI parity first, and D combines route cutover with result
activation.

### Q2 — execution input

Q2: the production owner must consume `VmReferenceProcessOutcomeV1` by value;
may it reconstruct status from a module/interpreter result?

Decision: consume the typed outcome by value. Module result,
`SourceEntryResultV1`, Box downcast, and symbol lookup remain non-authority.

### Q3 — fault reporting

Q3: `VmReferenceProcessDiagnosticAdapterV1` owns conversion of typed
`ProcessFaultV1` into a stable structured diagnostic report.

Decision: one named runtime diagnostic adapter after the typed outcome.
It may format code/detail but must not change status, retry execution, or
flatten unsupported results to success zero.

### Q4 — activation parity

Q4 requires the following before the first production caller:

```text
canonical outcome fixture parity
dirty/reused compiler retention
typed fault diagnostic parity
no legacy fallback
exact caller census
one selected entry only
```

Decision: yes. VM, LLVM, public Raw, and normal-entry caller counts must
be measured separately.

## Required design correction

The existing S2 carrier is post-projection; it is not an execution engine.
S3 therefore includes these mandatory subrows:

```text
S3-ENTRY-CARRY0
  selected source-entry identity is co-sealed once and moved to publication

S3-EXECUTION0
  exact Main/main/0 VM execution produces SourceEntryResultV1 once
```

The full manifest is not carried. A narrow continuation owns only the same
brand, selected Script/App route, Main key/symbol/arity target, and pairing
seal. The decode plan is derived from the retained root exit witness, so a
VM payload cannot turn `print(1)`, Local, or assignment into an integer source
result.

The typed execution terminal lives in `src/mir/compiler`; a runner shell may
consume only the final public report/status and never receives a bare module.

## Candidate executable row after acceptance

```text
ENTRY-RESULT-PROJECTION0-S3-RAW-VM-ACTIVATION0-S0
```

Suggested internal order:

```text
S3-ENTRY-CARRY0
  manifest -> narrow selected-entry continuation -> publication evidence

S3-DIAGNOSTIC0
  typed fault -> stable diagnostic report

S3-EXECUTION0
  published Raw owner -> exact Main/main/0 target
  -> fresh MirInterpreter -> VMValue/VMError
  -> sealed decode plan -> SourceEntryResultV1

S3-OWNER0
  typed compile kernel + explicit Raw VM production entry

S3-PARITY0
  actual Raw compile + VM execution + status/diagnostic/caller census

S3-G0
  no fallback, no legacy widening, no other backend/public caller
```

Exact execution is only `MirInterpreter::execute_function_with_args` with
`&[]`. `execute_module`, `NYASH_ENTRY`, module scans, Box coercion, and legacy
status helpers are forbidden in the new route. VM implementation is feature
gated; an unavailable VM-reference feature is a typed unsupported-capability
rejection, never a fallback.

## Non-authority

```text
module symbol / NYASH_ENTRY
positive Box handle decoding
legacy status conversion
normal compile_with_source
JSON / Program(JSON v0)
LLVM/native ABI without separate parity
```

## Non-claims

```text
production VM activation
LLVM/native ny_main
public Raw ingress
normal-entry cutover
JSON / Program(JSON v0)
executor / selfhost / fastmem
legacy retirement
CUT0
```
