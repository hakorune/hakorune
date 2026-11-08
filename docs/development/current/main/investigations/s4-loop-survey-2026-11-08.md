# S4 Loop: 現状と設計の比較（2026-11-08）

Status: **調査のみ・未選択**。main `d21376de4a`、productionは`b4ba7283b6`と同一。実装・受理・仕様は変更しない。
参照: [S4の承認範囲](mirbuilder-structure-refactor-queue-d0-2026-08-23.md)、[S3bの比較](s3b-analysis-options-2026-11-07.md)、[経路調査](mirbuilder-route-selection-survey-2026-11-06.md)。

**推薦は案B（物理側から）。最初の一責務はPlanLowererの未完PHI管理を既存PhiTxnへ統合すること。置換候補5関数は実測177行。ただし、それだけでS4完了とはしない。次に、実アプリで使われるchecked-source経路のincoming・CFG・操作契約を移し、その専用計算を消す必要がある。** 案Aは長期の共通意味CFGに近いが、今のobserverを不動点で繰り返し呼ぶだけでは正しくならず、最初の置換が大きい。

## 1. 測り方と用語

作業場所は`/mnt/workdisk/public_share/hakorune-take-fix`、branch `s4-loop-survey`。測定物は`W=/mnt/workdisk/claude-scratch/s4-loop-survey`。binaryは`W/hakorune.main`（nullable最終binaryの写し、SHA256 `3f61ce9134d2e6ed1e11afc31747bd4cddd2750fa126fe69367124f6d94f42ea`）。`git diff b4ba7283b6 d21376de4a -- src lang crates`は空。buildしていない。

- 宣言の数は既存CLIの`--emit-parser-evidence-ast-json`で数えた。JSONの互換aliasキー（cond/condition、args/arguments等）は二重走査しない。関数内のLoopを数え、別関数の本体は含めない。
- 「関数出現」は入力ごとの宣言数。「異なる関数」は宣言JSONを正規化したhashの重複を除いた数。同名でも本体が違えば別。`using`先の関数はsource宣言数に含めない。
- AST出力失敗は0関数と断定しない。コメント/文字列を除いた括弧対応の補助走査を別集計した。この値は字面の候補数であり、ASTの確定数に混ぜない。
- familyは三つの軸を分ける。**共通winnerは5 family、callable子loopの物理分岐は6系統、関数全体を閉じる既存profileは5種**。同じLoopCond等が複数の軸に現れるので、合計して言語のloop種類数にしない。Dynamicの未接続canaryも別に記す。
- 実行調査はloopを含むAST成功208入力と補助走査の16入力、計224。既存の`NYASH_DEBUG_LOCAL_CALLS=1`と`NYASH_JOINIR_DEBUG=1`だけを使った。hook追加なし。EXE生成131、コンパイル拒否93、コンパイルtimeout0。生成したEXEは個別実行し、130本が15秒以内に終了、1本はtimeout。
- traceはコンパイル中のlowering観測であり、実行時に何回loopや関数が走ったかのcoverageではない。sourceにないimport先の関数もtraceに含む。family未識別と未使用を混同しない。

## 2. Familyと担当

以下の`B/`は`src/mir/builder/`、`R/`は`src/mir/resolved_semantics/`、`C/`は`src/mir/compiler/`。行番号は上記基点で確認した。型名・呼び出しのgrepだけで分かる範囲はその旨を区別する。

### 2.1 所有解析側（4種類の既存loan）

入口は`R/home_new_prefix_scan.rs:410`の`loop_i64_prestate`と`loop_prestate`相談、受領型は`R/home_new_prefix.rs:24`の`SelectedLoopHomeLoanV1`。4 variantをmatchする入口であり、別のPHI物理ownerではない。

| loan | 選択と状態/incoming | CFG・call/Fault/Text/Dynamic/借用/cleanup |
|---|---|---|
| VariableAccumAfter | `scan.rs:429`。選択済みrecurrenceのafterを採用 | 自前で物理CFG/PHIを出さない。source bindingとHome前状態を照合するscalarのloan |
| NoExitArrayAfter | `scan.rs:421`。元Array residenceのafter証拠 | owned Arrayと生きたHomeの継続を照合。一般の配列/借用をここから推測しない |
| StaticI64Search | `scan.rs:439`、`home_new_prefix_search_loop.rs` | static呼び出しのsource証拠と前状態を結ぶ。`B/raw_loop_child_entry/static_i64_v2.rs:16`のentry/source/signature transportと対応。独立したPHI生成器ではない |
| SourceObservation | `scan.rs:484`から`home_new_prefix_source_loop.rs:7`の`observe` | 同じ文走査で条件/本体/子loop/call/Return/Array/借用とHomeを観測。Text/Dynamicも保持。logicalなHeader/Afterとport incomingは`home_prefix_source_loop_ports.rs:220`、物理IDを作らない |

