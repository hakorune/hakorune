# Phase 25.1n — MirBuilder Self‑Host 移植ライン（Rust SSOT → .hako 実装）

Status: planning（設計フェーズ。実装は 25.2 系と並行で段階移行）

## ゴール

- Rust 側で固めた **SSA/PHI SSOT（LoopForm v2 / IfForm / BodyLocal / PhiBuilderBox）** を、
  `.hako` 側の `MirBuilderBox` / `LoopFormBox` / `PhiBuilderBox` に「構造そのまま」移植できるようにするフェーズだよ。
- このフェーズでは:
  - Rust `MirBuilder` を **唯一のオラクル** として扱い、
  - その挙動を「表（ケース表＋制御構造の形）」と「テスト」で固定する。
  - `.hako` 側はその表とテストを見ながら、同じ SSA/PHI を組み立てる実装に寄せていく。
- 25.1/26‑E まででやってきた **LoopForm v2 / ExitPhiBuilder / BodyLocalPhiBuilder / IfForm / PhiBuilderBox** の成果を、
  Self‑Host 実装に届けるための「橋渡しフェーズ」だよ。

## スコープ（25.1n でやること）

### N-0: PHI まわりの箱とガードの現状メモ（2025-?? 時点）

- **PHI を建てる箱 (SSOT)**: `PhiBuilderBox`
  - If 形: `get_conservative_if_values` で **PhiInvariantsBox** による不変チェックを通すようにした（None/None フォールバックは撤去）。
  - Exit 形: `ExitPhiBuilder::build_exit_phis` でも **PhiInvariantsBox** を呼び、pred で未定義な値があれば fail-fast。
  - これにより「欠損 incoming で偶然成功する」経路は塞いだよ。
- **不変条件をチェックする箱**: `PhiInvariantsBox`（新設）
  - 役目: 「merge に必要な値が全 pred に存在するか」をチェックして early error。
  - 適用済み: If, Exit。未着手: Header PHI / ループ body PHI / observe::ssa での観測ガード。
- **解析ヘルパの箱化計画**: `if_phi.rs` にある `extract_assigned_var / collect_assigned_vars / infer_type_from_phi` は  
  将来 `IfAnalysisBox` のような解析専用箱へ移動する予定（まだ実装はしないが、移行先をここで宣言しておく）。

### N‑A: SSA/PHI SSOT の「表」化（Rust 側設計をテーブルに落とす）

- ファイル候補:
  - `docs/development/architecture/loops/loopform_ssot.md`（既存の A/B/C/D ケース表を拡張）
  - `docs/development/architecture/ssa/phi_cases_stage1.md`（新規）
- やること:
  - すでに存在する LoopForm ケース表（Case A/B/C/D）に対して、
    - `LoopVarClass`（Pinned / Carrier / BodyLocalExit / BodyLocalInternal）×
    - LoopCase (A/B/C/D) ×
    - place（header / exit / body‑if‑merge）
    を軸に「どこに PHI を張るか」を表にする。
  - If についても:
    - `then/else` の到達可否（break/continue/early‑return）と、
    - 変数のクラス（Pinned/Carrier/BodyLocal）
    から、「PHI / direct bind / pre 値そのまま」の 3パターンを表で決める。
  - これらを Rust コードに依存しない形で書き下し、
    - 「Rust 実装はこの表を実現しているだけ」という関係にする（SSOT = docs + テスト）。

### N‑B: Rust MirBuilder オラクルテストの整備

- ファイル候補:
  - `src/tests/mir_loopform_conditional_reassign.rs`
  - `src/tests/mir_stage1_using_resolver_verify.rs`
  - `src/tests/mir_stage1_cli_emit_program_min.rs`
