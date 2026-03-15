# Phase 25.1b — Selfhost Builder Parity (Planning → Design Deep‑Dive)

Status: Step0〜3 実装済み・Step4（Method/Extern）実装フェーズ

## ゴール

- Rust 側 Program→MIR (`env.mirbuilder.emit`) と Hakorune 側 selfhost builder (`MirBuilderBox.emit_from_program_json_v0`) の機能差を埋め、Stage1 CLI（launcher.hako）レベルの Program(JSON) を selfhost builder 単独で lowering できるようにする。
- `.hako → Program(JSON v0) → MIR(JSON)` のうち、「Program→MIR」を selfhost builder だけでも成立させ、provider 経路はあくまで退避路に下げていく。
- 最終的には `HAKO_SELFHOST_BUILDER_FIRST=1` を既定に戻し、Stage1 CLI EXE の I/O（JSON stdout + exit code）を Rust/llvmlite と同じ契約に揃える。

## 現状（Phase 25.1a 時点）

### Stage‑B（Program(JSON v0) emit）

- `compiler_stageb.hako` は `defs` を含む Program(JSON v0) を出力できる:
  - `HakoCli.run` / `HakoCli.cmd_emit_*` / `HakoCli.cmd_build_*` などのメソッドを `Program.defs` 配列として含む。
  - `FuncScannerBox` ＋ `HAKO_STAGEB_FUNC_SCAN=1` により、`static box` メソッド（暗黙の `me` 引数付き）も defs に載る。
- using 解決:
  - `Stage1UsingResolverBox`（`lang/src/compiler/entry/using_resolver_box.hako`）＋ `HAKO_STAGEB_MODULES_LIST` で `nyash.toml` の `[modules]` を参照し、`using lang.mir.builder.MirBuilderBox` 等をファイル結合前に解決。
- Stage‑B entry 側は string literal using を廃止し、`using lang.compiler.entry.using_resolver as Stage1UsingResolverBox` のように module alias を使用する。

#### Stage‑B func_scan トグルのデフォルト（HAKO_STAGEB_FUNC_SCAN）

- 目的:
  - Stage‑B を直接叩いたときに `HAKO_STAGEB_FUNC_SCAN` を立て忘れても、`HakoCli.run` や `TestBox.fib` のようなメソッド定義が `Program.defs` にきちんと入るようにする（selfhost builder / FuncLowering 側の前提を崩さない）。
- 実装（compiler_stageb.hako 内）:
  - 以前: `HAKO_STAGEB_FUNC_SCAN=1` のときだけ `FuncScannerBox.scan_all_boxes` を呼び出し、それ以外は defs を生成しなかった。
  - 現在: `HAKO_STAGEB_FUNC_SCAN` が未設定 (`null`) のときは既定で ON とみなし、**明示的に `"0"` が入っているときだけ OFF** として扱う。
    - これにより、`tools/hakorune_emit_mir.sh` や v2 スモーク以外から Stage‑B を直接呼び出しても、defs が常に生成される。
    - 既存のテストで func_scan を無効化したいケースでは、`HAKO_STAGEB_FUNC_SCAN=0` を明示すれば従来どおり defs をスキップできる。

#### Stage‑B の安定度と使用上の注意

- 正規経路:
  - Stage‑B は `tools/hakorune_emit_mir.sh` / `tools/selfhost/selfhost_build.sh` 経由で呼び出すことを前提としており、これらのラッパが Stage‑3 用 ENV（`NYASH_PARSER_STAGE3=1` / `HAKO_PARSER_STAGE3=1` / `NYASH_PARSER_ALLOW_SEMICOLON=1` など）を一括でセットする。
  - Phase 25.1b では「multi-carrier fib などの core 小ケースについては、このラッパ経由で呼ぶ限り Stage‑B 自体は十分に安定」とみなし、主な改善対象を Program→MIR の selfhost builder 側に置く。
- 手動実行時の注意:
  - Stage‑3 ENV を立てずに Stage‑B / VM を直接叩くと、`Undefined variable: local` のようなエラーが発生するが、これは構文/実装バグではなく「Stage‑3 キーワード（local など）を Stage‑1 と同じルールでパースしてしまっている」ため。
  - 詳細な原因と対処は `docs/development/troubleshooting/stage3-local-keyword-guide.md` にまとめてあり、selfhost 開発では「まずラッパスクリプトを使う → 必要な場合のみ ENV を明示して直叩きする」方針とする。

#### Stage‑B と selfhost CLI canary（HakoCli.run/2）の現状

- Historical note (2026-03-15): the “launcher Program(JSON) / MIR is defs-less” part of this section is superseded. Current launcher Stage‑B output includes `HakoCli.*` defs, and `--program-json-to-mir` now preserves `user_box_decls`. The remaining launcher-exe blocker is entry argv handoff.

- selfhost CLI の最小ケース（`tools/smokes/v2/profiles/quick/core/phase251/selfhost_cli_run_basic_vm.sh` が生成する HakoCli.run サンプル）に対しては、修正前は Stage‑B 実行中に VM エラー:
  - `❌ VM error: Invalid value: use of undefined value ValueId(N)`（%0 / 97842 / 22 など）が発生し、Program(JSON v0) が 1 行としては出力されなかった（`tools/hakorune_emit_mir.sh` が Program 抽出に失敗する）。
- `NYASH_VM_VERIFY_MIR=1` を立てて `lang/src/compiler/entry/compiler_stageb.hako` を直接叩くと、修正前は Stage‑B が生成した MIR に対して:
  - `Stage1UsingResolverBox._collect_using_entries/1`
  - `ParserStringUtilsBox.skip_ws/2`
  - `ParserIdentScanBox.scan_ident/2`
  - `ParserBox.parse_stmt2/2`
  - などに `Undefined value %0 used in block ...` が多数報告されていた（詳細は `docs/private/roadmap/phases/phase-20.33/DEBUG.md` の「Invalid value: use of undefined value ValueId(N)」節を参照）。
- Task先生による Rust MIR builder 側の修正（ValueId 割り当て統一＋loop PHI/pinned 変数の扱い修正）後は:
  - `%0` / 97842 / 22 に起因する Undefined value / Invalid value エラーは `NYASH_VM_VERIFY_MIR=1` / 実行時ともに解消済み。
  - pinned 変数（`__pin$*@recv` など）も loop header/exit で正しく PHI に乗るようになり、ループ後のメソッドレシーバーが未定義になる問題も再現しなくなった。
- 現時点で selfhost CLI サンプルに対して残っている課題は:
  - 1) Rust VM 実行時に `❌ VM error: Invalid value: use of undefined value ValueId(17)` が発生しており、拡張済みエラーメッセージ（`fn=Main.main, last_block=Some(BasicBlockId(3419)), last_inst=Some(Call { ... ParserBox.length() [recv: %17] ... })`）から「Stage‑B Main.main 内の `ParserBox.length()` 呼び出しにおいて recv スロット（ValueId(17)) が定義されていない」ことが分かっている。これは verifier がまだチェックしていない「Callee.receiver 側の SSA 漏れ」であり、Phase 25.1c の Stage‑B / LoopBuilder / LocalSSA 整理タスクで修正する前提。
  - 2) `if args { ... }` まわりの truthy 判定（ArrayBox を boolean 条件に使っている部分）の扱いに起因する型/意味論の揺れが残っており、こちらも 25.1c の型システム整理タスクで `ArrayBox` の truthy 規約を明文化した上で揃える想定。
  - 3) `NewBox HakoCli` が plugin 前提で解決されてしまう問題は、VM 側の static box factory 統合（静的 Box を User factory にも広告する）により解消済みであり、`NYASH_DISABLE_PLUGINS=1` でも静的 Box として HakoCli を生成できるようになっている（selfhost CLI canary では NewBox 自体はもはやブロッカーではない）。
- 対応方針（Phase 25.1b 時点）:
  - BoxTypeInspector / multi‑carrier LoopForm 経路とは独立した **Stage‑B/MIR 側の SSA／型システム／Box 解決の構造問題** として扱い、selfhost CLI canary（HakoCli.run/2 lowering）はこれらが片付くまでは「25.1c の構造タスク待ち」として扱う。
  - `tools/hakorune_emit_mir.sh` の `diagnose_stageb_failure()` は維持し、Stage‑B の標準出力に `Invalid value: use of undefined value` が含まれている場合には `NYASH_VM_VERIFY_MIR=1`＋`compiler_stageb.hako` 直叩き、および `docs/private/roadmap/phases/phase-20.33/DEBUG.md` への導線を表示する。加えて、VM 側の `MirInterpreter::reg_load` が `fn` / `last_block` / `last_inst` を含めてエラー文字列を出すようになったため、Stage‑B 由来の undefined value は「どの関数のどの Call（どの recv）」で発生しているかを 1 行で特定できる。

### Rust provider (`env.mirbuilder.emit`)

- `program_json_to_mir_json_with_imports`:
  - `Program.body` と `Program.defs` の両方を受理し、`FuncDefV0` から `func_map` を構築して Call を解決。
  - `HAKO_MIRBUILDER_IMPORTS` 経由で `MirBuilderBox` / `BuildBox` などの static box 名をインポートし、`Const(String(alias))` として扱う。
- JSON v0 ブリッジ:
  - `args` を暗黙パラメータとして扱う修正済み（`undefined variable: args` は解消）。
  - `hostbridge` は well‑known 変数として `Const(String("hostbridge"))` を生成し、`hostbridge.extern_invoke(...)` を含む Program(JSON) でも undefined にならない。