`scan_statement_flow`の呼び出しは入口`R/home_new_prefix.rs:605`、if枝`home_new_prefix_branch.rs:100`、loop本体`home_new_prefix_source_loop.rs:126`の3か所。loopは別の単純なforへ独立しておらず、同じ走査に再入する。

hookの「6種」は分類であって6関数ではない。begin、visit、exit/backedge、return/call/expression/child/array_append/ifの記録、end、headerのstage/finishという役割群がある。実例: `scan.rs:116,127,172,628`、`branch.rs:652`、`home_prefix_source_loop_control.rs:326,392,406,416,428,457,489,507,541`、`home_prefix_source_loop_ports.rs:413,457`。定義・呼び出しを確認した。hook群の総数を単純な「6削除」と見積もらない。

SourceObservationの値の種類は`home_prefix_source_loop.rs:29`と`home_prefix_source_loop_dynamic.rs:6`。Scalar/Text/Dynamicは同じbinding/edgeの証拠の種類であり、新たな構文familyではない。`home_prefix_source_loop_after.rs:67`は検証済みafterを元local状態へ戻す。sourceの型/classが既知でもHomeを捏造しない。

### 2.2 共通winner（5 family）

`C/loop_node_winner_spine.rs:77`のDirectAccum / NestedPredicate / LoopTrue / LoopCond / GenericG0。`loop_route_policy/family_selector.rs:147`は、lease/coverageを照合し、候補数>1をOverlap、0をNoCandidate、1をSelectedにする。ここは現在すでに「全部の観測を閉じて1つ選ぶ」方式で、先着ではない。

4つのnode familyは`C/loop_node_physical_admission.rs:81`で同じoperations+inputsへ渡り、`B/resolved_lowering/loop_recipe_physicalizer/loop_node_lowerer.rs:86`でcanonical CFG/SSA/PhiTxnを使う。GenericG0だけは`:95`でFunctionLevelFamilyとなり、関数全体の入口へ渡す。

| family | incoming/PHI・CFG | 扱う効果 |
|---|---|---|
| DirectAccum | common operation program→JoinSig/layout→canonical Binding SSA/PhiTxn/CFG | Read/Write、i64演算、比較。一般のcall/Fault/Text/Dynamicのbodyではない |
| NestedPredicate | 同上、子segment/afterを`recursive_after.rs:74`以降で閉じる | 同上 |
| LoopTrue | 同上。callable側の同名productとは入口が違う | 同上 |
| LoopCond | 同上。callable側のPlanLowerer経路と並存 | 同上 |
| GenericG0 | 関数単位の`generic_lowerer.rs:145`でCanonicalSsaFunctionSessionV2 | boundedなnested scalar loop。全loop coverageではない |

`loop_recipe_contract/schema.rs:199`のLoopOperationV1はReadBinding/ConstI64/BinaryI64/CompareI64/WriteBinding、値classはI64/Bool/Unit。共通physicalizerの`operation_dispatcher.rs:30`もPure/Read/CarrierSeed/Writeである。**共有できる名前があることと、checked-sourceの全操作を受けられることは別。**

`B/control_flow/joinir/route_entry/router.rs:122`はsource unit→membership→winner→admission→physicalizerの固定順。`:163`付近ではまだcondition/bodyのAST構造一致でresolver siteを逆引きし、同じ形が2つなら曖昧として拒否する。S4の「siteを入口から渡す」は未着手の実課題である。

### 2.3 Callable子loop（6物理経路）

入口は`B/raw_loop_child_entry.rs:222`。callable handoffの有無を決め、選択済みsource productがあれば別adapterへ横取りさせない（`:370`付近）。`raw-loop-routes`の診断は`:395`、最終issuerのfamily診断は`normal_callable_loop_source_facts/generic/issuer.rs:57`。

