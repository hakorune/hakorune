# static の呼び出しの結果の道を調べて、1 本にする案を比べる(読解と実測の記録)

Status: 調査の記録です。authority ではありません。branch `static-route-survey`(main `2bc4dfb372` から)。**source は変えていません**。調べるために `eprintln!` の計器を一時的に入れましたが、報告の前に `git checkout -- src` で戻しました(入れた物は §1.4)。行番号は特に断らない限り `2bc4dfb372` のものです。

## 0. 要約(ユーザー向け)

- **static の呼び出しの結果を caller が受ける道は 6 本あります**(§1)。結果の種類で分けると、I64 と Bool は 3 本(型つきの静的な実行、borrowed handle の formal を取る scalar cohort、式の値としての引数枠)、String は 3 本(引数なしの packet、String の引数を取る式入力、同じ式入力の Bool/I64 版)、object は direct の観測(bare な呼び出し)の 1 本だけです。**qualified の `Main.f(..)` で object を返す道は、前から 1 本もありません。**
- 受理の表(§2)は、384 組を流して 106 組が通りました。結果の種類で見ると、**object を返す組は 80 組(qualified 72 + bare/Other 8)のうち、qualified は 0 組、bare は宣言なしの引数なし 1 組だけ**です。
- 純粋な scalar の `main()` から `local a = Main.f(5)` を呼ぶ形は、宣言があっても `qualified-route-retired` で拒否されます。`new` を 1 つ持つ文脈か `main(args)` にすると通ります(§2.3)。道が **main の文脈で選ばれている**ことになります。
- instance の道が狭いのは、結果を読む側の `a.get()` の判定です。nullable は、宣言なしなら instance で通り、`T?` の宣言は全ての道で `UnsupportedResultType` で止まります(§3)。
- 案は 4 つ比べました(§4)。**勧めは案 B の限定版**: 型つきの静的な実行(道 1)に「結果の種類」を持たせ、引数がない、または全部 i64 の static で String と object を返せるようにします。公開 MIR の追加は要らない見込みです(`InvokeCallResultKind::{OwnedString,Handle}` は既にある)。ただし**未確認が 3 つ**あります(§5)。案 A は、direct の観測が bare な名前だけで、通る形が 1 組しか無いので勧めません。案 C は、形ごとの finish を足すので勧めません。

## 1. 道の一覧

### 1.1 数え方

- `LocalRouteWriterV1`(`ordinary_new_call_routes.rs:13`)が、local の呼び出しを書く段を 6 つ挙げています(`UnitResult`、`LexicalInstance`、`DirectStatic`、`DirectLifecycle`、`ObjectSlot`、`StaticPacket`)。これは**物理の側の書き手**で、道の数ではありません。
- 道は「caller が static の呼び出しの結果を受けると決める所」で数えました。決める所を、`has_routed_static_local_for_owner_v1`(`ordinary_new_borrowed_static_packet_entry.rs:451`)と、main の `static_local_route`(`builder/normal_callable_semantic_loan_port/main_root.rs:161-177`)から逆にたどり、計器(§1.4)で**実際にどの道を通ったか**を 398 本の実行で確かめました。
- 「通る」は EXE まで build して実行できたことです。

### 1.2 6 本の道