- 結果:
  - `launcher.hako` に対して ~62KB の MIR(JSON) を安定して生成できており、Phase 25.1a では provider 経路が事実上のメインルート。

### selfhost builder (`MirBuilderBox.emit_from_program_json_v0`)

- エントリ:
  - 入口で `HAKO_MIR_BUILDER_FUNCS=1` のときに `FuncLoweringBox.lower_func_defs(program_json, program_json)` を呼び出し、defs 専用の MIR 関数群を文字列として受け取る（`func_defs_mir`）。
  - その後、`internal` 配下の多数の `Lower*Box.try_lower` を順番に適用し、Program 全体を 1 関数（`main`）ベースの MIR(JSON) に落とす。
- `FuncLoweringBox` の現状:
  - `lower_func_defs` は Program(JSON) から `defs` 配列をパースし、各 def ごとに `_lower_func_body` を呼ぶ。
  - `_lower_func_body` がサポートするのは **単一の Return を持つ最小パターンのみ**:
    - `Return(Int)`
    - `Return(Binary(op, lhs, rhs))`（`+,-,*,/` のみ、かつ `Int/Var` 組み合わせ限定）
    - `Return(Call(name, args))`（Call 名は `func_map` を用いて `Box.method` に解決）
  - 複数ステートメント、`If`、`Loop`、メソッドチェインなど Stage1 CLI に実際に現れる構造はすべて `null` でスキップされる。
- `MirBuilderBox` の挙動:
  - 何らかの `Lower*Box` が Program 全体にマッチした場合は、その MIR(JSON) に対して `_norm_if_apply` を通し、`FuncLoweringBox.inject_funcs` で defs 分の MIR 関数を **追加注入** する。
  - どの `Lower*Box` もマッチしないが `func_defs_mir` が非空の場合は、`"{\"functions\":[" + defs + "]}"` という最小モジュールを組み立てて `_norm_if_apply` に渡す。
    - このケースでは main 関数を含まない defs だけの MIR モジュールになり、ny-llvmc 側でエントリポイント不足や空挙動 EXE を生む原因になる。
  - `func_defs_mir` も空で internal lowers も不発の場合は `null` を返し、最後に provider delegate（`env.mirbuilder.emit`）へフォールバックする。
- 現時点での不足点（要約・2025-11-16）:
  - `FuncBodyBasicLowerBox` が本番でカバーできているのは、Local/If/Return の基本形＋LoopForm 正規化済み Loop＋ごく一部の MethodCall（`args.size/get` と `String.length` の Return 形）に限られる。Stage1 CLI のような複雑な関数本体（複数 Local＋If ネスト＋Loop＋Method/Extern 混在）は、ほぼすべて Rust provider 経路にフォールバックしている。
  - ExternCall は `hostbridge.extern_invoke("env.codegen","emit_object"|"link_object", arg)` を `ExternCallLowerBox` で最小サポートしているだけで、それ以外の extern（`env.mirbuilder.emit` や console 系など）は現状 selfhost builder の対象外。
  - `HakoCli.run` 専用 lower（`CliRunLowerBox`）は MVP 用のシンプルな run パターンのみを想定しており、実際の `launcher.hako` の run（usage/unknown ログ付き）は shape mismatch で selfhost 降ろしの対象外。ここを Stage1 実形に合わせて広げることが Phase 25.1b の中心タスクになっている。

### Stage1 EXE / build_stage1.sh の現状

- `tools/selfhost/build_stage1.sh`:
  - 既定値 `HAKO_SELFHOST_BUILDER_FIRST=0`（provider-first）では、Stage1 EXE ビルドは成功し、`emit program-json` / `emit mir-json` / `build exe` の I/O も Rust/llvmlite と同等の JSON+exit code 契約を満たす。
  - `HAKO_SELFHOST_BUILDER_FIRST=1`（selfhost-first）では、Stage1 CLI のような複雑な Program(JSON) に対して selfhost builder が「defs のみ」MIR か mini stub MIR を返し、結果として「Result: 0 だけ出す空 EXE」になる。
- スモーク:
  - `tools/smokes/v2/profiles/quick/core/phase251/stage1_launcher_program_to_mir_canary_vm.sh` は provider-first ルートのみをカバーしており、selfhost builder 単独経路のギャップは検出できていない（今後 canary を追加する必要がある）。

## 25.1b のスコープ（案）

- selfhost builder 本体（Hakorune 側）:
  - `Program.defs` の処理を実装し、`box + method` ごとに MIR 関数を生成する。
    - 例: `HakoCli.run` / `HakoCli.cmd_emit_program_json` / `HakoCli.cmd_emit_mir_json` / `HakoCli.cmd_build_exe` 等。
  - `call` / `BoxCall` / `ExternCall` の解決（`func_lowering` + call resolve 相当）を Hakorune 側にも実装し、`Call("cmd_emit_mir_json")` が `Global` callee に解決できるようにする。
  - Loop / branch / compare / Array/Map 操作など Stage1 CLI で出現する構造を包括的に lowering するため、`lang/src/mir/builder/internal/*` の helper を本番経路に統合する。
- JSON 出力:
  - 現状の `"{\"functions\":[...]}\"` ベタ書きから、jsonfrag 等を用いた構造的出力に切り替え、複数関数を同一モジュールに含められるようにする。
  - 既存の mini パターン用 JSON 組み立てとの互換性を維持しつつ、Stage1 CLI で必要な関数数に耐えられる形に拡張する。
- 運用ポリシー:
  - Phase 25.1b 中は `HAKO_SELFHOST_BUILDER_FIRST=0` のまま（provider-first）とし、selfhost builder が Stage1 CLI を lowering し切れることを確認した時点で `=1` への切り替えを検討する。
  - lambda/FunctionBox (`NewClosure` 等) は本フェーズでは扱わず、従来どおり builder からは排除したままにする（別フェーズ 25.2b に委ねる）。

## Guardrails / ポリシー

- Rust 側変更の方針（Self‑Host First / Minimal Core）:
  - Rust 側の Program→MIR 実装は「LoopForm / SSA / VM コア」の安定化に限定し、言語機能や高レイヤのロジックは .hako/selfhost 側で実装する。
  - 変更を入れる場合も、LoopForm v2 / PHI / VM バグ修正など **構造的な安定化・根治** に目的を絞り、広域な新機能追加や仕様変更は行わない。
  - selfhost builder 側は「Rust 実装をオラクルとして参照しつつ追従する」方針を維持する。
- Fail‑Fast:
  - selfhost builder が Program(JSON) の一部に対応していない場合は、明確なタグ付きで失敗させる（例: `[builder/selfhost-first:unsupported:Match]`）ようにし、silent stub には戻さない。
  - provider 経路は退避路として残しつつ、Stage1 CLI の代表ケースでは selfhost builder が先に成功することを目標にする。

### LoopForm / PHI ポリシー（重要メモ）

- ループを含む関数本体を selfhost builder で扱うときは、**LoopForm 正規化を前提にする**:
  - 可能な限り `docs/guides/loopform.md` で定義された「キャリア＋1個の φ」モデルに従う。
  - break/continue を含むループは、LoopForm の制約（更新変数最大2個・セグメント整列など）を満たす範囲でのみ lowering 対象にする。
- MirBuilder 側で「生の while/for を直接 MIR の PHI に落とす」ような ad‑hoc 実装は行わない:
  - PHI ノードの生成・配置は既存の LoopForm/LowerLoop 系 helper（`loop_scan_box.hako`／`lower_loop_*_box.hako` など）に一元化し、builder 本体はそれを利用する立場にとどめる。
  - LLVM harness 側の PHI 不変条件（ブロック先頭グルーピング／well‑typed incoming）を崩さない。
- Phase 25.1b では:
  - まず LoopForm 前提で安全に扱える最小のループ形（既存 selfhost テストでカバー済みの while/for）から対応し、
  - LoopForm 未適用の複雑なループ（例: キャリア3変数以上・ネストが深いもの）は `[builder/selfhost-first:unsupported:loopform]` タグなどで Fail‑Fast する。

#### LoopForm 複雑ケースへの拡張方針（Rust builder をオラクルに使う）

- ねらい:
  - 複雑な LoopForm（キャリア複数・条件付き更新など）については、**Rust 側 MirBuilder/LoopForm 実装を「正解（オラクル）」として扱い**、Hakorune 側の `LowerLoop*Box` 群をそれに追従させる。
  - Hakorune 側は LoopForm の設計や PHI 配線を再実装せず、「入力 JSON のパターンマッチ＋既存 LowerLoop* の呼び出し」に専念する。
- 手順イメージ:
  1. Rust 側 loop スモーク（例: `docs/private/roadmap2/phases/phase-17-loopform-selfhost/` や `phase-21.6/21.8` 関連）に対応する .hako を特定し、provider-first（`HAKO_SELFHOST_BUILDER_FIRST=0`）で MIR(JSON) を採取する。
  2. 同じ .hako を selfhost-first（`HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_FUNCS=1 HAKO_SELFHOST_TRACE=1`）で通し、`LowerLoop*Box` がどこまで拾えているか／どのケースが `[builder/funcs:unsupported:loopform]` になっているかを観測する。
  3. 差分が出ているループだけを対象に、小さな `LowerLoopXXXBox`（または既存 LowerLoop* の強化）を追加する。
  4. ループの意味論差異（キャリア更新・退出条件・rc）が出ていないかは、VM/EXE canary（rc チェック）で確認する。
