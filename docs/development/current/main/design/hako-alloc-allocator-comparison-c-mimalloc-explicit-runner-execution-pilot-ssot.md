# Hako Alloc Allocator Comparison C Mimalloc Explicit Runner Execution Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-451A

## Decision: accepted

MIMAP-451A opens the first narrow C mimalloc comparison execution seam. The
execution is outside the Hakorune runtime and is performed only by an explicit
tool:

```bash
tools/allocator/c_mimalloc_explicit_runner.sh --out <evidence-file> --library <libmimalloc.so.2>
```

The tool loads `libmimalloc.so.2` by explicit path, calls `mi_malloc` /
`mi_free` through a small stable C workload, and writes a key-value evidence
file. This row records that evidence in `.hako` model space. It does not make
mimalloc the process allocator.

## Evidence Contract

The evidence file must contain these stable keys:

```text
c_mimalloc_runner=1
output_contract=allocator-comparison-c-mimalloc-explicit-runner-v0
workload=representative-small-block-v0
library_path=<explicit path>
result_code=0
run_count=1
allocation_count=<n>
free_count=<n>
requested_bytes=<bytes>
peak_rss_bytes=<bytes>
memory_usage_evidence=1
process_replacement_executed=0
hook_installed=0
backend_matcher_added=0
global_allocator_installed=0
hidden_discovery_used=0
provider_package_generated=0
summary=ok
```

The guard may use `--allow-ldconfig-discovery` only as a tool-level convenience
to find the library path before invoking the runner. The resolved path is
printed and passed to the C runner explicitly. No Hakorune runtime code, source
program, backend shim, or provider package may do hidden discovery.

## Hako Ledger

The `.hako` owner is:

```text
lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako
```

It consumes `HakoAllocAllocatorComparisonCMimallocExecutionDiagnosticReport`
from MIMAP-449A and records:

- diagnostic readiness;
- explicit runner invocation;
- stable output contract presence;
- memory-use evidence presence;
- runner result code;
- run/allocation/free/requested/peak-memory counters;
- closed process replacement / hook / backend matcher / global allocator flags.

## Reasons

| Reason | Meaning |
| --- | --- |
| 0 | explicit C mimalloc runner evidence accepted |
| 1 | diagnostic report missing |
| 2 | diagnostic report not ready |
| 3 | explicit runner was not invoked |
| 4 | runner output missing |
| 5 | memory-use evidence missing |
| 6 | stable output contract missing |
| 7 | runner returned a non-zero result |
| 8 | invalid run count |

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution from `.hako`.
- No hidden env or runtime discovery of mimalloc behavior.
- No provider package / DLL generation.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `external-runner-pilot`.

MIMAP-451A must run:

- explicit C runner compilation and execution;
- stable evidence key checks;
- VM proof app output contract;
- MIR JSON emit;
- route preflight;
- typed object / record declaration checks;
- `.inc` no-growth check for app / owner names.

This is not a heavy repeated benchmark. It is the first execution contract for
the later C mimalloc comparison ledger.
