# 293x-1050 MIMAP-428A Allocator Comparison Baseline Diagnostics

Status: selected current
Date: 2026-05-21

## Purpose

Add diagnostics for missing allocator comparison baseline inputs after
MIMAP-427A. This row should observe missing C mimalloc baseline, missing
hako_alloc baseline, missing throughput target, missing memory-usage target,
missing workload matrix, and invalid repeat count.

## Scope

- Consume the MIMAP-427A comparison baseline inventory report.
- Summarize missing comparison inputs.
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
