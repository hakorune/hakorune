---
Status: SSOT
Decision: provisional
Date: 2026-03-25
Scope: `stage2` String hot path の next implementation wave を、`search/slice` と `concat` に分けて decision-complete に固定する。
Related:
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - src/llvm_py/instructions/mir_call/method_call.py
  - src/llvm_py/instructions/mir_call/string_console_method_call.py
  - src/llvm_py/instructions/stringbox.py
  - src/llvm_py/instructions/binop.py
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/plugin/string.rs
  - tools/smokes/v2/suites/integration/phase29ck-boundary.txt
---

# Stage2 String Route Split Plan

## Goal

- `String route split` を 1 bucket ではなく 2 wave に分け、1 commit 単位で安全に進められる粒度へ落とす。
- `ny-llvm` / `ny-llvmc` を mainline judge に固定したまま、`llvmlite` は shared-contract keep としてだけ追従させる。
- `String` wave の途中で `HostFacade/provider/plugin` や allocator policy/state を混ぜない。

## Fixed Reading

- `StringCoreBox` の observer role はすでに fixed だよ。
- 次に薄くする対象は、AOT route tables と string leaf dispatch のみ。
- `llvmlite` は keep lane なので、次 wave で守るのは shared MIR / ABI / observer / fallback contract だけだよ。
- `FastLeafManifest` V0 はまだ `String observer` だけを含み、`search/slice/concat` は consumer でも row 対象でもない。

## Wave S1: Search/Slice Route Split

Status: landed, with a follow-up boundary-default pure-first repair landed on the `phase29ck_boundary` mainline acceptance lane.

### Exact target

- target owners:
  - `src/llvm_py/instructions/mir_call/method_call.py`
  - `src/llvm_py/instructions/mir_call/string_console_method_call.py`
  - `src/llvm_py/instructions/stringbox.py` (only if the shared route contract cannot be thinned without it)
- target operations:
  - `substring`
  - `indexOf`
  - `lastIndexOf`

### Out of scope

- `src/llvm_py/instructions/binop.py`
- `concat_hh`
- `concat3_hhh`
- `crates/nyash_kernel/src/exports/string.rs` concat planning/materialization
- `crates/nyash_kernel/src/plugin/string.rs`
- `HostFacade/provider/plugin loader`
- allocator / handle / barrier policy

### Acceptance

- `ny-llvm` mainline acceptance:
  - `bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_string_indexof_min.sh`
  - `bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_indexof_line_min.sh`
- keep-lane acceptance:
  - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_string_console_method_call`
  - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_strlen_fast`
  - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_mir_call_intrinsic_registry`
- regression-only:
  - `bash tools/smokes/v2/profiles/integration/string/index_substring_vm.sh`

## Wave S2: Concat Route Split

Status: landed.

### Exact target

- target owners:
  - `src/llvm_py/instructions/binop.py`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/string.rs` (cold keep only)
- target operations:
  - `concat_hh`
  - `concat3_hhh`
  - string `+` lowering monomorphic route

### Out of scope

- `substring`
- `indexOf`
- `lastIndexOf`
- `HostFacade/provider/plugin loader`
- allocator policy/state

### Acceptance

- `ny-llvm` mainline acceptance:
  - `bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_substring_concat_loop_min.sh`
- keep-lane acceptance:
  - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_binop_concat_helpers`
  - `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_binop_string_partial_tag`
- perf acceptance:
  - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
  - `bash tools/perf/microbench.sh --case stringchain`
  - `bash tools/perf/run_kilo_micro_machine_ladder.sh`

## Stop Line

- `S1 search/slice` と `S2 concat` を同じ patch series に混ぜない。
- `phase29ck boundary default pure-first` repair is already landed; do not reopen recipe/symbol transport in `S2` unless a fresh exact blocker proves it.
- `S2 concat` と `cold dynamic lane split` は landed 済みで、next exact bucket is `hako_alloc policy/state contract`.
- `String` wave を reopen するときは fresh blocker/evidence がある場合だけにする。
- `llvmlite` keep lane の都合で `ny-llvm` hot-path route を重くしない。
- `FastLeafManifest` V0 はこの 2 wave では widen しない。widen 判断は allocator/state stop-line が固まった後に別 wave で行う。
