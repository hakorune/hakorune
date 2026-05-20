# 293x-1053 MIMAP-431A Allocator Comparison Workload Matrix Diagnostics

Status: selected current
Date: 2026-05-21

## Purpose

Add diagnostics for missing allocator comparison workload matrix inputs after
MIMAP-430A. This row should observe missing small allocation, small free,
realloc, huge allocation, throughput, memory-usage, and invalid workload-count
families.

## Scope

- Consume the MIMAP-430A workload matrix inventory report.
- Summarize missing workload matrix inputs.
- Keep benchmark execution and process replacement closed.

## Stop Lines

- No benchmark execution.
- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```
