# Phase 25.1 — Stage0/Stage1 Bootstrap & Binary Layout

Status: design+partial implementation（Stage1 ビルド導線の初期版まで）

## Stage‑1 CLI 実験メモ（2025-XX）

- 2025-XX: `NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_PROGRAM_JSON=1 ./target/release/hakorune apps/tests/minimal_ssa_skip_ws.hako` を実行。
- ブリッジ自体は発火し、`stage1-cli/debug` ログと `Stage1CliMain.main/0` が生成されることを確認。
- しかし VM 側で `vm step budget exceeded`（max_steps=2000000）で終了。プラグイン未ロード警告あり（FileBox/ArrayBox など）。
- 対応メモ: ① NYASH_DISABLE_PLUGINS=1 + core-ro で FileBox は代替読込、② `HAKO_VM_MAX_STEPS` をさらに引き上げる or パーサ前処理を削る検討、③ Stage‑B 自前ビルドは `Unknown method 'main' on InstanceBox` で失敗中（StageBDriverBox 呼び出しが壊れている）。 
- Rust VM 側で `vm step budget exceeded` に **fn / bb / last_inst 情報** ＋ Span（MIR に付いていれば .hako 行番号）を付与。今回の Stage‑1 CLI 実行では `fn=ParserBox.parse_program2/1 ... (lang/src/runner/stage1_cli.hako:1:1)` まで出力された（Span は通ったが行位置はまだ粗い）。

## 25.1 サブフェーズの整理（a〜e 概要）

- **25.1a — Stage1 Build Hotfix（配線）**
  - ねらい: `.hako → Program(JSON v0) → MIR(JSON)` の Rust/provider 経路をまず安定させる。
  - 担当: `compiler_stageb.hako` ＋ `tools/hakorune_emit_mir.sh` 系の導線修復（Stage‑B emit を「実際に動く状態」に戻す）。
- **25.1b — Selfhost MirBuilder Parity（selfhost-first 設計）**
  - ねらい: Rust の `env.mirbuilder.emit` を「オラクル」として、Hakorune 側 `MirBuilderBox` を同じ意味論まで引き上げる。
  - 担当: `.hako → Program(JSON v0) → MIR(JSON)` のうち「Program→MIR」を selfhost builder だけでも成立させる準備。
- **25.1c — Env/Extern/Stage‑B 構造整理**
  - ねらい: `env.*` / `hostbridge.*` / `env.box_introspect.*` の責務と Stage‑B Main を箱単位で整理し、入口を一つに揃える。
  - 担当: Stage‑B を `StageBArgsBox` / `StageBBodyExtractorBox` / `StageBDriverBox` / `Stage1UsingResolverBox` に分解しつつ、挙動は変えない構造リファクタ。
- **25.1d — Rust MIR SSA/PHI Smokes**
  - ねらい: Rust 側 `MirBuilder + LoopBuilder + IfForm` の SSA/PHI バグを、小さな Rust テスト（Hako→AST→MirCompiler→MirVerifier）で炙り出して潰す。
  - 担当: Stage‑B/Stage‑1/selfhost で見えている Undefined Value / non‑dominating use を、まず Rust 階層だけで止血する。
- **25.1e — LoopForm PHI v2 Migration（Rust）**
  - ねらい: ループの PHI 生成の「SSOT」を LoopForm v2 + `phi_core` に寄せ、Legacy LoopBuilder 経路との二重管理を解消する。
  - 担当: 当初は `NYASH_LOOPFORM_PHI_V2=1` を使って Stage‑1 / Stage‑B 代表ループ（`_find_from` や stageb_min）を通し、`phi pred mismatch` / ValueId 二重定義を構造的に解消する計画だったが、現在は LoopForm v2 が既定実装となっており、フラグは不要（互換目的のみ）。

ざっくりとした進行順は「25.1a/c で配線と箱分割 → 25.1d/e で Rust MIR/LoopForm を根治 → その結果を踏まえて 25.1b（selfhost MirBuilder/LoopSSA）側に寄せていく」というイメージだよ。

## Legacy Loop/PHI 経路と削除方針（Phase 25.1 時点の整理）

### 正系統（SSOT）として見るべき箱

