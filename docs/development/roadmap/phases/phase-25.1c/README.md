# Phase 25.1c — Env / Extern / BoxIntrospect Structural Cleanup

Status: planning（構造整理フェーズ・挙動は変えない）

## ゴール

- `env.*` / `hostbridge.*` / `env.box_introspect.*` の責務と経路を整理し、型システムまわりの「正しい入口」を 1 箇所に揃える。
- Box 型情報 API（`env.box_introspect.kind` + `BoxTypeInspectorBox`）を **コア型システム**として扱えるようにする（plugins の有無に依存しない）。
- i64 / MapBox / ArrayBox の unwrap ロジックを SSOT に寄せ、MirBuilder / JsonEmit / LoopOpts / BoxHelpers が同じ前提で動くようにする。

## スコープ（何をここで扱うか）

- 対象:
  - Rust 側: `extern_registry.rs` / `handlers/externals.rs` / `handlers/extern_provider.rs` / `runtime/plugin_loader_v2/*`
  - Hako 側: `BoxTypeInspectorBox` / `BoxHelpers` / `JsonEmitBox` / `MirSchemaBox` / `LoopOptsBox`
  - ドキュメント: `docs/specs`（env externs / box_introspect / numeric view の設計メモ）
- 非対象:
  - 新しい言語機能や VM 命令の追加（Phase 25 ポリシーに従い、仕様拡張はしない）。
  - MirBuilder の意味論変更（multi‑carrier や LoopForm の設計は Phase 25.1b の範囲に留める）。

## やりたい整理（タスクリスト）

1. **env.* extern の SSOT を決める**
   - `env.get` / `env.mirbuilder.emit` / `env.codegen.emit_object` / `env.codegen.link_object` / `env.box_introspect.kind` を一覧化し、仕様（引数・戻り値・MIR 形）を `docs/specs/env_externs.md`（仮）に明文化する。
   - JSON v0 → MIR ブリッジ（`MapVars::resolve` / lowering）で、上記が必ず `ExternCall("env.*", ..)` に落ちることを確認・修正する。

2. **hostbridge.extern_invoke を「互換レイヤ」に押し込める**
   - 方針: 「`env.*` で表現できるものは ExternCall を正義とし、`hostbridge.extern_invoke` は互換用ラッパに限定する」。
  - Hako 側: `hostbridge.extern_invoke("env.*", ..)` は内部で `env.*` を呼ぶだけにする（新規コードは直接 `env.*` を使う）。
  - Rust 側: `"hostbridge.extern_invoke"` の実装は、`extern_provider_dispatch("env.*", ..)` に委譲する薄いブリッジに整理する。

3. **BoxIntrospect をコア型システムに昇格させる**
   - `env.box_introspect.kind` の実装を plugin loader v2 直下ではなく、コア runtime（例: `runtime/box_introspect.rs`）に寄せる。
   - コア型（MapBox / ArrayBox / StringBox / IntegerBox / BoolBox / NullBox）は runtime 側で `build_box_info` を定義し、plugin loader は「ユーザー Box の拡張」だけを担当する。
   - `BoxTypeInspectorBox` は `env.box_introspect.kind(value)` を唯一の情報源として扱い、repr ベースの fallback は「plugins も env.* も使えないデバッグ環境のみ」で使うことをコメントで明示する。

4. **Numeric view（i64 unwrap）の SSOT 化**
  - Hako 側: `string_helpers` / `BoxHelpers` / `MirSchemaBox` / `JsonEmitBox` / `LoopOptsBox` に散っている i64 unwrap ロジックを、小さなユーティリティ（仮: `box_numeric_view.hako`）に寄せる。
  - Rust 側: `NyashBox` から i64 を取り出す `as_i64` 的な関数を 1 箇所に置き、extern / BoxIntrospect 経路からはそれを使う。

