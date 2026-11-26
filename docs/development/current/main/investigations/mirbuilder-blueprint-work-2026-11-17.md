Status: Active
Date: 2026-11-17
Scope: 設計図に合わせて MirBuilder を整理する作業の、今の順・回ごとの数・文書の整理の記録。
Related:
- `docs/development/current/main/design/mirbuilder-blueprint.md`(設計図。設計はここ)
- `docs/development/current/main/investigations/mirbuilder-cleanup-2026-11-15.md`(お掃除 1〜9 の記録)
- `docs/development/adr/adr-003-counted-box-default.md`

# MirBuilder: 設計図からの作業(2026-11-17〜)

設計は、設計図に書く。ここには、順と、回ごとの数と、起きた事だけを書く。

## なぜ、進め方を変えたか

- MirBuilder は、今の code の中で少しずつ入れ替える形で、約 8 か月、作り直しを続けてきた。きれいにならなかった。
- お掃除 7〜9(2026-11-16〜17)で、値の出所の読み方を正しい向きへ直した。3 回とも、差 0 で入った。しかし本体は +1,292 行になった。**周りの層が同じままなので、部品を 1 つ直すたびに、その層へ合わせる中継が増えた。**
- 原因は 4 つ重なっていた:
  1. 設計の文書に、式・呼び出し・値の層の説明が無かった(設計の文書 876 本の中に 0 件)。図に無い所を、作る側が、その場で決めた。
  2. 作る AI は、前の回を覚えていない。決め方が、回ごとに違った。
  3. 文書が多く、ほとんどが `Status: SSOT` と名乗っていた(851 本のうち 480 本)。前の決め方を、確かめられなかった。
  4. 1 回の作業が「この形を 1 つ通す」だった。専用の部品を足すのが、いつも一番安全で早かった。
- 問題そのものは、難しくない。source の木を、再帰で下ろすだけである。形を先に見分けて、形ごとに道を作った事が、大きさの原因である(設計図 21 節)。

## 順(設計図 14 節)

**2026-11-17 に、進め方を変えた(メンテナ決定)。** 別の入口の新しい builder(`src/mir/blueprint_builder/`)は、撤去した。今ある MirBuilder を、責務ごとにまとめて、古い処理を消す(設計図 14 節)。

いきさつ:

- メンテナが望んでいたのは、最初から、「今ある code を責務に分けて、消しながら、小さく綺麗にする」だった。
- 監査の AI が、相談の答えにあった「別の入口で縦に作って比べる」を、設計図の決定の 1 行として通した。「今ある物を直す」と「新しく書く」を、選べる形で並べて、聞かなかった。
- 新しい builder は、数と Bool の範囲で、約 2,600 行で EXE まで通った(下の表と監査)。しかし builder が 2 つ並び、古い code は 1 行も減らなかった。Box の所有を入れる回では、作りながら、決まっていない事(持ち分の +1 の MIR の操作、field の上書き、`birth` の途中の Fault の片付けの順、Box を返す時の出所)が 4 つ出て、行数が約 2 倍になった所で、止めた。
- selfhost の source(`lang/src`、1,192 file)を数えたら、仮引数の 95%、戻り値の 87%、`local` の 99.5% に、型の注釈が無かった。新しい builder は型の注釈を必須にしていたので、通るのは 1 file だった。設計図を書く時に、これを数えていなかった。

撤去して、残した物:

- code は、統合を取り消した(git の履歴にある)。共有の C backend への変更も、一緒に戻した。
- 分かった事: 数と Bool の範囲は、形ごとの道を作らずに、1 つの再帰と 1 つの作業場で書けた。式・呼び出し・loop の入口は各 1 つ、形を 1 つ足す費用は 3 file・3 関数だった。これは、今の MirBuilder を寄せて行く先の形の、根拠になる。
- `birth` の途中の Fault の片付けの順(式の一時値 → `birth` の中の `local` → 初期化の済んだ field を宣言の逆順 → 呼んだ側が外側を回収)は、メンテナが承認した。reference への反映は、所有の回でやる。

## 今の散らばり（測る回1b、2026-11-17）

compilerの動くcodeは変更していない。coverage CLIは前回の同じbinaryを再利用し、A全5386入力で保持した通常CLIのMIR・拒否と一致（timeout0）。A5386/B1192とも全件profileあり・途中trace0。testも完走し、8375 PASS・155 FAIL・52 ignored、FAIL名は既存155本と一致した。

| 対象 | 計測artifactの本体行 | Aが通る | Bのみ | testのみ | いずれも通らない |
|---|---:|---:|---:|---:|---:|
| src/mir 全体 | 402826 | 247208 | 2543 | 90935 | 62140 |
| builder | 120539 | 62773 | 966 | 29267 | 27533 |
| semantic_package | 113487 | 94104 | 753 | 7713 | 10917 |
| compiler | 34917 | 13333 | 0 | 17006 | 4578 |
| そのほか | 133883 | 76998 | 824 | 36949 | 19112 |

行の母集団は、CLIと完走したtest artifactの実行可能source行の和集合から明示cfg(test)行を除いたもの。物理本体603802行（コメント等込み）と単位を分ける。uninstrumented行を実測0に含めない。構文分類に限界のある8fileと、generated関数の定義LOCは未確定のまま。これは未使用・削除可能性の判定ではない。

| 測定 | 実測できた範囲 | 未測定・限界 |
|---|---|---|
| M-1 | A5386/B1192全件。CLI収録の名前付き関数7631が両集合で未通過。watchlist138定義中53が両集合通過0 | generated descriptorを名前付き関数へ推測でまとめない。default CLI未収録はsourceの実測0と分ける |
| 入力結果 | A: 所定dump/EXE生成段の成功2473・拒否2797・parser拒否116。B: dump成功222・拒否901・parser拒否69。timeout0 | EXE実行は測定範囲外。source固定literal catalogue1805名中未出現1762、未解決template131件のため全source拒否名の未出現総数は未測定 |
| M-2 compiler警告（前回取得） | 定義識別子へ照合3230項目、通常警告に無い項目13 | 未照合348span、LOC不明27。直接caller数0とは違う |
| M-3 | 全lib test coverage取得。A/B未通過の名前付き関数のうち5983をtestが通過。test通過・CLI未収録の名前付き関数104は別集計 | 未対応generated symbolのcross-artifact同一性は未確定。test-onlyを削除候補とは判定しない |
| M-4 HIR（前回取得・下限） | 解決済み参照260967本からnamespace63286本を分離し項目参照197681本。輪1、最大252群 | 未照合336830参照（外部crate等含む）。typed method等未取得。新しいHIR/THIR dumpなし |