| 経路 | 物理入口 | incoming/PHI・CFG | 効果/責任 |
|---|---|---|---|
| VariableAccumRecurrence | `raw_loop_child_entry/variable_accum.rs:53`→共通node physicalizer | canonical SSA/CFG/PhiTxn | Read/Write source消費とwritebackを対応させる。一般call/Textを推論しない |
| Direct LoopBreak | `raw_loop_child_entry.rs:407`付近→loop_break_source→PlanLowerer | CoreLoopPlanの先決めPHI、LoopFrameのcontinue/break入力、PlanBuildSession | source portを通す効果/借用の保持。個々のText/Dynamic全受理範囲は未精査 |
| Composite LoopBreak | 同`:440`以降→composite source→PlanLowerer | 同上 | composite sourceの子関係と対応検査を保持。同名だけで前行と統合しない |
| LoopCond V1（retained productなし） | `raw_loop_child_entry/loop_cond.rs:35`→loop_cond_bc_source→PlanLowerer | 元source portのPhiSessionが使える時はPhiTxn。そうでなければ低レベルのprovisional/patch/検査 | source portへcall等を委譲。一般Text/Dynamic carrierをV1 algebraが扱うとは主張しない |
| LoopTrue V1（retained productなし） | `raw_loop_child_entry.rs:569`→loop_true_break_continue_source→PlanLowerer | CoreLoopPlan/LoopFrame側 | 同上 |
| CheckedSource V2 | `raw_loop_child_entry/loop_cond.rs:27`→`lowerer/source_loop_completion.rs:10` | `source_loop_driver.rs:179`で実CFGを手で作り、`source_loop_phi_inputs.rs:31`でsource/実edge/cleanupを照合してincomingを作る。`loop_cond_phi_session.rs:288`でPhiTxnを使うが、CanonicalCfgSessionには未統一 | call、Fault、Text/Dynamic carrier、Array、Return、子loop、scope cleanupを既存invocationで実行 |

`CallableLoopSoleFamilyV1`は3 tag（LoopCondBreakContinue / LoopTrueBreakContinue / LoopCondReturnInBody、`normal_callable_loop_source_route.rs:94`）。retained productありの3 tagはすべてLoopCondReadyのchecked V2入口を使う（`generic/issuer.rs:87`）。したがって**LoopTrueというtagを見ただけで旧LoopTrue物理laneを使ったとは判定できない**。

関数入口→callable子loop→node winnerという3層は残るが、すべてを先着と呼ぶのも誤り。typed selection、選択済みproductの優先、先効果の非membership dispatchが混ざる。`RouteNotFrontSelected(NonGenericOrOverlapping)`からnode入口へ移るのはまだproductを消費していない分岐（`raw_loop_child_entry.rs:603`）で、実行失敗後のretryに変えてはならない。

### 2.4 関数profileと未接続corridor

| profile | 入口 / common session | 確認範囲 |
|---|---|---|
| CallableSingleLoop | `normal_cataloged_box_method_lowering.rs:240`→physicalizer/callable_lowerer.rs:39 | canonical session、prelude call capability、loop operations、tail/Return。一般のbody call/借用範囲は未精査 |
| Main0 Continue | `normal_callable_semantic_loan_port/main0_root.rs:26`→main0_continue_lowerer.rs:294 | 同じcanonical session、initialized inputs、共通dispatch。入口でexact profileを選ぶ |
| Main0 InBodyStep | 同`:91`→main0_in_body_step_lowerer.rs:225 | 同上。stepの意味/入力を勝手にContinueと同一化しない |
| Main0 DerivedPredicate | 同`:159`→main0_derived_predicate_lowerer.rs:333 | 同上。derived carrierの元証拠を保持 |
| GenericG0 | `normal_callable_semantic_loan_port/generic_g0.rs:43`→generic_lowerer.rs:33 | canonical function session。node familyのG0と同じprofileなので二重に合算しない |
| Dynamic loop canary（上5種に含めない） | `normal_callable_dynamic_loop_prepare.rs:147`、`resolved_lowering/dynamic_loop_phi.rs:197` / `dynamic_loop_phi_close.rs:127` | common CFG/SSA/PhiTxnへの呼び出しを確認。`prepare_source_backed_dynamic_loop_ingress`のcaller検索は定義＋test3箇所のみ。今回のproduction入力での接続0。内部全契約の精査・実行はしていない |

S6Cはparkedのまま。共有canonical session内にS6Cの欄があっても、この調査は再開/削除の許可にしない。physicalizer/mod.rsの古いcaller-zeroコメントは、routerやGenericG0の実callerと一致しないため削除根拠には使わない。

## 3. 今通る形と測定値

### 3.1 Sourceのloop関数数