- ガード:
  - 新しい LoopForm 対応はすべて既存 `lower_loop_*_box.hako` 群の中に閉じ込め、FuncLowering/MirBuilder 本体では依然として「LoopForm 結果を `_rebind` で名前付けするだけ」にとどめる。
  - Rust 側の LoopForm/PHI 実装を変えずに selfhost 側のカバー率だけを上げるのが Phase 25.1b の範囲。

#### 25.1b で追加する LoopForm 用の新箱（足場）

- `lang/src/mir/builder/internal/lower_loop_multi_carrier_box.hako`
  - 目的:
    - Fibonacci 風の「multi-carrier」ループ（`i,a,b,t` など複数のキャリアを持つループ）を selfhost builder 側で検出・LoopFormBox (`mode="multi_count"`) に委譲する。
  - 現段階の挙動（2025-11-16 時点）:
    - Program(JSON v0) 内で `Loop` + `Compare` を検出し、キャリア初期値（`local a = 0; local b = 1; ...`）を 2 個以上抽出。
    - `i < N` の `N` については、`Int` リテラルだけでなく `n` のようなパラメータ `Var` もサポートし、`limit_kind=const|param` として `LoopFormBox.build2` に伝達。
    - 成功時は `[mirbuilder/internal/loop:multi_carrier:detected:limit_kind=param,value=...|param_reg=...]` タグを確実に出力し、そのまま LoopForm から返ってきた MIR を `_rebind` する。
    - ループ limit が `local` 参照など selfhost で扱えない場合は `[mirbuilder/internal/loop:multi_carrier:limit:unsupported]` を出して `null` を返し、Rust provider 経路へ退避。
  - スモーク:
    - `tools/smokes/v2/profiles/quick/core/phase251/selfhost_mir_loopform_multi_carrier_vm.sh`
      - Stage‑B で `TestBox.fib(n)` を emit し、selfhost builder が `[funcs/basic:loop.multi_carrier]` を出したうえで `TestBox.fib/1` を含む MIR(JSON) を生成するかをチェック。
      - `carriers` 長や `limit_kind=param` ログを条件に PASS/FAIL を分岐（provider fallback 時は SKIP）。
  - 今後の拡張:
    - `LoopFormBox.build_loop_multi_carrier` の `limit_kind=param` 実装を一般化し（現在は param register コピー → `reg_limit` 初期化まで対応済み）、break/continue 付き multi-carrier も下ろせるようにする。
    - 代表ケースとして `tools/smokes/v2/profiles/quick/core/vm_loop_phi_multi_carriers.sh` と同型の .hako を selfhost-first で通す canary を追加し、VM/EXE rc を Rust オラクルと比較する。

## Next Steps（実装フェーズに入るときの TODO）

1. 調査:
   - Rust 側 `program_json_to_mir_json_with_imports` の挙動をトレースし、どの AST ノードがどの MIR に降りているかを整理（特に defs/call/loop/boxcall）。
   - selfhost builder の現行 JSON 生成経路を洗い出し、stub を生成している箇所を特定。
2. 設計:
   - `Program.defs` → MIR 関数生成のインタフェース（必要なフィールドと lowering 手順）を定義。
   - call resolve 用の軽量マップ（`name -> qualified`）を selfhost builder に導入する。
3. 実装:
   - defs 対応・call resolve・loop/branch lowering を段階的に導入しつつ、各ステップで mini スモークを追加。
   - jsonfrag ベースの出力に切り替えながら、既存の mini テストを全て通ることを確認。
4. 検証:
   - `tools/hakorune_emit_mir.sh lang/src/runner/launcher.hako …` を `HAKO_SELFHOST_BUILDER_FIRST=1` で実行し、62KB クラスの MIR(JSON) が selfhost builder 単独で得られることを確認。
   - `tools/selfhost/build_stage1.sh` を selfhost-first でビルドし、Stage1 CLI EXE が `emit`/`build` コマンドを正しく実行できるか（JSON stdout + exit code）をスモークで検証。

## 設計 TODO（FuncLoweringBox / MirBuilderBox 拡張の方向性）

※ ここから先は「具体的にどこをどう広げるか」の設計メモ。実装はまだ行わない。

1. FuncLowering の対応範囲拡張
   - `_lower_func_body` を Stage1 CLI で実際に使われているパターンまで広げる:
     - 単一 Return だけでなく、ローカル変数定義＋if 分岐＋loop を含む「典型的な CLI ハンドラ」の形をサポート。
     - `MethodCall`（`args.size()` / `args.get(i)` / `FileBox.open/read/write` / `ArrayBox.push` など）を MIR `mir_call` か `call` に落とす処理を追加。
   - `func_map` / `resolve_call_target` を用いて、`HakoCli.cmd_emit_*` / `cmd_build_exe` などの内部 Call を `Global("HakoCli.cmd_emit_*")` 系に正規化。

2. MirBuilder 本体の出力構造
   - これまでの「Program 全体を 1 関数 main に落とす」前提から、「Program.body + defs を multi‑function MIR モジュールに落とす」前提へシフト:
     - `Program.body` からはエントリ `Main.main/1` 相当の MIR 関数を生成。
     - `Program.defs` からは `HakoCli.*` などの補助関数を生成。
   - `_norm_if_apply` / `inject_funcs` の役割を整理し、「main 関数を含まない defs‑only モジュール」を返さないように Fail‑Fast する。

3. Fail‑Fast とデバッグ
   - Stage1 CLI のような大きな Program(JSON) に対して selfhost builder が未対応の場合は:
     - `[builder/selfhost-first:unsupported:func_body]` などのタグ付きで明示的に失敗。
     - provider 経路（`env.mirbuilder.emit`）へのフォールバックは維持するが、「空 EXE になる stub MIR」は生成しない方針に切り替える。
   - `HAKO_SELFHOST_TRACE=1` 時に、FuncLowering がどの def でどこまで lowering できたかをログに出す。

4. 検証計画
   - selfhost‑first canary:
     - `stage1_launcher_program_to_mir_canary_vm.sh` の selfhost‑first 版を追加し、`HAKO_SELFHOST_BUILDER_FIRST=1` ＋ `MirBuilderBox.emit_from_program_json_v0` だけで 60KB 級の MIR(JSON) を生成できることを確認。
   - Stage1 build:
     - `tools/selfhost/build_stage1.sh` を selfhost-first で回し、生成された Stage1 EXE に対して `emit program-json` / `emit mir-json` / `build exe` スモークを追加。

このファイルは引き続き「Phase 25.1b の計画メモ（設計ディープダイブ）」として扱い、実装は Phase 25.1a の安定化完了後に、小さな差分に分割して順次進める。***

---

## 実装計画（順番にやる TODO）

備考: ここでは Phase 25.1b を「複数の最小ステップ」に分解して、順番/ゴール/ガードを具体的にメモしておくにゃ。

### Step 0 — Fail‑Fast・観測を揃える
- Status: implemented (2025-11-15). `MirBuilderBox` now tags `defs_only` / `no_match` failures and aborts, and `FuncLoweringBox` logs unsupported defs when `HAKO_SELFHOST_TRACE=1`.
- 目的: 既存の selfhost builder がどこで諦めているかを正確に観測し、stub MIR を返さずに Fail させる導線を整える。
- 作業:
  - `MirBuilderBox.emit_from_program_json_v0`
    - `func_defs_mir` だけが非空だった場合でも黙って `{ "functions": [defs] }` を返さず、`[builder/selfhost-first:unsupported:defs_only]` を出す。
    - internal lowers がすべて `null` の場合、`[builder/selfhost-first:unsupported:<reason>]` のタグを付与。
  - `FuncLoweringBox.lower_func_defs`
    - どの関数名で `_lower_func_body` が `null` を返したかを `HAKO_SELFHOST_TRACE=1` でログ出力。
  - `tools/hakorune_emit_mir.sh`
    - 既存の head/tail ログ出力で `[builder/selfhost-first:*]` タグがそのまま表示されることを確認済み（追加改修なし）。
    - Phase 25.1b 以降、`HAKO_SELFHOST_BUILDER_FIRST=1` で呼び出された場合は「Stage‑B → selfhost builder（＋Stage1 CLI）」経路のみを試行し、selfhost builder が失敗した場合は即座に非0で終了する。selfhost-first モードでは `env.mirbuilder.emit` / provider delegate へのフォールバックは行わず、MirBuilder の未整備を隠さない方針とする（provider 経路を使うときは `HAKO_SELFHOST_BUILDER_FIRST=0` を明示）。
- 成果物:
  - selfhost-first で Stage1 CLI を通したときに、どの関数/構造がまだ未サポートなのかがログで推測できる状態。

#### 補足: Ny selfhost パイプラインとの関係（Phase 25.1b 時点）
- `.hako → Program(JSON v0) → MIR(JSON)` のメイン経路は、Stage‑B（`compiler_stageb.hako`）と MirBuilder/selfhost builder で完結させる。Ny selfhost（`src/runner/selfhost.rs`）は `.ny` 用の補助ルートとして扱い、Phase 25.1b のスコープからは外す。
- `NYASH_USE_NY_COMPILER`:
  - Phase 25.1b で「明示 opt-in」（既定=0）に変更。Runner は `NYASH_USE_NY_COMPILER=1` が立っているときだけ selfhost パイプライン（Ny→JSON v0）を試行し、それ以外では従来どおり Rust parser/JSON v0 bridge を使う。
  - `.hako` 実行や `tools/hakorune_emit_mir.sh` 経由の Stage‑B/MirBuilder/selfhost builder には影響しない（これらは `NYASH_USE_NY_COMPILER=0` / `NYASH_DISABLE_NY_COMPILER=1` で起動）。
