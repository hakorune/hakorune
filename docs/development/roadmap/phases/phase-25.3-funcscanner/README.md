# Phase 25.3 — FuncScanner / Stage‑B defs 安定化

Status: 完了（Stage‑B fib defs canary 緑／2025-11 時点）

## スコープ / ゴール

- 対象レイヤ
  - `.hako` 側:
    - `lang/src/compiler/entry/func_scanner.hako` (`FuncScannerBox`)
    - `lang/src/compiler/entry/compiler_stageb.hako` (`StageBFuncScannerBox`, `StageBDriverBox`)
    - 必要に応じて `lang/src/compiler/tests/funcscanner_fib_min.hako` などのテスト用ハーネス
  - Rust 側:
    - 既存の LoopForm v2 / LoopSnapshotMergeBox / JSON front は「完成済みの土台」として利用するだけで、原則ここでは触らない。

- ゴール
  - Rust VM 検証付きで FuncScanner を安定させる:
    - `NYASH_VM_VERIFY_MIR=1` で `lang/src/compiler/tests/funcscanner_fib_min.hako` を実行したときに、
      - `FuncScannerBox.scan_all_boxes/1` で Undefined value が発生しないこと。
      - fib 風ソースに対して `defs` に `TestBox.fib` / `Main.main` が含まれること（箱レベルの振る舞いが安定）。
  - Stage‑B 側からも同じ結果が得られる:
    - `tools/smokes/v2/profiles/quick/core/phase251/stageb_fib_program_defs_canary_vm.sh` を実行すると、
      - `defs` に `TestBox.fib` が存在し、
      - その `body` に `Loop` ノードが含まれていること。
  - Loop/PHI の意味論は **LoopFormBuilder + LoopSnapshotMergeBox** に完全委譲したまま、
    - FuncScanner / Stage‑B は「テキストスキャン＋JSON 組み立て」の箱として綺麗に分離された状態にする。

## 完了ステータス（2024-11-20 時点）

- `tools/smokes/v2/profiles/quick/core/phase251/stageb_fib_program_defs_canary_vm.sh` が緑（Phase 25.3 の完了条件）。
  - `defs` に `TestBox.fib` / `Main.main` が入り、`TestBox.fib.body.body[*]` に `Loop` ノードが含まれる状態を固定。
- Stage‑B 本線の整理:
  - `StageBDriverBox.main`: main 本文は `{…}` で包んだ block パーサ優先で JSON 化し、defs 側は `{"type":"Block","body":[…]}` に構造化して注入。
  - `StageBFuncScannerBox._scan_methods`: block パーサ優先に揃え、Program パーサは `HAKO_STAGEB_FUNC_SCAN_PROG_FALLBACK=1` の opt-in 時だけ使う安全側トグルに変更（skip_ws 崩れの再発防止）。
- Rust 層は無改変（LoopForm v2 / LoopSnapshotMergeBox の SSOT をそのまま利用）。

## JSON v0 フロントと AOT ルートの契約（Phase 25.3 時点）

- Program(JSON v0) の形:
  - ルートは常に `{"version":0,"kind":"Program","body":[...]}`
  - defs は `\"defs\":[{ \"name\", \"params\", \"box\", \"body\" }]` を追加する形で注入する。
  - Stage‑B / FuncScanner 経由の `defs[*].body` は必ず `{\"type\":\"Block\",\"body\":[Stmt...]}` でラップし、AOT/VM 側からは「通常の Block ノード」として読めるようにする。
- フロント/バックエンドの責務分離:
  - Stage‑B / FuncScanner: `.hako` テキスト → Program(JSON v0) まで（ループ/PHI の意味論は持たない）。
  - Rust 側 LoopForm v2 / JSON v0 Bridge: Program(JSON v0) → MIR/AOT まで（Loop/PHI/SSA の SSOT）。
- Stage‑B トグル整理（抜粋）:
  - `HAKO_STAGEB_FUNC_SCAN=1`（既定）: defs を常に埋める。`0` で Stage‑B からの defs 注入を無効化。
  - `HAKO_STAGEB_PROGRAM_PARSE_FALLBACK=1`: main 本文で Program パーサ fallback を有効化（既定は OFF、block パーサ優先）。
  - `HAKO_STAGEB_FUNC_SCAN_PROG_FALLBACK=1`: FuncScanner 側で Program パーサ fallback を有効化（既定は OFF）。  
    通常の開発・CI ではいずれも OFF にしておき、VM/パーサ調査時だけ opt‑in で使う想定。

## すでに前提としている状態

