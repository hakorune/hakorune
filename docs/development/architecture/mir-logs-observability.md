# MIR ログ観測 (`__mir__.log`)

**目的**: `__mir__.log` の全使用箇所を一覧化し、dev専用か観測用かを分類する。

---

## 概要

`__mir__.log` は MIR レベルの観測・デバッグ用ログ出力。将来的にはdev向けの一時的なログと、本番でも使うMIR観測 APIに分離する予定。

---

## 使用箇所一覧表

### 1. Stage-1 CLI Debug（10箇所）

**場所**: `lang/src/runner/compat/stage1_cli.hako`

| 行番号 | タグ | 説明 | 用途 |
|-------|------|-----|------|
| 157 | `[stage1_main] args_safe at entry` | stage1_main entry point args debug | Dev専用 |
| 171 | `[stage1_main] config` | Config box values debug | Dev専用 |
| 231 | `[_cmd_emit] args before size check` | _cmd_emit args debug | Dev専用 |
| 235 | `[_cmd_emit] argc after size` | _cmd_emit argc debug | Dev専用 |
| 254 | `[_cmd_emit_program_json] args before size check` | _cmd_emit_program_json args debug | Dev専用 |
| 258 | `[_cmd_emit_program_json] argc after size` | _cmd_emit_program_json argc debug | Dev専用 |
| 282 | `[_cmd_emit_mir_json] args before size check` | _cmd_emit_mir_json args debug | Dev専用 |
| 286 | `[_cmd_emit_mir_json] argc after size` | _cmd_emit_mir_json argc debug | Dev専用 |
| 348 | `[_cmd_run] args before size check` | _cmd_run args debug | Dev専用 |
| 352 | `[_cmd_run] argc after size` | _cmd_run argc debug | Dev専用 |

**有効化環境変数**: `STAGE1_CLI_DEBUG=1`

**説明**:
- Stage-1 CLI の entry point / config 値のデバッグ
- MIR Builder type confusion およびValueId 伝搬の検証用
- argv/argc の SSA 変数追跡

**用途と削除時期**:
- Phase 25.x 完了後に MIR Builder type confusion 修正完了で削除予定
- Config box の初期化検証が完了したら不要になる debug ログが含まれる可能性あり

---

### 2. FuncScanner Debug（3箇所）

**場所**: `lang/src/compiler/entry/func_scanner.hako`

| 行番号 | タグ | 説明 | 用途 |
|-------|------|-----|------|
| 315 | `skip_ws/head` | skip_ws loop head (i, n) | 観測用 |
| 319 | `skip_ws/loop` | skip_ws loop iteration (i, n) | 観測用 |
| 326 | `skip_ws/exit` | skip_ws loop exit (i, n) | 観測用 |

**有効化環境変数**: 現在明示的な環境変数なし

**説明**:
- FuncScanner の skip_ws ループの反復をMIR レベルで観測
- LoopForm v2 / PHI 機能の正しさを検証
- Region+next_i 伝播の SSA 動作確認

**用途と削除時期**:
- **保持推奨**: 将来的な MIR 観測は、用途に応じてMIR観測 APIとして昇格が望ましい
- 当面は明示的な環境変数（例: `NYASH_FUNCSCANNER_DEBUG=1`）で制御することも検討

---

### 3. StringHelpers Debug（1箇所）

**場所**: `lang/src/shared/common/string_helpers.hako`

| 行番号 | タグ | 説明 | 用途 |
|-------|------|-----|------|
| 27 | `[string_helpers/to_i64] x` | to_i64 input value debug | Dev専用 |

**有効化環境変数**: `NYASH_TO_I64_DEBUG=1`

**説明**:
- Void 型が Integer として扱われる問題の検証用
- `NYASH_TO_I64_FORCE_ZERO` workaround の正しさを確認

**用途と削除時期**:
- Phase 25.x 完了後に型伝播が正しく修正されたら削除予定
- 型強制処理の妥当性が確立したら不要になる

---

### 4. Test Comment（1箇所）

**場所**: `lang/src/compiler/tests/funcscanner_skip_ws_min.hako`

| 行番号 | タグ | 説明 | 用途 |
|-------|------|-----|------|
| 6 | (コメント内) | テスト場所の注釈 | コメント |

**説明**: `__mir__.log` の使用例をコメントで示す

---

### 5. MeCall Arity Debug（Phase 25.x追加）

**場所**: `src/mir/builder/method_call_handlers.rs`

| 行番号 | タグ | 説明 | 用途 |
|-------|------|-----|------|
| 70-73 | `[me-call] arity mismatch (instance)` | Instance method arity不一致警告 | Dev専用 |
| 98-101 | `[me-call] arity mismatch (static)` | Static method arity不一致警告 | Dev専用 |
| 150-154 | `[static-call] emit` | Static method呼び出しトレース | 観測用 |

**有効化環境変数**:
- `NYASH_ME_CALL_ARITY_STRICT=1` - 厳密モード（不一致でエラー返却）
- `NYASH_STATIC_CALL_TRACE=1` - トレースモード（warning出力）

**説明**:
- `me.method(...)` 呼び出しのarity検証
- Instance method（receiver追加）vs Static method（receiver なし）の判別
- ParserStmtBox.parse_using/4 の5引数バグ等の検出

**用途と削除時期**:
- Phase XX（static box instance化）完了後に削除予定
- 過渡期のデバッグ支援ログ（static boxのsingleton化後は不要）

---

## 用途別集計表

| 用途 | 件数 | 内容 | 用途と削除時期 |
|-----|-------|-----|----------|
| **Dev専用** | 13 | Stage-1 CLI / StringHelpers / MeCall Arity デバッグ用 | 削除予定（Phase 25.x 完了） |
| **観測用** | 4 | FuncScanner ループ反復 / Static call トレース | 保持推奨（MIR観測 API 昇格） |
| **コメント** | 1 | テスト場所の注釈 | - |

---

## 用途に応じたMIR観測 API 設計（Phase 25.x 完了後）

将来的な `__mir__.log` は暫定的な環境変数制御から脱却し、用途に応じて `MirLogBox` 等の箱化されたAPI経由で呼び出す形に整理することが望ましい。

### 設計例
```nyash
static box MirLogBox {
  // Structured logging with levels
  method debug(tag, ...values) {
    if env.get("NYASH_MIR_LOG_LEVEL") != "debug" { return }
    // ... implementation
  }

  method trace(tag, ...values) {
    if env.get("NYASH_MIR_LOG_LEVEL") != "trace" { return }
    // ... implementation
  }

  // Performance tracing
  method trace_loop_iteration(loop_id, iteration, vars) {
    // ... implementation
  }
}
```

### 利点
- ログレベル設定（debug/trace/info）
- 条件付き出力（JSON形式等）
- 統合デバッグ機能拡張
- 用途に応じたMIR 観測とデバッグの分離

---

## コードの削除方針（Phase 25.4）

**現在の戦略では `__mir__` 使用箇所そのものには変更を加えず、分類と削除方針のみを記録。**

削除や統合は将来的な環境変数整理や機能分類時に実施する。

---

## 参考

- Phase 25.4 計画: `docs/private/roadmap2/phases/phase-25.4-naming-cli-cleanup/README.md`
- Stage-1 CLI compat owner: `lang/src/runner/compat/stage1_cli.hako`
- FuncScanner: `lang/src/compiler/entry/func_scanner.hako`
- StringHelpers: `lang/src/shared/common/string_helpers.hako`