- Python MVP:
  - `tools/ny_parser_mvp.py` は Phase 15 時点の Ny→JSON v0 実験用ハーネスであり、Phase 25.1b では `NYASH_NY_COMPILER_USE_PY=1` のときだけ有効にする。
  - 既定では Ny selfhost パイプラインから Python には落ちない（脱 Python 方針に合わせて dev 専用の補助線に格下げ）。
- inline selfhost compiler（`inline_selfhost_emit.hako`）:
  - `try_run_selfhost_pipeline` の最終手段として、`using lang.compiler.parser.box as ParserBox` / `using lang.compiler.stage1.emitter_box as EmitterBox` を含む小さな Hako を生成し、`ParserBox.parse_program2`→`EmitterBox.emit_program` で JSON v0 を得る経路が残っている。
  - 現状、この inline 経路は `.ny` の大きなソースに対して 60s タイムアウト＋ `Undefined variable: local` を伴うことがあり、ParserBox/Stage‑3/using 周りに無限ループ相当のバグが残っている疑いがある。
  - Phase 25.1b では `.hako` selfhost builder から Ny selfhost 経路を切り離すことを優先し、inline 経路のバグは `[ny-compiler] inline timeout ...` ＋ `[ny-inline:hint]`（stdout/stderr の head を添える）で可視化したうえで、後続フェーズ（25.1c 以降）の構造タスクとして扱う。

### Step 1 — defs injection の再設計
- Status: initial-implemented（main 必須チェックはトグル付き; multi-function への完全移行は後続 Step）。
- 目的: `FuncLoweringBox.inject_funcs` で main 関数の有無を意識し、multi-function モジュールの土台を整える。
- 作業:
  - `inject_funcs` 内で `HAKO_MIR_BUILDER_REQUIRE_MAIN=1` のときに `"name":"main"` を含まない `functions` 配列を拒否し、`[builder/funcs:fail:no-main]` をトレース出力して injection をスキップする（既定では OFF なので既存挙動は維持）。
  - 将来フェーズで、`Program.body` から生成した main と defs を「2段構え」でマージする API（main 名の受け渡しなど）を追加する。
- 成果物（現段階）:
  - Env トグルを有効化した状態では「main を含まない MIR に defs だけ差し込む」ケースを検知できるようになり、Stage1 実装時に安全に stricter モードへ移行する足場ができた。

### Step 2 — `_lower_func_body` の拡張（ローカル＋if）
- 目的: Stage1 CLI の `HakoCli.cmd_emit_program_json` のような「Local / Assign / If / Return だけで構成された関数」を selfhost builder で lowering できるようにする。
- 作業:
  - Body を JsonFrag で走査し、ローカル導入・代入・分岐を MIR ブロックへ展開する最小ロジックを FuncLoweringBox に追加。
  - Loop が出た時は `[builder/funcs:unsupported:loop]`（仮タグ）を出して Fail-Fast（Step 3 で LoopForm 対応を行う）。
- 成果物:
  - Loop を含まない defs は selfhost builder で MIR 関数にできるようになり、Stage1 CLI の emit/build ハンドラの半分程度を selfhost パスで賄える。

### Step 3 — Loop（LoopForm）の受理
- Status: initial-implemented (2025-11-15). `FuncBodyBasicLowerBox` now calls `LowerLoopSumBcBox`/`LowerLoopSimpleBox` from `_try_lower_loop`, tags unsupported loops with `[builder/funcs:unsupported:loopform]`, and delegates all PHI/carrier handling to LoopForm lowers.
- 目的: LoopForm 正規化済みの while/for を MIR に落とす。ループの正規化・PHI設計はLoopForm/既存lower_loop系Boxに任せ、FuncLowering/MirBuilder側はそれを使うだけにする。
- 作業内容（実装済み）:
  - `FuncBodyBasicLowerBox`に`_try_lower_loop`メソッド追加:
    - Loop判定 → `LowerLoopSumBcBox.try_lower` → `LowerLoopSimpleBox.try_lower` の順に試す。
    - 成功時は`_rebind`で関数名を`Box.method/arity`に付け替え。
    - 失敗時は`[builder/funcs:unsupported:loopform]`でFail-Fast。
  - `lower`メソッド冒頭でLoop優先処理:
    - Loop含む場合は`_try_lower_loop`を呼び、成功/失敗で明確に分岐。
    - Loopが無い場合のみ既存のLocal/If/Return処理に進む。
  - PHI地獄防止ポリシー徹底:
    - FuncBodyBasicLowerBox/FuncLowering側でPHIやキャリアを直接いじらない。
    - LoopForm制約外は必ずタグ付きでFail-Fast（Rust providerに退避可能）。
- 成果物:
  - `cmd_build_exe`の`loop(i < argc)`等、Stage1 CLIの代表的なwhile/forパターンをselfhost builderで通せる基礎が整った。
  - 追加アップデート（2025-11-16）: multi-carrier ループ（`TestBox.fib(n)` など）も `LowerLoopMultiCarrierBox` → `LoopFormBox.build_loop_multi_carrier` 経由で selfhost lowering できるようになり、limit が `Int` でなく `Var(n)` でも `[mirbuilder/internal/loop:multi_carrier:detected:limit_kind=param,...]` を出して処理できる。
  - 次のステップ: LoopForm対応の動作確認スモークテスト追加、Step4（MethodCall/ExternCall）へ進む。

#### Step 3.1 — Box 型情報 API（Rust Parity）★New
- 背景:
  - Stage‑3 VM では `"" + MapBox` のような「Box を文字列に暗黙変換する演算」が禁止されており、既存の `JsonEmitBox` / `BoxHelpers`（`LoopOptsBox.build2` から呼び出される）が `repr` 判定に依存しているため multi-carrier の JSON 生成が `Type error` で停止した。
  - Rust 側の MirBuilder は enum で型が決まっており `match` で分岐できる。Hakorune 側でも同等の「Box の種別を問い合わせる API」を用意して文字列ハックを撤廃する必要がある。
- 設計方針:
  1. `lang/src/shared/common/box_type_inspector_box.hako` を追加し、`BoxTypeInspectorBox.kind(value)` / `is_map(value)` / `is_array(value)` 等の API を提供する。
  2. 実装は Stage0 Rust 側に `env.box_introspect(kind, value)` 的な extern を追加し、`hostbridge.extern_invoke("env.box_introspect","kind",[value])` で種別名（例: `"MapBox"`, `"ArrayBox"`, `"Int"`）を返す。
  3. `BoxHelpers` / `JsonEmitBox` / `LoopOptsBox` など、Box 種別チェックが必要な箇所はすべてこの API に置き換え、`"" + value` を一切使わない。
  4. 返り値は最小で `Null/Bool/Int/String` と `MapBox/ArrayBox/HostHandleBox`（Stage1 で使用する型）をカバーし、将来的に `type_id` などを拡張する。
- 追加で行うこと:
  - `CURRENT_TASK.md` に Box 型 API 実装タスクを追加し、LoopForm multi-carrier の JSON 出力がこの API 依存であることを明示。
  - Stage0 側での対応（`env.box_introspect` 新規 extern）の設計も合わせて `phase-25.1b/README.md` に記述しておく（Selfhost 側で API 追加→Rust 側 stub→VM 反映の順）。
  - 現状（2025-11-16 時点）: Stage‑3 VM 経路で `BoxTypeInspectorBox.kind` / `is_map` / `is_array` が MapBox / ArrayBox を正しく認識し、小さな Hako テストで `hostbridge.extern_invoke("env.box_introspect","kind",[value])` → `env.box_introspect.kind` provider → plugin loader v2 の BoxIntrospect 実装までが end‑to‑end で動作することを確認済み。
  - fib multi‑carrier 経路と selfhost multi‑carrier smoke 用の canary ケース（`tools/smokes/v2/profiles/quick/core/phase251/selfhost_mir_loopform_multi_carrier_vm.sh`）は、2025‑11‑16 時点で `env.box_introspect.kind` provider 経路＋BoxTypeInspector 経由の multi-carrier LoopForm で PASS 済み。ログに `[mirbuilder/internal/loop:multi_carrier:detected:limit_kind=param,...]` と `[funcs/basic:loop.multi_carrier] -> TestBox.fib/1` が現れ、出力 MIR(JSON) に `"name":"TestBox.fib/1"` が含まれることを確認したため、「env.box_introspect.kind provider 経路完了 / multi‑carrier selfhost-first canary PASS」とみなす。

