# Phase 25.1 — Stage‑1 CLI / BuildBox ループ & Box 依存分析メモ

目的
- `Stage‑1 CLI → BuildBox.emit_program_json_v0` 実行時に VM が step budget を食い尽くしている原因を、  
  ループ構造と BoxCall 依存の観点から整理しておくためのメモだよ。

## 1. 観測された症状（修正前スナップショット）

- コマンド例:
  ```bash
  NYASH_CLI_VERBOSE=2 \
  NYASH_USE_STAGE1_CLI=1 \
  STAGE1_EMIT_PROGRAM_JSON=1 \
  ./target/release/hakorune apps/tests/minimal_ssa_skip_ws.hako
  ```
-- 状態（修正前）:
  - `[stage1-bridge/trace]` ＋ `[stage1-cli/debug] emit_program_json ENTRY` が出ており、  
    Stage1CliMain.main/0 〜 stage1_cli.hako 実行までは到達している。
  - その後 `BuildBox.emit_program_json_v0` 実行中に VM が `vm step budget exceeded`（max_steps=1_000_000→2_000_000 でも NG）。
  - FileBox/ArrayBox などの plugin 未ロード警告も併発。
  - 当時は **ステップ上限＋プラグイン依存** が阻害要因となって、program-json 自体は得られていなかった。

## 1.1 現在の状態（2025-11 時点）

- ParserBox / StringHelpers / 各種 parser_* 箱のすべての  
  `loop(cont == 1) { if cond { ... } else { cont = 0 } }` 形式のループを、
  `loop(true) { if cond { ...; continue } break }` 形式に統一した。
- 代表ループ（`parse_program2` 本体、ws_init、セミコロン消費、string/number/map/array/control/exception/stmt/expr 系ループ）を一括で MirBuilder-friendly な形に寄せた結果、
  - `NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_PROGRAM_JSON=1 HAKO_VM_MAX_STEPS=2000000 NYASH_STAGE1_INPUT=apps/tests/minimal_ssa_skip_ws.hako ./target/release/hakorune`
    で **step budget 超過無し**、Program(JSON v0) が `{"version":0,"kind":"Program","body":[]}` として出力されることを確認済み。
  - VM の step budget エラーには fn/bb/last_inst ＋ Span（あれば .hako:line）が付くようになっており、将来の類似ケースも追いやすい。

## 2. emit_program_json_v0 周辺のループ構造（概観）

調査時点でのループ構造のざっくり分類（ファイルは概略）だよ。

| ループ                         | ファイル                     | 複雑度         | 危険度    |
|------------------------------|--------------------------|-------------|--------|
| `ParserBox.parse_program2`   | stage1_cli.hako:84 あたり | exponential | 🔴 最有力 |
| alias_table パース (opts)      | build_box.hako:60-73     | O(n²)       | 🟡 中   |
| alias_table パース (env)       | build_box.hako:94-107    | O(n²)       | 🟡 中   |
| `FileBox._read_file`         | stage1_cli.hako:67       | I/O bound   | ⚠️ 注視  |
| bundle_names 重複チェック         | build_box.hako:41-51     | O(n²)       | 🟢 許容  |

直感:
- alias_table や bundle_names の O(n²) 系ループは、入力サイズが小さい間は budget を食い尽くすレベルではない。
- `ParserBox.parse_program2` は Stage‑3 parser の statement/expression を全部受け持つため、  
  実装によっては **指数的** にブロック数・ステップ数が増え得る。
- FileBox は I/O bound なので、budget ではなくタイムアウト側のリスクが大きい。

## 3. Box 依存度マトリックス

Stage‑1 CLI / BuildBox ラインで使われている Box の洗い出しと、  
「core だけで足りるか／外部プラグインが要るか」のざっくり分類だよ。

| Box       | 用途                            | プラグイン依存 | 影響度         |
|-----------|-------------------------------|---------|-------------|
| FileBox   | `_read_file` で source 読み込み      | YES     | 🔴 CRITICAL |
| ParserBox | `parse_program2` AST parse    | NO      | 🟡 MEDIUM   |
| ArrayBox  | bundles 配列格納                  | NO      | 🟢 LOW      |
| StringBox | alias_table / bundle_names parse | NO      | 🟢 LOW      |

メモ:
- FileBox は core-ro だけでは厳しく、実際には plugin / env 設定に依存するケースが多い。
- ParserBox は Stage‑3 parser の仕様そのものなので、LoopForm/JoinIR と独立に「parser の loop 形」問題として見る必要がある。

## 4. 今後の観測テンプレ（提案）

ステップ budget 溶けの原因をもう少し切り分けるために、次のようなコマンドで実験できるよ。

```bash
# Test 1: FileBox なしで inline source を直書き
NYASH_STAGE1_MODE=emit-program \
STAGE1_SOURCE_TEXT='static box Main { main(args) { return 0 } }' \
./target/release/nyash lang/src/runner/stage1_cli.hako

# Test 2: プラグイン OFF で挙動を見る
NYASH_DISABLE_PLUGINS=1 \
NYASH_STAGE1_MODE=emit-program \
STAGE1_SOURCE_TEXT='static box Main { main(args) { return 0 } }' \
./target/release/nyash lang/src/runner/stage1_cli.hako

# Test 3: ステップトレースを有効化
NYASH_VM_STATS=1 \
NYASH_STAGE1_MODE=emit-program \
STAGE1_SOURCE_TEXT='static box Main { main(args) { return 0 } }' \
./target/release/nyash lang/src/runner/stage1_cli.hako 2>&1 | grep -i step
```

目的:
- FileBox を完全にバイパスできる inline source モードで試し、  
  それでも budget が溶けるかを見る（ParserBox 起因かどうかの切り分け）。
- プラグイン OFF/ON で挙動がどれだけ変わるか確認する。

## 5. 方針メモ

- Stage‑1 CLI の「ブリッジまでは動いているが、そのあとで止まる」という現象は、
  - Stage‑B/BuildBox 経由の Program JSON emit
  - ParserBox / FileBox のループ
  のどこに問題があるかを 1 ループずつ切り出していけば、局所化できる見込み。
- joinIR 側とは独立に、このファイルでは「Stage‑1 CLI 実行ルートのうち、どのループと Box が危ないか」の観測結果を積み上げていくよ。

## 6. VM エラー出力改善メモ（2025-XX）

- Rust MIR Interpreter の step budget エラーに **fn / bb / last_inst / steps** ＋ Span を付与したよ。
- 例（Span 付き MIR の場合）: `vm step budget exceeded (max_steps=2000000, steps=2000001) at bb=bb49 fn=ParserBox.parse_program2/1 last_inst_idx=12 last_inst_bb=bb48 last_inst=... (file.hako:312:7)`
- Stage‑1 CLI の現行 MIR には Span が載っていないため、いまは fn/bb/inst だけが出る（`apps/tests/minimal_ssa_skip_ws.hako` 実行で確認）。Span 付与を通せば .hako 行まで出る想定。
