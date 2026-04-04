# Examples Quick Start (Minimal)

This is the archived copy of the top-level examples quick-start page.

The original page documented a small set of compat/proof VM example commands
for the legacy selfhost route. That surface is no longer a day-to-day owner.

Historical notes:
- `--backend vm` in these examples is explicit compat/proof keep
- the examples directory itself remains as sample payloads and fixtures
- use current guides for direct/core day-to-day routes

Original structure:

## 1) HH直実行（Map.get_hh）
- 目的: 受け手/キーが関数パラメータのとき、JIT HostCallを許可（HH経路）。
- 実行:
```bash
NYASH_JIT_EXEC=1 NYASH_JIT_THRESHOLD=1 NYASH_JIT_HOSTCALL=1 NYASH_JIT_EVENTS=1 \
  ./target/release/hakorune --backend vm examples/jit_map_get_param_hh.hako  # explicit compat/proof keep
```
- 期待: `allow id: nyash.map.get_hh` イベントが出る。戻り値は `value1`。

## 2) mutating opt-in（JitPolicyBox）
- 目的: 既定 read_only。必要最小の書き込みだけホワイトリストで許可。
- 実行:
```bash
NYASH_JIT_EXEC=1 NYASH_JIT_THRESHOLD=1 NYASH_JIT_HOSTCALL=1 NYASH_JIT_EVENTS=1 \
  ./target/release/hakorune --backend vm examples/jit_policy_optin_mutating.hako  # explicit compat/proof keep
```
- 期待: 1回目は `policy_denied_mutating` でfallback、whitelist後の2回目はallow。

## 3) CountingGc デモ
- 目的: GCのカウント/トレース/バリア観測の導線確認（VM経路）。
- 実行:
```bash
  ./target/release/hakorune --backend vm examples/gc_counting_demo.hako  # explicit compat/proof keep
```
- Tips: 詳細ログは `NYASH_GC_COUNTING=1 NYASH_GC_TRACE=2` を併用。

## 4) Policy whitelist（events分離）
- 目的: read_only下でのfallback→allow（whitelist）と、compile/runtimeのphase分離をイベントで確認。
- 実行（しきい値=1を明示／またはDebugConfigBoxでapply後にRunnerが自動設定）:
```bash
NYASH_JIT_THRESHOLD=1 NYASH_JIT_HOSTCALL=1 \
  ./target/release/hakorune --backend vm examples/jit_policy_whitelist_demo.hako  # explicit compat/proof keep
```
- 期待: `policy_events.jsonl` に `phase:"lower"`（計画）と `phase:"execute"`（実績）が出る。

---

## 5) AOT最小手順（--compile-native）
- 目的: Craneliftでオブジェクトを生成し、`libnyrt` とリンクしてEXE化。
- 事前: `cargo build --release --features cranelift-jit`
- 実行例（String/Integer/Consoleの最小）:
```bash
./target/release/hakorune --compile-native examples/aot_min_string_len.hako -o app && ./app
# 結果は `Result: <val>` として標準出力に表示
```
- Python最小チェーン（RO）:
```bash
./target/release/hakorune --compile-native examples/aot_py_min_chain.hako -o app && ./app
```
- スクリプト版（詳細な手順）: `tools/build_aot.sh <file> -o <out>`（Windowsは `tools/build_aot.ps1`）

## 6) Scheduler（Phase 10.6b 準備）
- 目的: 協調スケジューラのSafepoint連携を観測
- 実行（デモ）:
```bash
NYASH_SCHED_DEMO=1 NYASH_SCHED_POLL_BUDGET=2 \
  ./target/release/hakorune --backend vm examples/scheduler_demo.hako  # explicit compat/proof keep
```
- 期待: `[SCHED] immediate task ran at safepoint` と `[SCHED] delayed task ran at safepoint` が出力