| # | 道 | 入口(file:line) | 受ける formal | 結果の種類 | 受ける使い先 | その道だけが使う production の file(行数) |
|---|---|---|---|---|---|---|
| R1 | **型つきの静的な実行** | `direct_call_static_execution.rs:106` `join_static_execution_v1`(加える site は `:180` の `additions.push`)、`validate`(`:45-64`)、`seal_local`(`:87`)、emission は `StaticExecutionEmissionV1`(`:21`) | 全部 `ExactTrivial(I64)`(`:55-58`)。formal なしも可 | I64、Bool(`scalar_invoke_result()`。`:151`) | `local`、後続の値の読み | `direct_call_static_execution.rs`(259) |
| R2 | **scalar cohort の finish** | `select_static_scalar_local_routes_v1`(`ordinary_new_borrowed_static_packet_entry.rs:212`)、`checked_completed_static_scalar_cohort_v1`(`..._scalar_input_finish.rs:134`)、`finish_static_scalar_input_actuals_v1`(`:211`) | borrowed handle の formal 1 つ(`is_ordinary_borrowed_handle`。`..._scalar_input_finish.rs:271` 付近) | I64、Bool(`qualified_scalar_result_v1().is_some()`) | `local` | `ordinary_new_borrowed_static_scalar_input_finish.rs`(529)、`..._scalar_caller_rows.rs`(298)、`..._bool_entry_finish.rs`(387) |
| R3 | **Text の引数なしの packet** | `select_static_zero_local_routes_v1`(`packet_entry.rs:188`)、`is_static_zero_packet_cohort_v1`(`:175`)、`selected_static_local_source_v1` の zero packet の枝(`:302`。計器の branch 6) | なし(`is_zeroarg_text_v1`。`qualified_static_incoming_source.rs:94-97`) | String(`ExactString`) | `local`、`print`、`return` の転送 | `ordinary_new_borrowed_static_packet_entry.rs`(844。800 行を超えている)の一部 |
| R4 | **String の引数を取る式入力(tagged/mixed/one を含む)** | `is_static_expression_input_owner_v1`(`static_input_selection.rs:86`)、`checked_completed_static_expression_input_v1`(`static_input_validation.rs`)、`checked_static_tagged_packet_actuals_v1`(`..._tagged_input_finish.rs:150`)、`checked_static_mixed_packet_actuals_v1`(`..._mixed_input_finish.rs:254`)、`checked_completed_static_one_actuals_v1`(`..._one_input_finish.rs:38`)。計器の branch 2 | String(`x: String`)、1 個の借用の actual | I64、Bool、String(宣言なしの形) | `local`、`print` | `..._tagged_input_finish.rs`(430)、`..._mixed_input_finish.rs`(567)、`..._one_input_finish.rs`(326)、`..._input_finish.rs`(540)、`static_input_selection.rs`(232)、`static_input_validation.rs`(521) |
| R5 | **式の値としての引数枠** | `packet_entry.rs:500-515`(`CallExecutorV1::at(..).is_argument_frame()`。`expression_position.rs:88`)。計器では何も出ない(`selected_static_local_source` を通らない) | 全部 I64 | I64 | `print(Main.f(5))` のような式の値 | `ordinary_new_borrowed_static_source_actuals.rs`(545)、`static_argument_projection.rs`(387)、`ordinary_new_borrowed_static_selected_actual.rs`(203)の一部 |
| R6 | **direct の観測(bare な呼び出し)** | 観測は `resolved_semantics/direct_call.rs:27` `ResolvedDirectCallObservationV1`(名前と arity と引数の site だけ)。route は `ordinary_new_coseal_issue_local_route.rs:42-56`(`DirectI64`/`DirectMap`/`DirectHandle`)。結果の class は `direct_call_lifecycle.rs:455-472` `local_result_class_v1` | 引数なしのみ通った(§2) | I64、Map、**Handle**(`construction_result_callee`。`:111`) | `local`(読まない形) | `direct_call_lifecycle.rs`(747)、`direct_call_loan.rs`(531) |

補足:
- 6 本のうち、結果を「呼び出し側の claim」で決めるのは R1〜R5(`qualified_static_call_claim.rs:154-171`。`scalar_result_class` が I64/Bool だけを返す)で、**R3 と R4 の String は claim の索引に入らず**、`StaticIncomingSourceV1` の `source_result()` が `ExactString` かで決まります(`qualified_static_incoming_source.rs:94-97`、`result_contract.rs` の `corroborates_static_text_source_v1`)。
- main がこの 6 本のどれかを選べない時は、廃止された qualified の経路を名前つきで拒否します(`main_root.rs:161-177`、`qualified-route-retired`)。つまり**6 本が全部漏れた結果**がこの名前です。
- `qualified_scalar_result_v1()` を使う所は 39 か所(`grep -rn "qualified_scalar_result_v1" src/mir | grep -v "tests\|fn qualified_scalar"` の行数。R1〜R5 の file に散っている)。