- ループまわりの SSOT:
  - `src/mir/loop_builder.rs` … LoopForm v2 の構造的 lowering（header/body/latch/continue_merge/exit）。
  - `src/mir/phi_core/header_phi_builder.rs` … header PHI（Pinned/Carrier）の宣言と seal 用メタデータ。
  - `src/mir/phi_core/loop_snapshot_manager.rs` … continue/exit スナップショット管理と LoopSnapshotMerge への導線。
  - `src/mir/phi_core/loop_snapshot_merge.rs` … preheader + continue_merge + latch + exit の snapshot を LoopForm 単位でマージする箱。
- if/merge まわりの SSOT:
  - `src/mir/builder/if_form.rs` … IfForm を用いた構造化 if lowering。
  - `src/mir/phi_core/if_phi.rs` … if merge ブロックでの PHI 生成（ControlForm ベースのラッパを含む）。
  - 今後: `BodyLocalPhiBuilder` を if-merge 側にも拡張して、BodyLocal 変数の PHI 判定を exit だけでなく body 内 if にも適用する予定（Phase 25.x/26.x で対応）。

### Legacy 経路（新規利用禁止・将来削除予定）

- `src/mir/phi_core/loop_phi.rs`
  - 冒頭コメントどおり、LoopForm v2 + LoopSnapshotMergeBox への移行後は「legacy scaffold」としてのみ残存している。
  - 現在の役割:
    - 一部の dev/分析用ドキュメントや smokes から参照される互換レイヤ。
    - 旧 LoopBuilder 互換 API（`prepare_loop_variables_with`, `seal_incomplete_phis_with`, `build_exit_phis_with` など）の受け皿。
  - Phase 25.1 のポリシー:
    - **新しいコードから `phi_core::loop_phi` を直接呼ばない**（LoopForm v2 系の箱のみを使う）。
    - Legacy テスト／smoke のためにしばらく残すが、本線の PHI/SSA 設計の説明は LoopForm v2 系のファイルに寄せる。
  - 削除条件（Phase 31.x 以降で実施予定）:
    - すべての本線経路が `loopform_builder.rs` + `header_phi_builder.rs` + `loop_snapshot_merge.rs` に移行済みであること。
    - `loop_phi.rs` を参照するのが「docs／analysis／legacy-smoke」のみになっていること。
    - `docs/private/roadmap/phases/phase-31.2/legacy-loop-deletion-plan.md` に記載の条件（参照 0＋対応テスト移行）が満たされた時点で削除。

### HashMap 利用の線引き（決定性の観点）

- Phase 25.1 以降、**PHI／LoopForm／IfForm の決定性に関わるマップ**は次のルールに従う:
  - 変数スナップショットや PHI 入力候補のように「順序が意味を持つ」構造:
    - `BTreeMap` / `BTreeSet` / `Vec + sort_by_key` のいずれかを使用して、イテレーション順を決定的にする。
    - 例:
      - `MirBuilder::variable_map` / `value_types` / `value_origin_newbox` → `BTreeMap` 化済み。
      - `phi_core::if_phi::compute_modified_names` → `BTreeSet` で変数名を収集したうえで決定的順序でマージ。
  - メタ情報やインデックス（型とは無関係なキャッシュ・診断用データ構造など）:
    - HashMap 維持可とし、「決定性には影響しない」ことをコメントで明記する。
    - 例:
      - `MirBuilder::weak_fields_by_box`, `property_getters_by_box`, `plugin_method_sigs` など。

- 例外: `phi_core::loop_phi::sanitize_phi_inputs`
  - 現在も内部で一度 `HashMap<BasicBlockId, ValueId>` に詰め替えた後 `Vec` に戻して `sort_by_key` しているため、出力順自体は決定的になっている。
  - ただし、このユーティリティは **legacy 経路専用** と位置付けており、新しい LoopForm v2 系のコードでは `PhiInputCollector`（BTree ベース）側を SSOT として扱う。

## ゴール

- Rust 製 `hakorune` を **Stage0 ブートストラップ**と位置付け、Hakorune コード（.hako）で構成された **Stage1 バイナリ**を明確に分離する。
- Rust 側の責務を「プロセス起動＋最小 FFI＋VM/LLVM コア」に縮退し、それ以外の機能（パーサ高レイヤ、Stage‑B、MirBuilder、AotPrep、numeric core 等）は Stage1 に寄せる。
- 将来的に「Stage1 hakorune（Hakorune 実装の EXE）」を日常利用の標準とし、Rust Stage0 は非常用ランチャ／ブートシードとして保持する。