- やること:
  - 代表的な構造（LoopCase A〜D / Stage‑1 UsingResolver / Stage‑B fib/defs）について、
    - `.hako` 入力 → Rust `MirCompiler` → `MirVerifier` の結果を **MIR テキストとして固定**するテスト（golden テストに近い）を 1〜2 本ずつ用意する。
  - これらのテストは「Rust MirBuilder の挙動を凍結する」役割のみを持ち、
    - `.hako` 側実装が追いつくまでは「期待値 = Rust 実装」の位置付けにする。
  - 将来は、`.hako` 実装の MIR と diff を取る比較テストに発展させる（本フェーズでは準備だけ）。

### N‑C: .hako MirBuilderBox への API 設計（移植用インターフェース定義）

- ファイル候補:
  - `lang/src/compiler/mir/mir_builder_box.hako`（仮）
  - `lang/src/compiler/mir/loopform_box.hako`（仮）
  - `lang/src/compiler/mir/phi_builder_box.hako`（仮）
- やること:
  - Rust の `MirBuilder` / `LoopFormBuilder` / `PhiBuilderBox` の公開インターフェースから、
    - `.hako` 側で必要になる API を抜き出し、Nyash の Box としてのシグネチャだけ先に決める。
  - 例:
    - `MirBuilderBox.emit_block(fn_name, ast)` → MirModule にブロック/関数を追加。
    - `LoopFormBox.build_loop(condition, body_ast)` → LoopForm v2 構造を Nyash 側で組み立て。
    - `PhiBuilderBox.emit_if_phi(pre_snapshot, then_snapshot, else_snapshot, control_form)` → 既存表に沿って PHI を配置。
  - このフェーズでは **実装はまだ書かず**、I/F と責務コメント、簡単な docs のみを `.hako` 側に置く。

### N‑D: Self‑Host 用ミニパイプラインの設計

- ファイル候補:
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/development/architecture/mir-selfhost-pipeline.md`（新規）
- やること:
  - Self‑Host MVP のパイプラインを定義する:
    - Stage‑0 Rust CLI → Stage‑B (.hako) → Stage‑1 MirBuilderBox (.hako) → MIR(JSON) → VM 実行。
  - MVP では:
    - 1〜2 の代表ケース（fib/defs, minimal_program）だけを対象とし、
    - `.hako` MirBuilder は Rust MirBuilder の完全互換ではなく「代表ケースに十分」な subset に留める。
  - これを Phase 25.2 以降の実装フェーズのターゲットとして書き切る。

## このフェーズで「やらない」こと

- Rust MirBuilder 実装のロジック変更:
  - 25.1n はあくまで「Rust 実装の挙動を SSOT として表＋テストに落とす」フェーズであり、  
    MirBuilder/LoopForm/IfForm/BodyLocalPhiBuilder のロジック変更は 26.x までに終わっていることを前提にする。
- `.hako` MirBuilder 実装の本格実装:
  - ここでは Box のシグネチャと責務、テスト用の I/F だけを決める。実装は 25.2/25.2b などのフェーズで段階的に行う。
- GC や Region/RefSlotKind の統合:
  - 25.1l の Region 観測レイヤーはあくまで Rust 側のみ。  
    `.hako` 側 GC/寿命管理は別フェーズ（25.1m 以降）の仕事とし、MirBuilder Self‑Host とは分離する。

## 受け入れ条件（25.1n）

- Docs:
  - LoopForm/IfForm/BodyLocal/PhiBuilder について、SSA/PHI の挙動が表形式で整理されている（Rust コードを読まずに「この形ならどの PHI が立つか」が分かる）。
  - Self‑Host 用 MirBuilderBox / LoopFormBox / PhiBuilderBox の .hako 側 I/F が定義されている（未実装でも良い）。
- テスト:
  - Rust MirBuilder オラクルテストが 2〜3 本（LoopForm ケース / UsingResolver / Stage‑1 CLI minimal）追加され、安定して緑になっている。
  - これらのテストは「将来 .hako 実装と比較する」前提で、MIR 構造を固定する役割を持つ。
- 実装範囲:
  - Rust 側の MirBuilder ロジックには手を入れていない（設計とテストの “凍結フェーズ” として完了できている）。
