---
Status: Design consultation stop
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted
---

# Failure/Outcome S1 `VMValue::Void` Owner Design Stop

## Why This Stops

The S1 evidence queue contains 132 `VMValue::Void` rows in
`runtime_backend`. The same current carrier appears in several incompatible
roles:

```text
successful no-result / Unit return
optional absence from env/provider lookup
recoverable provider/file failure
empty weak upgrade result
compatibility MissingBox/Void behavior
backend or helper fallback after an unsupported/missing result
```

A token-only or file-wide classification would assign one semantic meaning to
multiple owners. That would violate the 3505 relation table and make the
inventory appear exhaustive while hiding the actual boundary split.

## Evidence Snapshot

```text
VMValue::Void runtime_backend rows = 132
ConstValue::Void total evidence rows = 123
provider direct rows already classified = 5
semantic runtime activation = 0
```

Representative owner surfaces include:

```text
src/backend/vm_types.rs
src/backend/abi_util.rs
src/backend/mir_interpreter/handlers/externals.rs
src/backend/mir_interpreter/handlers/weak.rs
src/backend/mir_interpreter/handlers/boxes_void_guards.rs
src/backend/mir_interpreter/handlers/calls/method/dispatch.rs
src/mir/join_ir_ops.rs
src/backend/wasm/codegen/instructions.rs
```

## Questions For Decision

```text
Q1. Should each semantic operation boundary receive its own inventory site,
    even when multiple operations currently return VMValue::Void, with
    VMValue::Void retained only as current_carrier evidence?

Q2. Which owner should classify generic `Ok(VMValue::Void)` and missing
    register fallbacks: the operation owner, a backend capability owner, or an
    explicit internal-sentinel class?

Q3. Should compatibility equality/boxing rows be classified as
    compatibility_only independently from Unit and Option absence rows?

Q4. Should Wasm/LLVM zero/null projections be separate foreign/backend rows,
    even when their current carrier is numerically equivalent to Void?
```

## Fixed Constraints

```text
parser behavior change = 0
VMValue change = 0
ConstValue change = 0
backend lowering change = 0
fallback addition = 0
runtime activation = 0
```

Until Q1-Q4 are decided, only evidence rows with an explicit local owner may
be classified. The remaining `VMValue::Void` rows stay pending; no heuristic
owner or file-wide default is allowed.

## Requested Minimal Answer

Decide the site granularity and owner split only. Do not activate Unit,
Option::None, Result::Err, Fault, Weak upgrade, or null migration in this
consultation.

## Accepted Decision

```text
inventory semantic unit = semantic operation outcome branch
token occurrence = evidence only
file = never a semantic owner
VMValue::Void = current-carrier evidence only
generic write_void helper = evidence/projection only
new internal-sentinel semantic class = no
missing register canonical branch = contract_fault
missing register tolerate branch = compatibility_only
compatibility equality/boxing = independent compatibility_only sites
Wasm/LLVM zero projections = separate backend-projection sites
foreign_null = declared FFI nullable boundary only
runtime activation = 0
```

Backend and bridge projections must carry `projects_site`; they may not invent
semantic meaning from a zero/null bit pattern. Provider-missing branches are
`contract_fault` unless an explicit API contract maps them to `Result::Err`.
Weak upgrade dead/freed branches are optional-absence candidates, not
compatibility rows.

## Next Task

```text
LANGV1-FAILURE-OUTCOME-S1-SEMANTIC-SITE-GRAPH-001
```

The next task changes inventory tooling and manifests only. It does not change
parser profiles, MIR, `VMValue`, `ConstValue`, runtime behavior, cleanup, or
backend lowering.