## レイヤ構成（Stage0 / Stage1 / Runtime）

### Stage0 — Rust Bootstrap Binary

**想定バイナリ:**
- 将来: `target/release/hakorune-bootstrap`
- 現在: `target/release/nyash`（Rust 製 CLI。Stage0 ブートストラップとして扱う）

**責務:**
- OS エントリポイント（`main()`）とプロセス起動。
- 標準入出力・環境変数・argv の取得と最低限の整形。
- LLVM / OS FFI への極小ラッパ（`rt_mem_*` などの intrinsic の土台）。
- Rust VM/LLVM のコア（MIR インタプリタ／コード生成）の提供。
- Stage1 で AOT されたコア関数（後述）を呼び出すランチャ。

**禁止/抑制（緩和版）:**
- パーサ高レイヤ／Stage‑B／MirBuilder／AotPrep／numeric core の**意味論そのもの**を Rust 側に新規実装しない（Self‑Host の責務）。
- 新しい Box 実装や高レベルランタイム機能を Rust に持ち込むのは原則避ける（ただし Stage‑1 ブリッジやエラーログ改善など、導線・可観測性に必要な最小限の変更は許可）。

### Stage1 — Hakorune Selfhost Binary

**想定バイナリ:**
- `target/selfhost/hakorune`（Stage0 が AOT して生成する EXE; ファイル名で Stage1 を表し、配置ディレクトリで Stage0 と分離）

**構成要素（.hako 側で実装/AOT）:**
- Stage‑B コンパイラ（`lang/src/compiler/entry/compiler_stageb.hako` など）。
- MirBuilder / MIR v1→v0 アダプタ。
- AotPrep（`selfhost.llvm.ir.aot_prep.*`、numeric core パスを含む）。
- Ring1 VM／runtime の一部（System Hakorune subset で書かれたコアロジック）。

**責務:**
- Source(.hako) → Program(JSON) → MIR → 実行／LLVM AOT の全パイプラインを Hakorune コードで担う。
- Stage1 自身を再ビルドできる最小セット（自己ホストコア）を提供する。

**起動イメージ:**
- Stage0 `main()`:
  - 環境・argv を集約。
  - AOT 済み `hakorune_main(argc, argv_ptr)`（Stage1 側関数）を呼び出すだけの薄い導線。

### Runtime Lines（共通）

- VM 実行エンジンと LLVM バックエンドは Stage0/Rust に残す（Ring0）。
  - Ny 側からは `env.mirbuilder.emit` / `env.codegen.emit_object` / `env.codegen.link_object` といった extern 経由で利用する。
  - Stage1 は Rust CLI（`nyash`）を「バックエンド CLI」として前提にせず、C-ABI/extern 経由で Ring0 機能にアクセスする。
- その上で Stage1/Hakorune コードを AOT したものをリンクして「言語本体」を構成する。
- 長期的には、Stage1 からさらに Stage1' を再ビルドして差分が収束する自己ホストサイクルを目指す。
  - 具体的には「Stage0→Stage1（本バイナリ）」に加えて「Stage1→Stage1'」を実行し、両者の挙動/インターフェース一致を確認するチェックを設ける。

## ディレクトリ/バイナリ配置案

### Rust Stage0（Bootstrap）

- ソース配置案:
  - `src/bootstrap/` … Stage0 専用のエントリポイント／FFI／VM/LLVM コアの窓口。
  - 既存の Rust コードは徐々にここへ整理（広域リファクタは別フェーズで慎重に）。
- バイナリ:
  - 現在: `target/release/nyash` … Stage0 実行ファイル（Rust 製 hakorune 相当）。
  - 将来: `target/release/hakorune-bootstrap` … Stage0 専用バイナリ（名称を分離予定）。

### Hakorune Stage1（Selfhost）

- ソース:
  - 既存どおり `lang/src/**` に配置（Stage‑B / MirBuilder / AotPrep / VM など）。
  - Stage1 としてビルドすべきモジュールセットを `tools/selfhost/` 以下のスクリプトで管理する。