式9/call12/loop12/if6の旧一覧はwrapperと定義の束を含んでいた。実在定義のwatchlist138個（expression22/call78/loop22/if16）のA/B入力本数を記録し、全138定義を定義位置で照合した。独立lowererの数ではない。

地図は`src/mir/README.md`の上位60群。全群・全関数・入力結果・未出現固定literalは`tools/mir_map/out/`（ignored）へ出す。入口は`python3 tools/mir_map/run.py --help`。取得済み記録の再生成は`python3 tools/mir_map/run.py --work <work> --reuse <reuse> --stage render --write-docs`。資源見張りは入力/LLVM/Cargo/testの起動前から有効、停止0回・最大子孫RSS16529964KiB。前回のTHIR OOM記録は残し、この回の監視で防いだとは主張しない。

## タスクの一覧(上から順。1 つ終わるたびに、消した行数を書く)

決まり(設計図 14 節、`RULES.md` 8 節): 機能を保つ。足さない。まとめた古い処理と、その test は、同じ回で消す。合計が減らない回は、統合しない。

### 最初に: 測って、地図を作る(メンテナ指示 2026-11-17「推測で開発しない。MirBuilder が、どうなっているかを、完全に分かる状態にする」)

上の「今の散らばり」は測る回1bの実測値。未測定と下限はそのまま区別し、段0/段1の内容はこの回では変えない。

| # | 測る物 | 測り方 | 出る物 |
|---|---|---|---|
| M-1 | 実際に通るcode | coverage CLIでA5386/B1192の全件を入力別記録 | **済み**。247208 A行・2543 Bのみ行。watchlist138中53定義が両集合通過0。全source拒否名の総数は未測定 |
| M-2 | compilerのdead/unused警告 | sourceを変えずforce-warn取得 | **警告取得済み・照合部分完了**。3230項目／未照合348span、LOC不明27。直接caller数0の一覧ではない |
| M-3 | testだけが通るcode | 監視下jobs2でbuild、thread1で全lib test | **済み**。testのみ90935行、155 FAIL名一致。generated symbolの同一性未確定は別欄 |
| M-4 | 群の依存 | compiler HIRのres/PerNSをstream解析 | **HIR部分取得**。項目参照197681、輪1・最大252群。THIR失敗、typed method等は未測定、全M-4完了ではない |

- 地図は、script で作り直せる形にする。回が 1 つ終わるたびに作り直して、数の変化を記録する。
- `src/mir/README.md` を、「directory → 責務 → 通る入力の本数」の表だけにする(D-2 と一緒)。
- 「通らない」「test だけ」と測れた物から、消す。読んだ判断だけで、消さない。
- お掃除 10 で分かった事: **静的に使い手が 0 の本体は、ほとんど無い(関数 14 個)。** 全部が、どこかにつながっている。だから、消せる物を見つけるには、M-1(実際に通るか)と M-4(依存の輪)が要る。

### 段 0: 消すだけ(動く code を変えない)

| # | 消す物 | 見込み | 状態 |
|---|---|---:|---|
| 0-1 | ずっと赤い test のうち、もう無い道の形だけを見ている 122 本。その test だけが使っていた code。使い手が 0 の code **−3,134 行**(整形後。本体 −1,102、test −2,032。足した行は 5) | 済み(2026-11-17)。122 本のうち、消せたのは 50 本。72 本は、所有・Fault・借りの検査を含むと判断して残した。使い手が 0 の本体は、関数 14 個だけだった |
| 0-2 | 本体から呼ばれない入口と、その先: `compile_resolved` と 3 つの cutover、`raw_*` の 17 module、`control_flow/plan/parts` の raw 系、`normal_module_transaction` の test だけの入口 | 約 1.1 万行 + その test | 0-1 の報告と突き合わせてから |
| 0-3 | test だけの module と、内部の型だけを見ている test(`compiler` に 67 module・約 1.7 万行、`builder` に約 3.6 万行)。**緑の test を消すので、先に、同じ動きが固定の入力(`.hako` と期待の出力)で守られている事を確かめる** | 数万行 | 0-2 の後 |

### 段 1: まとめて、消す(1 つの置き換えを、設計・置き換え・監査の 3 回で閉じる)

危なくない順。上の物が閉じてから、次へ進む。

| # | まとめる責務 | 今 → 目標 | 消える見込み | 危なさ |
|---|---|---|---:|---|
| 1-1 | 呼び出しを書く所の重複(同じ名前の包みが 3 層、同じ関数の定義が 4〜5 か所) | 1 か所 | 数千行 | 低い |
| 1-2 | 意味の包みの中で、外から使われる所が少ない群: `map_*`、`borrowed_compare`/`mul`、`loop_static`、`borrowed_object`/`instance`/`guarded`/`nullable`、`borrowed_static` | 共通の規則 1 つ | 約 3 万行 | 低い〜中 |
| 1-3 | `if` | 入口 6 → 1、PHI の書き手 4 → 1 | 未測定 | 中 |
| 1-4 | 式 | 入口 9 → 1(`raw_expression_dispatch` を持ち主にする) | 未測定 | 中 |
| 1-5 | 呼び出し | 入口 12 → 1(receiver は役割) | 約 2 万行のうち | 中 |
| 1-6 | Script の root の道(callable の道と同じ仕事) | 1 本 | 約 2 万行のうち | 中〜高 |
| 1-7 | module と root の取引・receipt(normal と raw の 2 本の鎖) | 1 本 | 約 3.2 万行のうち | 中〜高 |
| 1-8 | loop(家族ごとの pipeline、`compiler` の家族ごとの投影、`loop_recipe_contract`) | 入口 12 → 1 | 約 12 万行のうち | 高い。最後 |
| 1-9 | source を断る所を、Resolve に寄せる。正本は設計図 23 節(拒否の表: compile error 50 行、未実装 38 行)。表に無い名前は、移さずに道と一緒に消す。最後にまとめてやらず、1-2〜1-8 の各回の中でやる。名前の数の check(増えたら通さない)を、測る回の後に入れる | 約 2,200 種類 → 表の行だけ | 1-2〜1-8 と一緒に減る | — |

