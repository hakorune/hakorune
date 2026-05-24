---
Status: Landed
Date: 2026-05-25
Scope: add a target-local bridge for the extracted hakmem mimalloc-bench corpus.
Related:
  - tools/allocator/hakmem_external_bench.py
  - /home/tomoaki/git/hakmem_20260525_extracted/hakmem
---

# 295x-82 Hakmem External Bench Bridge

## Blocker

```text
MIMALLOC-COMPARISON-HAKMEM-EXTERNAL-BENCH-BRIDGE-295X-001
```

## Decision

The extracted `hakmem_20260525` corpus may be used as an external benchmark
alignment source for phase-295x, but it is not direct phase evidence yet.

The bridge copies runnable `mimalloc-bench` executables into:

```text
target/hakmem-bench/
```

and leaves the repository clean of tracked benchmark binaries and mutable
`benchres.csv` output.

## Tool

List the supported bridge inputs:

```bash
tools/allocator/hakmem_external_bench.py --list
```

Prepare the target-local executable copy without running a benchmark:

```bash
tools/allocator/hakmem_external_bench.py --prepare-only
```

Run a small smoke benchmark:

```bash
tools/allocator/hakmem_external_bench.py \
  --bench cfrac \
  --allocator sys \
  --allocator mimalloc \
  --out target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

The mutable `mimalloc-bench` output remains:

```text
target/hakmem-bench/out/bench/benchres.csv
```

The copied snapshot is the `--out` path under:

```text
target/hakmem-bench/results/
```

The tool emits:

```text
output_contract=hakmem-external-bench-bridge-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
```

## Use

Allowed:

- copy extracted `mimalloc-bench` executables into `target/hakmem-bench`;
- run selected external benchmark workloads locally;
- overwrite `target/hakmem-bench/out/bench/benchres.csv`;
- copy selected result snapshots under `target/hakmem-bench/results`;
- use results for workload vocabulary and schema-adapter design.

Not yet allowed:

- import historical `benchres.csv` as current phase repeated measurement
  evidence;
- compute speed or memory winners;
- activate provider/DLL/replacement/hook/global allocator paths;
- treat external LD_PRELOAD benchmark execution as Hakorune runtime behavior.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-HAKMEM-SCHEMA-ADAPTER-SELECTION-295X-001
```

The next row should choose whether to add a catalog-only adapter for existing
`benchres.csv` / `hakozuna_compare_*.log` files, or to continue `.hako`
workload porting first.

## Verification

```bash
python3 -m py_compile tools/allocator/hakmem_external_bench.py
tools/allocator/hakmem_external_bench.py --list
tools/allocator/hakmem_external_bench.py --prepare-only
tools/allocator/hakmem_external_bench.py --bench cfrac --allocator sys --allocator mimalloc --out target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