### Step 4 — MethodCall / ExternCall パリティ（設計メモ・Rust層読解込み）
- Status: design-only（Rust 層の挙動を踏まえた設計まで）
- 目的: `hostbridge.extern_invoke` / `FileBox` / `ArrayBox` など Stage1 CLI で多用される呼び出しを selfhost builder でも再現し、Rust 側の `build_method_call` / extern handler と意味論を揃える（ただしスコープは Stage1 必要最小限に限定）。
- 対象（Phase 25.1b で扱う範囲に限定）:
  - Stage1 CLI (`lang/src/runner/launcher.hako`) 内で出現する代表パターン:
    - `FileBox` 系: `fb.open(path,"r")` / `fb.read()` / `fb.write(content)` / `fb.close()`
    - `ArrayBox` 系: `args.size()` / `args.get(i)` / `args.push(v)`
    - `MapBox` 系（必要になれば）: `m.get(k)` / `m.set(k,v)` / `m.size()`
    - `String` 系: `s.length()`（現状 Step2 でも使われている）
    - self-call: `me.cmd_emit_program_json(args)` / `me.cmd_emit_mir_json(args)` / `me.cmd_build_exe(args)` など
    - ExternCall 的なもの: `hostbridge.extern_invoke("env.codegen","emit_object",args)` / `hostbridge.extern_invoke("env.codegen","link_object",args)`

- 設計方針:
  1. **MethodCall → mir_call(Method)**（box メソッド呼び出し）
     - Stage‑B Program(JSON v0) での形（実測）:
       - `return arr.size()` は defs 内で次のように現れる:
         ```json
         {"type":"Local","name":"arr","expr":{"type":"New","class":"ArrayBox","args":[]}}
         {"type":"Return","expr":{
           "type":"Method",
           "recv":{"type":"Var","name":"arr"},
           "method":"size",
           "args":[]
         }}
         ```
       - `hostbridge.extern_invoke("env.codegen","emit_object", a)` は:
         ```json
         {"type":"Expr","expr":{
           "type":"Method",
           "recv":{"type":"Var","name":"hostbridge"},
           "method":"extern_invoke",
           "args":[
             {"type":"Str","value":"env.codegen"},
             {"type":"Str","value":"emit_object"},
             {"type":"Var","name":"a"}
           ]
         }}
         ```
     - FuncLoweringBox / FuncBodyBasicLowerBox 側で扱う基本パターン:
       - `Return(Method recv.method(args))` を検出し、
         - `recv` が **パラメータ由来の Var**（例: `args.size()`）のときだけ selfhost lowering 対象にする。
         - ローカル由来（`local fb = new FileBox(); return fb.read()`）は Phase 25.1b では対象外とし、今後のフェーズでローカルテーブル導入後に扱う。
       - 引数は Int/Var/String のみ対象（lambda や複合式は未対応）。
     - MIR 側では `mir_call` の Method 形を使う:
       ```json
       {"op":"mir_call","dst":R,
        "mir_call":{
          "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":recv_reg},
          "args":[ /* 必要に応じて追加 */ ],
          "effects":[]
        }}
       ```
       - `box_name` / `method` は whitelisted な Box のみ（当面は `ArrayBox` と `String` 程度）をハードコードで対応。
       - receiver は `receiver` フィールド（または args の先頭）でレジスタ番号を渡し、Rust 側の `mir_call` 仕様と揃える。

  2. **self-call（me.cmd_*）の解決**
     - Stage1 CLI の `me.cmd_*` は、Stage‑B の FuncScanner から `Program.defs` に `box:"HakoCli", name:"cmd_*"` として載る。
     - これを FuncLoweringBox の `func_map` に登録済みなので、
       - `Call("cmd_emit_mir_json", args)` → `func_map("cmd_emit_mir_json") = "HakoCli.cmd_emit_mir_json"`
       - という形で Global 関数名を解決できる。
     - Step4 では `Return(Call("cmd_*", ...))` だけでなく、「単独の Call 文」や「MethodCall 内からの self-call」も対応させる余地があるが、
       - Phase 25.1b ではまず `Return(Call(...))` パターンの範囲内で self-call を `Box.method/N` に揃えるところまでに留める（広げるのは後続フェーズ）。

  3. **ExternCall（hostbridge.extern_invoke）の扱い**
     - Rust 側では `hostbridge.extern_invoke("env.codegen","emit_object",args)` 等を特別扱いし、C-ABI 経由で `env.codegen` provider にルーティングしている。
     - selfhost builder 側では、Stage1 CLI の以下のパターンのみをサポート対象とする:
       - `"env.codegen","emit_object",[mir_json]`
       - `"env.codegen","link_object",[obj_path,(out_path)]`
       - `"env.mirbuilder","emit",[program_json]`（必要なら）
     - JSON 上では `expr.type:"Method"` ＋ `recv:Var("hostbridge")` ＋ `method:"extern_invoke"` で表現されているので、
       - `args[0]` / `args[1]` が `"env.codegen"`, `"emit_object"` or `"link_object"` であることを確認し、
       - static なパターンマッチで MIR の `extern_call` に落とす:
         ```json
         {"op":"externcall","func":"env.codegen.emit_object","args":[ /* regs */ ]}
         ```
     - ここでは「すべての extern を一般化する」のではなく、Stage1 CLI が実際に使っている env 名とメソッド名だけを point fix する（Rust Freeze Policy に従い、意味論は Rust 版を真似るが範囲は狭く保つ）。

  4. **未対応パターンの Fail-Fast**
     - MethodCall/ExternCall の lowering 中に、
       - 複雑なオブジェクト式（ネストした MethodCall/Array/Map リテラルなど）、
       - 引数に対応していない型（lambda など）、
       - 未サポートの env 名 / メソッド名（`env.codegen` 以外）、
       が見つかった場合は、`[builder/funcs:unsupported:call]` タグを出して `null` で戻る。
     - これにより、「知らない形をなんとなく MIR にする」ことを避け、Rust provider や legacy CLI delegate に退避できるようにする。

#### Step 4.1 — Rust 層 Call/ExternCall 契約の整理（移植元 SSOT）

- 目的:
  - Stage1 側の MethodCall/ExternCall lowering を「Rust 実装の振る舞い」に正確に揃えるため、Rust 層の Call/ExternCall/hostbridge 経路を SSOT として整理しておく。
  - ここでの整理は構造レベルに留め、意味論の“拡張”は行わない（Hako 側はこの契約に従うだけ）。

- Rust 側のコア断面（ざっくり構造）:
  - **MIR ビルダ（呼び出し生成）**:
    - `src/mir/builder/builder_calls.rs`
      - `emit_unified_call(dst, CallTarget, args)`:
        - `CallTarget::Method { box_type, method, receiver }` → `Callee::Method` を作り、`MirInstruction::Call { callee: Some(Callee::Method{..}), ... }` を emit。
        - `CallTarget::Extern(name)` → 文字列 `"env.codegen.emit_object"` などを `ExternCall` に変換（`iface_name="env.codegen"`, `method_name="emit_object"`）。
        - `CallTarget::Global(name)` → `Callee::Global(name)` 付き `Call` を emit（`execute_global_function` へ）。
  - **VM 側 Call ハンドラ**:
    - `src/backend/mir_interpreter/handlers/calls/global.rs`:
      - `execute_global_function(func_name, args)`:
        - まず `functions` テーブルにあれば module 内関数として実行。
        - そうでない場合、`normalize_arity_suffix("name/1")` した base 名に対して:
          - `"print"` → `execute_extern_function("print", args)`。
          - `"hostbridge.extern_invoke"` → `execute_extern_function("hostbridge.extern_invoke", args)`（SSOT: hostbridge 経由の extern は必ずここを通る）。
          - `"env.mirbuilder.emit"` / `"env.codegen.emit_object"` / `"env.codegen.link_object"`:
            - それぞれ `crate::host_providers::{mir_builder,llvm_codegen}` を直接呼ぶ「グローバル関数版」ルート。
    - `src/backend/mir_interpreter/handlers/calls/externs.rs`:
      - `execute_extern_function(iface, method, args)`:
        - `("env.mirbuilder","emit")` / `("env.codegen","emit_object")` / `("env.codegen","link_object")` などを `extern_provider_dispatch` に委譲。
        - `"hostbridge.extern_invoke"` base 名もここから `extern_provider_dispatch("hostbridge.extern_invoke", args)` に流す。
  - **ExternCall / hostbridge.extern_invoke の provider**:
    - `src/backend/mir_interpreter/handlers/externals.rs`:
      - ExternCall 形（`MirInstruction::ExternCall`) を `iface_name`,`method_name` ごとに振り分け:
        - `("env.mirbuilder","emit")` → `extern_provider_dispatch("env.mirbuilder.emit", args)`。
        - `("env.codegen","emit_object")` → `extern_provider_dispatch("env.codegen.emit_object", args)`。
        - `("env.codegen","link_object")` → 第3引数 ArrayBox `[obj_path, exe_out?]` を取り出して C-API ルートへ。
        - `("hostbridge","extern_invoke")` → `extern_provider_dispatch("hostbridge.extern_invoke", args)`（なければ Invalid）。
    - `src/backend/mir_interpreter/handlers/extern_provider.rs`:
      - `extern_provider_dispatch(key, args)`:
        - `"env.mirbuilder.emit"`:
          - `args[0]` を `program_json` にし、`HAKO_MIRBUILDER_IMPORTS` から imports マップを読む。
          - `host_providers::mir_builder::program_json_to_mir_json_with_imports` を呼んで MIR(JSON) 文字列を返す。
        - `"env.codegen.emit_object"`:
          - `args[0]` を MIR(JSON) 文字列にして v1 へ normalize → `llvm_codegen::mir_json_to_object`。
        - `"env.codegen.link_object"`:
          - `args[0]`=obj_path, `args[1]`=exe_out を文字列化し、C-API ルート（`NYASH_LLVM_USE_CAPI=1` + `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`）で `link_object_capi`。
        - `"env.get"` / `"env.box_introspect.kind"` / `"hostbridge.extern_invoke"` もここで扱う（BoxIntrospect は plugin_loader_v2 に委譲）。