- 1-2 より下は、言語の決め事(型の注釈の無い source の扱い、決定 8〜11)が要る所が出る。出た所で、メンテナに聞く。
- 所有の規則(`ordinary_new_admission` の中)は、「選ばれた `new`」の形からしか届かない。1-5 の後で、1 つの持ち主に寄せる。

### loop 段 5（設計図 24.2c、2026-11-19）

- SizeClassBox.bin_size の移行を試し、元アプリの検証付き EXE は一致。後続の print 3 例と単純な return 6 例で拒否から成功へ変わったため試作を破棄した。
- plan の loop 入口 4 → 4。MIR 本体・test の行増減は各 0。bc、複合、直の loop-break、loop-true の退役はこの境界が決まってから進める。

### loop 段 6・7(2026-11-19〜20)

- 段 6(`890896d144`): 検査済みの loop を、古い planner の家族なしで、残した source から下ろす。家族の再照合 9 → 0、本体 −886 行。拒否 → 成功 40 本は全部、期待の値と一致(決定 22)。`--dump-mir` だけで失敗の種類が変わった 65 本は、メンテナが承認。
- 段 7(`beeb34e635`): 使い手 0 の直の loop-break と LoopTrue の plan の書き手を消した。plan の loop 入口 4 → 2、本体 568,945 → 567,325、拒否の名前 1,644 → 1,602。成功 → 拒否 0、失敗の種類の変化 0(廃止した `fn(...) {...}` の拒否が、Text の backend の前の検査より先に出る順へ戻した)。
- 残る入口 2 つ(bc、複合)の使い手と理由:
  - `HakoAllocPage.seedBlocks` の借りた field への `push`: 既存の field append の loan が、完成した RootFlow / Completion を要求し、その完成にはこの文の効果が先に要る(循環)。
  - 型の注釈の無い値との比較(`Stage1UsingResolver` の `loop(i < n)`、`release_source` の `!= 1`): 決定 14 の実行時の検査が、まだ無い。
  - 複合の拒否の使い手(`parser_literal_box`、`json_cur`、`json_loader` など): loop を下ろした後の、別の所で断られる。
- 決定 14 の下調べ(実装していない): loop の外でも、型の注釈の無い値と整数を、確かめずに比べる MIR がある(固定入力 66 か所、selfhost 15 か所。順序 36 / 6、等値 30 / 9)。既存の `DynamicCarrierInteger` の Invoke と検査付き整数の ABI は、登録済みの token を検査する物。注釈の無い仮引数の型の印と値を使う所まで運び、正常 / TypeError / 後始末を表す輸送が足りない。新しい命令が要るかは未確定。**メンテナの判断が要る。**
- 2026-11-23 の下調べ(main `cfe6a4d387`、読む中心): 上の 3 つの理由は古くなった。型の無い値との比較の検査は決定 24・30 で入り、新しい命令は要らなかった。今の拒否の多くは loop の前にある(`release_source` は最初の `if` の呼び出しの準備、`parser_literal_box` は `PrefixNotCovered`)。Stage1 の `loop(i < n)` は、loop の header の条件の検査で拒否される。`json_cur` の `i < s.length()` は、型の無い呼び出しの結果との大小で、まだ範囲の外。field の `push` の循環は残る。道 c(variable_accum)は、成功する使い手が 0 本(step 7 の前の coverage)。
- 廃止した書き方の拒否は、今は 2 か所で決めている(raw dispatcher と、Text の backend の前の検査)。設計図 23 節のとおり、Resolve へ寄せる時に 1 つにする。

### 正規化(設計図の決定 23、25 節。2026-11-20〜)

| # | やる事 | 状態 |
|---|---|---|
| N-1 | `local x = e` の入口を 1 つにする | 済み(`2273fa9e95`)。MIR の差 0 |
| N-2 | 2 つの台帳を 1 つの `SourceLedgerV1` にし、文と式の選択を `source_node.rs` の 1 か所にする | 済み(`6d7c40c702`)。constructor の site が Script 側へ行く誤りを 3 か所直した |
| N-3 | `if` の書き手を `if_form.rs` へ寄せる | 済み(`38be6ea11d`)。外の書き手 5 → 3(古い plan、static recipe、recursive recipe。理由は道案内) |

### Dynamic の比較(設計図の決定 24。2026-11-20〜)

