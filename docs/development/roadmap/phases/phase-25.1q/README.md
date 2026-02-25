# Phase 25.1q — LoopForm Front Unification (AST / JSON v0)

Status: in-progress（LoopBuilder 側の canonical continue_merge 導入済み / 25.1e + 25.2 でスコープ＆スナップショットモデルは確定済み / JSON v0 front は移行中）

## ゴール

- ループ lowering / PHI 構築の **SSOT を `phi_core + LoopFormBuilder` に完全に寄せる**。
  - Rust AST → MIR（`MirBuilder` / `LoopBuilder`）と JSON v0 → MIR（`json_v0_bridge::lower_loop_stmt`）のフロントを整理し、
    「どこを直せば loop/PHI の意味論が変わるか」を 1 箇所に明示する。
- 実装者・LLM の混乱ポイントを減らす:
  - Stage‑B / FuncScanner のようなループバグ調査時に、「Rust AST 側を触るべきか」「JSON v0 側を触るべきか」で迷わない構造にする。
  - 今回のように `loop_.rs` 側だけにログ・修正を入れてしまう誤りを防ぐ。

## 現状（25.1m / 25.1p / 25.1q 部分実装時点の構造）

- **バックエンド（SSOT）**
  - `src/mir/phi_core/loopform_builder.rs` / `src/mir/phi_core/loop_snapshot_merge.rs`
    - LoopForm v2 / LoopSSA v2 の本体。
    - `LoopFormBuilder` + `LoopFormOps` として、ヘッダ PHI / exit PHI / continue・break スナップショットなどを一元的に扱う。
    - Phase 25.2 で導入した `LoopSnapshotMergeBox` が、continue/exit スナップショットのマージと PHI 入力ベクタの構成を一元管理している。
  - `src/mir/phi_core/loop_phi.rs`
    - legacy ルート（JSON v0 bridge など）向けの互換 API（`prepare_loop_variables_with` / `seal_incomplete_phis_with` / `build_exit_phis_with`）。
    - Rust AST ルート新規実装では使用しない（25.1e で LoopForm v2 を SSOT として固定済み）。

- **Rust AST → MIR 経路**
  - `ASTNode::Loop`
    - `src/mir/builder_modularized/control_flow.rs::build_loop_statement`
    - `src/mir/builder/control_flow.rs::cf_loop`
    - `src/mir/loop_builder.rs::LoopBuilder::build_loop_with_loopform`
  - こちらはすでに LoopForm v2 / ControlForm v2 に統一済みで、「Rust パーサで読んだ .hako」を MIR に落とす主経路。
  - 25.1q 先行作業として、**canonical continue_merge ブロック** を LoopBuilder 側に導入済み:
    - 各ループごとに `continue_merge_id` ブロックを新設し、`LoopBuilder::continue_target` を `header` ではなく `continue_merge_id` に設定。
    - `do_loop_exit(Continue)` はすべて `continue_merge_id` へジャンプし、`continue_merge_id` から `header` への `Jump` を 1 本だけ張る構造になった。
    - `LoopShape::continue_targets` は「continue の canonical backedge」として `continue_merge_id` のみを持つ（存在する場合）。
    - `LoopFormBuilder::seal_phis` に渡す `continue_snapshots` は、Phase 25.2 時点では
      「すべての continue 経路のスナップショットを `LoopSnapshotMergeBox` で `continue_merge_id` 上の PHI に集約したうえで、
      1 件の `merged_snapshot` として `continue_merge_id` から header に渡す」という形に統一されている。
      ヘッダ側の PHI は `preheader` / `continue_merge_id` / `latch` の 3 系統を前提に動く。
  - ループ関連の検証状況:
    - `mir_stageb_loop_break_continue::*` / `mir_loopform_exit_phi::*` / `mir_stageb_like_args_length::*` など、LoopForm/Exit PHI まわりの代表テストは現在すべて PASS。
    - 手書きの簡易ループ（sum=10）および `continue` を含むループ（sum=8, `continue=[BasicBlockId(...)]`）も LoopForm v2 経由で正常動作している。