| 集合 | 入力 | AST失敗 | loop関数出現 | 異なるloop関数 | loop文出現 | loopを含むAST成功入力 |
|---|---:|---:|---:|---:|---:|---:|
| 229入力 | 229 | 15 | 25 | 13 | 25 | 9 |
| apps100 | 100 | 1 | 72 | 63 | 77 | 51 |
| 329入力 | 329 | 16 | 97 | 64 | 102 | 60 |
| 1459入力 | 1459 | 87 | 174 | 144 | 186 | 148 |
| 384組のsource | 384 | 0 | 0 | 0 | 0 | 0 |

AST失敗の補助走査で見つかったloop関数候補は、229/apps100/329/384は0、1459は**77候補・16入力**。この16入力も全て正規EXEを試し、全てコンパイル拒否だった。1459のsource全体を「174だけ」と断定せず、「AST確定174＋失敗source内の字面候補77」と読む。失敗には廃止構文・ownership署名をAST-onlyへ投影できない拒否等がある。`probe-init/existing.hako`はAST出力が30秒timeoutで、unknownとして残した。

主な入力: 早期break/continue、累積・条件更新、二重loop、loop内return、文字列の更新・scan、Array/Map操作を伴うアプリ。shape名から担当は推測せず、次のtraceとコードを照合した。

### 3.2 正規EXEと担当

| 集合 | loop入り入力のEXE生成 | その入力に含むloop関数数 | 15秒以内に終了 | checked V2の選択が観測された入力 |
|---|---:|---:|---:|---:|
| 229 | 8/9 | 24 | 8 | 8 |
| apps100 | 51/51 | 72 | 51 | 51 |
| 329 | 59/60 | 96 | 59 | 59 |
| 1459（AST成功分） | 72/148 | 89 | 71 | 71 |
| 1459（補助走査分） | 0/16 | 0 | 0 | 0 |
| 384 | loop source 0 | 0 | 対象外 | 0 |

集合間に重複があるので合算しない。関数数は**成功した入力内の宣言**であり、全関数が実行されたという意味ではない。`using`先は含めない。一方、下のtrace観測はimport先も含む。

既存の選択診断を関数名＋family＋product有無で入力ごとに重複除去した観測数（同じ関数に同じtagのloopが複数なら1観測、loop site数ではない）:

| 集合 | CondBC / productなし | CondBC / productあり | ReturnInBody / productあり | TrueBC / productあり |
|---|---:|---:|---:|---:|
| 229 | 8 | 26 | 2 | 0 |
| apps100 | 4 | 67 | 4 | 3 |
| 329 | 12 | 93 | 6 | 3 |
| 1459 | 0 | 86 | 3 | 3 |

productありはコード上checked V2（`generic/issuer.rs:87`→`raw_loop_child_entry/loop_cond.rs:27`）。productなしCondBCはV1側。例: allocator-stressのimport先`HakoAllocPage.seedBlocks/0`、`SizeClassBox.bin_size/1`はV1、`AllocatorStress.run/0`はV2。json-streamの`ingest`、`JsonLine.find`、`StringHelpers`もV2が観測された。

- VariableAccum / DirectBreak / CompositeBreakの選択flagが立った成功入力は**0**。旧LoopTrue V1（productなし）も**0**。これは今回の入力集合の観測であり、repo全体のcaller-zero証明ではない。
- node winnerの4 family、CallableSingleLoop、Main0の3 profileは、既存ログから個別の成功選択を確認できた入力**0**。専用ログがない所は「未識別」であり、実際の使用0とは断定しない。既存callerは残る。測定用hookを足して埋めていない。
- `tl/g0.hako`はnested loopのEXEがexit3、raw-loop診断なし。コード上はtop-level GenericG0の関数入口と整合するが、familyの実選択そのもののtraceは無いので**静的経路の読み**として扱う。GenericG0を未使用として消さない。
- `v/c.hako`はEXE生成成功、実行15秒timeout。`i > 0`の間`i=i+1`するsourceであり、終了/出力の正しさを確認した入力には数えない。修正や期待値の差し替えはしていない。

## 4. 重なりと規模

行数は物理行（空行/コメント込み）、test命名fileとinline `#[cfg(test)] mod`を除外。個別のtest用小itemがproduction file内にある場合までcompilerの実行可能LOCとして厳密に分けた数ではない。重複する範囲を合算しない。