| # | やる事 | 状態 |
|---|---|---|
| DC-1 | `run(x) { x < 3 }`(注釈なし仮引数)を検査つきの比較にする | 済(`0724d671e5`)。Invoke の入力形 `DynamicBorrowedInteger` と kernel 関数 1 つ(決定 24 の追記)。`Dynamic < I64`・`Dynamic > I64`・`I64 < Dynamic`。非整数は TypeError 302 を比較の site に。持ち主は route guide の比較の行。全 5,936 入力で新しい拒否 0、MIR 差 14(実行同じ 13、TypeError 1) |
| DC-2 | Resolve で Dynamic に依ると確かめた、残りの raw `icmp` を直す(挙動の直し。commit を分ける) | 済(2026-11-20、code の変更なし)。契約のある比較で検査の抜けは 0。契約の無い演算子・向きの raw 比較は 1,606(Eq 1,533、Ne 46、Le 26、Ge 1)。注釈なし仮引数の `x <= 3` は、整数で正しく、Bool では診断なしの exit 70。C の test 4 本は、元の Bool 入力がまだ compile できず止まる |
| DC-1c | 順序比較の残り 5 組(`Dynamic <= I64`・`Dynamic >= I64`・`I64 > Dynamic`・`I64 <= Dynamic`・`I64 >= Dynamic`)を DC-1 の形で検査つきにする | 済(`4faaa82292`)。新しい ABI なし。整数 15/15、Bool 5/5 で比較の site に TypeError 302。DC-2 の未照合 68・I64 証明待ち 5 は残る |
| DC-3 | `Dynamic != I64`(4.3 の適用。等値の否定)と、戻りの型が無い呼び出しの結果(`release_source`) | 3a 済(`fef17df98e`): Int と Float の等値の 2^63 の境界を直した。呼び出しの借用 Home を通した(拒否→成功 4、値一致)。元の `release_source` の呼び出しは、user box の受け手を実行時に解決する契約が無く止まる。混合等値の ABI は保留。続きは UBD-1(決定 25) |
| DC-4 | 古い plan の入口(bc・複合)の使い手を移し、入口と中の `if` の書き手を消す | LR-8・LR-10 に置き換わった(古い loop の道は全部消えた) |
| DC-5 | Local の初期化の比較(`local c = x < 3`)と入れ子の `if` も、検査つきの借用整数の比較にする。Text の実引数は今ある Return の式で評価する | 済(DC-5..5d、統合 `f233f01a9e`): 名簿で dump 成功 2,295→2,323、EXE 2,318→2,333。拒否→成功の 3 本と u01 の dump、拒否名の変化 37 段はメンテナが 1 本ずつ OK。入れ子の分岐は、最適化で省かれた空の飛び先の後も、元の分岐を照合する。本体 +80 行 |
| LC-1 | 引数の渡し方の型 `PhysicalCallableLaneCarrierV1` を `hakorune_mir_defs` に移す(crate の境界の 1 つ目) | 済(`3ed1145b9f`、統合 `4b77ad3047`): 検証から `compiler` への参照は本番 8→0。名簿の全部で差 0。移した行は本体 29・test 11。整形で `src/mir` は +51 行(長い path が短くなり、rustfmt が展開した)、生の行は −42 |
| SI-1 | static の実引数で、引数 0 個と混在の古い道を消し、行が要る呼び手を 1 つの判定にする | 済(SI-1・1b、統合 `32719192cb`): 行を省けるのは、closed-app の co-seal が省いたと記録した呼び手だけ。root・Birth・選ばれた物・判別できない物は行が要る。名簿で成功・拒否・MIR の差 0。拒否名の変化 3 件と test の削除 2 本はメンテナが OK。本体 −1,077 行 |
| RU-1 | Script の、試験用の main の下書きを別に組み立てる道を消す | 済(`0bb74c886c`、統合 `5172da03e9`): `entry_session.rs` を削除。本番の Script は既定の root の catalog が作り、`RootValidation::Script` が照合する。共有の Return は残す。guard は古い file・型の要求だけを外した。専用の test 2 本はメンテナの OK で削除。名簿の全部で差 0。本体 −228 行 |
| RU-3 | local の呼び出し結果の MIR を二重に書く所を、同じ file の持ち主にまとめる | 済(`dba9a56bf8`、統合 `9f8b49b217`): `emit_local`・`emit_local_lexical` の手書きの Invoke・Normal・結果の取り出しを消し、`emit_call_normal` を使う。検査・採番・Fault・台帳は呼び手のまま。名簿の全部で差 0。本体 −19 行。RU-2(DirectAccum の古い発行器)は、今の入口で途中の失敗を作る手段が無く止めた |
| BD-3 | 本番から届かない Raw root の家族(物理状態・receipt 台帳・source facts・環境の install・Script の物理出口・本体出口・module の shell)と、旧公開のいつも None の分岐を消す | 済(`4444adbd89`・`31c882b690`、統合 `4b273e2bbc`): 共有の code を確かめる test 42 本は、今の通常の root/catalog・collector・finishing の test に移してから消した。消えた code だけの test 96 本はメンテナの OK で削除。名簿の全部で差 0。本体 −15,793 行 |
| BD-4 | 呼び手のいない canonical の Invoke・Compare の書き手と、それだけが使う証明を消す | 済(`6d0fd6eee0`・`4e56234fb3`、統合 `c202484493`): 名簿の本物の入力で、loop の発行が仮の PHI の後に失敗しても live が変わらず、同じ compiler で次が成功する事を test にした。test 15 本はメンテナの OK で削除。名簿の全部で差 0。本体 −1,185 行。古い DirectAccum の発行器は BD-5 で扱う |
| BD-5 | 古い DirectAccum の loop の発行器と、family_admission の test だけの五行 window の組み立てを消す | 済(`0b2dd4b9fb`・`85639768e1`、統合 `600220a613`): 今の入口の途中の失敗のやり直しは BD-4 の test が確かめる。GenericG0 は単独の observation から demand を作る形だけを残した。共有の検査 8 本は本番の道に移し、test 30 本はメンテナの OK で削除。名簿の全部で差 0。本体 −2,259 行 |
| BD-6c | 開発用の環境変数でだけ動く control_tree と strict の検査を消す。CallableSingleLoop は残す | 済(`6054e712d6`・`26d787c717`、統合 `b4d5d50d25`): 変数は他の読み手があるので残す。strict の拒否名 1 つと StepTree の行が消える事はメンテナが OK。CallableSingleLoop は App Main の loop の test 2 本がそれだけで成功するので残した(BD-6 で停止)。test 13 本はメンテナの OK で削除。名簿の全部で差 0。本体 −1,257 行 |
| BD-7 | 本番が通る file の中の、本番の build が名前でも使わない関数を消す | 済(`30172eb18f`・`9965fa5d76`・`601c58a958`、統合 `7350083803`): BD-0 の候補 3,246 件を、本番の lib と bin の参照で見分けた。共有の検査 18 組は今の持ち主を呼ぶ形に付け替えた。test 5 本と確かめ 1 か所はメンテナの OK で削除。名簿の全部で差 0。本体 −1,565 行。確かめ切れない 1,987 件は BD-8 で扱う |
| BD-8 | BD-7 で決め切れなかった関数を、compiler で確かめて消す | 済(`80b2b989c2`、統合 `03dd72fc43`): 一時の `cfg(test)` の印で本番の lib と bin の参照を確かめ、使い手が消える連鎖も拾った。1,161 件は本番が使い、671 件は共有の test が使うので残した。共有の検査 54 組は今の入口に付け替えた。test 6 本はメンテナの OK で削除。名簿の全部で差 0。本体 −1,278 行 |
| BD-9 | trivial の道と stage_missing の道を消す | 済(`0e897bc391`・`88ee5132e7`・`419f23e4b1`、統合 `cea85304af`): cataloged の static method は source driver を通る。変わる入力(MIR 53 本、拒否の名前 55 か所、外の包み 1 本)は全部メンテナの OK。成功・拒否・実行は不変。本体 −233 行 |
| LB-4 | file 関数と static box の状態だけが使っていた code を消す | 済(`bfeeff209e`..`1b2b539ac7`、統合 `66de516cfe`): parser の単独関数の入口、選択・投影・結果契約、file 関数用の Generic G0 の本番 emitter、static box の状態の解析、古い VM の static instance の別名を消した。raw AST・直接 MIR・C の入力が使う部品は残す。test 4 本はメンテナの OK で削除。名簿の全部で差 0。本体 −3,130 行(うち 1,800 は test へ移動) |
| NR-1 | `N/` の小さな組み替え(外から見える変化なし) | 済(`b6bbae1f2c`..`0e12cb9ea4`): CoreMethod の表の往復、呼び手の無い入口、位置ごとの包み 8 本、claim の取り出しの重なり、Resolve の行の写しを消した。test 1 本はメンテナの OK で削除。名簿の全部で差 0。本体 −659 行 |
| LB-5 | static box の中の `me.f(x)` を動かし、箱の名前なしの `f(x)` を `call/target-missing` にする | 済(`1ea8328f33`・`5ccf4c7ce2`・`44d3bb0d5b`、統合 `8f6c39aacd`): `me.g(x)` は `Box.g(x)` と同じ書き手。変わる入力と test 7 本の削除はメンテナの OK。本体 +257 行(仕様の実装) |
| LB-5c | 箱の名前なしの予約の名前を仕様の特別な形だけにする(math を外す、`panic` を method に結び付けない) | 済(`55e634b841`..`684489d2a4`、統合 `fbd6c7235e`): 変わる試しの入力と名簿の拒否名 1 本はメンテナの OK。CallableSingleLoop の書き手は名簿でも試しでも MIR 0 本。本体 +17 行 |
| LB-5e | 残りの予約の名前を決めた形にそろえる | 済(`0f5e8e6a3f`..`42e8c06c11`、統合 `46a3e3373a`): `print` と `mem.*` を仕様に書き、`exit`・`str`・`Range` と `a .. b` の糖衣を退役。どの予約の名前も同じ名前の method に結び付かない。名簿・監査の差 0。試しの入力の変化と test 1 本の削除はメンテナの OK。本体 +8 行 |
| UBD-1 | ユーザーの box の Dynamic 呼び出しを、program ごとの表で選ぶ(決定 25)。型付きの受け手・整数の引数・I64 の結果。入力形 1、tag 1、metadata 1、C ABI 2、kernel export 2 | 済(`ac50c423b2`): 拒否→成功 3(値一致)、成功→拒否 0。3 つ目の class で MirBuilder 差分 0。MIR 本体 +約 630 |
| DEL-1 | 使い手が 0 の code を消すだけの回 | 済(`753d5121bd`): production −548、test −918 行。名簿・B は全部同じ。feature・flag の道は残した |
| UBD-2 | Text の引数(決定 26): 引数は元の種類のまま。入力形 `BorrowedText` 1、tag 4 を使い直し、新しい tag・export 0 | 済(`fd76ae108a`): 拒否→成功 2(値一致)、成功→拒否 0。受け手の class が 1 つだけの時に古い道へ入る穴は、2b(`651ebe331c`)で直した。I64 を直接返す呼び出しだけ。それ以外の結果では、静的な候補の道が残る |
| LOOP-1 | loop の止まり(`UnsupportedAncestor{ProgramBody}`) | 一部済(`7f7dfbf250`): 解決済みの loop 木を読む形に寄せて、本体 −75 行。名簿は差 0。本当の止まりは `parser_control_box.hako:102` の裸のブロック `{ … }` で、その scope の通常の出口で Home・carrier を End する接続(`transport_port.rs:33`)が無い。止まる線で止まった |
| LOOP-2 | 裸のブロックの scope の閉じ | 済(`3652ef7525`): 普通の終わりを `finish_source_body_v1` から今の後始末(`source_home_end_selection_v2`)につなぐ。scope の親の走査は `scopes_until` に 1 つ。正規 EXE で 17 本が期待の値で通り、成功→拒否 0。2b: 外の変数へ入れた Text の End の順を C 側が証明できない形は、発行前に `retained-text-order-unavailable` で断る(`apps/tests/corpus/lexical-block/a3-move-out.hako`)。release-source は `app-main-identity` で止まり、ブロックに届かない |
| APP-1 | `app-main-identity` の止まり | 済(`8f0264aed9`): 取り込んだ module の未選択の呼び手を Main と照合していた。今ある omitted static の key を、宣言の読みに渡す(本体 +25 行)。名簿は差 0。release-source の 2 本は次の `borrowed-static/text-expression-execution-pending` で止まり、8 行目は未達 |
| TXE-1 | `text-expression-execution-pending` の止まり | 済(`4133382207`): 止まった式は `string_helpers.hako:10` の `me.int_to_str(0 - v)` の実引数(I64)。未注釈の formal 全体の SUM でなく、実引数の到達値から I64 を決める(本体 +25 行、重なっていた判定を 1 つに)。名簿は差 0。release-source の 2 本は次の `borrowed-static/core-source-identity: UnresolvedCaller` で止まり、8 行目は未達 |
| UCR-1 | `core-source-identity: UnresolvedCaller` の止まり | 済(`60f641b02a`): `StringHelpers.index_of` の静的な呼び出しに、別の関数の値の受け手の `ctx.index_of` が名前と arity だけで混ざっていた。今ある静的な受け手の判定 `admits_static_receiver_v1` を、観測と候補の照合で共有(本体 +12 行)。名簿は結果の差 0。release-source の 2 本は次の `borrowed-static/forward-source-missing`(owner 1/29)で止まり、8 行目は未達 |
| FSM-1 | `borrowed-static` の止まりを原因ごとに | 済(`b134d443d2`): 285 本を原因で 14 に分けた。直したのは、検査済みの引数なしの i64 呼び出し(26 本中 25 本)と、完成した Text の結果を受け手にする形(8 本)。名簿で拒否→成功 35 本(値は全部一致)、成功→拒否 0。単独 MIR dump で take の実引数がある物は、名前つきで断る(内部検証エラー 75 本→名前つきの拒否)。release-source は `i2s(i)` の `i` の SUM が閉じず同じ所で止まる |
| SFW-1/1b/1c | 決定 27: 型の無い値を static の呼び出しへ tagged のまま運ぶ | 済(`5a32c90ce2`): 入口は 1 つ(loop 専用の選び方を消した)。呼び先の結果は呼び先の Return の証拠で決め、呼び手の値から決めない(検証で落ちていた 3 本は名前つきの拒否へ)。名簿の差 0。release-source は `StringHelpers.skip_ws(src,i)` の `required-integer-source-unproved` で止まる |
| SIS-1/SIC-1 | static の入力の入口を調べて、まとめる | 済(`7c78393d9b`): 入口は 4 つ(Input 286 本・One 4・Scalar 4・Mixed 0)。One・Scalar の file を消し、仕上げと packet の読みを 1 つにした。形ごとの準備(転送・整数・Mixed)と Bool は残る(決定 28 の運び方で消す予定)。本体 −42 行。名簿の差 0 |
| DYN-1 | 決定 28 の最初の回: 形④⑤ | 済(`b6bd6af2d8`): 同じ関数に転送した値と literal を渡す形、呼んだ後に渡した値を読む形が通る(監査の新しい入力 8 本で値が一致)。`i - 5` のような型の無い値の計算は、まだ止まる。形ごとの準備は理由つきで残る。本体 +166 行。名簿の差 0。結果の ABI と所有の表を作った |
| DYR-1 | 決定 29: 型の無い値を結果として返す(A・B) | 済(`eedfb73190`): instance の `id(v)` が、整数・Bool・null を `== null` で、Text を literal と比べて通る(元の借用が終わった後に読む)。Box は名前つきの拒否。名簿の拒否→成功 4 本、成功→拒否 0。返った値を数や Bool と比べると C の段で止まる(DYR-1b、決定 30)。本体 +1,162/−507 行 |
| DYR-1b | 決定 30: 仮引数と返った値で同じ等値 | 済(`86c7d1437f`): `true == 1` は false、`5 == 5` は true、左右逆も同じ、`true < 5` は TypeError 302。監査の新しい入力 9 本が期待の値。名簿の拒否→成功 8、成功→拒否 0。本体 +1,053/−270 行(古い関数・file の削除 0)。次は片付け CL-1〜CL-3(設計図 26 節) |
| DYR-1b2 | 決定 30: 仮引数と戻り値の Dynamic 等値 | 実装・検証済み。I64/Bool の左右両順で同じ比較。大小は比較位置の 302。失効した carrier が正常な false にならないことは kernel の既存領域に独立のテストを追加。名簿 5,992 本の EXE 回帰 0、受理 8、固定 suite 8/8。本体 +1,053/−270 行、test +109/−67 行（rustfmt 後）。 |
| CL-1 | 片付け: 戻り値の種類を呼び先で 1 度だけ決める | 済(`649211f03c`): 呼び手の上書きと既定の I64 を本番で 0 にした。`direct_call_result.rs`・`uniform_call_result_kind`・仮の I64 を書き換える分岐を退役。名簿の成功→拒否・拒否→成功 0、拒否の名前の変化 7(Bool の結果)。監査の新しい入力 12 本が基準と同じ。本体 +533/−537、test ±7(rustfmt 後、差し引き約 −5 行。見積もり 100〜130 行には届かず) |
| CL-2 | 片付け: 静的な呼び出しの入力の照合を 1 つに | 済(`51cfbc169b`): Forwarded・Numeric・Mixed・Bool の仕上げが、呼び先・site・ordinal の同じ照合(`ordinary_new_borrowed_static_source_actuals.rs`)を読む。仕上げの関数と file は残した(usize の ABI、Mul、Bool の入口、Forwarded の借用元は別の契約)。名簿の結果と拒否の名前の変化 0。監査の新しい入力 10 本が基準と同じ。本体 +144/−164(rustfmt 後、−20 行) |
| CL-3 | 片付け: 初期化と再代入を 1 つの書き込みに | 済(`c500983f88`): 意味は `CheckedBindingWriteV1`、右辺は `emit_binding_write_rhs_v1` 1 つ。`refine_expression_plan`・`initializer_expression_v1`・2 つの `corroborate_loop_frame_v2` を退役。別の束縛へ書く MIR と End の後に install する MIR を独立の検証が拒む。右辺に 1 つ足す試しで初期化・再代入の file の変更 0。右辺の形を見る判定 22→21。名簿の変化 0、新しい入力 11 本が基準と同じ。本体 −74、test +34(rustfmt 後、−40) |
| LR-1 | 片付け: loop の古い道 c(variable_accum)を丸ごと退役 | 済(`06ebda3ada`): 名簿の全部で道 c の入口と発行が 0 回(印を入れて測定)。道 c だけの 13 file を退役、physicalizer と道 d・`g0` の共有は保持。loop の他の経路 5→4。道 c が拒否していた 5 本は別の名前の拒否になり、失敗の種類は変化 0。名簿の結果の変化 0、新しい loop の入力 10 本が基準と同じ。本体 −2,468、test −1,215(rustfmt 後、−3,683) |
| PR-1 | 片付け: print の重複した手順を 2 つ消す | 済(`10b5394851`): field の確定を `ExpressionPlanInstallationV1::with_fields` へ、発行の途中の 2 度目の source 照合を外した。完成後の照合は保持。右辺に 1 つ足す試しで print 側・引数側の変更 0(共通の評価 2 file)。名簿の結果と拒否の名前の変化 0、新しい入力 7 本が基準と同じ。本体 +16/−22(rustfmt 後、−6)。ADR-003 の物差しで `src/mir` 605,469 → 605,463 |
| LR-2 | 退役: loop の道 d(JoinIR の router) | 済(`f8468bf2dd`): 4 段の印で、名簿 5,992 本のうち古い入口に来たのは 89 本、winner の発行・physical admission・MIR を書いた数は 0。router・routing・winner spine・node admission・node の書き手など 13 file を消した。callable の検査つきの道が無い loop は、書く前に `[raw-loop-child-entry/checked-source-route-unavailable]` で拒否(関数名と位置つき)。拒否の名前の変化 89 本、失敗の種類の変化 0、受け付けの変化 0。新しい入力 16 本が基準と同じ。本体 +178/−2,278、test +116/−674(rustfmt 後、−2,658)。ADR-003 の物差しで `src/mir` 605,463 → 603,363 |
| AR-1 | 片付け: field の確定の手書きの写しを 3 つ消す | 済(`08255ccef2`): 引数・条件・field の書き込みが、手で書いた「結ぶ → 置けるか確かめる → 一括で確定」をやめ、`ExpressionPlanInstallationV1::with_fields` を使う(同じ source の Rc)。`f(mark(1), mark(2))` は 1・2・12 の順のまま。field を別の field に差し替えた MIR を検証が拒否。名簿の結果と拒否の名前の変化 0、新しい入力 11 本が基準と同じ。本体 +6/−48、test +21/−1(rustfmt 後、−22)。ADR-003 の物差しで `src/mir` 603,361 → 603,319 |
| LR-3 | 測る: loop の道 e(static の callable loop) | 段 0 だけ(code の変更なし)。e を通る関数は `SizeClassBox.size_to_bin/1` の 1 つ、入力 7 本(EXE 成功 4)。e だけの file は 23 個、本体 5,576 行・test 1,251 行。e を外すと 4 本とも失敗。持ち主の道に足りないのは、同じ box の method の I64 の結果を検査済みの値として条件と比較に渡す事(`home_local_source_scalar_call.rs:447`、`qualified_static_incoming_source.rs:107`)。足すと e の外でも受け付けが広がるので、退役は見送り、メンテナと相談 |
| LR-4 | つなぐ: 同じ box の I64 の呼び出しを、共通の比較の式へ | A(`4c93347375`)は動いたが、e を外すと直接の `return Box.f(x)` が `static-input/cohort-unfinished` で止まった。A だけは統合せず、LR-5 へ続けた |
| LR-5 | 消す: loop の道 e | 済(`75e0103c06`): 直接の return と local の初期値が、同じ `OriginalInputs` と `checked_completed_static_expression_input_v1` で元の実引数を照合(A2)。callee の Sub・Div・Mod は共通の式の走査を使う。e の file 25 個を削除(B)。`size_to_bin` は境界 0/9/4194304/4194305 → 1/2/72/73、4 本の EXE は同じ。成功→拒否 0。拒否→成功は全部期待と一致。型の無い値を別の box へ渡す 1 本の成功と、失敗の種類の変化は、メンテナ OK。rustfmt 後 −9,092 行、`src/mir` 596,234 行 |
| LR-6 | つなぐ: field から借りた配列への loop 内の push | 済(`bb56bce80c`): `seedBlocks` の push の受け手と引数を、共通の Home の走査が 1 度だけ決め、今の field 配列の書き手(`field_array_append.rs::emit`)が書く。`NoExitArrayAfter`・`prove_array_home_after`・`install_no_exit_array_after` を削除。4 本の app はバイト一致。容量 0/1/3 の値と、壊した MIR 3 つの拒否を確認。rustfmt 後 本体 −730 |
| LR-6b | 直す: push の引数の Sub・Div・Mod | 済(`129c836f4a`): Resolve が受けた引数を、書き手が書けなかった(監査で見つけた)。引数の評価を共通の一覧へ載せて解消。正と証明できない除数は前と同じ拒否。本体 +5 |
| LR-7 | 段 0 で停止 | Stage1 の `resolve_for_source` は、条件の他に、型の無い受け手の `get` と、型の無い値の return が持ち主の道に無い。新しい受け付けの範囲なので、この系列ではやらない。loop の条件の検査つきの比較は、道 b の系列でやる |
| LR-8 | 消す: loop の道 a(LoopCond plan) | 済(`1a47b9adc7`、統合 `07f9f2a4c6`): Rust 15 file・guard 11 file・lib test 45 件を削除。Stage1 の fixture の単独 dump と、lib の 3 入力(型の無い値どうしを確かめずに `icmp` で比べていた)は、名前つきの拒否に変わった(EXE と実行は同じ。メンテナ OK)。拒否の名前だけが変わった入力 25 本もメンテナ OK。名簿 5,992 本で、成功→拒否はその 1 本の dump だけ。系列の全体で rustfmt 後 本体 −3,347・test −2,417、`src/mir` 592,887 行 |
| LR-9 | 測る: loop の道 b(code の変更なし) | 済: b の成功は `parse_or_null/4` の 2 本だけで、どちらの `main` もその関数を呼ばない。その関数は loop の外も、型の無い値を確かめずに比べていた(13 か所)。名簿で型の無い値に直に届く `icmp` は 234 入力・1,741 命令。ChatGPT Pro に相談して、b を消すと決めた |
| LR-10 | 消す: loop の道 b(LoopBreak composite)と旧 CorePlan | 済(`5d6655c3ae`、統合 `17ca266829`): 158 file を削除(Rust 132)。旧 `PlanVerifier`・normalizer・lowerer も消えた。`release_source.hako` と `loop_short_circuit_blockexpr_if.hako` の dump と EXE は名前つきの拒否に変わった(メンテナ OK。本物の selfhost の source を単体で compile できなくなった)。拒否の名前だけが変わった 21 本と、消した test 137 本もメンテナ OK。rustfmt 後 本体 −21,713・test −8,630、`src/mir` 571,174 行。loop の古い道は 0 本 |
| UBD-2 以降 | `release_source` まで(決定 26 の順): 裸のブロックの scope の閉じ → 受け手の後始末 → `!= 1`。普通の呼び出しを通した Text の引数は、`finite-domain-unclosed` で止まる(`ordinary_new_borrowed_formal_entry_finite.rs:112`) | 次: 元の引数 plan と位置の照合の共有(`call_argument_source.rs:55`・`ordinary_new_argument_expression_lifecycle.rs:5`)、`id(id(v))`、`pick` |

