# Phase 131-15: print/concat segfault (LLVM harness)

Status: Active  
Scope: Case C が “ループ自体は動くが、print/concat で segfault” する問題の切り分け。  
Related:
- SSOT (LLVM棚卸し): `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- ENV: `docs/reference/environment-variables.md`（`NYASH_LLVM_DUMP_IR`, `NYASH_LLVM_STRICT`, `NYASH_LLVM_TRACE_*`）

## 状態

- ループ本体（counter 更新/比較/break）は進む（例: `1,2` までは観測できる）。
- `print("Result: " + counter)` のような “concat + print” 経路で segfault。

## 切り分け（最優先）

同じ backend（LLVM harness）で、以下の3つを順に試す:

1. `return counter`（print なし）
2. `print(counter)`（concat なし）
3. `print("Result: " + counter)`（concat + print）

狙い:
- 1 と 2 が通って 3 だけ落ちるなら concat/coercion が本命。
- 2 で落ちるなら console 呼び出し ABI / routing が本命。

## 典型原因クラス

- **ABI routing の誤り**: `nyash.console.log(i8*)` に i64(handle/int) を渡している、またはその逆。
- **String coercion の誤り**: `to_i8p_h` / `concat_*` の引数が “handle と integer” で混線している。
- **dst_type hint の解釈違い**: MIR JSON の `dst_type` が Python 側で誤った関数選択に使われている。

## 観測手順

- LLVM IR dump:
  - `NYASH_LLVM_DUMP_IR=/tmp/case_c.ll tools/build_llvm.sh apps/tests/llvm_stage3_loop_only.hako -o /tmp/case_c`
- strict + traces:
  - `NYASH_LLVM_STRICT=1 NYASH_LLVM_TRACE_VALUES=1 NYASH_LLVM_TRACE_PHI=1 NYASH_LLVM_TRACE_OUT=/tmp/case_c.trace ...`

IR で見るポイント:
- concat 直前の値の型/変換:
  - `nyash.string.to_i8p_h(i64 <arg>)` の `<arg>` が StringBox handle か（整数を渡していないか）
  - `nyash.string.concat_*` の引数型が意図と一致しているか
- print の呼び先:
  - 文字列は `nyash.console.log(i8*)` か（または handle→string 変換済み）
  - handle/int は `nyash.console.log_handle(i64)` か

## Done 条件

- 3つの切り分けケースが VM/LLVM で一致し、segfault が消える。
- `NYASH_LLVM_STRICT=1` でフォールバック無し（miss→0 など）が維持される。