- **JSON v0 → MIR 経路**
  - `Program(JSON v0)`（`ProgramV0`）:
    - `src/runner/json_v0_bridge/lowering.rs::lower_stmt_with_vars`
    - `StmtV0::Loop { .. } => loop_::lower_loop_stmt(...)`
    - `src/runner/json_v0_bridge/lowering/loop_.rs::lower_loop_stmt`
  - 25.1q 時点では:
    - JSON front も `LoopFormBuilder` + `LoopFormOps`（`LoopFormJsonOps`）経由で preheader/header/latch/continue_merge/exit の PHI を構築するように統一済み。
    - ループ意味論・PHI の仕様変更は AST ルートと同様に `loopform_builder.rs` / `loop_snapshot_merge.rs` 側だけを触ればよい。
    - `loop_.rs` は「ブロック ID を準備し、スナップショット（break/continue/exit）を `LoopSnapshotMergeBox` に引き渡すだけ」の薄いフロントに収束しており、canonical continue_merge も AST/JSON 共通の形になった。
    - JSON v0 だけを入力にした軽量スモーク（`tests/json_program_loop.rs`）でも、ループ種別（通常・continue・body-local exit）がすべて `MirVerifier` で緑になることを確認済み。

- 結果として:
  - Stage‑B / FuncScannerBox のような「Rust AST 経路」を見たいときに、誤って `loop_.rs` 側だけを触る、といった混乱が起きやすい。
  - 一方で JSON v0 経路は provider (`env.mirbuilder.emit` / `--program-json-to-mir`) で重要なので、急に削除はできない。

## スコープ（25.1q でやること）

1. **LoopForm / phi_core / LoopSnapshotMergeBox を SSOT として明文化（ドキュメント整理）**
   - `docs/private/roadmap2/phases/phase-25.1b/` / `phase-25.1m/` / 本 `phase-25.1q` で:
     - ループ意味論（preheader/header/body/latch/exit、continue/break スナップショット、PHI）の SSOT を
       `LoopFormBuilder` + `LoopSnapshotMergeBox` に一本化すると明言する。
     - legacy `phi_core::loop_phi` は JSON v0 bridge など互換レイヤ限定、と位置づける。
     - `LoopBuilder`（Rust AST フロント）と `json_v0_bridge::lower_loop_stmt`（JSON フロント）は「薄いアダプタ」に留める方針を書いておく。

2. **LoopForm v2 の continue 経路を正規化（canonical continue merge の設計＋導入）**
   - 目的: 「if‑merge → continue → header」パスでの PHI/SSA 破綻を、LoopForm 側で構造的に潰す。
   - 方針（設計レベル）:
     - 各ループに、必要に応じて `continue_merge_bb`（continue 統合ブロック）を 1 個だけ用意する。
     - ループ本体内のすべての `continue` は、直接 `header` ではなく一旦 `continue_merge_bb` にジャンプする。
     - `continue_merge_bb` から `header` への backedge を 1 本張り、LoopForm から見た backedge を
       「`latch` + `continue_merge_bb` の 2 系統」に正規化する。
   - 責務分離:
     - LoopForm v2 / phi_core:
       - preheader/header/latch/exit/continue_merge/break の **ループ骨格と PHI** を一元的に扱う。
       - loop‑carried / pinned / body‑local live‑out を「ループ単位の箱」の中で完結させる。
     - IfForm / FuncScanner / Stage‑B 側:
       - ループ内部の純粋な if/&&/|| のみを扱い、continue に伴う SSA/PHI を意識しない。
   - 25.1q では:
     - 先行ステップとして、Rust AST 経路の `LoopBuilder::build_loop_with_loopform` に
       canonical continue_merge ブロックを導入（**実装済み**）。
       - continue 経路はすべて `continue_merge` → `header` という 1 本の backedge に集約。
       - `LoopShape` / `ControlForm` から見た continue backedge も `continue_merge` 1 箇所に正規化。
     - JSON v0 front（`loop_.rs`）でも canonical continue_merge を導入し、continue 経路は `continue_merge → header` に一本化。  
       ⇒ AST / JSON のどちらでも backedge は「latch」と「continue_merge」の 2 系統だけを見ればよい状態になった。