### 1.3 道に共通する物

- 結果の契約の行は、callee の `VerifiedCallableResultDispositionV1`(`callable_result_representation/disposition.rs`。`ExactI64`/`ExactString`/`ExactBool`/`ExactNominalBox`)が 1 つ持っています。`ExactNominalBox` だけは `required_i64_arguments` を持たず(`expression_proof.rs:371-375` の `New` が引数の要求を捨てる)、`required_i64_arguments()`/`scalar_result_class()` が `None` を返すので、**object の callee は claim の索引に入りません**(`qualified_static_call_claim.rs:154-171`)。
- 物理の発行は、道ごとの writer が `record_lifecycle_local_call_site` で site を登録します(`direct_call_static_execution.rs:101` の `DirectStatic`、`packet_entry.rs:444` の `StaticPacket`、`direct_call_lifecycle.rs:591,742` の `DirectLifecycle`)。

### 1.4 入れた計器(戻した物)

`eprintln!` を次の場所に入れ、`hakorune.dbg` を build して 398 本を流し、戻しました(`git status` が文書だけの変更であることを確認)。
- `direct_call_static_execution.rs`: `additions.push` の前(R1 の加入)
- `ordinary_new_borrowed_static_packet_entry.rs`: `selected_static_local_source_v1` の 6 つの `return Ok(Some(..))`
- `ordinary_new_coseal_issue_local_route.rs`: direct の観測の枝と、qualified/current-owner の static claim(claim の結果の種類も出す)
- `ordinary_new_borrowed_formal_result_pending.rs`、`..._composition.rs`: `source-not-i64` を作る 4 か所

## 2. 今の受理の表

### 2.1 入力と選んだ理由

入力は生成 script `gen.py` で作りました(`/mnt/workdisk/claude-scratch/static-route-survey/in/`、一覧 `names.txt`、結果 `table.txt`、道の判定 `per.json`)。文は改行で区切っています。

- **結果**: i64(`7` か `v + 1`)、Bool(`true` か `v > 1`)、String(`"abc"`)、object(`new Leaf(5)` か `new Leaf(v)`)。
- **引数**: なし、i64(`v: i64`)、String(`x: String`)、object(`p: Leaf`)。callee の本体は引数を使わない定数、または `v` を使う式。
- **使い先**: `local`(`local a = CALL` を作って使わない)、`print`(`print(CALL)`)、`after`(受けた後に使う: i64 は `return a + 1`、Bool は `if a { return 1 }`、String は `print(a + "x")`、object は `return a.get()`)、`fwd`(callee の呼び出しを `return` する wrapper `g` を作り、main が `Main.g()` を受ける。qualified の文脈なしだけ)。
- **呼び方**: qualified(`Main.f(..)`)、bare(`f(..)`)、別の static box(`Other.f(..)`)。
- **宣言**: あり(結果と formal に型を書く)・なし(どちらも書かない)。
- **main の文脈**: なし(`main()` だけ)、`new Ctx()` を 1 つ持つ、`main(args)`。道は main の文脈で変わる(§2.3)ので、qualified の呼び出しはこの 3 つで流した。
- 選び方: 64 組(結果 4 × 引数 4 × 使い先 4)を、qualified の 3 つの文脈で全部流した。bare と別の static box は、結果 4 × 引数 {なし, i64} × 使い先 {local, print} だけ。**選んだ理由**: bare と別の static box は、qualified と同じ callee の道を通るかを見るだけなので、道の境目が現れる最小の組にした。
- 合計 **384 組**(`q_`: 128、`cn_`+`ca_`: 192、`b_`+`x_`: 64)に、追加の 14 本(§2.4 の単体入力と instance の 10 本)。