| 範囲 | file | production側の行数 |
|---|---:|---:|
| control_flow/plan 全体 | 262 | 44,533 |
| うちlowerer | 24 | 5,495 |
| うちsource_loop* lowerer | 5 | 1,640 |
| resolved_lowering/loop_recipe_physicalizer | 24 | 5,471 |
| resolved_semantics/home_prefix_source_loop* | 6 | 2,986 |
| home_new_prefix_source_loop＋search_loop | 2 | 393 |
| normal_callable_loop_source_facts下 | 11 | 4,631 |
| loop_recipe_contract/source_loop* | 8 | 3,482 |

### 4.1 同じ問いへの別の答え

| 問い | Plan/checked-source側 | canonical physicalizer側 | 統合時に残す物 |
|---|---|---|---|
| PHIを先に定義、incomingでpatch、失敗を戻す | `lowerer/loop_preparation.rs:125`、`phi_processing.rs:160,211,269`、`loop_cond_phi_session.rs:288,299` | `loop_node_lowerer.rs:86`＋`PhiTxn` | 完全なincoming、実CFG/支配、失敗原子性。単に検査を消さない |
| どのpredecessorの値がHeader/Afterへ届くか | `lowerer/source_loop_phi_inputs.rs:85`のmap、`phi_processing.rs:72`のdeferred continue入力 | `canonical_ssa/identity.rs:190,347`、`recursive_after.rs:74` | sourceのlogical edge証拠と実MIRの対応検査。source値の記録と物理PHIの発行を混同しない |
| block/edgeを作り、いつ閉じるか | `source_loop_driver.rs:179,632`の手書きdriver、通常PlanBuildSession | `loop_node_lowerer.rs:86`、`topology.rs:118`、CanonicalCfgSession | Fault/cleanupの実際の終端をpredecessorにすること。cleanup前blockで代用しない |
| 呼び出し/式/cleanupをどの順に出すか | `source_loop_frames.rs:20,91,230,368,472`、checked Expression/Invocation packet | `operation_dispatcher.rs:30`はpure/read/write/seedのみ | 機能の差がある。scalar dispatcherへ丸投げするとcall/Fault/Text/Dynamicを失う |
| function prelude→dispatch→after→tail/Return | callable/generic/main0各lowerer（例 generic_lowerer.rs:145,181 / callable_lowerer.rs:39） | 同じsegment allocator/dispatcher/recursive-after | 入力/署名/Completionの異なる契約。共通化対象は手順でありsemantic receiptを偽装しない |

source_loop_phi_inputsの128行は「重複だから全部削除」としない。実edgeとcleanup/元sourceを照合する責務と、生成用のincoming mapの組立てが一緒にある。後者を共通SSAへ移しても、前者のsource対応/独立MIR検証は保持する。

### 4.2 loopの別のHome並び

「6か所」を再確認すると、対象は線形walkの`HomesV1`への直接writeではなく、**`loop_recipe_contract/source_loop_execution.rs:533`のexecution検証/計画で持つ別のVec**だった。body内は次の6か所（関数実測127行）:

| 行 | 更新 |
|---|---|
| 559 | stable Home追加 |
| 587 | Text初期化のHome追加 |
| 596 | scalar-result等に対応するstable Home追加 |
| 604 | Text再代入前の旧責任をretainで除外 |
| 628 | Text再代入後の新責任を追加 |
| 652 | ifのjoined_homesを採用 |

さらに`:742`でportのFault列へText acquisitionをinsertする別の組立てがある。6という数字だけに合わせて7番目を無視しない。`normal_port(:359、30行)`と`home_prefix_source_loop_ports.rs:485`も逆順/跨ぐscopeのEndを求める。同じ責任列を扱うが、前者には検証の役割があるため、既存`HomesV1`へ機械置換して独立性まで消す案は採らない。共有すべきものはoperation/edgeの不変な所有契約、残すものは生成物・元sourceとの照合である。

### 4.3 GenericG0と境界

GenericG0は既に共通sessionを使うbounded helperであり、「汎用loop全体」でも「caller-zero」でもない。`generic-loop-source-to-portable-recipe-ssot.md:5`のstop lineと一致する。GenericG0をLoop cohortと同時に扱うcardの保留を維持する。S4でG0のentry materializationと共通dispatch/SSAを照合するが、新しい選択profileや受理拡張を足さない。

