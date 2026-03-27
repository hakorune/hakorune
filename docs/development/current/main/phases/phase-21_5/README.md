---
Status: Parked
Decision: accepted
Date: 2026-03-21
Scope: `phase21_5_perf_kilo_text_concat_contract_vm.sh` が残していた `nyash.any.length_h` residual route は、`ArrayBox` string-element propagation と boxcall set-route alignment で解消済み。daily portability gate は green に戻したので、この phase は regression pin として parked 扱いにする。
Route note: `bench_kilo_kernel_small.hako` の route pin は direct emit を canonical source owner とする。`hako-helper` / `hako-mainline` Stage1 emit は bootstrap/reduced route のまま監査対象から外す。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/chip8/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/entry_mode/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/mir_mode/README.md
  - src/llvm_py/cfg/utils.py
  - src/llvm_py/tests/test_cfg_stringish_arraybox.py
  - tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# Phase 21.5: Kilo any.length_h Residual Route Narrowing

## Goal

- `ArrayBox` の string-element 事実を `collect_stringish_value_ids(...)` が追えるようにする。
- `phase21_5_perf_kilo_text_concat_contract_vm.sh` の `main` IR から generic `nyash.any.length_h` を消す。
- `dev_gate portability` を green に戻す。

## Non-Goals

- macOS FFI candidate resolution の再変更
- smoke taxonomy / suite manifest の再設計
- p8 smoke split の再開
- runtime-wide string semantics の拡張

## Fixed Order

1. `src/llvm_py/cfg/utils.py` で `ArrayBox` の string-element 伝播を `RuntimeDataBox` と同じ契約へ寄せる。
2. 回帰テストで `ArrayBox.push/get` の stringish propagation を固定する。
3. `phase21_5_perf_kilo_text_concat_contract_vm.sh` を再実行して `nyash.any.length_h` 不在を確認する。
4. `tools/checks/dev_gate.sh portability` を green に戻す。
5. そのあとで `phase21_5/perf/apps/{entry_mode,mir_mode,case_breakdown,compile_run_split,crosslang_bundle,emit_mir_jsonfile_route,startup_subtract}` を前提に、bundle root の singleton 残差をなくす。

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_cfg_stringish_arraybox`
2. `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh`
3. `bash tools/checks/dev_gate.sh portability`
4. `git diff --check`

## Next

1. Keep the residual route slice parked unless the `any.length_h` route regresses again.
2. `phase21_5/perf/{chip8,kilo}` と `phase21_5/perf/apps/{entry_mode,mir_mode,case_breakdown,compile_run_split,crosslang_bundle,emit_mir_jsonfile_route,startup_subtract}` は別 family として landed 済みなので、bundle root には live smoke を増やさない。