- バイナリ:
  - 現在（Phase 25.1 初期実装）:
    - Dev line:
      - `tools/selfhost/build_stage1.sh` → `apps/selfhost-runtime/runner.hako` を AOT し、`target/selfhost/hakorune` を生成する。
      - 「Ny Executor（MIR v0 ランタイム）＋CLI 実験」の開発用 EXE（最新版）。
    - Stable line:
      - `lang/build/build_runner.sh` → `lang/bin/hakorune` を生成（pure-lang launcher / legacy bring-up）。
      - 安定した `target/selfhost/hakorune` を `lang/bin/hakorune` に昇格させて配布基準とする運用を想定。
  - 将来:
    - `lang/bin/hakorune` を「標準 hakorune」として日常利用のメインバイナリに昇格させる（dev line は常に先行する実験用バイナリ）。
    - Stage0 は `hakorune-bootstrap` として非常用ランチャ／自己ホストの起点として残す。

## ビルド導線（Phase 25.1 初期版）

このフェーズでは「Rust Stage0 バイナリ」と「Hakorune Stage1 バイナリ」を、ビルド導線レベルで分離するところまでを行う。

### Makefile ターゲット（開発用）

- `make stage0-release`
  - 役割: Rust Stage0（`target/release/nyash`）をビルドする。
  - 実体: `cargo build --release`（既定機能のみ、Rust CLI のみを対象）。
- `make stage1-selfhost`
  - 役割: Stage0 を利用して Stage1 selfhost バイナリをビルドする。
  - 実体:
    - `make stage0-release`（Stage0 準備）
    - `tools/selfhost/build_stage1.sh`
  - 出力: `target/selfhost/hakorune-selfhost`（Ny Executor 最小 EXE）。

### Stage1 ビルドスクリプト

- `tools/selfhost/build_stage1.sh`
  - 入力: `apps/selfhost-runtime/runner.hako`（Ny Executor エントリ）。
  - 経路:
    1. `tools/hakorune_emit_mir.sh` で Stage‑B＋MirBuilder を通し、MIR(JSON v1) を生成。
    2. `tools/ny_mir_builder.sh --emit exe` で ny-llvmc 経由の EXE を生成。
  - 出力: `target/selfhost/hakorune-selfhost`。
  - 備考:
    - EXE のインターフェースは開発用（MIR v0 ファイルを引数に取る Ny Executor）。フル CLI 化は後続フェーズで行う。
    - `NYASH_LLVM_SKIP_BUILD=1` を指定すると、既存の ny-llvmc / nyash_kernel ビルド成果物を再利用して高速化できる。

## フェーズ内タスク（25.1 設計 TODO）

### A. Stage0/Stage1 境界のドキュメント固定

- [x] 本ファイル（phase-25.1/README.md）に Stage0/Stage1 の責務と禁止事項を明文化する。
- [x] Phase 25 README に Stage0/Stage1 の関係をリンク（Ring0/Ring1 の上位概念として扱う）。
- [x] CURRENT_TASK.md に「Stage0=Rust bootstrap / Stage1=Hakorune selfhost」の方針を追記。

### B. Stage1 コアセットの定義

- [ ] Stage1 で AOT すべきモジュール一覧をドラフトする（例: Stage‑B / MirBuilder / AotPrep / numeric core）。
- [ ] それらのエントリポイント関数（例: `hakorune_main/argc,argv` 相当）を .hako 側で定義する設計メモを追加。

### C. ビルド/配置戦略（設計のみ）
### C. ビルド/配置戦略（設計 → 初期実装）

- [x] `tools/selfhost/` 以下に Stage1 ビルド用スクリプト名と役割を決める（`build_stage1.sh`）。
- [x] `target/selfhost/` ディレクトリに Stage1 バイナリを配置する方針を Cargo/Makefile コメントに記載。
- [x] Makefile に `stage0-release` / `stage1-selfhost` ターゲットを追加し、Stage0/Stage1 のビルド導線を分離。

### D. 将来の自己ホストサイクルの入口を定義

- [ ] Stage0→Stage1→Stage1' のビルドシーケンスを文章で定義（どの組み合わせで自己一致チェックを行うか）。
- [ ] 「普段使うのは Stage1」「問題発生時に Stage0 から再生成」という運用パターンを docs に記載。

### E. Stage‑1 UsingResolver / LoopForm v2 対応（設計）