SSOTのone-function Binding SSA（binding-ssa-first SSOT:23、CanonicalCfgSessionの`finish`:371）は全関数を閉じる。loop断片だけ新sessionでfinishして外側prefix/Fault blockを無視する案は成立しない。一方、node lowererは現状nodeごとのsessionも持つ。この差はS4が直す既存の未収束であり、目標を「すでに共通」と読み替えない。

While/LoopRangeを独立した新しいphysical familyとして増やす案は出さない。既存のstrict capability境界は`control_flow/joinir/control_tree_capability_guard.rs:126`で拒否/metadata-onlyの診断を持つ。今回取得したAST JSONにLoopRangeは無かった（kind検索の観測）。surfaceのrange仕様と、この実装coverageの境界は分ける。

仕様上は最寄りloopへのBreak/Continue（semantic-kernel.md:137）、cleanupのFaultがpending Return/Break/Continueを上書きする規則（同:112）が必要。range loopの評価/continue先はEBNF.md:1139以降。古いrange実装注記のloop-carried拒否を全loopの仕様禁止へ一般化しない。今回、構文や意味の変更は提案しない。

## 5. 案Aと案B

### 案A: 所有解析の進み方から置き換える

resolverのsite/評価順から未公開CFGを作り、loopをback edgeと固定点で扱う。既存の型/結果catalog・明示move/share/take・借用/commit契約を入力にする。最初の置換単位は**関数内のstatement-flow driver全体（ifとloopを含む一責務）**。loopだけ別解析を再帰起動して旧walkを残すのは、S3b案2の問題を解いていない。

- 退役候補の実測: scan_statement_flow 622行、walk_branch 103行、source-loop observe 185行、計910行。BranchPathとSourceLoopHeaderCheckpointV1もdriverの状態として置換候補。stage/finish headerは77＋98、exit/backedge/port記録は43＋27＋40行。これらは上の910行と別の関数である。
- ただしhome_prefix_source_loop*の2,986行全てが消えるわけではない。Text/Dynamic producer、元source証拠、consumer向けsummary/検証は引き続き必要。scan本体にも使うtransferの腕があるため、622行を全て意味の純削減として数えるのも誤り。
- 追加見積もり: CFG/順序700–1,200、pure transfer/固定点500–900、summary/issuer adapter500–900、計1,700–3,000行。これは測定値ではない。既存の910＋hookを置換する比較から、最初の段だけでは純増の危険が大きい。移した腕を新旧で二重に残すなら整理の完了とはしない。
- 状態は有限なsource identityで抽象化し、実ループの周回ごとにHomeを新発行しない。joinが合わない入力は現行の拒否を維持する。最初の拒否はworklistの処理順ではなく元source順で決める。
- 既存observer/closureを固定点の各iterationで呼んではいけない。ledger書き込み、call-route、selected_loop_products等は同site一度の発行が前提。操作の解決/不変な契約確定→pure状態解析→一度だけ発行へ分ける費用を含む。
- node=文だけなら既存のFault prefixが腕に隠れたままで、値の&&/||には足りない。式のExpressionChildReach/When、call-entry前後、Normal/Fault、cleanupの点までCFGへ表す必要がある。「腕を無変更で再利用できる」という強い前提は採らない。

### 案B: 物理側から共通ownerへ移す（推薦）

所有解析は既存walkのまま、callable/node/function profileの物理生成を共通の操作契約・CFG・JoinSig・PhiTxnへ寄せる。まず一責務を閉じ、次に実際に使うchecked-source側を移す。

**B1: PlanLowererのprovisional PHI lifecycleをPhiTxnへ統合。**

| 消す候補（現在の全callerを検索済み） | 実測行数 |
|---|---:|
| loop_preparation::insert_provisional_phis（125–149） | 25 |
| phi_processing::compact_error_for_freeze（19–33） | 15 |
| phi_processing::rollback_unpatched_provisional_phis（35–65） | 31 |
| phi_processing::validate_no_unpatched_phis（211–264） | 54 |
| phi_processing::validate_no_unpatched_phis_after_patch（269–320） | 52 |
| 合計（定義の物理行） | **177** |

既存`PhiTxn::define_provisional_phi / patch_phi_inputs / commit / abort_on_err`を使う。新規family、producer、公開型は0。削除する型は0（SourceLoopPhiSessionV1や元sourceのcapabilityをここで消さない）。既存source transactionは同じtxnを借り、normal経路だけ第2の所有者を作らない。patch_all_phis_with_transactionの既存入口も利用する。追加は制御/borrow配線と診断境界60–110行程度、旧callsiteの整理を含め純減は**概算67–117行程度**。177行全てを無条件に削除できるとの確約ではなく、互換診断の費用が超えれば純減まで同じ責務内で見直す。