- 道案内: `design/mirbuilder-route-guide.md`。
- 確かめ方: 編集中は、変えた所を通る入力だけ。main に入れる前に 1 度、全入力の比較・全 lib・固定 suite。**到達の地図の取り直しは、毎回はしない**(道をまとめて消した回か、数回に 1 度)。

### 文書

| # | やる事 | 状態 |
|---|---|---|
| D-1 | 「古い」と注意書きを付けたのに `Status:` が SSOT / Active のままの 32 本を、Historical にする | 済み |
| D-2 | 入口(`DOCS_LAYOUT.md`、設計の README、`15-Workstream-Map.md`、`RULES.md` の 0〜3 節)を、今の進路に合わせて短くする。古い文書を「現役」と紹介している行を消す | 次 |
| D-3 | 中身の無い札 155 本、`design/archive/` 46 本、元の場所の札 40 本を消す(入口の link と台帳を、同じ commit で直す) | D-2 の後 |
| D-4 | `phases/`(5,426 本)と `design/fixtures/`(1,512 本)。`tools/checks` の script 約 2,000 本が path を見ているので、script と同じ塊で消す | 後 |


## 撤去した新しい builder から、分かった事

- 数と Bool の範囲(`local`、代入、`if`、loop、`break`/`continue`、`return`、短絡、入れ子の呼び出し)は、約 2,600 行で、source から EXE まで通った。式・呼び出し・loop の入口は各 1 つ。形ごとの分岐は 0。形を 1 つ足す費用(`>=`)は 3 file・3 関数。
- 共有の C backend は、計算した Bool を引数に渡す形と、Bool 同士の `==` を、正しく扱えなかった(compile の時に止まる。間違った値は出ない)。今の既定の道では起きていないが、Bool を広げる時に当たる。
- 文の拒否の位置が `1:1` になる。parser が、代入・`local`・range の loop に位置を保存していない。
- 詳しい数は、git の履歴(`blueprint-builder-1-2026-11-17.md`、取り消しの前の commit)にある。

