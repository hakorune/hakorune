# Stage‑1 UsingResolver — LoopForm v2 対応メモ（設計ドラフト）

目的: Stage‑1 UsingResolver / collect_entries 系のメインループを Region+next_i 形に揃え、Carrier / Pinned / BodyLocalInOut モデル（Phase 25.1e〜）と整合する形で SSA を安定させる。実装前の設計メモとして、Rust 側の読みどころと .hako 側のリライト方針を先に固定しておく。

## 読むべき Rust 側の入口（構造把握用）
- JSON v0 → MIR/AOT: `src/runner/json_v0_bridge/lowering/`（Program(JSON) を LoopForm v2 に落とす部分）
  - 特に `lowering/loop_.rs`（LoopForm v2 への薄いアダプタ）と `phi_wiring` 周辺。
- LoopForm v2 / snapshot:
  - `src/mir/loop_builder.rs`
  - `src/mir/phi_core/loopform_builder.rs`
  - `src/mir/phi_core/loop_snapshot_merge.rs`
  - Stage‑1 UsingResolver テストの観察点:
  - `src/tests/mir_stage1_using_resolver_verify.rs`
    - collect_entries 系ループ（JSON スキャン）
    - Region+next_i 形の entries ループ
    - modules_list 分割ループ（Region+next_start）
    - resolve_for_source 相当で entries ループと modules_map 参照を同時に行うケース
  - 既存の JSON フロント経路でどのブロック/値が PHI 化されているかを dump しておくと導線が追いやすい。

## JSON v0 フロント側の契約（Stage‑B → Stage‑1）
- Program(JSON v0) 形: `{"version":0,"kind":"Program","body":[...], "defs":[ ... ]}`
- defs の body: Stage‑B から渡ってくるのは `{"type":"Block","body":[Stmt...]}` ラップ。Stage‑1 UsingResolver ではこの形を前提に扱う。
- ループ/PHI の意味論は Rust LoopForm v2 側に委譲する。Stage‑1 は「JSON を正しい構造で渡す箱」として振る舞う。

## Rust 観測メモ（LoopForm v2 / JSON v0 bridge）
- JSON → LoopForm v2 の入口は `src/runner/json_v0_bridge/lowering/loop_.rs`。
  - ブロック構成は preheader → header → body → latch → exit に加え、canonical `continue_merge` ブロックを用意してから LoopFormBuilder を呼ぶ。
  - LoopFormJsonOps は `me/args` 名ベースで parameter 判定（pinned）を行う。それ以外は carrier。Stage‑1 側で変数名が崩れると pinned 判定が効かないので注意。
  - writes 集合は preheader snapshot と body_vars を比較して検出、LoopFormBuilder::seal_phis に渡す。
  - continue スナップショットは PhiInputCollector で `continue_merge` に集約し、header PHI へ 1 本バックエッジを張る形に正規化。
  - exit PHI は LoopFormBuilder::build_exit_phis が LoopSnapshotMergeBox を使って生成する（LoopForm v2 が SSOT）。
- LoopForm v2 本体は `src/mir/phi_core/loopform_builder.rs`:
  - prepare_structure で preheader copy / header PHI の ValueId を先に全確保（Carrier/Pinned 分類）。
  - seal_phis で latch + continue_merge スナップショットから header PHI を張り直し、兄弟 NaN を避けるガードあり。
  - exit PHI は build_exit_phis で pinned/carriers/BodyLocalInOut をまとめ、LoopSnapshotMergeBox に委譲。

### JSON v0 → LoopForm v2 ざっくり導線（テキスト版）
- Program(JSON v0).body / defs.body(Block) → lowering/stmt::lower_stmt_list_with_vars
- Loop ノード → lowering/loop_.rs::lower_loop_stmt
  - 事前に preheader/header/body/latch/exit/continue_merge を生成
  - LoopFormBuilder.prepare_structure → header PHI の ValueId を全確保（carrier/pinned）
  - header で cond を評価し branch(header→body/exit)
  - body を lower し、writes 集合と continue/exit スナップショットを集める
  - continue_snapshots を continue_merge で正規化 → header backedge を 1 本に圧縮
  - LoopFormBuilder.seal_phis(header) / build_exit_phis(exit) で PHI 完成