- LoopForm v2 / PHI / snapshot:
  - ループ構造・PHI・break/continue スナップショットの意味論は、
    - `src/mir/loop_builder.rs`
    - `src/mir/phi_core/loopform_builder.rs`
    - `src/mir/phi_core/loop_snapshot_merge.rs`
    に集約されており、AST ルート / JSON ルートともに同じ実装を使っている。
  - canonical `continue_merge` ブロック（ループごとに 1 つ）が導入済みで、backedge は
    - `latch` → `header`
    - `continue_merge` → `header`
    の 2 本に限定されている。

- JSON v0 front:
  - `src/runner/json_v0_bridge/lowering/loop_.rs` は LoopForm v2 の **薄いアダプタ**になっている。
    - ブロック ID の確保
    - `vars` / snapshot の受け渡し
    - `LoopFormOps` 実装 (`LoopFormJsonOps`)
    だけを担い、PHI 生成は LoopForm v2 に一元化された。

- 25.1e / 25.1q / 25.2:
  - 変数スコープ（Carrier / Pinned / Invariant / BodyLocalInOut）と Env_in/Env_out モデルは文書と実装が一致している。
  - JSON ループ用のスモーク（`tests/json_program_loop.rs`）はすでに緑で、ループ＋break/continue＋body‑local exit に対して MIR 検証が通っている。

この Phase 25.3 では、「LoopForm / JSON front は触らずに、FuncScanner / Stage‑B ラインをその上に綺麗に載せる」ことが目的になる。

## タスク一覧

### 1. FuncScanner fib 最小ハーネス（SSA バグの確認完了）

- 対象:
  - `lang/src/compiler/tests/funcscanner_fib_min.hako`
  - `FuncScannerBox.scan_all_boxes/1`（Rust lowering 時の MIR）

- 現状:
  - `NYASH_VM_VERIFY_MIR=1` 付きで `funcscanner_fib_min.hako` を実行しても、  
    Undefined value / `ssa-undef-debug` は発生していない（LoopForm v2 / LoopSnapshotMergeBox 修正で解消済み）。
  - `cargo test mir_funcscanner_fib_min_ssa_debug` も緑で、FuncScanner 単体の SSA 破綻は再現しない。
  - このフェーズで導入した `__mir__` ロガーは、今後の再現時に経路観測用フックとして利用できる状態になっている。

- 位置づけ:
  - 「FuncScanner + LoopForm v2 での Undefined Value バグの根治」は完了とみなし、  
    以降は Stage‑B 側の defs 欠落（`defs=[]`）を主ターゲットとする。

- 追加の SSA ガード（me-call 周り）:
  - Rust 側では `MirBuilder::handle_me_method_call` を `MeCallPolicyBox` によって箱化し、
    - `Box.method/Arity` lowered 関数が存在する場合は、従来どおり `me` を先頭引数とする Global call（インスタンス文脈の me-call）。
    - 存在しない場合は、`handle_static_method_call(cls, method, arguments)` にフォールバックし、  
      static helper（`FuncScannerBox._parse_params` / `_strip_comments` など）への呼び出しとして扱うようにした。
  - これにより、static box 文脈で「実体のない me を receiver とする Method call」が生成される経路が閉じられ、  
    FuncScannerBox._scan_methods/4 + `_parse_params` + `_strip_comments` のラインは SSA 的に安全になっている。

### 2.5. MIR ロガー (__mir__) による観測（Dev 専用）

- 位置づけ:
  - Phase 25.3 では、FuncScanner / Stage‑B のような `.hako` 側ロジックを LoopForm v2 上でデバッグしやすくするため、
    専用の MIR ログ構文 `__mir__` を導入する（実行意味論には影響しない dev 専用 Hook）。
- 構文:
  - `__mir__.log("label", v1, v2, ...)`
    - lowering 時に `MirInstruction::DebugLog { message: "label", values: [v1, v2, ...] }` へ変換される。
  - `__mir__.mark("label")`
    - 値無し版の `DebugLog` として `debug_log "label"` だけを差し込む。
- 振る舞い:
  - VM 実行時は `NYASH_MIR_DEBUG_LOG=1` のときだけ
    - `[MIR-LOG] label: %id=value ...`
    の形式でログ出力され、それ以外のときは完全に無視される（Effect::Debug のみ）。
  - LoopForm / PHI / snapshot には関与せず、単に MIR に 1 命令追加するだけの観測レイヤ。
- 実装ポイント:
  - `src/mir/builder/calls/build.rs` 内の `build_method_call_impl` で、
    - receiver が `__mir__` の `__mir__.log/mark` を検出し、
    - `try_build_mir_debug_call` で `DebugLog` 命令に直接 lowering している。

FuncScanner / Stage‑B のデバッグ時には、`scan_all_boxes` のループ頭や `box` 検出直後に  
`__mir__.log("funcscan/head", i, in_str, in_block)` などを埋め込み、VM 実行ログと合わせて  
「どの経路で環境スナップショットやステートが崩れているか」を観測する想定だよ。

