# Phase 26-F — Loop Exit Liveness / BodyLocal PHI Guard Line

Status: planning（設計 + 受け口整備。MIRスキャン本体は後続フェーズ）

## ゴール

- LoopForm v2 / Exit PHI まわりで残っている BodyLocal 変数の未定義バグを、「箱」と「MIR スキャン」に分けて根治する。
- すでに Phase 26-E までで固めた PHI SSOT（PhiBuilderBox / IfForm / ExitPhiBuilder / BodyLocalPhiBuilder）を前提に：
  - **分類**（LoopVarClassBox）と
  - **実際の使用**（LoopExitLivenessBox, 将来の MIR スキャン）
  をきちんと分離する。
- その上で FuncScanner / Stage‑B / Stage‑1 入口で発生している `use of undefined value` を、構造から潰す足場を固める。

## スコープ（26-F でやること）

### F‑A: Exit PHI 4箱構成の確定とガード

- ファイル:
  - `src/mir/phi_core/loop_var_classifier.rs`
  - `src/mir/phi_core/local_scope_inspector.rs`
  - `src/mir/phi_core/body_local_phi_builder.rs`
  - `src/mir/phi_core/loop_exit_liveness.rs`（新設済み）
  - `src/mir/phi_core/phi_invariants.rs`
  - `src/mir/phi_core/exit_phi_builder.rs`
- やること:
  - 4箱構成をドキュメントに固定する（コードはほぼ出来ているので設計を追記する）:
    - LoopVarClassBox: Pinned / Carrier / BodyLocalExit / BodyLocalInternal の分類専用。
    - LoopExitLivenessBox: ループ exit 後で「生きている変数」の集合を返す箱（Phase 26-F 時点では保守的近似＋環境変数ガード）。
    - BodyLocalPhiBuilder: 分類結果と `live_at_exit` を OR 判定し、「どの BodyLocal に exit PHI が必要か」だけを決める箱。
    - PhiInvariantsBox: 「全 pred で定義されているか」を Fail‑Fast でチェックする箱。
  - `NYASH_EXIT_LIVE_ENABLE` が未設定のときは **従来挙動（Phase 26-F-3 相当）** に固定されることを明文化。
  - `NYASH_EXIT_LIVENESS_TRACE=1` で、将来の MIR スキャン実装時にトレースを出す方針を記録。

### F‑B: MIR スキャン前提の ExitLiveness 受け口設計

- ファイル:
  - `src/mir/phi_core/loop_exit_liveness.rs`
  - `src/mir/phi_core/exit_phi_builder.rs`
  - （将来）`src/mir/loop_builder.rs` / `src/mir/function.rs`
  - `src/mir/query.rs`（MirQuery/MirQueryBox）
- やること:
  - ExitLiveness 用のトレイトを決める → **実装済み**。
    - `ExitLivenessProvider::compute_live_at_exit(header_vals, exit_snapshots) -> BTreeSet<String>`
    - 既定実装は `LoopExitLivenessBox`（Phase 26-F-3 と同じ空集合、env ガード付き）
    - Phase 2+ 用の `MirScanExitLiveness` を追加済み。現時点では「header_vals + exit_snapshots に出現する変数の union」を返す簡易スキャンで、`NYASH_EXIT_LIVE_ENABLE=1` で opt-in。後続フェーズ（26-G 以降）で MIR 命令列スキャンに差し替える想定。
  - Phase 26-F では「箱とインターフェース」だけ決めて、実装は**保守的近似 or ダミー**のままに留める。
  - `ExitPhiBuilder` は **ExitLivenessProvider の結果だけを見る**ようにしたので、後続フェーズで `MirScanExitLiveness` に差し替え可能（依存逆転）。

### F‑C: FuncScanner / parse_params / trim 用の再現ケース固定

- ファイル:
  - `lang/src/compiler/entry/func_scanner.hako`
  - `lang/src/compiler/tests/funcscanner_skip_ws_min.hako`
  - `lang/src/compiler/tests/funcscanner_parse_params_trim_min.hako`
  - `src/tests/mir_funcscanner_skip_ws.rs`
  - `src/tests/mir_funcscanner_parse_params_trim_min.rs`
- やること:
  - すでに作成済みの 2 本の Rust テストを「ExitLiveness / BodyLocal PHI のカナリア」として位置づける:
    - `mir_funcscanner_skip_ws_direct_vm`
    - `mir_funcscanner_parse_params_trim_min_verify_and_vm`
  - この 2 本が、今後の MIR スキャン実装（Phase 26-G 相当）で「ExitLiveness を差し込んだときに必ず緑になるべき」ターゲットであることを docs に固定。
  - `_trim` / `skip_whitespace` 本体には、`__mir__.log` ベースの軽量観測が既に仕込まれているので、その存在を `mir-logs-observability.md` 側にリンクしておく。

### F‑D: 将来フェーズ（MIR スキャン本体）への橋渡し

- ファイル候補:
  - `docs/private/roadmap2/phases/phase-26-F/README.md`（本ファイル）
  - `docs/development/architecture/loops/loopform_ssot.md`
- やること:
  - Phase 26-F では「箱」と「受け口」と「環境変数ガード」までに留め、MIR 命令列の実スキャンは次フェーズ（26-G など）に分離する方針を書き切る。
  - LoopFormOps を拡張して MIR 命令列にアクセスする案（`get_block_instructions` 等）を、設計レベルでメモしておく。
  - ExitLiveness の MIR スキャン実装は:
    - exit ブロック（必要ならその直後）での use/def を収集し、
    - 逆 RPO で固定点反復して `live_in/ live_out` を決める、
    といった最小の liveness アルゴリズムで良いことを明記。

## このフェーズで「やらない」こと

- LoopFormOps / MirBuilder の広範な API 拡張や、大規模な構造変更。
  - MIR スキャン本体の導入は 26-F ではなく 26-G 以降に分離し、ここではあくまで箱と受け口とドキュメントに留める。
- 既存の LoopForm v2 / Exit PHI ロジックの意味的変更。
  - `NYASH_EXIT_LIVE_ENABLE` が未設定のときの挙動は、Phase 26-F-3 と同等に保つ（テストもそれを期待する）。
- Stage‑B / Stage‑1 CLI / UsingResolver の本線仕様変更。
  - 26-F で触るのはあくまで PHI/SSA のインフラ層のみ。高レベル仕様は 25.x の各フェーズに従う。

## 受け入れ条件（26-F）

- Docs:
  - `docs/development/architecture/loops/loopform_ssot.md` に 4箱構成（LoopVarClassBox / LoopExitLivenessBox / BodyLocalPhiBuilder / PhiInvariantsBox）の役割が追記されている。
  - 本 README に ExitLiveness の受け口設計（ExitLivenessProvider 相当）と、MIR スキャン本体を次フェーズに送る方針が書かれている。
- コード:
  - `NYASH_EXIT_LIVE_ENABLE` が未設定のとき、Phase 26-F-3 と同等かそれ以上のテスト結果（PASS 増 / FAIL 減）を維持している。
  - `LoopExitLivenessBox` / `BodyLocalPhiBuilder` / `PhiInvariantsBox` / `ExitPhiBuilder` の依存関係が一方向（解析→判定→生成→検証）に整理されている。
- テスト:
  - `mir_funcscanner_skip_ws_direct_vm` / `mir_funcscanner_parse_params_trim_min_verify_and_vm` が引き続き「ExitLiveness/BodyLocal PHI カナリア」として動作し、PHI/SSA の変更時に必ず確認される位置づけになっている。