- [x] 設計ドラフトを追加（`stage1-usingresolver-loopform.md`）。Region+next_i 形ループと Carrier/Pinned 対応の指針を明文化。
- [x] Rust 側の観測結果を反映し、具体的なリライト手順とテスト項目を更新する（JSON→LoopForm v2 導線／Stage‑B→Stage‑1 データフローのテキスト図を追記）。

## 実装チェックリスト（25.1 実行順案）

### 1. バイナリ命名と役割の明確化

- [x] Cargo.toml に Stage0/Stage1 の bin ターゲット方針を書き出す（ドキュメントコメントレベル）。
  - 現状: `[[bin]] name = "nyash"` を Stage0（Rust bootstrap）として扱い、Stage1 は `tools/selfhost/build_stage1.sh` で生成される `target/selfhost/hakorune` として外部管理。
- [ ] CURRENT_TASK.md に「ユーザーが使うのは `hakorune` / Stage0 は `hakorune-compat`」という運用ポリシーを追記。

### 2. Stage1 ランチャー（Hako側 Main）の骨組み

- [ ] `lang/src/runner/launcher.hako` を Stage1 の論理エントリポイントとして固定し、コメントに責務（モード切り替え）を書く。
- [ ] ランチャーから呼ぶパイプラインインターフェースを設計する:
  - [ ] `.hako → Program(JSON)` を呼ぶ関数（Stage‑B）。
  - [ ] `Program(JSON) → MIR(JSON)` を呼ぶ関数（MirBuilder）。
  - [ ] `MIR(JSON) → PREP(MIR)` を呼ぶ関数（AotPrep + numeric_core）。
  - [ ] `MIR(JSON) → 実行/EXE` を呼ぶ関数（VM/LLVM）。
- [ ] `launcher.hako` の `Main.main(args)` から、上記インターフェースを呼び分ける最小のモード分岐を定義する設計メモを追加（実装は後続フェーズでもよい）。

### 3. selfhost 用ビルドスクリプトの足場

- [ ] `tools/selfhost/` ディレクトリを作成（存在しない場合）。
- [ ] `tools/selfhost/build_stage1.sh`（仮称）の skeleton を追加:
  - [ ] 必要な Hako モジュールセット（Stage‑B / MirBuilder / AotPrep / runtime）をコメントで列挙。
  - [ ] 現時点では no-op または「未実装」のメッセージだけにして、呼び出し位置を固定。
- [ ] README（本ファイル）に build_stage1.sh の役割と将来の AOT 手順（.hako→MIR→ny-llvmc→EXE）を文章で書いておく。

### 4. Stage0 ↔ Stage1 の切り替えポリシー

- [ ] docs に「普段は Stage1 の `hakorune` を使い、壊れたときだけ Stage0 の `hakorune-compat` を直接叩く」という運用例を追記。
- [ ] `tools/selfhost/` に便利ラッパの案をメモしておく:
  - 例: `hako-vm.sh`（Stage1 + `--backend vm`）、`hako-exe.sh`（Stage1 + `--backend llvm --exe`）。

### 5. 将来の自己ホストルートへの接続

- [ ] Stage1 の `Main.main(args)` から「自分自身を再ビルドする」エントリポイント名だけ決めておく（例: `selfhost_build_main`）。
- [ ] Phase 26 以降で、このエントリポイントを `tools/selfhost/build_stage1.sh` から呼ぶ形にする想定を書き残す。

このチェックリストは「コードを書く前に何を決めておくか」と「どこから小さく実装を始めるか」の順序を示すだけで、実装自体は後続フェーズで少しずつ進める前提だよ。

## このフェーズでやらないこと

- Rust コードの削除や広域リファクタ（責務の再ラベリングとロードマップ策定に留める）。
- Stage1 バイナリを CI で標準に昇格させる変更（ローカル開発用の設計段階に留める）。
- Stage1 ランチャー（フル CLI モード切り替え）の実装本体（このフェーズでは Ny Executor 最小 EXE まで）。

Related docs:
- `docs/private/roadmap2/phases/phase-25/README.md` … Stage0/Ring0-Ring1 再編と numeric_core BoxCall→Call パスのまとめ。
- `docs/development/runtime/cli-hakorune-stage1.md` … Stage1 hakorune CLI のサブコマンド設計と Stage0 との役割分離。
 - `docs/private/roadmap2/phases/phase-25.1a/README.md` … Stage1 build パイプライン（Program→MIR/selfhost AOT）のホットフィックス計画。***