## 文書の整理(2026-11-17)

- 設計図を置いた。`RULES.md`・`CURRENT_STATE.toml`・設計の README・`src/mir/README.md` が、設計図を指す。ADR-003 と ADR-004 は、`RULES.md` から届く。
- source → MIR に関わる文書 140 本に、印を付けた: 32 本は `design/archive/` へ動かして元の場所に札を残した(code と tool から見られていない物だけ)。58 本は頭に「古い。新しい作業の根拠にしない」、35 本は「設計図が上書きした所がある」、4 本は「家族の見分けは、診断と最適化のため」、source の中の README 11 本は「今の code の記録。設計は設計図」。
- **分類は、頭と見出しだけを読んで付けた物が多い。** 迷う物は動かさず、注意書きだけにした。間違って「古い」と書いた文書は、その 1 行を消せば戻る。
- 触っていない物: allocator の文書(約 393 本)、`current/main/` の直下の中身の無い札(155 本)、`investigations/`(約 1,515 本)、`phases/`(約 5,426 本)。入口の道からは外してある。
- 残っている食い違い: 動かした後に札を残すかどうかが、`DOCS_LAYOUT.md` の中で 2 通り書いてある(今回は、今までの実際のやり方に合わせて、札を残した)。

## 分かっている赤(今回の作業と関係なく、前からある物)