5. **Stage‑B Main を箱に分割して SSA/デバッグを軽くする**
   - 現状の `compiler_stageb.hako: Main.main` は:
     - CLI 引数パース (`--source` / `--bundle-*` / `--require-mod`)
     - bundle/require 解決 (`BundleResolver`)
     - body 抽出 (`body_src` の抽出ロジック)
     - ParserBox 呼び出し (`parse_program2` → emit JSON)
     - defs スキャン (`FuncScannerBox.scan_all_boxes`)
     が 1 関数に詰め込まれており、MIR 上でも巨大な `Main.main` になっている。
   - 25.1c ではこれを「箱理論」に沿って分割する方針を立てており、Phase 25.1c 冒頭でまず Stage‑B 側を 4 箱構造にリファクタした:
     - `Main`（エントリ薄箱）: `main(args){ return StageBDriverBox.main(args) }` のみを担当。
     - `StageBDriverBox`（オーケストレーション）: `StageBArgsBox.resolve_src` → `StageBBodyExtractorBox.build_body_src` → `ParserBox.parse_block2` → defs 挿入 → `print(ast_json)` だけを見る。
     - `StageBArgsBox`（CLI 引数と bundle/require の扱いだけを担当）: もともとの「args/src/src_file/HAKO_SOURCE_FILE_CONTENT/return 0」ロジックを完全移動。
     - `StageBBodyExtractorBox`（`body_src` 抽出ロジック＋bundle/using/trim を担当）: もともとの `body_src` 抽出〜コメント削除〜BundleResolver/Stage1UsingResolverBox〜前後 trim までを丸ごとカプセル化。
   - いずれもロジックはそのまま移動であり、コメント・using・ログを含めて挙動は完全に不変（同じ Program(JSON v0)、同じログ、同じ `VM error: Invalid value`）であることを selfhost CLI サンプルで確認済み。エラーの発生箇所は `Main.main` から `StageBArgsBox.resolve_src/1` に関数名だけ変わっており、SSA/Loop 側の根本修正はこの後のタスク（LoopBuilder / LocalSSA 整理）で扱う。

6. **LoopBuilder / pin スロットの型付け・箱化**
   - いまの LoopBuilder は `__pin$*$@recv` のような文字列ベースの「内部変数名」を `variable_map` に直接突っ込んで、SSA/phi/pin を管理している。
   - 25.1c では、Loop 状態を「箱」として切り出して型付けする:
     - 例: `LoopStateBox`（Rust 側構造体）に
       - `recv_slots`（Method receiver 用）
       - `index_slots`（ループカウンタ用）
       - `limit_slots`（limit/上限 expr 用）
       を明示的に持たせる。
   - `LoopBuilder::emit_phi_at_block_start` / `update_variable` は、この LoopStateBox を通じてのみ pin/phi を操作し、「recv に Null/未定義が混ざらない」ことを構造レベルで保証する。

7. **ビルダー観測用の専用レイヤ（デバッグ箱）**
   - すでに `NYASH_BUILDER_TRACE_RECV` / `NYASH_BUILDER_DEBUG` などで ad-hoc に eprintln を入れているが、出力箇所が複数ファイルに散っていて再利用しにくい。
   - 25.1c ではこれを `builder.observe` 的なモジュール（箱）に集約する:
     - 例: `observe::recv::log(fn, bb, name, src, dst)`、`observe::phi::log(fn, bb, dst, inputs)` など。
   - ポリシー:
     - すべて dev トグル（NYASH_BUILDER_TRACE_*）越しに呼ぶ。
     - 本番挙動は変えず、「どこをどうトレースできるか」を構造として明示する。

8. **Stage‑B 向けの極小 MIR 再現ハーネス**
   - `docs/private/roadmap/phases/phase-20.33/DEBUG.md` にあるような Stage‑B 向けメモを踏まえ、Stage‑B/MirBuilder 用の「極小 Hako → MIR テスト」を 1 つ用意する。
   - 例:
     - 100〜200 行程度の `.hako` を `lang/src/compiler/tests/stageb_min_sample.hako` のようなファイルに固定。
     - Rust 側で MirBuilder に直接その AST を食わせて MIR を生成し、`NYASH_VM_VERIFY_MIR=1` で「Undefined value」が出ないことを確認するユニット／スモークを足す（構造バグ検知用）。
   - これにより、Stage‑B/LoopBuilder に関する修正が `.hako` 本番コード全体に依存せず、小さな再現ケースで検証できるようにする。

## 進め方メモ