3. **json_v0_bridge::lower_loop_stmt の責務縮小（薄いフロント化）**
   - 目標: `loop_.rs` は「JSON から LoopForm に渡すための最低限の橋渡し」に限定する。
   - 具体案:
     - 余計なデバッグログや独自判定を段階的に削り、やることを
       - preheader/header/body/latch/exit/continue_merge のブロック ID を用意する
       - ループ開始時点の `vars` を LoopFormOps 実装に渡す
       - break / continue のスナップショット記録を呼び出す
       に絞る。
     - ループ構造・PHI の仕様変更は **`LoopFormBuilder` + `LoopSnapshotMergeBox` 側だけ** に集約し、`loop_.rs` 側には分岐や条件を増やさない。

4. **ログ・デバッグ経路の整理**
   - `HAKO_LOOP_PHI_TRACE` / `NYASH_LOOPFORM_DEBUG` などのトグルについて:
     - どのフロント（Rust AST / JSON）からでも同じタグで観測できるようにし、ログの出し場所を整理する。
     - `loop_.rs` に残っている「一時的な ALWAYS LOG」などはすでに削除済みだが、今後も dev トレースは必ず env ガード越しに行う。

5. **JSON v0 → AST → MirBuilder 統合の検討（設計レベルのみ）**
   - 将来案として:
     - `ProgramV0` を一度 Nyash AST 相当の構造体に変換し、`MirBuilder` の `build_loop_statement` を再利用する形に寄せる。
     - これが実現すると、`loop_.rs` 自体を削除しても LoopForm/PHI の意味論は完全に一箇所（LoopBuilder + phi_core）に集約される。
   - 25.1q ではここまでは踏み込まず、「やるならどのフェーズで、どの単位の差分にするか」を設計メモとして残す。

## スコープ外（25.1q ではやらないこと）

- ループ意味論そのものの変更:
  - `loop(cond){...}` の評価順序や break/continue の意味論を変えない。
  - Stage‑B / Stage‑1 / 自己ホストルートで既に green な LoopForm/SSA テストの挙動は不変とする。
- 新しいループ構文・最適化の追加:
  - `while` / `for` / range loop など、新構文の導入は別フェーズ（言語拡張側）に任せる。
- JSON v0 スキーマの変更:
  - `StmtV0::Loop` などの JSON 形は既存のまま（schema v0/v1 は維持）。

## 他フェーズとの関係

- 25.1m（Static Method / LoopForm v2 continue + PHI Fix）:
  - ここで LoopForm v2 / continue + header PHI は Rust AST 経路でほぼ安定している。
  - 25.1q では、その成果を JSON v0 経路にも構造的に反映し、「LoopForm v2 がどこから使われているか」を明示する役割を担う。

- 25.1p（MIR DebugLog 命令）:
  - DebugLog を使って LoopForm/PHI の ValueId を観測しやすくすることで、25.1q での統一作業時に「AST ルートと JSON ルートの差」を追いやすくする。
  - 25.1q は DebugLog 基盤が整っていることを前提に、小さな JSON v0 → MIR のテストケースで CFG/PHI を比較するフェーズとする。

- 25.2（LoopSnapshotMergeBox / Snapshot Merge Unification）:
  - ここで `LoopSnapshotMergeBox` を導入し、continue/break/exit スナップショットのマージと PHI 入力構成を一元化した。
  - 25.1q では、この箱を前提として AST / JSON 両フロントを「LoopForm v2 + LoopSnapshotMergeBox にぶら下がる薄いアダプタ」に揃えることで、
    Stage‑B / FuncScanner / Stage‑1 など、どの経路からでも同じ LoopScope/Env_in/out モデルでデバッグできるようにする。
