# Phase 25.4 — NamingBox SSOT & CLI 設定箱化

**目的**: static box / global 呼び出しの名前決定を一元化し、Stage-1 CLI の環境変数処理を Config 箱に集約する。

---

## Task A: NamingBox SSOT化（✅ 完了）

**Rust側統一**:
- `src/mir/naming.rs` を SSOT に設定
- Builder/VM/Entry選択ロジックを統一

**Python側ミラー**:
- `src/llvm_py/naming_helper.py` 作成
- Rust NamingBox と完全同一の意味論

**完了コミット**: fa9cea51 (Rust), bceb20ed (Entry選択), 419214a5 (Python)

---

## Task B: Stage-1 CLI 設定箱（進行中）

### 目的
`stage1_main` から env/トグル処理を分離し、1箇所の「設定箱」で env を解釈する。

### Config フィールド設計

#### **実行制御フィールド**（本番用）
```nyash
static box Stage1CliConfigBox {
  // 1. Mode selection
  mode: String  // "emit_program_json" | "emit_mir_json" | "run" | "disabled"

  // 2. Backend selection
  backend: String  // "vm" | "llvm" | "pyvm" (default: "vm")

  // 3. Input sources
  source_path: String       // STAGE1_SOURCE: .hako file path
  source_text: String       // STAGE1_SOURCE_TEXT: direct source text (test use)
  program_json_path: String // STAGE1_PROGRAM_JSON: pre-built Program JSON path

  // 4. Control flags
  use_stage1_cli: Integer  // NYASH_USE_STAGE1_CLI: 1=enable, 0=disable
}
```

#### **Dev専用フラグ**（開発用）
```nyash
static box Stage1CliConfigBox {
  // Debug & dev toggles
  debug: Integer           // STAGE1_CLI_DEBUG: 1=verbose logging
  to_i64_force_zero: Integer  // NYASH_TO_I64_FORCE_ZERO: 1=force 0 on Void (dev workaround)
}
```

### 環境変数マッピング

| 環境変数 | Config フィールド | 用途 | デフォルト |
|---------|------------------|-----|----------|
| `NYASH_USE_STAGE1_CLI` | `use_stage1_cli` | CLI有効化 | 0 (無効) |
| `STAGE1_EMIT_PROGRAM_JSON` | `mode` | "emit_program_json" | - |
| `STAGE1_EMIT_MIR_JSON` | `mode` | "emit_mir_json" | - |
| `STAGE1_BACKEND` | `backend` | backend選択 | "vm" |
| `STAGE1_SOURCE` | `source_path` | .hakoパス | - |
| `STAGE1_SOURCE_TEXT` | `source_text` | 直接ソース（テスト用） | - |
| `STAGE1_PROGRAM_JSON` | `program_json_path` | Program JSONパス | - |
| `STAGE1_CLI_DEBUG` | `debug` | デバッグログ | 0 (無効) |
| `NYASH_TO_I64_FORCE_ZERO` | `to_i64_force_zero` | Void→0強制化（dev用） | 0 (無効) |

### Mode 決定ロジック

```nyash
method from_env() {
  local cfg = new MapBox()

  // Mode selection (排他的)
  if env.get("STAGE1_EMIT_PROGRAM_JSON") == "1" {
    cfg.set("mode", "emit_program_json")
  } else if env.get("STAGE1_EMIT_MIR_JSON") == "1" {
    cfg.set("mode", "emit_mir_json")
  } else if env.get("NYASH_USE_STAGE1_CLI") == "1" {
    cfg.set("mode", "run")
  } else {
    cfg.set("mode", "disabled")
  }

  // Backend
  local b = env.get("STAGE1_BACKEND")
  if b == null { b = "vm" }
  cfg.set("backend", "" + b)

  // Sources
  cfg.set("source_path", env.get("STAGE1_SOURCE"))
  cfg.set("source_text", env.get("STAGE1_SOURCE_TEXT"))
  cfg.set("program_json_path", env.get("STAGE1_PROGRAM_JSON"))

  // Flags
  cfg.set("debug", env.get("STAGE1_CLI_DEBUG"))
  cfg.set("to_i64_force_zero", env.get("NYASH_TO_I64_FORCE_ZERO"))

  return cfg
}
```

### 内部専用環境変数（Config 外）

以下は **Config に入れない**（内部処理・参照のみ）:

- `HAKO_STAGEB_APPLY_USINGS` - Stage-B using 適用制御（BuildBox内部）
- `HAKO_MIR_BUILDER_DELEGATE` - MirBuilder delegate トグル（MirBuilderBox内部）
- `NYASH_SCRIPT_ARGS_JSON` - スクリプト引数（`env.set`で設定、読み取りは Stage0）

---

## Task C: MIR ログ観測リスト（後続）

`__mir__.log` のタグと用途を一覧化し、dev専用/観測用を分類する。

詳細: 後続セクション参照

---

## 実装計画

### B-1. 設計（✅ 完了）
- Config フィールド設計: 上記

### B-2. Config 箱実装（次）
- `Stage1CliConfigBox` 作成
- `from_env()` メソッド実装

### B-3. stage1_main リファクタ（次）
- `local cfg = Stage1CliConfigBox.from_env()` を先頭で呼び出し
- `env.get` 呼び出しを `cfg.get` に置換

### B-4. テスト（次）
- `cargo test mir_stage1_cli_stage1_main_shape_verifies`
- `cargo test mir_stage1_cli_entry_like_pattern_verifies`
- `tools/stage1_debug.sh` 手動実行確認

---

## 参考

- Phase 25.4-A 完了コミット: fa9cea51, bceb20ed, 419214a5
- Stage-1 CLI 本体: `lang/src/runner/stage1_cli.hako`
- Stage-1 テスト: `src/tests/stage1_cli_entry_ssa_smoke.rs`
