---
Status: SSOT
Date: 2026-05-27
Scope: task order for making the `.hako` mimalloc port comparable with C mimalloc performance.
Related:
  - docs/development/current/main/phases/phase-296x/README.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# Hako Mimalloc Performance Parity

## Goal

Make the `.hako` mimalloc port measurable against C mimalloc on the same
workloads, then use the evidence to close the performance gap without hiding
the owner of each cost.

The target is performance parity evidence, not an allocator product decision.

## Non-goals

Keep these out of this lane unless a later decision row explicitly opens them:

- process-wide allocator replacement;
- provider activation;
- hooks or `#[global_allocator]`;
- C ABI `malloc/free` replacement as the main result;
- hakozuna versus mimalloc product selection;
- winner claims before the parity gates are stable.

`hakmem` LD_PRELOAD compatibility is a planned bridge, not part of the first
parity measurement. It exports process allocator symbols and therefore must
stay behind a separate decision row.

## Subject Model

Use stable subject ids so results can be compared across adapters:

```text
hako_mimalloc_exact_exe
c_mimalloc_explicit_runner
hakozuna_reference
provider_package_hako_mimalloc_explicit
```

`hakozuna_reference` is a reference subject only. It must not change the goal of
this lane from `.hako` mimalloc parity to allocator-product selection.

## Workload Ladder

Promote workloads in this order:

1. representative small block alloc/free;
2. realloc and aligned allocation;
3. remote-free publish/collect;
4. mixed small allocation;
5. large allocation / huge backing;
6. page-source reserve/commit/decommit;
7. selected external `hakmem` workload family.

## Hakmem Integration Ladder

Use the external corpus at:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

The integration order is:

```text
Stage A:
  adapt existing hakmem results and workload identities into Hakorune benchmark
  contracts; do not execute provider or replacement code.

Stage B:
  run equivalent workloads through explicit provider API calls; keep
  replacement_active=0.

Stage C:
  add an optional LD_PRELOAD-compatible shim that exports malloc/free-family
  symbols and forwards to a selected `.hako` mimalloc provider.
```

Stage C is useful for reusing hakmem's existing allocator benchmark scripts,
but it is not the same safety tier as explicit provider calls.

Required Stage C stop line:

```text
ld_preload_compatible=1
explicit_decision_row=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

The first Stage C row may build and smoke-test the shim only. It must not claim
that Hakorune has replaced the host allocator for normal execution.

Each workload must have:

```text
workload_id
operation_family
operation_repeat
sample_count
warmup_count
summary_statistic=min,median,max
winner_claim=0
```

## Gap Taxonomy

Every optimization row must first classify the current gap into exactly one
primary owner:

```text
allocator_algorithm
compiler_lowering
c_abi_memory_bridge
osvm_page_source
provider_wrapper
benchmark_harness
```

Do not optimize by guesswork. Use the owner-first perf method before code
changes:

```text
tools/checks/dev_gate.sh quick
tools/perf/bench_micro_c_vs_aot_stat.sh ...
tools/perf/bench_micro_aot_asm.sh ...
```

The row may read source only after the hot symbol / hot block is known.

## Promotion Gates

A workload can move from probe to keeper only when the row records:

```text
same_workload=1
same_operation_count=1
same_sample_policy=1
subject_count>=2
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
gap_owner=<one primary owner>
```

If the gap owner is `allocator_algorithm`, the fix belongs in `.hako` mimalloc.
If the owner is `compiler_lowering`, the fix belongs in the compiler lane and
must not be hidden in allocator source. If the owner is `benchmark_harness`,
fix the measurement first.

## Selfhost Handoff Gate

The lane is ready to point back toward selfhosting only after:

```text
small_block_parity=accepted
remote_free_parity=accepted
mixed_small_parity=accepted
large_or_page_source_gap_classified=1
no_unclassified_hot_gap=1
winner_claim=0
replacement_active=0
```

At that point, the next selfhost task can use the `.hako` allocator evidence as
a runtime/allocator confidence baseline without requiring hakozuna or provider
activation to be decided.
