# Phase 25.1d — Rust MIR SSA / PHI Smokes

Status: planning（構造バグ切り出しフェーズ・挙動は変えない／Rust側のみ）

## ゴール

- Rust MIR builder（`MirBuilder` + `LoopBuilder` + IfForm）の SSA / PHI 周りのバグを「Rust テスト／スモーク」で淡々と炙り出して潰すフェーズ。
- Stage‑B / Stage‑1 / selfhost で見えている ValueId 未定義問題を、Rust 側の最小ケースに還元してから直す。
- Nyash 側 MirBuilder（.hako 実装）は Phase 25.1c / 25.1e 以降に扱い、まずは Rust 層の PHI 不整合を止血する。

## 方針

- 新機能追加ではなく **テスト＋バグ修正のみ**。
- 1バグ1テストの原則で、「再現用 Hako もしくは AST 構築 → MirCompiler → MirVerifier」のパターンを増やしていく。
- 既に報告されている Undefined Value / non‑dominating use / Phi 不足を、そのまま Rust テストケースに落とし込む。

## タスク粒度（やることリスト）

1. **Stage‑B 最小ハーネスの Rust テスト化**
   - 既存: `lang/src/compiler/tests/stageb_min_sample.hako` + `tools/test_stageb_min.sh`。
   - やること:
     - Rust 側に小さなテストを追加（例: `src/tests/stageb_min_mir_verify.rs`）:
       - `Hako` → `AST` → `MirCompiler::compile(ast)` → `MirVerifier::verify_module`。
       - 期待: Stage‑B 最小サンプルのみを対象に Undefined Value が 0 件であること。
   - 目的: shell スクリプトに依存せず、`cargo test` ベースで Stage‑B 由来の MIR を検証できる足場を作る。

2. **単一関数向け PHI スモークの追加**
   - 対象関数（Rust 側で直接 AST を組む／Hako を読む）:
     - `TestArgs.process(args)` 型: `if args != null { local n = args.length(); loop(i < n) { ... } }`
     - `TestNested.complex(data)` 型: if + nested loop + method call。
   - やること:
     - 簡単な Hako を `tests/mir_phi_*` ディレクトリか `src/tests/*` に置き、MirCompiler でコンパイルして verifier を通すテストを書く。
     - ここでは Stage‑B を通さず、直接 Rust MirBuilder に食わせて PHI / recv の挙動を見る。

3. **LoopBuilder / IfForm の PHI 不整合の切り出し**
   - すでに verifier が報告している場所:
     - `JsonScanBox.seek_array_end/2` の non‑dominating use。
     - `Stage1UsingResolverBox._collect_using_entries/1` / `resolve_for_source/1` の Phi 不足。
     - `ParserBox.parse_program2/1` の merge block Phi 不足。
   - やること:
     - 各関数について「最小に削った MIR 再現ケース」を Rust テストとして切り出し（AST 直書きでもよい）、`MirVerifier` が通るように LoopBuilder / IfForm / PHI 挿入コードを修正する。
   - ポイント:
     - 1 関数ずつ、小さなテスト＋小さな修正で前に進める（大量に一気にいじらない）。

4. **Stage‑B 関数群の Rust スモーク**
   - `compiler_stageb.hako` から抜き出された関数:
     - `StageBArgsBox.resolve_src/1`
     - `StageBBodyExtractorBox.build_body_src/2`
     - `StageBDriverBox.main/1`
     - `Stage1UsingResolverBox.resolve_for_source/1`（Stage‑1 using 解決）
   - やること:
     - AST もしくは Hako→AST 変換経由で、これらの関数だけを MirCompiler にかけるテストを用意。
     - 各テストで `MirVerifier::verify_function` を呼び、Undefined Value / Phi 不足が無い状態を目標に、Loop/If lowering を順番に修正していく。
     - 特に `StageBArgsBox.resolve_src/1` については、`args.get(i)` のような Map/Array に対する `.get` が unified call 経由で誤って `Stage1UsingResolverBox.get` の Method callee に化けないこと（`box_name` が UnknownBox/MapBox のまま、receiver が常に定義済み ValueId であること）を Rust テストで固定する。

5. **Verifier 強化（Method recv / PHI に特化したチェック）**
   - 追加したいチェック:
     - `MirInstruction::Call` で `callee = Method{receiver: Some(r)}` のとき、`r` がその関数内で一度以上 `dst` として定義されているか。
     - Merge block で predecessor 定義値をそのまま読む場合に「Phi が必須」な箇所を強制エラーにする。
   - これを入れた上で、上記の小さなテストが全部緑になるように MirBuilder 側を直す。

## 箱化メモ（Stage‑B / Stage‑1 の責務分離）