### 2.2 結果(384 組、通ったのは **106 組**)

| 見方 | 流した | 通った |
|---|---:|---:|
| qualified、文脈なし(`main()`) | 128 | 30 |
| qualified、`new` の文脈 | 96 | 35 |
| qualified、`main(args)` | 96 | 35 |
| bare | 32 | 1 |
| 別の static box | 32 | 5 |

qualified(3 つの文脈、320 組)の内訳:

| 結果 | 宣言なし 通った/流した | 宣言あり 通った/流した |
|---|---:|---:|
| i64 | 20 / 40 | 17 / 40 |
| Bool | 17 / 40 | 16 / 40 |
| String | 20 / 40 | 10 / 40 |
| **object** | **0 / 40** | **0 / 40** |

| 引数 | 通った/流した | 使い先 | 通った/流した |
|---|---:|---|---:|
| なし | 38 / 80 | local | 39 / 96 |
| i64 | 12 / 80 | print | 15 / 96 |
| String | 26 / 80 | after(受けた後に使う) | 42 / 96 |
| object | 24 / 80 | fwd | 4 / 32 |

### 2.3 道と結果・文脈の対応(計器の出力)

計器の出力で、通った 107 本(格子 106 + 追加 1)を道で分けた結果(2 本は 2 つの道の計器を通るので、合計は 109 になる: `q_is_fwd_u`=R1+R4、`q_ss_fwd_u`=R3+R4):

| 道 | 通った本数 | 通った組の特徴 |
|---|---:|---|
| R1 型つきの静的な実行(`typed-static-execution addition`) | 28 | I64/Bool の結果、formal は全部 i64 か無し、**main に `new Ctx()` か `main(args)` の文脈がある**(declared/undeclared で変わる: `ii` は宣言ありだけ) |
| R2 scalar cohort(claim は I64/Bool、`selected_static_local_source` を通らない) | 24 | formal が object(`p: Leaf`)、結果が I64/Bool(`io`、`bo`)、`local`/`after` |
| R3 Text の引数なしの packet(branch 6) | 25 | 結果が String、引数なし(`sn`)。`local`、`print`、`after`、`fwd`、別の static box でも |
| R4 式入力(branch 2) | 26 | formal が String(`x: String`)、**宣言なし**(`is`、`bs`、`ss` の `u`) |
| R5 引数枠 | 5 | `print(Main.f(5))` で I64 の結果(`ii print d`) |
| R6 direct | 1 | bare、object の結果、引数なし、宣言なし、`local`(読まない) |

純粋な scalar の `main()` では、R1 の `join_static_execution_v1` の `context`(`direct_call_static_execution.rs:124-141`。owner が「呼び出し関係を持つ、`new` を要する、map や名前付き配列を持つ」時だけ加える)に入らないため、**宣言あり・なしにかかわらず `qualified-route-retired` になります**(`q_ii_local_*`、`q_ii_after_*`)。同じ入力に `local z = new Ctx()` を足すか `main(args)` にすると通ります(`cn_ii_local_d`、`ca_ii_local_d`)。

### 2.4 追加の入力(単体)

`v1`〜`v4`(`f(v: i64): i64` を `local a = Main.f(5)`/`return a`/`return a + 1`/`print(Main.f(5))` で受ける。`v4` だけ通る)、`inst_q04/q07/q10/q11/q14`(Opus の入力、宣言ありとなし)、`nl_inst_u/d`、`nl_inst0_u/d`、`nl_stat_u/d`(nullable。§3)。

## 3. instance の道の狭さ

宣言の有無で前から同じに拒否される形(Opus の入力 `q04`、`q07`、`q10`、`q11`、`q14` と、宣言なしの `u` 版)を流しました。**宣言ありもなしも同じ名前で止まる**ことを 10 本で確かめました。