- ループ意味論（PHI/snapshot マージ）は LoopForm v2 側が SSOT、bridge は ValueId/ブロック配線と snapshot 受け渡しだけ担当。

## Stage‑B → Stage‑1 データフロー（テキスト版）
- Stage‑B (`compiler_stageb.hako`):
  - source → body 抽出（Main.main 内）→ block パーサ優先で Program(JSON v0) を構成
  - defs: FuncScanner でメソッド本文を block パーサ優先で JSON 化し、`{"type":"Block","body":[…]}` でラップして Program.defs に注入
- Stage‑1 UsingResolver:
  - Program(JSON) を入力に using/extern を解決（今は apply_usings=0 でバイパス多め）。defs/body の構造はそのまま Rust LoopForm v2 に渡る前提。
  - region+next_i 形ループで JSON スキャン・modules_map を決定、prefix 結合するだけのテキスト担当箱。
  - using 解決の意味論そのものは `UsingResolveSSOTBox` に委譲する（Stage‑1 は「SSOT に渡す JSON/front を整える箱」）。
- Rust bridge (Stage0):
  - Program(JSON v0) → json_v0_bridge lowering → LoopForm v2 → MIR → VM/LLVM
  - Loop/PHI/SSA の SSOT は Rust 側。Stage‑1/.hako は「正しい形の Program(JSON) を渡す」責務に徹する。

## Stage‑1 CLI インターフェース設計メモ（ドラフト）

詳しい CLI サーフェスとサブコマンド設計は `docs/development/runtime/cli-hakorune-stage1.md` 側を SSOT とし、
ここでは Stage‑1 UsingResolver／LoopForm v2 との接続と、Rust Stage0 から呼ばれる stub
（`lang/src/runner/stage1_cli.hako`）の責務に絞って整理する。
- 入口関数（.hako 側で定義予定）:
  - `emit_program_json(source: String)` → Program(JSON v0)（Stage‑B 呼び出しラッパ）
  - `emit_mir_json(program_json: String)` → MIR(JSON)（MirBuilder 呼び出しラッパ）
  - `run_program_json(program_json: String, backend: String)` → 実行（VM/LLVM を選択）
  - `stage1_main(args)` → CLI 分岐（下記トグルでモード決定）
- Rust Stage0 側ブリッジ（既定 OFF トグル想定）:
  - Stage1 CLI を呼ぶときだけ Program(JSON/MIR(JSON)) を引き渡す薄い層にする。普段は現行 CLI と同じ振る舞い。
- パイプライン図（テキスト案）:
  - source (.hako) --(Stage‑B)--> Program(JSON v0) --(Stage‑1 UsingResolver)--> Program(JSON v0, defs付き) --(MirBuilder)--> MIR(JSON) --(VM/LLVM)--> 実行/EXE
- トグル/引数案（ドラフト）:
  - `--emit-program-json` / env `STAGE1_EMIT_PROGRAM_JSON=1`
  - `--emit-mir-json` / env `STAGE1_EMIT_MIR_JSON=1`
  - `--backend vm|llvm` （既定 vm）
  - self-host 経路は env `NYASH_USE_STAGE1_CLI=1` で有効化（既定 OFF）
  - 現状の stage1_main は argv 依存を外し、env だけを見る仕様（NYASH_USE_STAGE1_CLI + STAGE1_*）。Void 返りの数値変換は `NYASH_TO_I64_FORCE_ZERO=1` で明示的に潰す（stage1_debug.sh でセット済み）。