- plugin_loader v2 側の env.*:
  - `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`:
    - `extern_call(iface_name, method_name, args)` で `env.*` を一括処理。
    - `handle_mirbuilder("emit", args)`:
      - `args[0]` の Program(JSON v0) 文字列を受け取り、`host_providers::mir_builder::program_json_to_mir_json` で MIR(JSON v0) を返す。
    - `handle_codegen("emit_object", args)`:
      - `args[0]` の MIR(JSON v0) 文字列を受け取り、ny-llvmc ラッパ (`llvm_codegen::mir_json_to_object`) で object (.o) のパスを返す。

- Bridge（JSON v0 → MIR）の特別扱い:
  - `src/runner/json_v0_bridge/lowering/expr.rs` / `lowering.rs`:
    - `MapVars::resolve`:
      - `hostbridge` / `env` を特殊変数として扱い、それぞれ Const(String) `"hostbridge"` / `"env"` を生成する（Method チェーンを降ろすためのプレースホルダ）。
      - `me` については、Bridge 環境の `allow_me_dummy` が ON のときだけ NewBox を注入する（通常は JSON defs 側で明示パラメータとして扱う）。
    - `lower_expr_with_scope`:
      - `ExprV0::Extern { iface, method, args }` → `MirInstruction::ExternCall { iface_name, method_name, ... }`。
      - `ExprV0::Method` の特別ケース:
        - `ConsoleBox` の `print/println/log` → `ExternCall env.console.log`。
        - `env.box_introspect.kind(value)` パターン → `ExternCall env.box_introspect.kind` に正規化。
    - defs 降下（`lowering.rs`）:
      - JSON v0 の `defs` に対して、`box_name != "Main"` の関数を **インスタンスメソッド** とみなし、
        - `signature.params` に「暗黙 `me` + 明示パラメータ」を載せる。
        - `func_var_map` に `me` → `func.params[0]` を、残りのパラメータ名を `params[1..]` にバインドする。
      - これにより Stage‑B / Stage‑1 側で `_build_module_map()` のような「params: [] だが `me` を使う」メソッドでも、
        Rust VM 実行時に `me` 未定義にならず、BoxCall が正しく解決されるようになった。

### IfForm / empty else-branch の SSA fix（Stage‑1 UsingResolverFull 対応）

- `src/mir/builder/if_form.rs`:
  - `if cond { then }` のように else ブランチが省略されたケースでも、
    - `else` の入口で pre_if の `variable_map` を使って PHI ノードを生成し、
    - その結果の `variable_map` を `else_var_map_end_opt = Some(...)` として merge フェーズに渡すように修正した。
  - 以前は empty else の場合に `else_var_map_end_opt` が `None` になり、`merge_modified_vars` が pre_if 時点の ValueId にフォールバックしていたため、
    - merge ブロックで古い ValueId（PHI 適用前の値）を参照し、`Undefined value %0` などの SSA violation を引き起こしていた。
  - 修正後は、then/else 両ブランチで「PHI 適用後の variable_map」が merge に渡されるため、
    empty else でもヘッダ/merge の SSA が崩れない。

- 検証:
  - `src/tests/mir_stage1_using_resolver_verify.rs::mir_stage1_using_resolver_full_collect_entries_verifies` が、
    - `MirVerifier` 緑（UndefinedValue/InvalidPhi なし）、
    - `Stage1UsingResolverFull.main/0()` の merge ブロックで PHI 後の値（例: `%24`）を正しく参照していることを MIR dump で確認。

- Selfhost への移植指針（Rust SSOT に沿った箱設計）:
  - `MethodCall`:
    - Hako 側では「どの Box のどのメソッドを MIR の `mir_call(Method)` に落とすか」を Box 単位の helper で管理する（`LoopOptsBox` や `Cli*Box` と同様に）。
    - Rust 側の `CallTarget::Method` → `Callee::Method` の変換ルール（receiver レジスタの扱い、box_name/method 名）を Step 4 の設計メモと揃える。
  - `ExternCall`:
    - `hostbridge.extern_invoke("env.codegen","emit_object"/"link_object", args)` や `env.mirbuilder.emit` などは、
      - Rust では最終的に `ExternCall` → `extern_provider_dispatch("env.*", args)` → `plugin_loader_v2::extern_call("env.*", method, args)` / `host_providers::*` という構造になっている。
    - Hako 側では「env 名＋メソッド名の組（= key）」を列挙した薄い `*BridgeBox` でラップし、そのうえで `ExternCallLowerBox` が `externcall func="env.codegen.emit_object"` を emit する。
    - 未対応の name/method 組は必ず Fail-Fast（タグ付き）で provider に回す。

この Step 4.1 を「Rust 側の SSOT」として固定しておき、Phase 25.1c 以降ではこの契約に沿って Hako 側の MethodCall/ExternCall lowering 箱を実装・整理していく（Rust 側に新ルールは追加しない）方針とする。

- 実装イメージ（Phase 25.1b 中にやるときの TODO）:
  1. `FuncLoweringBox` に小さな helper を追加:
     - `_lower_method_call(body_json, func_name, box_name, params_arr)` → MethodCall パターン検出＋`mir_call Method` 生成。
     - `_lower_hostbridge_extern(body_json, ...)` → env.codegen/env.mirbuilder 用の最小 ExternCall 生成。
  2. `_lower_func_body` の冒頭か、既存 `Return(Call)` の前後でこれら helper を呼び出し、マッチした場合のみ MIR を返す。
  3. Tag/ログ:
     - `HAKO_SELFHOST_TRACE=1` 時に `[funcs/basic:method.*]` / `[funcs/basic:extern.*]` trace を出し、どの defs が Method/Extern 経由で lowering されたか観測できるようにする。
  4. スモーク:
     - `tools/smokes/v2/profiles/quick/core/phase251` に `selfhost_mir_methodcall_basic_vm.sh` のような canary を追加し、
       - ArrayBox.push / FileBox.open/read/write / env.codegen.emit_object/link_object の代表ケースを selfhost builder-first で通し、
       - `mir_call` / extern call が出力 MIR に含まれていることを確認する。

### Step 1 — CLI entry detection (CliEntryLowerBox)
- 目的: Stage1 CLI の入口構造（`Main.main` → `HakoCli.run`）を Program(JSON v0) 上で検出し、selfhost builder が「どの Program が CLI エントリを含むか」を把握できるようにする（観測専用）。
- 作業:
  - `lang/src/mir/builder/func_body/cli_entry_box.hako` に `CliEntryLowerBox` を追加。
    - `scan(program_json)` で以下を確認し、すべて満たすときだけタグを出す:
      - `Program.body` 内に `New(HakoCli)` 相当の JSON（`"class":"HakoCli"`）が存在する。
      - `defs` 配列に `{"box":"HakoCli","name":"run", ...}` が存在する。
      - 入口付近に `method":"run"` を含む Method 呼び出しがある（軽いヒューリスティック）。
    - 条件を満たした場合、`HAKO_SELFHOST_TRACE=1` のときに  
      `[builder/cli:entry_detected] main=Main.main run=HakoCli.run/2`  
      を出力し、戻り値は常に `null`（MIR は生成しない）。
  - `FuncLoweringBox.lower_func_defs` の先頭で `CliEntryLowerBox.scan(program_json)` を呼び出し、defs 降ろし処理とは独立に「入口構造だけを観測」できるようにする。
- レイヤリング:
  - この Step1 はあくまで ring1（Stage1 CLI ソース）の Program(JSON v0) を観測するだけで、ring0（Rust env.mirbuilder/env.codegen）には一切影響を与えない。
  - 今後の Step2/3 で `HakoCli.run` / `Main.main` の本体を MIR に降ろすときの「入り口インデックス」として使う想定。

### Step 2 — HakoCli.run 形状スキャン（CliRunShapeScannerBox）＋ lower stub
- 目的: Stage1 CLI の `HakoCli.run` がどのような分岐（run/build/emit/check 等）を持っているかを Program(JSON v0) から把握し、将来の専用 lower（CliRunLowerBox）が安全に使えるかを事前に観測する。
- 作業:
  - `lang/src/mir/builder/func_body/cli_run_shape_box.hako` を追加（CliRunShapeScannerBox）。
    - `scan(program_json)` で:
      - `{"box":"HakoCli","name":"run", ...}` を defs 内から検出し、
      - Program 全体の文字列から `"run"`, `"build"`, `"emit"`, `"check"` などのリテラルを簡易に拾って、`branches` 配列として記録。
    - `HAKO_SELFHOST_TRACE=1` のときに  
      `[builder/cli:run_shape] has_run=1 branches=<count>`  
      を出力し、戻り値として `{"has_run":1,"branches":[...]} (MapBox)` を返す（現状はタグ主体で利用）。
  - `lang/src/mir/builder/func_body/cli_run_lower_box.hako` を追加（CliRunLowerBox）。
    - 現段階では stub 実装:
      - `lower_run(func_name, box_name, params_arr, body_json, func_map)` は `HakoCli.run` だけをターゲットにし、`HAKO_SELFHOST_TRACE=1` 時に  
        `[builder/cli:run_lower:stub] box=HakoCli func=run`  
        を出して常に `null` を返す。
      - 実際の MIR 降ろし（run/build/emit/check 分岐を持つ run 本体の lowering）は、Step2 後半〜Step3 以降で実装する前提。
  - `FuncLoweringBox` への統合:
    - `lower_func_defs` の先頭で `CliRunShapeScannerBox.scan(program_json)` を呼び、Stage1 CLI run の形状をタグ付きで観測。
    - `_lower_func_body` の冒頭で `box_name=="HakoCli" && func_name=="run"` のときだけ `CliRunLowerBox.lower_run(...)` を呼び出す。現状は常に `null` なので、従来の BasicLowerBox / provider 経路と挙動は変わらない。