- 観測されたバグ（`StageBArgsBox.resolve_src/1` 内で `Stage1UsingResolverBox.get` に化ける / 未定義 receiver）が示す通り、「Stage‑B CLI 引数処理」と「Stage‑1 using 解決」が Rust 側の型メタデータで混線している。
- Phase 25.1d 以降の設計メモとして、以下の箱化方針を採用する:
  - Stage‑B:
    - `StageBArgsBox` : CLI 引数 (`args` / env) から純粋な文字列 `src` を決めるだけの箱（Map/Array などの runtime 依存を最小化）。
    - `StageBBodyExtractorBox` : `src` 文字列から `box Main { method main(...) { ... } }` の本文を抜き出す箱（コメント除去・バランスチェック専任）。
    - `StageBDriverBox` : 上記 2 箱＋ ParserBox / FuncScannerBox を束ねて Program(JSON v0) を出すオーケストレータ。
  - Stage‑1:
    - `Stage1UsingResolverBox` : `[modules]` と `HAKO_STAGEB_MODULES_LIST` のみを入力に、using 展開済みソース文字列を返す箱。
    - Stage‑B からは「文字列 API（`resolve_for_source(src)`）」でのみアクセスし、Map/Array/JsonFragBox などの構造体を直接渡さない。
- Rust MirBuilder 側では:
  - static box ごとに `BoxCompilationContext` を必ず張る（`variable_map / value_origin_newbox / value_types` を box 単位で完全分離）。
  - ✅ **構造ガード実装済み**（2025-11-17）:
    - `CalleeBoxKind` enum追加: `StaticCompiler` / `RuntimeData` / `UserDefined` の3種別で Box種類を構造的に分類。
    - `classify_box_kind()`: Box名から種別を判定（Stage-B/Stage-1コンパイラBox、ランタイムDataBox、ユーザー定義Boxを明示的に列挙）。
    - `convert_target_to_callee()`: Callee::Method生成時にbox_kindを付与。
    - `apply_static_runtime_guard()`: 静的Box名とランタイム値の混線を検出・正規化:
      - box_kind==StaticCompiler かつ receiver型==同一Box名 → me-call判定、静的メソッド降下に委ねる。
      - box_kind==StaticCompiler かつ receiver型==異なるランタイムBox → 正規化（MapBox/ArrayBoxなど実際のruntime型に修正）。
    - 効果: `StageBArgsBox.resolve_src/1` 内の `args.get(i)` が `Stage1UsingResolverBox.get` に化ける問題を根絶。
    - ファイル: `src/mir/definitions/call_unified.rs`, `src/mir/builder/calls/call_unified.rs`, `src/mir/builder/calls/emit.rs`

## スコープ外

- Nyash 側 MirBuilder（`lang/src/mir/builder/*.hako`）の本格リファクタ。
  - ここは Phase 25.1c / 25.1e で箱化・モジュール化しつつ直す想定（receiver=0 ハードコード撤去など）。
- 新しい MIR 命令追加や意味論変更。
  - 既存の MIR 命令セットの範囲で SSA / PHI の整合性を取る。

## まとめ

- Phase 25.1d は「Rust MIR SSA / PHI のスモークを増やしてコツコツ直す」フェーズ。
- やることは単純で、やる量は多い:
  - 小さいテストを書く → verifier で赤を出す → LoopBuilder / IfForm / MirBuilder を直す → 緑になるまで繰り返す。
- これにより、Stage‑B / Stage‑1 / selfhost の土台となる Rust MIR 層が安定し、その上に Nyash selfhost 側の MirBuilder を載せやすくする。
- なお、Stage‑B 最小ハーネス（`stageb_min_sample.hako`）については、Rust MIR builder 経由の直接 VM / MIR verify は既に緑であり、残っている stack overflow は `compiler_stageb.hako` 側の Nyash ボックス連鎖に起因するものと考えられる。Rust 層では `emit_unified_call` / BoxCall / legacy 警戒の再入防止フラグと再帰深度カウンタを導入済みであり、以降は Nyash 側に浅い再帰ガードを置いて原因ボックスを特定するフェーズへ引き継ぐ。

### 実績メモ（Conservative PHI Box 完了）

- IfForm / LoopForm v2 の PHI 生成は「Conservative PHI Box」実装により根治済み:
  - If については `merge_modified_vars`（`src/mir/builder/phi.rs`）が **全変数の union に対して PHI を張る保守的実装**に切り替わり、
    - 片側の branch でしか定義されない変数（`then` のみ / `else` のみ）についても、
    - pre_if スナップショットまたは `const void` を使った安全な SSA 定義に統一。
  - Loop については LoopForm v2（`LoopFormBuilder`）側で header/exit PHI の扱いを整理し、
    - Carrier / Pinned / Invariant に分離したうえで exit PHI から `variable_map` を一貫して再束縛する構造に寄せた。
- Box 理論の観点では:
  - Conservative Box: まず「安全側」に全変数に PHI を張る（正しさ優先）。
  - Elimination Box: 将来の最適化フェーズで、使われない PHI を削る（効率最適化）。
- この Conservative PHI 実装により、Stage‑1 using resolver 一式の代表テスト:
  - `mir_parserbox_parse_program2_harness_parses_minimal_source`
  - `mir_stage1_using_resolver_min_fragment_verifies`
  - `mir_stage1_using_resolver_full_collect_entries_verifies`
  がすべて緑になっており、「片側 branch だけで定義された変数の non‑dominating use」系のバグは Rust 側では止血済み。***