- `tools/checks` の検査のうち、自分が実行できるかを最初に確かめる物は、この checkout では全部落ちる(script が実行の印なしで登録されていて、`core.fileMode=false`)。設計の文書の path を見ている検査 333 本のうち、281 本がこれで赤。**検査をまとめて走らせても、文書を動かした事の確認にはならない。** 直す時は、別の回で。
- 設計の台帳は `mode = "warning"`、台帳に無い文書が 77 本。

## lib の test(メンテナ決定 2026-11-17。設計図の決定 7)

- archive しないで消す。消す前に、言語の動きを見ている物を `.hako` へ取り出す。検証の負の例は残す。消すのは、置き換えの時に、古い code と同じ回。
- ずっと赤い 205 本(名前の集合は、お掃除 5 から同じ)を、読むだけで分類した(2026-11-17)。test は走らせていない。全文を読んだのは約半分で、残りは頭と失敗の文だけ。

| 判定 | 本数 | 中身 |
|---|---:|---|
| 消す | 122 | 古い道が選ばれた事、古い台帳の行、block の数、古い拒否の名前だけを見ている |
| 本物の不具合(見込み) | 37 | 正しい source が拒否される、など(下) |
| 分からない | 24 | うち 12 は、builder の後ろの最適化の pass の test。古い入口で source を作っているので赤い。入口を付け替えてから見直す |
| 取り出す | 20 | 言語の動きを見ている。`.hako` と期待の出力に書き直す |
| 残す | 2 | PHI を入れる所の test。fixture が定義の無い値を渡していて赤い |