| 入力 | 形 | 止まる名前 | 止まる所(file:line) | 理由(読んだ範囲) |
|---|---|---|---|---|
| `q10`/`q10u` | 受けた local の入れ直し `a = m.mk()` | `ordinary-new/borrowed-nullable/local-initializer` | `ordinary_new_result_class_claim/call_witness.rs:439-453` | `binding_is_unrebound` が偽(入れ直しがある binding は claim を作らない)。`sole_initializer` が 1 つであることを要る |
| `q04`/`q04u` | field の返却 `return me.f` | `ordinary-new/borrowed-result/source-not-i64` | 作る所は `ordinary_new_borrowed_formal_result_composition.rs:607`(proof が `IngressVerified` でない、または `grounded` に無い owner を `Err` にする) | `me.f` の返却は「callee が `new` する class」の claim に入らず、結果の proof が grounded にならない |
| `q07`/`q07u` | 自分の型を返す `next(): Maker` | 同上 | 同上 | 同上(`new Maker(me.k + 1)` を返す callee の結果を、`n.get()` で読む所) |
| `q11`/`q11u` | 受けた後の `a.get() + 1` | 同上 | 同上 | 受けた object の method の結果を**算術の operand**にする使い先 |
| `q14`/`q14u` | 引数つき `m.mk(5)` を受けて `a.get()` | 同上 | 同上 | 引数つきの callee の結果を受けて読む形 |

観察:
- **読まない形は通る**: `nl_inst_u`(引数つき `m.mk(1)` を受けて使わない)、`f1d`/`f2d`/`f4d`(前の series)のように、`return a.get()` や `local n = a.get(); print(n)` は通る。`source-not-i64` は、`composition.rs:607` が「proof を `Err` にする」所で、**最終的にどの消費者がその `Err` を読んで拒否するかは確かめていません**(未確認)。計器では、通った 107 本でも `composition.rs:607` が実行されているので、この `Err` を作ること自体が拒否ではなく、読む側が拒否します。
- **nullable**: `T?` の宣言は、道を選ぶ前に `callable_parameter_contract/issuer.rs`(`UnsupportedResultType`)で止まります(`nl_inst_d`、`nl_inst0_d`、`nl_stat_d` の 3 本)。宣言なしの nullable は、**instance は通り**(`nl_inst_u`、`nl_inst0_u`: `if` で `new` か `null` を返す)、**static は object と同じ `selected-header-missing` で止まる**(`nl_stat_u`)。つまり nullable の結果も、static の道の欠如が同じ理由です。

## 4. 案を比べる

### 4.1 案の定義

- **案 A**: qualified の `Main.mk(5)` を direct の道(R6、`DirectHandle`)に載せる。
- **案 B**: 型つきの静的な実行(R1)に結果の種類を持たせ、i64、Bool、String、object を 1 本の道にする。1 本で足りる道を消す。
- **案 C**: object を返す static 専用の cohort の finish を足す(前の報告 §11 の (a)〜(d))。
- **案 D(足した案)**: 結果の種類を、道を選ぶ前に callee の contract の行から 1 つの表で決めて(R12 の延長)、道は「formal の種類」だけで選ぶ。結果の種類ごとに道を持たない。

### 4.2 比べる表