到達範囲の注意: 測定でproductなしCondBCを観測したが、この入口も`loop_cond_after_role.rs:104`でsource PhiSessionをportへ渡している。B1の低レベルfallback5関数そのものを通ったかを既存traceからは確認できない。B1をアプリの受理拡張や実行速度の改善とは数えず、残存する二重管理の整理と位置付ける。最も利用されるchecked V2を移すB2は別に必須である。

危険: 現在のvalidate関数はstrict/dev＋planner_requiredの診断条件を持つ（phi_processing.rs:218,276）。新Txnの無条件commitとの違いを無視すると、拒否理由/最初の失敗/rollback後の状態が変わる。既存の診断policy境界で整合を取り、検査を無効化せず、元の失敗を保持する。これが成立したMIR/拒否/既存安全性検証でしか着地しない。空入力PHIを消す等の最適化は混ぜない。

**B2: checked-sourceのincoming・CFG・操作契約を移す。**

対象はsource_loop_driver/phi_inputsとSourceLoopPhiSessionの生成部分。5 lowerer fileは実測1,640行、phi sessionは466行（128行のphi_inputsは前者に含む）。しかし実CFG/source照合を丸ごと削る予算ではない。bodyを既存Invocation/Expression packetの共通操作として渡し、bindingの到達値をcanonical SSA、実edge/closeをcanonical CFG、PHIを同じTxnへ一度だけ発行させる。消える最終名の候補は生成用`VerifiedSourceLoopPhiInputsV2`の専用mapと`EmittedSourceLoopGraphV2`の専用CFG生成部分であり、source/実MIR照合の責務自体は残す。

- pure/read/writeしかないLoopOperationV1をそのまま増やせばよいとはしない。これはSerializeを持つportable Recipeのschemaでもある。call/Fault/Text/Dynamicの既存packetを扱う**未公開の共通operation contract**を既存ownerで整備し、公開JSONへ新形を漏らさない。source/物理IDを偽装しない。
- 一つの関数ownerのsessionへprefix/子loop/cleanup/Faultを含める。関数finishをloopごとに呼ばない。stageした操作とsourceの関係を共通入力にし、familyごとの再解析を止める。
- 費用の粗い範囲: 共通effect/CFG接続と元契約のadapter900–1,800行、旧生成部分500–1,200行が置換候補。範囲が重なるので総量の単純差や確実な純減は約束しない。独立検証/対応検査を除いた実際の削除境界を実装時に測り、純増の準備状態をS4完了とはしない。
- B1だけで止めてS4全体完了とはしない。B2以降で対象family専用のincoming計算・PlanLowerer/physicalizerの重複とadapterが消えた時が承認済みS4の完了である。

### 比較

| 観点 | A | B |
|---|---|---|
| 最初の効果 | 所有walkのif/loop再帰をCFG driverへ置換 | B1でPHI rollback/patch管理5関数を既存Txnへ置換 |
| 最初の純減 | 不確か。910行に腕も含み、共有契約への分離費用が大きい | 177行の具体的予算。追加60–110は見積もり。差分で判定 |
| 新旧が並ぶ期間 | driverの全出力/ledger発行/拒否順が揃うまで一段全体。部品を別解析として並行起動しない | B1の一commit内。B2はproducer/consumer切替と旧生成部分削除を同seriesで閉じる |
| 公開MIR/ABI/descriptor/C | 私有CFG/transferの置換なら不要の想定。ただし実装未検証 | B1は不要。B2も既存Invoke/cleanupを私有操作として再利用する想定。portable Recipe schemaを変えるなら再判断 |
| 受理の危険 | 全所有解析、join、固定点、first-error、issuer副作用が対象 | B1はPHI失敗/診断、B2はCFG edge/cleanup/実入力順序が対象 |
| 基本式 | op粒度CFGなら同じ土台に乗る | 物理共通化は使える。意味/型解析を新しく得るわけではない |
| edit | 同じPlace/commit/借用状態へ繋げられる設計。既存腕をpureにする費用が必要 | CFG/PHIは使えるが、書込Placeや排他の意味解析は別途必要 |
| 値の&&/|| | ExpressionChildReachまでCFG化すれば乗る。文CFGだけでは不可 | CFGの分岐生成を再利用できるが、旧所有walkのままでは自動的に受理されない |
| 6か所の別Home並び | operation/edgeの共通transferへ移す主対象 | B1では触らない。B2でimmutableな所有契約を入力にし、検証/生成を分離して整理 |
| GenericG0 | 新解析へ同時に移す範囲を契約で決める必要がある | 今のbounded canonical sessionを比較対象にできる。勝手な汎用化は不要 |