分かった事:
- **一番多い原因は、意味の包みが、正しい source を拒否している事。** 26 本の test から source を取り出して、今の binary(お掃除 9 の後)の command に通した。**26 本のうち 23 本が、`ordinary-new/…` の名前で拒否された**(`new` が 1 つも無い source を含む)。例: `me.normalize_size(size)` を呼ぶ static の method、引数の field を読む、`"x" + line + count`、loop の中で `maker.make(i)` の結果の method を呼ぶ、呼ばれない method の中の Map のリテラル。
- **test の入口が、command の入口と違う。** 約 35 本は、古い入口(AST から直接)で compile している。古い入口は、`main` の中の普通の `loop` や、同じ file の中の static の呼び出しを拒否する。**test が「拒否された」と言っても、command が拒否するとは限らない。** 新しい builder では、入口を 1 つにする。
- ほかの不具合の見込み: vm-hako の橋が `externcall` を全部拒否する(6 本。`lang/src` で使っている)。Map の `get` の結果が、整数を 1 回 `set` しただけで「整数」と決まる(2 本)。`once` の property が、名前付きの拒否ではなく内部の誤りで落ちる(1 本)。最後の持ち主を放した後も、weak から取り出せる(1 本。runtime の側)。
- 23 本の拒否される source は、意味の包みの拒否を直す回の入力に使う。

分類から出た仕様の事は、メンテナが決めた(2026-11-17。設計図の決定 8〜11): 宣言した型と違う値を field に入れられない / 宣言した戻りの型と違う値を返せない(`true` も `1.5` も) / `birth_once` は外す(selfhost の使用 0) / 値を関数として呼ぶ形は、closure の形が決まるまで拒否。新しい builder の Resolve の決まりで、今の builder は置き換えまで変えない。

## まだ決めていない事

- **型の注釈の無い source の扱い。** selfhost の source(`lang/src`)は、仮引数の 95%、戻り値の 87%、`local` の 99.5% に、型の注釈が無い(2026-11-17 に数えた)。**決まった(メンテナ 2026-11-17。設計図の決定 12〜17):注釈が無い = Dynamic。** 仕様は `docs/reference/language/types.md` の "Static contract checks"。受ける範囲を広げる回(Dynamic を通す)は、掃除の後に、別の回でやる。その時に、selfhost の source の、初期値の無い `local`(21 か所、5 file)を `= null` に直す。成果は、拒否が消えた数ではなく、正しい MIR・EXE まで行った入力の数で数える。