| 観点 | 案 A | 案 B | 案 C | 案 D |
|---|---|---|---|---|
| 内容 | qualified を direct に載せる | R1 に結果の種類を持たせる | object 専用の finish | 結果の種類の表を 1 つにし、道は formal で選ぶ |
| **消せる物** | R6 の一部(対象が増えるだけで、消えない) | R3(Text の引数なしの packet。`packet_entry.rs:175-211`、`is_zeroarg_text_v1` ほか約 60〜100 行)。formal なしの String は R1 で足りる。R5(引数枠)は I64 の式の値で別の使い先なので残る。R2、R4 は formal が i64 でないので残る | なし | R3、R5 の一部。R1〜R5 の 39 か所の `qualified_scalar_result_v1()` を表の 1 引きにできる |
| **足す物** | resolver の direct 観測を qualified にも出す、callable_index の seal に box の集合を渡す、`root-instance-call` が受けた Home を認める | `StaticExecutionEmissionV1` に結果の種類、`materialize_call` の結果の種類(`MirCall` の result)、`scalar_invoke_result()` を種類つきの表に、claim の索引に Handle/Text、`ExactNominalBox` に引数の要求 | 新しい finish 1 つ(引数の検証、source、publication handoff)、受けた Home の receiver、物理の発行 | 表 1 つ(結果の種類 × 使い先 = R15)と、39 か所の書き換え |
| **公開の追加(MIR、opcode、ABI、descriptor、C shim)** | 要らない見込み(`DirectHandle` は既存の `InvokeCallResultKind::Handle`) | 要らない見込み(`InvokeCallResultKind::{OwnedString,Handle}` は `instruction/invoke.rs:245,259` に既にある。静的な Call の結果としての使用が既存の descriptor で表せるかは**未確認**) | 要らない見込み | 要らない見込み |
| **受理を広げすぎる危険と防ぎ方** | direct の経路は読む形(`a.get()`)を通さない(§2、`d1`)ので、広がりにくい | `scalar_result_class` に Handle/Text を足すと、39 か所の `is_some()` が Handle を scalar として通す(`Main.mk(5) + 1`)。防ぐ手は R15 の使い先の表: 算術/i64 lane は I64 だけ、`local` は種類を問わない、を表の列にする。**前の試行では、`required_i64_arguments()` と `scalar_result_class()` を変えずに Handle 用の accessor を別に足せば、scalar の判定は触らずに済んだ**(§1.3、前の報告 §12) | 道が別なので広がらないが、形ごとの file が増える | 表で防ぐ。最も安全だが書き換えが大きい |
| nullable の結果 | 宣言なしは `NullableHandle` の route がある(`LexicalNullable`)。direct に nullable は無い | 表に行を足すだけ(`InvokeCallResultKind::NullableHandle`)。`T?` の宣言は `UnsupportedResultType`(`issuer.rs`)を先に外す必要がある | 別の finish がもう 1 つ | 表に行を足す |
| S4 の loop | 影響が小さい | loop の body の static 呼び出しは `ordinary_new_static_loop_*`(別の道)なので、R1 の変更は届かない | 同左 | 表を loop 側も読めば効く |
| 基本の式の series | 式の値としての static 呼び出しは R5 | R5 と R1 の「式の値」と「local」の使い先が表で統一される | 効かない | 効く |
| 段階の数と証拠 | 3(resolver の観測、callable_index の seal、receiver) | 4: (1) 表の整理(R1 の `exact_i64` を結果の種類の表へ。MIR 同一)、(2) Text の引数なしを R1 へ移す(R3 を消す。`sn` の 24 組が同じ出力)、(3) Handle の結果(`mk(v: i64): Leaf`。`Main.mk(5) + 1` の拒否入力を置く)、(4) 受けた Home の receiver(`a.get()`) | 4(a)〜(d) | 6 以上 |
| 証拠 | 329 + 1,459 本の MIR の同一。`d1`/`d3` 型の bare 入力 | 同上 + §2 の 384 組の表を前後で比べる(通った組が減らず、`o*` の組だけ増える) | 同上 | 同上 |

### 4.3 案ごとの所見

