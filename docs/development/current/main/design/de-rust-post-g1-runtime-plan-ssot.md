---
Status: Active (accepted)
Decision: accepted
Scope: compiler selfhost（G1）達成後に、runtime de-Rust と VM/LLVM 最適化を迷走なく進めるための実行順序と受け入れ基準を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/phases/phase-29y/README.md
---

# De-Rust Post-G1 Runtime Plan (SSOT)

## 0. Conclusion

- Yes: 順序は `D4 docs同期 -> runtime de-rust (29y.1) -> source-zero cutover -> .hako VM 本体移行` で正しい。
- ただし最小差分として、runtime de-rust 着手前に 1 タスクを挟む。
  - D5-min1: Selfhost Stage-A runtime route の Program->MIR（Rust `json_v0_bridge`）境界を切り、MIR-first 契約へ固定する。

## 1. Execution Order (fixed)

1. D4 closeout（docs 証跡の粒度統一）
2. D5-min1（Stage-A runtime route の MIR-first 化 + fail-fast）
3. Phase 29y.1（ABI shim -> RC insertion minimal -> observability）
4. .hako VM dual-run parity（S0 subset から開始）
5. runtime/plugin source-zero（compat default-off 固定 + Rust source撤去）
6. VM/LLVM optimization（verify 可能な局所最適化から）
7. Optional GC/cycle collector（意味論非変更を前提に最後に実施）

## 1.5 GC policy lock (2026-02-13)

- lifecycle の意味論は GC 必須ではない（`fini` は決定的、物理解放は実装責務）。
- GC は最適化/診断補助の位置づけで、ON/OFF で意味論を変えない。
- 逆参照は weak 推奨。強循環は no-cycle-collector ではリークし得る（許容挙動）。
- Consolidated SSOT: `docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md`

## 1.6 ABI boundary lock (2026-02-13)

- Canonical ABI lanes are fixed to two:
  1. Core C ABI (NyRT runtime boundary)
  2. TypeBox ABI v2 (plugin method boundary)
