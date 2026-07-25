# ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME0

Decision: `ENTRY-RESULT-PROJECTION0-S1-PROJECTION-CONSUME-prime-r1`
Status: implementation authorized; disconnected fixture/contract row only

## Objective

Consume `PhysicalSourceEntryCarrierV1` once through a backend-neutral prepared
projection. Preserve the carrier beside `ProcessTerminationV1`; do not open a
Builder, scan a module, call a backend, or activate public ingress.

## Owner chain

```text
PhysicalSourceEntryCarrierV1
  -> prepare_process_projection(self)
       -> Err(RejectedSourceEntryProjectionV1 { exact carrier })
       -> Ok(PreparedSourceEntryProjectionV1)
            -> project(self)
            -> ProjectedSourceEntryV1
```

The prepared owner records only the canonical profile and the already sealed
source result. `ProcessExitProjectionV1` remains the sole status authority.
The carrier is retained by value in the success product, so route/manifest
evidence cannot be reconstructed from a status or discarded accidentally.

## Failure law

The only first-slice failure is the disconnected legacy-profile request. It
returns the exact carrier with stage, typed cause, and `discard(self)` only.
No retry, fallback to zero, legacy converter, process exit, or publication is
allowed. Canonical result projection itself is infallible after preparation.

## Acceptance matrix

```text
Script Unit                 -> ProjectedSourceEntryV1 + Exit(0)
App Integer(0/255)          -> ProjectedSourceEntryV1 + exact Exit
Integer(-1/256)             -> ProjectedSourceEntryV1 + typed range Fault
Bool/Float/String/Object    -> ProjectedSourceEntryV1 + typed unsupported Fault
source Fault                -> ProjectedSourceEntryV1 + reserved-70 Fault
legacy profile              -> exact carrier rejection + discard-only
route/manifest retention    -> Script/App and manifest retained by value
normal/JSON/backend callers -> 0
```

## Non-claims

```text
VM execution integration
LLVM/native ny_main wiring
public compile_raw_with_source
normal compile_with_source cutover
JSON/Program(JSON v0)
executor/selfhost/fastmem
legacy retirement
CUT0
```

All new or modified source/check files remain below 800 lines.

## Closeout evidence

```text
RUSTFLAGS='-Awarnings' cargo test -q --lib source_entry_projection -- --test-threads=1
  3 passed

python3 tools/checks/lib/entry_result_projection0_s1_projection_consume_guard.py
  one_prepare=1 one_commit=1 carrier_retained=1 no_backend=1 below_800=1
```