## （ドラフト）Stage‑1 CLI パイプライン図（文書向け簡略版）
```
          +-----------------+       +------------------+       +-----------------+
source -> | Stage-B (block) |  -->  | Stage-1 UsingRes |  -->  | MirBuilder (.hako) |
(.hako)  |  Program JSON   |  defs |  (using/defs keep|       |  MIR(JSON)        |
          +-----------------+       +------------------+       +-----------------+
                  |                          |                          |
                  | Program(JSON v0)         | Program(JSON v0, defs)   | MIR(JSON)
                  v                          v                          v
             (Rust Stage0)             (Rust Stage0)               (Rust Stage0)
             json_v0_bridge            LoopForm v2                 VM / LLVM
```
- 入口APIイメージ: `emit_program_json(source)`, `emit_mir_json(program_json)`, `run_program_json(program_json, backend)`, `stage1_main(args)`.
- Rust Stage0 は既定では従来 CLI のまま。self-host パスは明示トグルで有効化し、Bridge 部分だけ薄く持つ。
- スタブ配置: `lang/src/runner/stage1_cli.hako`（Phase 25.1 は骨組みのみ、実装後続）。
- Rust bridge (stub path):
  - `NYASH_USE_STAGE1_CLI=1` で Stage0 側が `lang/src/runner/stage1_cli.hako` を子プロセスとして起動し、`STAGE1_EMIT_PROGRAM_JSON=1` / `STAGE1_EMIT_MIR_JSON=1` / `STAGE1_BACKEND=vm|llvm|pyvm` をもとに `emit program-json` / `emit mir-json` / `run --backend ...` のスクリプト引数を組み立てる。
  - 再入防止に `NYASH_STAGE1_CLI_CHILD=1` を橋渡しで付与。entry は `STAGE1_CLI_ENTRY` で上書き可能（既定はスタブパス）。
- Phase 25.1A-3 現在の実装状態:
  - `emit program-json` は Stage‑B/BuildBox + Stage‑1 UsingResolver(prefix結合) で Program(JSON v0) を出力（Stage0 からは Stage‑1 stub 経由で子プロセス実行）。
  - `emit mir-json` は `MirBuilderBox.emit_from_program_json_v0` を呼び出し（delegate toggles 未設定時は失敗ログを返す）。
  - `run --backend <vm|pyvm>` は MIR(JSON) を stdout に吐くだけの暫定挙動。`--backend llvm` は `env.codegen.emit_object` まで通す（link/exec は未着手）。
- Phase 25.1A-4 追加メモ:
  - Using SSOT: `lang/src/using/resolve_ssot_box.hako` に README/I/F を整備（resolve_modules/resolve_prefix は現状 no-op だが I/F 固定）。
  - Stage1UsingResolverBox に `resolve_for_program_json` を追加し、SSOT を呼ぶ入口だけ用意（現状は pass-through）。
  - BuildBox から古い `include` を除去し `using ... as BundleResolver` に置換（Stage‑B パーサの include 非対応を回避する足場）。

## .hako 側でやること（Region+next_i 形への揃え）
- 対象ファイル: `lang/src/compiler/entry/using_resolver_box.hako`（または同等名）
- ループ形の目標:
  - `loop(pos < n) { local next_pos = pos + 1; ...; pos = next_pos }`
  - 途中の continue/break は可能なら `next_pos` の書き換え＋末尾で合流、複数経路を region 1 本でまとめる。
- 変数の役割分離:
  - `pos`（carrier）と `next_pos`（body-out）を明示。
  - in/out をまたぐワーク変数は極力 `local` を region 内に閉じ込め、Pinned/BodyLocalInOut が LoopForm に素直に伝わる形にする。
- 分岐の扱い:
  - `if ... { ...; next_pos = ... } else { next_pos = ... }` のように各分岐で next_pos を決め、末尾で `pos = next_pos` だけにする。多重 continue を避けて SSA を単純化。
### ループ洗い出しメモ（entry / pipeline_v2）
- entry UsingResolver（lang/src/compiler/entry/using_resolver_box.hako）
  - Region+next_i 化済み: entries イテレーション / JSON スキャン / modules_list 分割
  - 追加テスト:
    - modules_list 分割ループ（start/next_start）をそのまま MIR 化できることを `mir_stage1_using_resolver_module_map_regionized_verifies` で固定。
    - resolve_for_source 相当で entries ループと modules_map 参照（MapBox.has/get）を同時に行うケースを `mir_stage1_using_resolver_resolve_with_modules_map_verifies` で固定。
  - 残り: なし（現状の 3 ループは all Region 形）

### UsingResolveSSOTBox 境界メモ

