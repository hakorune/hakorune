## JSON v0 Bridge

Pointers:
- repo-wide selfhost compiler ownership map:
  - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- current bootstrap/authority contract:
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- active MIR-direct bootstrap phase:
  - `docs/development/current/main/phases/phase-29ch/README.md`

- 役割: Stage‑B / self‑host 側で生成した `Program(JSON v0)` を、Rust 側の `LoopFormBuilder + LoopSnapshotMergeBox` に渡して MIR を生成する薄いフロント。
- 責務:
  1. JSON を `ProgramV0` にデシリアライズし、`lower_stmt_with_vars` / `loop_.rs` などへ流す。
  2. ループについては `LoopFormJsonOps` を介して preheader/header/body/latch/continue_merge/exit のブロック ID とスナップショットを用意し、PHI 構築は LoopForm 側に委譲する。
  3. break / continue / exit の snapshot を `LoopSnapshotMergeBox` に渡して canonical continue_merge/backedge を構築する。

LoopForm/PHI の意味論を変更したい場合は、`loopform_builder.rs` / `loop_snapshot_merge.rs` を更新すること。`loop_.rs` 内での ad-hoc な PHI 実装は禁止。

### Rust パーサー経路との関係

- 共通 SSOT:
  - 制御フロー: `src/mir/ssot/cf_common.rs`（branch/jump/compare/phi 挿入）
  - 算術・比較: `src/mir/ssot/binop_lower.rs`（二項演算 lowering）
  - ループ形: `src/mir/ssot/loop_common.rs` ＋ `LoopFormBuilder`
- フロントエンド（入口）:
  - Rust パーサー経路: `parser/*` + `mir/builder/*` が AST から MIR を構築する。
  - JSON v0 経路: このディレクトリの lowering 群が `ProgramV0` から MIR を構築する。
  - どちらも「PHI/LoopForm/CF の意味論」は mir/ssot 側に委譲し、入口側では構造準備のみに留める。

### Bridge 固有ロジックの箱化方針

- JSON v0 → MIR の際に必要な「ブリッジ固有」の処理は、入口ファイルにベタ書きしない:
  - `hostbridge` / `env` / `me` ダミーなどの特別な変数解決
  - Stage‑1/Stage‑B の try-result モードに合わせた throw ルーティング
- これらは `lowering/globals.rs` / `lowering/throw_ctx.rs` などの小さなモジュールに閉じ込め、
  - JSON v0 経路
  - 将来の Rust パーサー self‑host 経路
 から同じ箱を使い回せるようにする。

このディレクトリでは「JSON → MIR 変換」と「最低限のブリッジ設定」だけを行い、その先の最適化や意味論変更は `mir/*`（optimizer/pass, LoopForm, VM/LLVM）側に任せる。

Current note:
- この層は bootstrap-only compatibility boundary として扱う。
- current authority は `stage1-env-mir-source` であり、この bridge を current authority へ戻さない。
- `Program(JSON v0)` route authority は退いているが、この bridge 自体は repo-wide retirement 完了ではない。
- caller inventory が 0 になるまでは bootstrap-only keep として扱い、hard delete/read-path removal は `phase-29ci` の owner に従う。
