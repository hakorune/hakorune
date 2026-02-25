# Phase 194: JoinLoopTrace / Debug Integration

**Status**: In Progress（trace.rs は実装済み、呼び出し置き換えと docs 反映が残り）
**Date**: 2025-12-06

## Goal

JoinIR / ループ周辺のデバッグ出力を `JoinLoopTrace` に集約し、環境変数ベースの制御と `logging_policy.md` の方針に沿った形に整理する。

---

## Implemented Infrastructure

### JoinLoopTrace モジュール

- File: `src/mir/builder/control_flow/joinir/trace.rs`
- 役割:
  - JoinIR ループまわりの `eprintln!` を 1 箱に集約する。
  - 環境変数からフラグを読み取り、どのカテゴリのトレースを出すかを制御する。
- 対応環境変数:
  - `NYASH_TRACE_VARMAP=1` – variable_map トレース（var → ValueId）
  - `NYASH_JOINIR_DEBUG=1` – JoinIR 全般のデバッグ（パターン routing、merge 統計など）
  - `NYASH_OPTION_C_DEBUG=1` – PHI 生成（Option C）周りのトレース
  - `NYASH_JOINIR_MAINLINE_DEBUG=1` – mainline routing（代表関数の routing）トレース
  - `NYASH_LOOPFORM_DEBUG=1` – LoopForm 関連トレース（レガシー互換）

### 主なメソッド

- `pattern(tag, pattern_name, matched)` – パターン検出・選択
- `varmap(tag, &BTreeMap<String, ValueId>)` – variable_map の状態
- `joinir_stats(tag, func_count, block_count)` – JoinIR モジュールの統計
- `phi(tag, msg)` – PHI 関連の操作
- `merge(tag, msg)` – JoinIR→MIR マージ進行
- `exit_phi(tag, var_name, old_id, new_id)` – Exit PHI 接続
- `debug(tag, msg)` / `routing(tag, func_name, msg)` / `blocks(tag, msg)` / `instructions(tag, msg)` – 汎用デバッグ

グローバルアクセサ:

```rust
pub fn trace() -> &'static JoinLoopTrace
```

でどこからでも呼び出せる。

---

## Remaining Work (Phase 194)

### Task 194-3: 生 `eprintln!` の JoinLoopTrace 置き換え

対象（例）:

- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`
  - まだ 8 箇所程度 `eprintln!` ベースのログが残っている。
  - ここを順次:
    - varmap ログ → `trace().varmap(...)`
    - パターン / ルーティング系 → `trace().pattern(...)` / `trace().routing(...)`
    - それ以外の debug → `trace().debug(...)`
  に差し替える。

同様に、他の JoinIR / loop 関連ファイルに散在している `[joinir/...]` 系 `eprintln!` も、必要に応じて `trace.rs` 経由に寄せる。

### Task 194-4: logging_policy.md 反映

- File: `docs/development/current/main/logging_policy.md`
- 追記内容:
  - JoinIR / ループ系のトレースカテゴリを 1 セクションにまとめる：
    - varmap（NYASH_TRACE_VARMAP）
    - joinir-debug（NYASH_JOINIR_DEBUG）
    - phi-debug（NYASH_OPTION_C_DEBUG）
    - mainline-debug（NYASH_JOINIR_MAINLINE_DEBUG）
  - 開発時のみ ON、本番パスでは OFF を前提とする運用メモ。
  - `trace.rs` による prefix（`[trace:pattern]`, `[trace:varmap]`, ...）を簡単に説明。

---

## Success Criteria

- JoinIR / ループ周辺の `eprintln!` が、意味あるかたちで `JoinLoopTrace` 経由に置き換わっている。
- `NYASH_TRACE_VARMAP=1` や `NYASH_JOINIR_DEBUG=1` の挙動が `logging_policy.md` に説明されている。
- デフォルト（env 未設定）ではトレースは出ず、既存の代表テスト（Pattern 1〜4）はログ無しで PASS する。

---

## Notes

- Phase 194 は **挙動を変えない** リファクタフェーズ（観測レイヤーの整形）として扱う。
- Loop パターンや ExitBinding まわりは Phase 193/196/197 で安定しているので、それを壊さない形でログだけを寄せることが目的。
Status: Historical