- ファイル: `lang/src/using/resolve_ssot_box.hako`。
- 責務（SSOT 箱）:
  - IO 禁止。modules_json / using_entries_json / ctx(map) だけを入力に取り、解決結果（path/prefix/modules_json_resolved）を返す純粋関数群。
  - 主な I/F:
    - `resolve(name, ctx)`
      - ctx.modules / ctx.using_paths / ctx.cwd から純粋に path を合成（MVP では modules の厳密一致＋相対ヒントのみ）。
    - `resolve_modules(modules_json, using_entries_json, ctx)`
      - nyash.toml 相当の modules_json と UsingCollector 出力を突き合わせ、競合解決済み modules_json を返す予定（MVP では echo-back）。
    - `resolve_prefix(using_entries_json, modules_json, ctx)`
      - using_entries_json と modules_json から prefix 文字列を構成（MVP では空文字、Stage‑1 entry 側で prefix 結合だけを担当）。
- Stage‑1 UsingResolverBox との分担:
  - Stage‑1 UsingResolverBox は「JSON フロントから using_entries / modules_map を Region+next_i で収集・整形」する箱。
  - UsingResolveSSOTBox は「その結果をもとに最終的な modules/prefix を決める唯一の箱（SSOT）」として、IO なしで名前解決の意味論を持つ。
- pipeline_v2 UsingResolver（lang/src/compiler/pipeline_v2/using_resolver_box.hako）
  - 役割: modules_json 上で alias を解決する stateful helper（テキスト収集は entry 側）。
  - ループ: `loop(true)` で RegexFlow.find_from を使い key/tail マッチを走査する単一路。continue/backedge の多経路は無く、Region+next_i へのリライトは不要と判断。
  - 境界: entry 側が「ファイル読み込み＋using 収集」、pipeline_v2 側が「modules_json をもとに alias 解決」という分担で keep。

## テスト方針（構造固定）
- 既存: `src/tests/mir_stage1_using_resolver_verify.rs` を読み解き、期待 SSA が何をチェックしているか整理する。
- 追加候補（1〜2 本、軽量構造テスト）:
  - 「pos/next_pos が forward するだけの region」で PHI が揺れないこと。
  - `using` 収集で early-exit しても merge 後の `pos` が決定的になること。
  - いずれも LoopForm v2 経路（JSON front→Rust）で MirVerifier 緑を確認するスモークとして追加。
- 追加済みテスト（Rust `src/tests/mir_stage1_using_resolver_verify.rs`）でカバーする LoopForm パターン:
  - `mir_stage1_using_resolver_modules_map_early_exit_verifies`: Region+next_i + break（name=="" で早期終了）
  - `mir_stage1_using_resolver_modules_map_continue_and_break_verifies`: Region+next_i + continue/break 混在（"" を skip, "STOP" で break）
  - `mir_stage1_using_resolver_resolve_with_modules_map_verifies`: entries ループ＋modules_map 参照を同時に行う本線形
  - `mir_stage1_using_resolver_collect_entries_early_exit_verifies`: JSON スキャン＋early-exit sentinel で next_pos を決める Region+next_pos 形
  - `mir_stage1_using_resolver_module_map_regionized_verifies`: modules_list 分割（start/next_start）で MapBox set を行う Region 形
  - `mir_stage1_using_resolver_modules_map_continue_break_with_lookup_verifies`: entries ループ＋modules_map.has/get＋continue/break が同居する本線寄りパターン
- Rust 側テストの取り扱い:
  - `src/tests/mir_stage1_using_resolver_verify.rs` に追加した構造テストは cargo test 経路で維持する。v2 quick スモークへの昇格は実行時間とノイズを見つつ後続フェーズで再検討（今回の設計タスクでは据え置き）。
- 観測ログ: MIR dump を残す場合は dev オンリー（`NYASH_LOOPFORM_DEBUG` / `HAKO_LOOP_PHI_TRACE`）に限定し、ログ経路は docs にも記載しておく。

## 移行ガードレール
- 既存の動作を崩さないよう、トグル追加は慎重に（既定 OFF、既存経路不変）。
- ループリライト前後で Program(JSON) の shape が保たれているかを `NYASH_JSON_ONLY=1` で観測できるようにする。
- バグ時は Stage‑B と同様に block パーサ優先の形に戻せるよう、作業メモを CURRENT_TASK に残すこと。