- 先にドキュメントを書く（env extern / BoxIntrospect / numeric view の仕様を `docs/specs` 配下に整理）→ そのあとで Bridge / VM / Hako を小さく揃える。
- 既存フェーズとの関係:
  - Phase 25.1b: selfhost builder / multi‑carrier / BoxTypeInspector 実装フェーズ（機能側）。
  - Phase 25.1c: そのうち「env.* / hostbridge.* / BoxIntrospect」に加えて、Stage‑B Main / LoopBuilder / builder 観測レイヤの構造と責務も整理するメタフェーズ（構造側）。
  - 挙動を変えないこと（Fail‑Fast / default path は現状維持）を前提に、小さな差分で進める。

### デバッグ方針（25.1c で踏みたい順番）

- 1) Stage‑B rc=1 の発生箇所を箱単位まで特定する
  - 代表 canary:
    - `tools/smokes/v2/profiles/quick/core/phase251/stageb_fib_program_defs_canary_vm.sh`
    - `tools/smokes/v2/profiles/quick/core/phase251/selfhost_cli_run_basic_vm.sh`
     - まずは `compiler_stageb.hako` の流れを箱ごとに分解してログする:
      - `StageBArgsBox.resolve_src`
      - `StageBBodyExtractorBox.build_body_src`
      - `ParserBox.parse_program2` / `ParserBox.parse_block2`
      - `StageBFuncScannerBox.scan_all_boxes`（Phase 25.1c 時点では Stage‑B ローカル実装）
  - 各箱の入口/出口に `[stageb/trace:<box>.<method>:enter|leave]` のような軽いタグを置き、どの箱が rc=1 の直前で止まっているかを特定する（挙動は変えない）。

- 2) Rust Region レイヤを「正解ビュー」として .hako 側を寄せる
  - Rust 側にはすでに `Region / RefSlotKind / FunctionSlotRegistry + ControlForm` があり、`StageBBodyExtractorBox.*` 周辺のスロット（`src/body_src/bundle_*` など）がどの Loop/If Region に属しているかを `NYASH_REGION_TRACE=1` で観測できる。
  - 25.1c では .hako 側に観測専用 Box（案: `StageBRegionObserverBox`）を追加し、
    - `enter_region(kind, name, slots_json)`
    - `leave_region()`
    のような API で「箱名・構造名・スロット名の集合」だけを JSON で print する。
  - `StageBBodyExtractorBox` の外側ループ・内側 if など、問題になりやすい箇所でこの Box を呼び出し、Rust の `[region/observe]` ログと「木構造＋スロット名」が対応しているかを確認する。  
    → ずれている Region（例: `body_src` などが早期に欠落するスコープ）から優先的に修正する。

- 3) Stage‑B 用の極小ハーネスを Rust / .hako 両方に用意する
  - fib canary はやや重いため、100〜200 行程度の「using + 1 box + 1 loop」だけの最小サンプルを `lang/src/compiler/tests/stageb_min_sample.hako` のようなファイルとして固定する。
  - Rust 側:
    - そのサンプルを AST→MIR に通し、`NYASH_VM_VERIFY_MIR=1` で `Undefined value` が出ないことを確認する小さなテストを用意（既存の Stage‑B 向け MIR テスト群に揃える）。
  - .hako 側:
    - 同じサンプルを入力として `StageBDriverBox` の簡易版（mini driver）を作り、`StageBArgsBox.resolve_src` → `StageBBodyExtractorBox.build_body_src` → `ParserBox.parse_program2` だけを通す driver を追加する。
    - これにより、Stage‑B/LoopBuilder に対する修正を「本番 compiler_stageb.hako 全体」ではなく「ミニマムな Hako 断片」で検証できるようにする。

- 4) Stage‑1 CLI (`HakoCli.run`) の selfhost ラインは Stage‑B が緑になってから扱う
  - Historical note (2026-03-15): this exact blocker is superseded. Current launcher Stage‑B route does emit Program(JSON) and reaches HakoCli MIR generation; the remaining launcher-exe blocker moved to entry argv handoff.
  - 25.1c ではまず Stage‑B 側（fib defs / stageb_min / mini driver）を rc=0 に戻し、  
    その Program(JSON v0) を固定入力にして Phase 25.1b 側の selfhost builder / HakoCli.run MIR を Rust ラインと diff する、という順番で進める。
