# ENTRY-RESULT-PROJECTION0-S3 runtime activation design stop

Decision: `ENTRY-RESULT-PROJECTION0-S3-RUNTIME-ACTIVATION-DESIGN-STOP`
Status: design consultation required; S2 is complete and no production
runtime caller is authorized

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

## Questions to close

### Q1 — first production owner

Choose exactly one:

```text
A (recommended): explicit Raw VM-reference production entry
B: existing general VM/MIR runner widening
C: LLVM/native ny_main activation
D: normal compile_with_source cutover
```

Recommendation: A. A named Raw entry can select the canonical projection
without silently changing legacy callers. B mixes compatibility status laws,
C requires ABI parity first, and D combines route cutover with result
activation.

### Q2 — execution input

Must the production owner consume `VmReferenceProcessOutcomeV1` by value, or
may it reconstruct status from a module/interpreter result?

Recommendation: consume the typed outcome by value. Module result,
`SourceEntryResultV1`, Box downcast, and symbol lookup remain non-authority.

### Q3 — fault reporting

Who owns conversion of typed `ProcessFaultV1` into user-visible diagnostics?

Recommendation: one named runtime diagnostic adapter after the typed outcome.
It may format code/detail but must not change status, retry execution, or
flatten unsupported results to success zero.

### Q4 — activation parity

Before the first production caller, require:

```text
canonical outcome fixture parity
dirty/reused compiler retention
typed fault diagnostic parity
no legacy fallback
exact caller census
one selected entry only
```

Recommendation: yes. VM, LLVM, public Raw, and normal-entry caller counts must
be measured separately.

## Candidate executable row after acceptance

```text
ENTRY-RESULT-PROJECTION0-S3-RAW-VM-ACTIVATION0-S0
```

Suggested internal order:

```text
S3-DIAGNOSTIC0
  typed fault -> stable diagnostic report

S3-OWNER0
  exact production owner and one consuming entry

S3-PARITY0
  status/diagnostic/reuse/caller census

S3-G0
  no fallback, no legacy widening, no other backend/public caller
```

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