「BならAの問題も全部解決する」とはしない。推薦理由は、実在する共有ownerへ、小さく消せる責務を先に移せること。B2の主戦場を既存アプリが使うchecked V2に固定し、使われていないprofileの殻だけ整理し続けない。

## 6. 選択後の最初の規則の下書き（未承認・未実装）

- L1. S4は受理・意味を変えず、元source identity、型/結果契約、評価順、Normal/Faultの責任と独立検証を保持する。
- L2. 同じprovisional PHIは一つの既存PhiTxnだけが所有し、define/patch/commit/abortの全経路をそこへ集約する。
- L3. 既存source transactionを借りる入口に別Txnを並べず、成功/失敗ともpending PHIを残さない。
- L4. 最初の失敗、strict/devの診断policy、rollback失敗の情報を既存境界で保持し、検査を弱めて互換にしない。
- L5. 元source/実MIRの照合を消さず、familyごとの生成用incomingはcanonical Binding SSAから得る。
- L6. cleanupのNormal/Fault後の実際の終端をCFG predecessorとし、cleanup前のsource辺で代用しない。
- L7. 一つの関数のCFG/SSA sessionをprefix・loop・子loop・Fault・cleanupに使い、全辺が揃う前にseal/finishしない。
- L8. call/Fault/Text/Dynamic/借用は元の検査済み操作契約を共通入力に保持し、scalar/Unknown/偽Bindingへ変換して通さない。
- L9. 選択は効果より前に一度だけ行い、旧優先順を無条件の全候補Overlap拒否へ置換せず、選択後の失敗でretryしない。
- L10. route_loopへ元resolver siteを入口から渡す。旧AST同形曖昧拒否が変わる場合は受理不変の整理と分け、ユーザー判断へ戻す。
- L11. 置換した関数/専有adapterは同じseriesでcaller-zeroと削除を確認し、追加した共通処理を含むproduction純減で閉じる。
- L12. 生成物と拒否の比較は329＋1459、229入力/apps100/384の既存成功、既存安全性testを用い、空filter/空marker/timeoutをPASSにしない。

B1の完了条件: 上記5関数の実callerが新共通入口へ切り替わって旧定義が消える、production純減（移動/圧縮による帳尻なし）、対象外のsource/物理意味変更0、MIR/拒否が比較範囲で同一、既存のPHI/rollback/所有/Fault検証と製品EXEの結果が維持されること。今回はこの検証や実装は行わない。

## 7. 未確認・判断してほしいこと

1. **Aからか、Bからか。推薦はB1→B2。** ユーザーの選択後に別指示書で実装する。今回のbranchには文書だけを置く。
2. resolver siteを渡す時、今の「同形のloopが二つあると曖昧」という拒否を解消するか。正確なsiteによる生成が可能でも、受理不変の整理で勝手には開かない。これは文法変更ではなく既存coverageの判断候補。
3. B2でportable RecipeのSerialize schemaまで変更する必要が出た場合は、新しい公開契約の判断が必要。現時点では私有operation contractの整理で既存MIRを使う案であり、公開変更が必須だと確認できたわけではない。
4. CFG/SSA/PhiTxn共通化の全体行数・速度は予測しない。B1の純減/既存MIR同一を実測して次へ進む。B2の借用・Text/Dynamic・Faultの全組合せが既存session APIで閉じるかは未実装。
5. 全plan fileや全legacy coverage rowの内部は読んでいない。表で引用した入口/型/該当箇所は読んだが、262 file全部の意味の重複を証明した調査ではない。個別profileの未精査箇所と未識別のfamily選択を、caller-zero/不要コードとは扱わない。

測定scriptとlogは全てW。`measurement-metadata.json`（input hash/環境）、`sets.json`、`ast-results.json` / `ast-summary.json`、`lexical-fallback.json`、`trace-results.json` / `trace-extra-results.json`、`selected-family-results.json`、`line-counts.json`、`function-spans.json`。各入力のAST/trace/compiler logも`ast/`と`trace/`に残す。sourceを変えず、build・全lib・suite再実行・Python C検証はしない（文書/調査のみ）。
