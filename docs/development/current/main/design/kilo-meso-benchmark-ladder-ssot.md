---
Status: SSOT
Decision: accepted
Date: 2026-03-28
Scope: `kilo_micro_*` と `kilo_kernel_small_hk` の間にある観測穴を埋めるため、string/edit workload の meso benchmark ladder を固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - docs/development/current/main/design/transient-text-pieces-ssot.md
  - tools/perf/run_kilo_micro_machine_ladder.sh
  - tools/perf/bench_micro_c_vs_aot_stat.sh
  - benchmarks/bench_kilo_micro_substring_concat.hako
  - benchmarks/bench_kilo_kernel_small.hako
---

# Kilo Meso Benchmark Ladder SSOT

## Goal

`kilo_micro_*` がほぼ parity まで詰まっている一方で、`kilo_kernel_small_hk` はまだ大きく離れている。

この差を「leaf 単体の遅さ」ではなく、`store / retained boundary / loop-carry` のどこで急に開くかとして観測するために、
`micro` と `kilo` の間へ **meso benchmark ladder** を置く。

## Core Rule

- meso benchmark は `benchmark` 名ベースの最適化 owner ではなく、`recipe family` の観測穴を埋めるための proof lane だよ
- 各 case は **1 段だけ** complexity を増やす
- `.hako` と `C` を同じ workload shape で揃える
- 新しい runtime route / token / policy は増やさない

## Cases

### 1. `kilo_meso_substring_concat_len`

目的:
- `substring + concat + len` だけを見る
- `StringBox birth` と pure read-only observer の合成コストを測る

含めるもの:
- `substring`
- `concat`
- `length`

含めないもの:
- `ArrayBox.set`
- loop-carried retained string

### 2. `kilo_meso_substring_concat_array_set`

目的:
- `substring + concat` に `ArrayBox.set` を足して、store boundary の cost を分離する

含めるもの:
- `substring`
- `concat`
- `length`
- `ArrayBox.get/set`

含めないもの:
- loop-carried `text = out.substring(...)`

### 3. `kilo_meso_substring_concat_array_set_loopcarry`

目的:
- store 後に `substring` backedge を戻して、mainline に近い retained/freeze density を測る

含めるもの:
- `substring`
- `concat`
- `ArrayBox.get/set`
- loop-carried `substring` backedge

含めないもの:
- `indexOf("line")` branch loop
- branch-loop wide row scan

## Reading Rule

この ladder で見るのは 3 段だけだよ。

1. `micro -> meso_len`
   - pure text recipe が合成されるとどれだけ開くか
2. `meso_len -> meso_array_set`
   - store boundary がどれだけ開くか
3. `meso_array_set -> meso_loopcarry`
   - retained / freeze / backedge がどれだけ開くか

もし gap が

- `meso_len` で急に開くなら: `concat_hs` / `concat3_hhh` / `StringBox birth`
- `meso_array_set` で急に開くなら: `array_set_by_index_string_handle_value`
- `meso_loopcarry` で急に開くなら: retained boundary / freeze density

を次の exact leaf として選ぶ。

## Scripts

日常入口は次の 2 本に固定する。

```bash
bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
bash tools/perf/run_kilo_meso_machine_ladder.sh 1 3
```

各 case の C/AOT 比較は既存の stat runner を使う。

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_len 1 3
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set 1 3
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3
```

## Non-Goals

- `kilo_kernel_small_hk` の branch-loop 全体を meso に持ち込むこと
- `Python` parity lane を meso に追加すること
- benchmark-specific optimization route を増やすこと
- `micro` script の meaning を変えること

## Acceptance

- 3 本の `.hako` / `C` benchmark が同じ shape で存在する
- `tools/perf/run_kilo_meso_machine_ladder.sh` で 3 本まとめて回せる
- `kilo_micro_*` と `kilo_kernel_small_hk` の間にある gap を、`len/store/loopcarry` の 3 段で読める

## First Reading (2026-03-28, `warmup=1 repeat=3`)

```text
kilo_meso_substring_concat_len                    c_ms=3  ny_aot_ms=37
kilo_meso_substring_concat_array_set              c_ms=3  ny_aot_ms=69
kilo_meso_substring_concat_array_set_loopcarry    c_ms=3  ny_aot_ms=69
```

判断:
- first large jump is `len -> array_set`
- `loopcarry` is not the first dominating boundary in this ladder reading
- the next exact leaf should stay on string store / `array_set_by_index_string_handle_value` before reopening loop-carry shaping