- レイヤリング:
  - Step2 も Step1 と同様、ring1（Stage1 Hako CLI）の構造を観測する箱のみを追加する。
  - MIR 生成はまだ Rust provider /既存 lower に任せており、ring0 の責務（env.* extern や実行エンジン）にも影響を与えない。
  - 専用 lower（`CliRunLowerBox` が実際に MIR を返す形）は、Stage‑B Program(JSON v0) の形状を十分観察してから、小さなパターン（シンプルな run/build/emit/check 分岐）ごとに段階的に実装する。

### Step 2.x — HakoCli.run lowering（設計メモ＋MVP 実装状況）
- ゴール:
  - `HakoCli.run(me,args)` のうち「単純な run/build/emit/check 分岐」だけを selfhost builder で MIR 関数に降ろす。
  - 形が少しでも崩れていたら必ず `null` を返し、Rust provider にフォールバックする（Fail‑Fast）。
- 対象とする JSON v0 の形（MVP 想定）:
  1. `Local argc = Int(0)`
  2. `If cond Var("args") then { Local argc = Method Var("args").size() }`
  3. `If cond Compare(Var("argc") == Int(0)) then { Return Int(1) }`
  4. `Local cmd_raw = Method Var("args").get(Int(0))`
  5. `Local cmd = Binary("+", Str(""), Var("cmd_raw"))`
  6. 連続する `If` で `cmd == "run"|"build"|"emit"|"check"` を判定し、それぞれ `Return Call("cmd_*", [me,args])` を持つ。
  7. 最後に `Return Int(2)`（unknown command）を持つ。
- 実装状況（CliRunLowerBox 内）:
  1. ターゲット判定（実装済み）  
     - `box_name=="HakoCli" && func_name=="run"` 以外は即 `null`。
  2. 構造パターンの検証 `_check_shape`（実装済み）  
     - `body_json` を文字列として走査し、上記 1〜7 のステートメントが順番どおりに現れているかを `JsonFragBox` で確認（ローカル名やメソッド名も一致させる）。  
     - OK のとき `1`、不一致のとき `0` を返し、`HAKO_SELFHOST_TRACE=1` で `[builder/cli:run_lower:shape-ok]` / `[builder/cli:run_lower:unsupported]` を出す。
  3. MIR 生成（MVP） `_emit_mir`（実装済み・既定 OFF）  
     - params_arr=["me","args"] を r1,r2 とみなし、固定レイアウトのレジスタ配置で簡略 MIR(JSON) を構築。  
     - ブロック構造（要約）:
       - argc を `boxcall size(box=2)` で計算し、`argc==0` のときは `ret 1`。
       - `args.get(0)` で cmd_raw を取得し、`"run"|"build"|"emit"|"check"` との比較で `cmd_run/cmd_build/cmd_emit/cmd_check` を `boxcall` で呼び出してそのまま ret。
       - どれにもマッチしない場合は `ret 2`（unknown command）。
     - 環境変数 `HAKO_MIR_BUILDER_CLI_RUN=1` のときにだけ `_emit_mir` を呼び、それ以外は shape OK でも `null` を返して provider/既存 lower にフォールバックする（既定挙動は不変）。
  4. タグと Fail‑Fast（実装済み）  
     - 形が完全に一致し、`HAKO_MIR_BUILDER_CLI_RUN=1` のときにだけ MIR を返し、`HAKO_SELFHOST_TRACE=1` で `[builder/cli:run_lower:ok] HakoCli.run/2` を出す。  
     - 途中でパターンが崩れていた場合は `[builder/cli:run_lower:unsupported] reason=...` を出して `null` を返す（provider が引き継ぎ）。
  5. 現在のカバレッジ:
     - `.hako` に HakoCli.run + cmd_* を直接書いた最小ケースでは、selfhost builder だけで `HakoCli.run/2` の MIR(JSON) を生成できることを  
       `tools/smokes/v2/profiles/quick/core/phase251/selfhost_cli_run_basic_vm.sh` で確認済み。
     - 実際の Stage1 launcher.hako の `run` は usage メッセージ出力などを含むため、この MVP はまだ Stage1 本体には適用していない（今後 Stage‑B Program(JSON v0) を詳細に比較しながら対応範囲を広げる）。

#### Stage1 HakoCli.run（本番）とのギャップ整理（provider MIR ベース）

現状、このホスト環境では Stage‑B 実行中に `Undefined variable: local`（Stage‑3 キーワード）で Program(JSON v0) 抽出が失敗しているため、実 `launcher.hako` の HakoCli.run 形状は Rust provider の MIR(JSON) から推定している（`/tmp/launcher_provider_mir.json` で確認）。

MVP run パターンと Stage1 実装の主な差分:
- argc==0 の場合:
  - MVP: `if argc == 0 { return 1 }`（副作用なし）。
  - 実装: usage メッセージ `[hakorune] usage: hakorune <command> [options]` を `print` してから `ret 1`。
- サブコマンド unknown の場合:
  - MVP: 単純に `return 2`。
  - 実装: `[hakorune] unknown command: <cmd>` を `print` してから `ret 2`。
- 比較まわり:
  - 両者とも `"run"|"build"|"emit"|"check"` 文字列と一致判定しているが、実装側は usage/unknown メッセージ用の追加 `binop`/`call` を複数ブロックに分割している（MIR 上でブロック数が多い）。
- 呼び出しターゲット:
  - 両者とも `cmd_run/cmd_build/cmd_emit/cmd_check` を呼び出す点は一致（MIR 上では `boxcall`、将来 `FuncLowering` の call_resolve で Global/Method に寄せる予定）。

今後 Stage‑B Program(JSON v0) が安定して取れるようになったら、上記の差分を JSON レベルで再確認し、
- usage/unknown の `print` ブロックを「前置/後置のサイドエフェクト」として `_check_shape` の許容パターンに追加するか、
- あるいは run 本体を「MVP サブセット（引数分岐）＋印字専用ブロック」に分けて扱うか、  
を決める予定。

#### Stage1 用 run パターン拡張方針（設計）

Stage1 launcher.hako の本番 `HakoCli.run` を selfhost lowering の対象に含めるための方針は次のとおり:

- 目的:
  - Rust CLI（Stage0）と同じ意味論（exit code とメッセージ）を維持したまま、Stage1 側の run を selfhost builder からも扱えるようにする。
  - logging/usage 部分（print）は「サイドエフェクトのある前置/後置ブロック」として明示的に扱い、分岐ロジック本体とは分離して考える。

- 拡張の方向性（案）:
  1. **前置 usage ブロックの許容**  
     - `_check_shape` で「argc==0 → usage print → ret 1」という形を、  
       「`argc == 0` の then 側に `StringBox` const / `binop` / `call(print)` が含まれてもよい」  
       というルールに緩和する。
     - lowering 時には:
       - まず usage 用の文字列構築＋`print` をそのまま MIR に反映（`boxcall` / `externcall env.console.log` など）。
       - そのあとで `ret 1` を emit する（MVP では usage 文言は provider MIR に揃える）。
  2. **unknown ブロックの後置許容**  
     - MVP では「unknown なら `ret 2`」のみを扱っているが、本番では  
       `[hakorune] unknown command: <cmd>` を出力してから `ret 2` している。
     - `_check_shape` を「末尾の Return Int(2) の前に StringBox/`binop`/`print` パターンが挟まっていてもよい」と解釈できるようにし、  
       lowering 側でもそれをそのまま MIR に降ろす（print ブロック + `ret 2`）。
  3. **run 本体（分岐ロジック）との分離**  
     - `_check_shape` を二層に分ける:
       - `check_core_shape` … argc/args/cmd/サブコマンド分岐の「副作用なし」部分の形状チェック。
       - `check_logging_shape` … usage/unknown の印字パターンのみを許容する緩いチェック。
     - `CliRunLowerBox` はまず core を `_emit_mir_core` で生成し、logging 部分は必要に応じて前後にブロックを足す形で統合する。

- トグルと適用範囲:
  - Stage1 本番への適用は常に `HAKO_MIR_BUILDER_CLI_RUN=1`（既定 OFF）でガードし、  
    かつ Stage‑B Program(JSON v0) で `check_core_shape` / `check_logging_shape` の両方を満たしている場合だけ有効にする。
  - それ以外のケース（print の形がずれている / 追加の case が増えたなど）は、今まで通り provider 経路に退避する。

### Step 5 — call resolve / methodize 後処理
- 目的: 生成済み MIR に対して call resolver / methodize pass をかけ、Rust provider と同じ命名・呼び出し形式を実現。
- 作業:
  - `HAKO_MIR_BUILDER_CALL_RESOLVE=1` を本格利用し、call の `Const("Box.method/N")` を `mir_call` に変換。Stage1 CLI で `mir_call Method` を使うケースをテストし、Methodize との組み合わせでも崩れないことを確認。