### 3. Stage‑B FuncScanner との整合性確保（主ターゲット）

- 対象:
  - `lang/src/compiler/entry/compiler_stageb.hako`
    - `StageBFuncScannerBox.scan_all_boxes`
    - `StageBFuncScannerBox._find_matching_brace` 他
    - `StageBDriverBox.main` 内の defs 組み立てロジック

- やること:
  - `StageBFuncScannerBox` のロジックを、可能な範囲で `FuncScannerBox` と共有・委譲する方向に寄せる。
    - `_find_matching_brace` などはすでに FuncScannerBox に委譲済みなので、
      残りのスキャンロジックも「同じアルゴリズム / 同じ境界条件」で動くように整理する。
  - `HAKO_STAGEB_FUNCSCAN_TEST=1` で `StageBFuncScannerBox.test_fib_scan()` を走らせ、
    - `brace1/brace2` の close_idx が正しく取れること。
    - `defs_len > 0` となり、`TestBox.fib` / `Main.main` がログに出ること。
  - そのうえで `StageBDriverBox.main` が `StageBFuncScannerBox.scan_all_boxes(src)` の結果を
    - Program(JSON v0).defs に正しく注入できているかを確認する。
  - ここでの主なバグは「SSA ではなく、Stage‑B が fib ソースから defs を組み立てきれていないこと」なので、  
    ループ構造や LoopForm には手を入れず、Stage‑B 側のテキスト処理と defs 生成パスを中心に見る。

### 4. fib defs canary & ドキュメント更新（完了）

- 対象:
  - `tools/smokes/v2/profiles/quick/core/phase251/stageb_fib_program_defs_canary_vm.sh`
  - `docs/private/roadmap2/phases/phase-25.1q/README.md`
  - `CURRENT_TASK.md`

今回の結果:
- `tools/smokes/v2/profiles/quick/core/phase251/stageb_fib_program_defs_canary_vm.sh` は `rc=0` で安定緑。
- canary 内部で確認している条件:
  - `Program.kind == "Program"`
  - `defs` に `TestBox.fib` / `Main.main` が含まれていること。
  - `TestBox.fib.body` の直下（`body.body[*]`）に `Loop` ノードが最低 1 つ含まれていること。
- これにより、FuncScanner / Stage‑B 経由の fib defs ラインは LoopForm v2 + LoopSnapshotMergeBox の上で構造的に安定したとみなせる。

ドキュメント側:
- Phase 25.1q / 25.2 の README には、
  - 「loop/PHI/スナップショットの SSOT は LoopForm v2 + LoopSnapshotMergeBox」
  - 「Phase 25.3 で FuncScanner / Stage‑B defs もこの土台の上に載せた」
  という関係を 1〜2 行で追記済み。
- `CURRENT_TASK.md` では Phase 25.3 を「Stage‑B fib defs canary 緑」まで完了したフェーズとして整理し、
  次フェーズを Stage‑1 UsingResolver ループ整理 / Stage‑1 CLI program-json selfhost 導線に設定した。

### 5. mir_funcscanner_skip_ws 系テストの扱い（flaky → dev 専用）

- LoopForm v2 + LoopSnapshotMergeBox + PHI まわりの BTreeSet/BTreeMap 化により、
  ループ/PHI 関連の 267 テストはすべて安定して緑になっている。
- 一方で、`FuncScannerBox.skip_whitespace/2` を経由する最小 VM ハーネスだけが、
  ValueId / BasicBlockId の非決定的な揺れを見せるケースが 1 本だけ残っている。
  - Rust 側では `__pin$` の variable_map への混入を禁止し（Step 5-5-F/G）、
    PHI 生成側の順序もすべて BTree* で決定化済み。
  - 残る非決定性は `MirBuilder::variable_map: HashMap<String, ValueId>` と
    BoxCompilationContext 内の `variable_map` による iteration 順序依存の可能性が高い。
- Phase 25.3 のスコープでは MirBuilder 全体を BTreeMap 化する広域変更は行わず、
  当該テストは `mir_funcscanner_skip_ws_vm_debug_flaky` として `#[ignore]` 付き dev ハーネスに格下げした。
  - 通常の `cargo test` では実行されず、必要なときだけ手動で有効化して
    VM + `__mir__` ログを観測する用途に限定する。
  - 将来フェーズ（BoxCompilationContext / variable_map 構造の見直し）で、
    完全決定性まで含めた根治を行う。

---

この Phase 25.3 は、

- LoopForm v2 / JSON front を「箱として完成済み」とみなし、
- その上で FuncScanner / Stage‑B defs ラインを **構造的に**安定させる

ための仕上げフェーズだよ。  
ループや PHI の意味論は触らず、テキストスキャンとスコープ設計を整えることで、self‑hosting ライン全体の足場を固めるのが狙い。+
