# Phase 21.5 — Optimization Readiness (MirBuilder + Runtime)

Goal
- Target: ≥ 80% of equivalent C for core micro‑benches（同一アルゴリズム、最適化ON）。
- Scope: Nyash AOT/LLVM ライン（ハーネス or EXE）、ホットパスの構造最適化（挙動不変）。
- Guard: 既定挙動/ABI不変、devトグルでの可視化と段階導入、ロールバック容易。

Readiness (done)
- Normalizer を MirBuilder 本線/Min に既定OFFで配線、ON時のみタグ（静音トグル付）。
- f64 正規化の共通化（JsonNumberCanonical）・phi整列・ret値明示・const重複排除。
- Canary: 冪等 / cross‑block no‑dedupe / rcパリティ（if/loop/binop）/ f64（指数/−0.0/末尾0）/ phi（many/nested）。

Plan
1) Baseline & Harness
   - 代表ベンチ（LoopSum / StrLen / BoxAlloc）で C baseline と Nyash(LLVM) を比較。
   - 計測を tools/perf/microbench.sh に集約。`--runs N` で中央値/平均を算出。
2) Hotspot Discovery
   - Builder 出力の冗長箇所（const/compare連打、不要な一時値）、Runtime の alloc/型分岐を観測。
   - 代表ケースで alloc 回数/branch 回数をロギング（devトグル下、既定OFF）。
3) Small-step Optimization（構造優先・挙動不変）
   - Builder 側: テキスト結合の抑制（集約バッファ）、重複constの先頭集約（既に導入済）。
   - Runtime 側: 軽量 fast‑path（整数演算/比較の直通）、Box 生存期間短縮（スコープ明確化）。
4) Acceptance
   - 代表ベンチ3種で C の ≥80%（中央値）を達成。変動が大きい環境では ±10%を許容しつつCIはスキップ扱い。

How to run (examples)
- Quick with Normalizer ON（重いEXEありのため timeout 120 推奨）:
  - `HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 ./tools/smokes/v2/run.sh --profile quick --timeout 120`
- Microbench（C vs Nyash/LLVM ハーネス）:
  - `tools/perf/microbench.sh --case loop --n 5000000 --runs 5`
  - `tools/perf/microbench.sh --case strlen --n 2000000 --runs 5`
  - `tools/perf/microbench.sh --case box --n 100000 --runs 5`
 - VM runtime counters（dev 診断、既定OFF）:
   - `NYASH_VM_STATS=1 ./target/release/hakorune --backend vm apps/tests/CASE.hako` → `[vm/stats] inst=… compare=… branch=…` を出力

Toggles
- `HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1` — 正規化ON（既定OFF）
- `HAKO_MIR_BUILDER_NORMALIZE_TAG=1` — タグ出力（既定静音）
- `SMOKES_DEV_NORMALIZE=1` — devヘルパから正規化ON注入（quick限定ON例は test_runner にコメント記載）

Notes
- ベンチは OS/CPU/周辺負荷に依存するため、CIでの厳格判定は避け、ローカル/任意ジョブで傾向を確認。
- 構造最適化優先（条件分岐/インライン化での挙動差を避ける）。