### Step 6 — selfhost-first canary / build_stage1.sh
- 目的: selfhost builder を既定 ON に戻す準備。
- 作業:
  - `stage1_launcher_program_to_mir_canary_vm.sh` の selfhost-first 版を追加して selfhost builder 単独で 60KB 級 MIR を生成できることを検証。
  - `tools/selfhost/build_stage1.sh` を selfhost-first で回し、selfhost builder 由来の MIR で `emit program-json` / `emit mir-json` / `build exe` が通ることを確認。
  - 問題が無ければ `HAKO_SELFHOST_BUILDER_FIRST=1` を既定に戻す（別PRでも可）。

作業順は Step 0 → 1 → 2 → 3 → 4 → 5 → 6 を想定、各ステップで必ず docs（このファイル＆CURRENT_TASK）とスモーク/テストを更新する。

現状（2025-11-16 時点）の進捗:
- Step0〜3: 実装済み（Fail-Fast 導線・Local/If/Return 基本形・LoopForm 正規化済み Loop の取り込み）。
- Step4:
  - MethodCall: `FuncBodyBasicLowerBox._try_lower_return_method` で `Return(Method recv.method(args))` 形のうち、ArrayBox.size/get（params ベース receiver）と StringBox.length（引数なし）を最小カバー済み。生成される MIR は `mir_call { callee: { type: "Method", box_name: "<ArrayBox|StringBox>", method, receiver }, args: [...] }` 形式。
  - ExternCall: `lang/src/mir/builder/func_body/extern_call_box.hako` に `ExternCallLowerBox.lower_hostbridge` を追加し、`Return(hostbridge.extern_invoke("env.codegen","emit_object"|"link_object", Var(arg)))` を `externcall env.codegen.emit_object|link_object`＋`ret` に lowering する最小パターンを実装。`FuncLoweringBox._lower_func_body` から BasicLowerBox の次に呼び出すよう配線。
  - Canary: `tools/smokes/v2/profiles/quick/core/phase251/selfhost_mir_methodcall_basic_vm.sh` / `selfhost_mir_extern_codegen_basic_vm.sh` を追加（現状は「対象パターンにまだ到達していない」場合に SKIP する canary として動作）。
- Step5〜6: 未着手（Method/Extern のカバー範囲が実際の Stage1 CLI パターンまで広がった段階で selfhost-first canary / build_stage1.sh へ進める）。

### Stage1 CLI defs vs selfhost builder 対応状況（スナップショット）

Stage1 CLI ランチャ（`lang/src/runner/launcher.hako`）について、`tools/hakorune_emit_mir.sh` を provider-first（`HAKO_SELFHOST_BUILDER_FIRST=0`）で実行し、Rust provider が出力した MIR(JSON) から各関数の Method/Extern 風パターンを集計した結果:

- 集計コマンド（例）:
  - `HAKO_SELFHOST_BUILDER_FIRST=0 NYASH_JSON_ONLY=1 bash tools/hakorune_emit_mir.sh lang/src/runner/launcher.hako /tmp/launcher_provider_mir.json`
  - Python で `functions[].blocks[].instructions[].op in {"boxcall","mir_call","externcall"}` を走査し、Method/Extern らしき箇所を抽出。
- 注意:
  - 現行の provider MIR では、Stage1 CLI のメソッド呼び出しはすべて `boxcall` で表現されており、`mir_call(Method)` にはまだ正規化されていない。
  - selfhost builder は Stage‑B の Program(JSON v0) 上で `Method` ノードを見て lowering する設計であるため、下表の「Method 名」は Program 側のメソッドセットを推定するための参考情報として扱う。

| MIR function                     | Method パターン（provider MIR 上の boxcall/mir_call）                                                                 | Extern 風パターン                           | selfhost builder 側の現状             |
|----------------------------------|------------------------------------------------------------------------------------------------------------------------|--------------------------------------------|--------------------------------------|
| `HakoCli._read_file/3`           | `open`, `read`, `close`（FileBox 由来と推定）                                                                           | なし                                       | FileBox 系メソッドは未対応           |
| `HakoCli._write_file/5`          | `open`, `write`, `close`（FileBox）                                                                                     | なし                                       | 同上（未対応）                        |
| `HakoCli.cmd_build/2`            | `cmd_build_exe`, `get`, `size`（`args.get/size` など）                                                                  | なし                                       | `args.size/get` 形は Step4 helper あり（ただし関数本体全体は未対応） |
| `HakoCli.cmd_build_exe/2`        | `_read_file`, `emit_from_program_json_v0`, `emit_program_json_v0`, `extern_invoke`, `get`, `indexOf`, `push`, `size`   | `extern_invoke` が hostbridge 経由の Extern 相当 | `extern_invoke("env.codegen",…)` 用 ExternCallLowerBox 追加済み／他メソッドは未対応 |
| `HakoCli.cmd_emit/2`             | `cmd_emit_mir_json`, `cmd_emit_program_json`, `get`, `size`                                                             | なし                                       | `args.size/get` のみ helper 対象候補 |
| `HakoCli.cmd_emit_mir_json/2`    | `_read_file`, `_write_file`, `emit_from_program_json_v0`, `emit_program_json_v0`, `get`, `indexOf`, `set`, `size`      | なし                                       | FileBox 系／`indexOf`／`set` は未対応 |
| `HakoCli.cmd_emit_program_json/2`| `_read_file`, `_write_file`, `emit_program_json_v0`, `get`, `indexOf`, `size`                                          | なし                                       | 同上                                 |
| `HakoCli.run/2`                  | `cmd_build`, `cmd_check`, `cmd_emit`, `cmd_run`, `get`, `size`                                                          | なし                                       | `args.size/get` helper 対象／`me.cmd_*` self-call は未対応 |
| `main`                           | `run`（`Main.main → HakoCli.run` 呼び出し）                                                                            | なし                                       | main 相当は provider/main 降ろしに依存 |

このスナップショットから分かる不足点（2025-11-16 現在）:

- MethodCall 側:
  - Stage1 CLI で実際に使われているメソッドは、FileBox（open/read/write/close）、Array/Map 系（size/get/set/push）、String 系（indexOf）など多岐にわたるが、selfhost builder が defs 側で専用に扱えているのは **`args.size/get` と `String.length` の単純な Return 形のみ**。
  - `me.cmd_*` 系 self-call（`cmd_build_exe` など）は、現状 `_lower_return_call` ベースの簡易 Call 降ろしに頼っており、Stage1 CLI の複雑な本体にはまだ対応できていない。
  - ExternCall 側:
  - Stage1 CLI の AOT 経路で重要な `hostbridge.extern_invoke("env.codegen","emit_object|link_object", args)` は、ExternCallLowerBox で最小対応済みだが、実際の Stage1 CLI defs からこの helper に到達しているかどうかは、今後 Program(JSON v0) 側のパターンを精査する必要がある。
  - それ以外の Extern（`env.mirbuilder.emit`, `env.console.*` など）は、selfhost builder では現時点で扱っておらず、Rust provider / ハーネス側に依存している。

この表をベースに、今後の小さな helper 拡張（例: FileBox 用メソッド降ろし箱、Array/Map の `push/set/indexOf` 降ろし箱、`me.cmd_*` self-call 用の専用 CallLowerBox など）を段階的に追加していく予定。  
Phase 25.1b の残タスクとしては、「Stage1 CLI で本当に必要なメソッド／Extern パターン」だけを優先し、それ以外は引き続き Rust provider を退避路として使う方針を維持する。

### スモーク構成方針（Rust builder と selfhost builder のペアリング）

- 目的:
  - Rust 側の loop/method/extern スモークを **そのまま「正解」として使い回しつつ**、同じ .hako を selfhost builder で通す canary を横に並べ、「どの経路が壊れているか」をフォルダ名だけで判別できるようにする。
- 基本ルール:
  - 既存の v2 スモーク構造（`tools/smokes/v2/profiles/quick/core/phaseXXXX/`）は維持し、その直下に「provider-first」と「selfhost-first」の **ペアスクリプト** を置く。
  - 命名例:
    - `*_provider_vm.sh` … 既存どおり Rust builder（provider-first）経路を確認するスモーク。
    - `*_selfhost_vm.sh` … 同じ .hako / 期待 rc を selfhost-first（`HAKO_SELFHOST_BUILDER_FIRST=1`）で確認するスモーク。
  - ループ系:
    - 例として `phase2100` の LoopForm/PHI canary（Rust ベース）に対応して、
      - `tools/smokes/v2/profiles/quick/core/phase2100/loop_jsonfrag_provider_vm.sh`
      - `tools/smokes/v2/profiles/quick/core/phase251/loop_jsonfrag_selfhost_vm.sh`
      のような組み合わせを想定（実際のファイル名は今後の実装で確定）。
  - Stage1 CLI 系:
    - 既存の `stage1_launcher_program_to_mir_canary_vm.sh`（provider-first）に対して、
      - `stage1_launcher_program_to_mir_selfhost_vm.sh`（selfhost-first; builder MIR で 60KB 級出力を期待）
      を `phase251` 側に追加する。
- 運用:
  - quick プロファイルでは provider-first スモークを既定 ON とし、selfhost-first スモークは Phase 25.1b 中は任意（開発用）とする。
  - selfhost-first スモークが十分に安定し、Stage1 build も selfhost-first で通るようになった時点で、必要に応じて CI quick プロファイルへの昇格を検討する。

***