- `hako_abi_v1` は設計ドラフト扱い（非canonical）として凍結し、第三の意味論ABIにはしない。
- Boundary SSOT:
  - `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
  - `docs/reference/abi/ABI_INDEX.md`

## 2. D5-min1 Contract (must)

- Stage1-first 運用では Stage-A runtime route は MIR(JSON v0) のみ受理する。
- Program(JSON v0) 入力は strict/dev(+planner_required) で fail-fast。
- 診断は 1 行タグ固定（例: `[contract][runtime-route][expected=mir-json]`）。
- 受け入れ:
  - 正常系: MIR(JSON v0) 実行が green
  - 異常系: Program(JSON v0) 入力が確実に freeze/fail

## 3. Top Risks and Fail-Fast Acceptance

### Risk-1: stage1 route parity drift（env-min / subcmd）
- Acceptance:
  - stage1 contract smoke を daily 必須に固定
  - `--cli-mode auto` は compat-only（full evidence では禁止）

### Risk-2: Program/MIR ownership mixing（compiler vs runtime）
- Acceptance:
  - D5-min1 を通し、Stage-A runtime route の Program->MIR 変換を runtime lane から排除
  - 未対応入力は silent fallback せず fail-fast

### Risk-3: pass bug が後段で発火（原因追跡距離が長い）
- Acceptance:
  - pass 境界ごとに verify（strict/dev では必須）
  - verifier failure は救済せず、タグ付きで即停止

## 4. Dual-Run Parity Start Scope (S0)

- 進捗（2026-02-13）:
  - 実装済み: `Const(i64/bool)`, `BinOp(add/sub/mul/div/mod)`, `Compare(eq/lt)`, `Branch/Jump`, `UnaryOp(neg/not)`, `Copy`, `Nop`, `Safepoint`, `KeepAlive`, `ReleaseStrong`, `Return`
  - 限定実装: `NewBox(ArrayBox/StringBox/Main)`, `BoxCall(birth/id0/id1-int direct)`, `Call(id0/id1-int bridge; optimize-on固定)`, `ExternCall(print 1-reg; compare-origin bool は 1/0 契約)`
  - compat withdrawal: Rust側 canonicalizer / subset-check の `mir_call(print)` 受理は全モード撤去済み（`mir_call(legacy-removed)` fail-fast、`.hako vm` 実行は externcall-only）。
- 残り（S0完了まで）:
  - なし（S0 完了）
- 粒度SSOT:
  - 詳細キューは `de-rust-compiler-thin-rust-roadmap-ssot.md` の `D6 Queue` を参照（`D6-min44` / `RCL-3` / `NCL-2` まで完了、以後は failure-driven 運用）
- 除外:
  - Box/heap/RC/weak/thread/atomic は S0 に入れない
- Gate:
  - 同一 MIR を Rust VM と .hako VM で実行し、stdout + exit code を比較
- Fallback policy:
  - strict/dev(+planner_required) では Rust fallback 禁止（未実装 op は即 fail-fast）
- Compat retirement policy:
  - `.hako vm` の `mir_call(print)` 実行分岐は撤去済み。Rust側 canonicalizer / subset-check でも `mir_call(print)` 受理は全モード撤去済み（`legacy-removed` fail-fast）。

## 5. Optimization Order

- 先にやる（移行中でも安全）:
  - const fold, dead code elimination, cfg simplification, limited copy-prop
- 後でやる（runtime parity 安定後）:
  - inline, loop transform（LICM/unroll/vectorize）, scheduling/register 依存最適化

## 6. Done Definitions

### 6.1 Compiler selfhost done
- Stage1 route only で identity full が継続 green
- stage0 は compat-only（明示指定時のみ）

### 6.2 Runtime parity done
- 同一 MIR 入力で Rust VM / LLVM / .hako VM の観測結果が一致
- ABI/RC/observability の Phase29y smoke が継続 green

### 6.3 Source-zero done
- runtime/plugin の Rust 実装 source が撤去済み
- daily/CI/selfhost 主経路が `.hako + ABI` のみで回る
- compat fallback は既定OFF（明示時のみ）
- Core C ABI / TypeBox ABI v2 の 2 面契約が継続 green

## 7. Two-Week Plan (2026-02-11 start, provisional)

1. Day 1: D4-min24（CURRENT_TASK Daily Checkpoint に identity full 証跡を同期）
2. Day 2: D5-min1（Stage-A runtime route MIR-first + fail-fast gate 2 本）
3. Day 3: parity harness v0（Rust VM vs LLVM, fixture 3 本）
4. Day 4: flake cleanup / 診断タグ固定
5. Day 5: buffer（gate 不安定点の修復）
6. Day 6: Phase29y.1 Task1（NyRT handle ABI shim + smoke）
7. Day 7: ABI conformance smoke（borrowed args / owned return 1 本）
8. Day 8: Phase29y.1 Task2（RC insertion pass 接続 no-op）
9. Day 9: RC insertion minimal（overwrite-release 1 ケース）
10. Day 10: Phase29y.1 Task3（observability handles）
11. Day 11: observability category 拡張（locals または temps 1 つ）
12. Day 12: buffer（回帰修復）
13. Day 13: .hako VM backend 枠導入（未実装は fail-fast）
14. Day 14: vm-hako S0 最小実装 + parity gate S0

## 8. Rust maintenance after selfhost (portability lane)

- 目的:
  - `.hako` 主導へ移行後も、Rust を「ランナー/ABI/実行基盤」の保守層として健全に維持する。
  - Linux だけでなく Windows / macOS の退行を早期に検知する。
- 境界:
  - 仕様決定と高レベル最適化は引き続き `.hako` 側が主担当。
  - Rust 側は bridge/runner/ABI/linker 導線の保守と fail-fast 契約を担当する。

### 8.1 Windows lane (WSL -> cmd.exe)

- 既定運用は WSL cross build + Windows cmd smoke。
- 週次の推奨コマンド:
  - `bash tools/checks/windows_wsl_cmd_smoke.sh --build --cmd-smoke`
- 日常 preflight:
  - `bash tools/checks/windows_wsl_cmd_smoke.sh`
- 契約:
  - WSL から `cmd.exe` を呼び出して `hakorune.exe --help` が成功すること。
  - `backtrace-on-stack-overflow` は `cfg(not(windows))` 依存とし、Windows target では無効化する（cross build fail 防止）。

### 8.2 macOS lane (no hardware yet)

- mac 実機がない間は「事前破壊防止」を目的に静的ガードを回す。
- 週次の推奨コマンド:
  - `bash tools/checks/macos_portability_guard.sh`
- 契約:
  - plugin loader が `.dylib` 候補を維持する。
  - LLVM C-ABI FFI ローダが `.dylib` 候補を維持する。
  - FFI ビルド導線が Darwin で `.dylib` を生成できる状態を維持する。

### 8.3 Dev gate entry

- 導線:
  - `tools/checks/dev_gate.sh portability`
- 注意:
  - `portability` は軽量導線（既定 preflight）で、週次の build/smoke は 8.1 の明示コマンドで実行する。