- **案 A** は、direct の観測が **bare な名前だけ**(`direct_call.rs:27` `ResolvedDirectCallObservationV1` は `name` と `arity` と引数の site)で、qualified の `Main.f` は `ResolvedMethodCallReceiverSourceV1::QualifiedUnbound` の別の観測です。**bare の組 32 のうち通ったのは 1 組(`b_on_local_u`)**で、引数つきの bare は `Resolver`(`AppMainDirectCall(TargetMissing)`)で全部止まり、宣言ありの bare も `callable_index.rs:414-420` の seal で止まります。通る形が 1 組しかない道に qualified を載せても、`a.get()` を読む形(`d1`)は `root-instance-call-new-missing` で止まります。
- **案 B** は、R1 が「全 formal が i64」の static にだけ効く(`validate` の `formal.kind != ExactTrivial(I64)` で弾く。`direct_call_static_execution.rs:55-58`)。そのため、`mk(v: i64): Leaf`、`t(): String`、`t(x: i64): String` は B で足りるが、`f(p: Leaf)`(R2)と `f(x: String)`(R4)は残る。**残る道は 3 本(R2、R4、R5)**で、全部が 1 本になるわけではない。
- **案 C** は、道を 1 本増やす。既に 6 本あるので、形ごとの file を増やさない決まりに反する。
- **案 D** は、39 か所の書き換えが要り、1 回の series では危険が大きい。案 B の最後の段階で表が育てば、結果として D になる。

## 5. 勧め

**案 B の限定版**を勧めます: R1(型つきの静的な実行)に結果の種類を持たせ、formal が無い、または全部 i64 の static の結果で、i64、Bool、String、object を 1 本で受ける。R2 と R4 は残す。理由:

1. R1 は、`DirectStatic` writer と `seal_local` で**既に「local の呼び出し」を物理に書いている**道で、必要なのは結果の種類(`scalar_invoke_result()` を表に)だけ。
2. 実測で、`mk(v: i64): Leaf` と `t(x: i64): String` の組(`io` ではなく `oi`、`si`)が、R1 の対象(formal が全部 i64)に当たる。
3. Text の引数なしの packet(R3)は R1 に統合でき、消せる物がある。
4. 公開の追加は要らない見込み。

**勧めが変わる条件**:
- `StaticExecutionEmissionV1::materialize_call` が作る `MirCall` に結果の種類を持たせても、`published-lifecycle` の物理 ABI が静的な Call の Handle の結果を既存の descriptor で表せないと分かった時は、公開の追加が要るので、案 B を止めてユーザーに聞く。
- 受けた object を読む形(`a.get()`)の `root-instance-call-new-missing` が、R1 の変更では直せない構造だと分かった時は、「読まない形だけ通る」で止めるか、instance の道(lexical)に寄せる別の案(§6 の案 E)を考える。

**未確認の所**:
1. `InvokeCallResultKind::Handle` が、静的な Call(`Callee::Global`)の結果として物理 ABI に渡せるか(`published_lifecycle` の descriptor の検査)。
2. 受けた Home を `a.get()` で読む判定(`ordinary_new_root_instance_call.rs:112-118,209`)が、R1 の local の呼び出しの結果でも、instance の lexical の経路と同じに働くか。
3. `source-not-i64`(§3)を最終的に出す消費者。
4. `qualified_scalar_result_v1()` の 39 か所のうち、R15 の使い先の表に当てて「Handle を通してよい」と読める所の数。
5. R5(式の値の引数枠)が、String や object の結果を式の値として使う形(`print(Main.mk(5))`)に広げられるか。
6. 別の static box の呼び出し(`Other.f`)の道は、qualified と同じ表(§2.2 の別の static box 32 組で 5 組)と読んだが、`Other` の box が複数ある時は試していない。

## 6. その他の案(未評価)

- **案 E**: static の呼び出しを、receiver の無い instance の呼び出し(lexical の経路)として扱う。instance の道は結果の種類が全部あり(Handle、NullableHandle、Text、I64)、前の series で宣言ありを通した。ただし lexical の経路は receiver の Home を要る(`lexical_instance_call.rs`)ので、receiver の無い呼び出しに広げる設計が要る。**未評価**(この調査では時間の範囲で読めていない)。

## 7. 時間と範囲

3 時間の範囲内で、道の一覧(6 本)、受理の表(384 組 + 追加 14 本 + nullable 6 本)、instance の止まる所、案 4 つの比べを書いた。読み切れていない所は §5 の未確認と §6 に書いた。
