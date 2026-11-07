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

---

# 続き(2 回目の読み。2026-11-07 23:09〜)

この節は、§5 の未確認 1〜3 と §6 の案 E を読んで答え、勧めを書き直す。**source は変えていない。計器も入れていない**(既存の `NYASH_BUILDER_DEBUG=1` の出力だけを使った)。行番号は `2bc4dfb372` のもの。

## 8. 案 E の評価(static の呼び出しを、receiver の無い instance の呼び出しとして扱う案)

### 8.1 lexical の経路が receiver に何を要るか

instance の `local x = recv.m(..)` は、次の 3 つの層で receiver を使う。

1. **対象の決定**: `lexical_claim_local_target`(`ordinary_new_coseal_issue_lexical.rs:21-100`)が、(a) 呼び出しの receiver が `ResolvedMethodCallReceiverSourceV1::Lexical(Local(binding))`(`:49-53`)、(b) その binding が `Local` で入れ直しが無い(`:54-67`)、(c) 初期化がちょうど 1 つ(`:68-75`)、(d) その binding が **`new` の claim の destination**(`candidates.iter().filter(|c| c.destination == binding)`。`:76-80`)、を全部満たす時だけ、`(key, batch_slot, callee_owner, arity)` を返す。つまり**receiver は `new` で作った Home でなければならない**。
2. **結果の class の決定**: 対象が決まった後の半分(`lexical_local_route`、`:162-191`)は receiver を使わない。`callable_result_classes.get(&key)` の claim が `NullableObject` なら `LexicalNullable`(`nullable_exits_hold`)、そうでなければ `all_formals_exact_i64`(`:107`)を満たし `construction_result_callee`(`direct_call_lifecycle.rs:111`)が真なら `LexicalHandle`、claim が無く `borrowed_i64_results.is_normal_i64_result_v1` なら `LexicalI64`。
3. **対象の表と物理の発行**: `LexicalInstanceCallSourceTargetV1`(`ordinary_new_lexical_instance_call_source.rs:13-27`)が `receiver_site` と `receiver: LexicalInstanceCallReceiverV1` を持つ。`ordinary_new_lexical_instance_call.rs`(683 行)が、receiver の値を引数に Invoke の Call を作り、`LocalRouteWriterV1::LexicalInstance`(`ordinary_new_call_routes.rs:13`)で local の呼び出しを登録する。

また、受けた object を `a.get()` で読む側の class は、`initializer_class`(`ordinary_new_lexical_instance_call_provenance.rs:391-538`)が決める。`local a = <初期化>` の初期化が (i) `new C`、(ii) `move`、(iii) `me.f`、(iv) **`recv.m(..)` で `recv` が lexical な local**(`:508-533`: `variable_ref(object)` が `Local` でなければ `Ok(None)` を返す)、のどれかなら class が決まる。**static の `Main.mk(5)` は (iv) の `object`(`Main`)が local ではないので `None`**。これが §3 の `a.get()` の止まる原因の 1 つ(未確認 2)。

### 8.2 receiver の代わりに置く物

static の呼び出しには receiver が無いので、(1) の対象の決定は流用できない。置く物は次の 2 つで足りる。
- **対象の決定**: qualified の名前から選ばれる static の key。claim の索引に既にある(`StaticIncomingSourceV1`、`qualified_static_incoming_source.rs`。`target`、`target_batch_slot`、`callee_owner`、`argument_sites`、`parameters`)。
- **receiver の無い呼び出し**の物理: R1 の `StaticExecutionEmissionV1::materialize_call`(`direct_call_static_execution.rs:66`)が `Callee::Global` の `MirCall` を作る。**instance の `Callee` に receiver を持たせる部分を除けば同じ**。

つまり案 E の実質は、「**結果の class の決定(2)を、instance と static で共有する**」。対象の決定(1)と物理の発行(3)は、receiver の有無で別のまま。

### 8.3 運べる結果の種類と、今どこで決まるか

| 結果 | 決める所(file:line) | instance | static(R1〜R6) |
|---|---|---|---|
| Handle | `lexical_local_route` の `construction_result_callee`(`..._lexical.rs:183`)、direct は `local_result_class_v1`(`direct_call_lifecycle.rs:455-472`) | 通る | R6(bare)だけ |
| NullableHandle | 同 `nullable_exits_hold`(`:180`) | 通る(宣言なし) | 無い(§3) |
| Text | callee の `ExactString`(`callable_result_representation`)。instance は `corroborates_instance_text_source_v1`(`result_contract.rs`)、static は `corroborates_static_text_source_v1`(`:277`) | 通る | R3(引数なし)、R4(String 引数) |
| I64 | `scalar_result_class`(`disposition.rs`)の claim、direct は `Some(I64)` | 通る(`LexicalI64`) | R1、R2、R4、R5 |
| Bool | 同 `ExactBool` | 無い(instance は `is_normal_i64_result_v1` で I64 だけ。Bool の規則 §7) | R1、R2、R4 |

### 8.4 消せる道と、足す物

- **消せる道**: 案 E 単体では**消えない**。R1〜R5 は、formal の種類(i64/borrowed handle/String)と結果の種類の組で選ばれていて、案 E は結果の class の決定を共有するだけなので、道は減らない。結果の class の決定は、instance の `lexical_local_route`(約 30 行)、direct の `local_result_class_v1`(約 20 行)、R1〜R5 の `qualified_scalar_result_v1()` の 39 か所、に散っているので、**3 か所の判定が 1 つの表になる**のが得(約 90 行の判定が表 1 つ)。
- **足す物**: (a) 結果の class の表(結果の種類 → route の種類と物理の Invoke の結果の種類。R12 の「結果の種類は契約の行から」の延長)、(b) 初期化が static の呼び出しの時の `initializer_class` の枝(§8.1 の (iv) に、static の key を選んで `callable_result_class(&key)` を返す枝を足す。約 15 行)、(c) R1 の gate の拡張(§9 の未確認 1)。
- **公開 MIR、opcode、ABI、descriptor、C shim の追加**: 要らない見込み(§9 の未確認 1)。

### 8.5 危険と防ぎ方、段階と証拠

- **広げすぎる危険**: 結果の class の表を共有すると、`Handle` が scalar の判定(`qualified_scalar_result_v1().is_some()` の 39 か所)に漏れる危険がある。防ぐのは、scalar の判定を変えず、Handle を別の accessor(`local_call_result_class`)で出すこと(前の series の試行で、これで 39 か所を触らずに済むと確かめた)。そのうえで、**Handle を通してよい使い先**(local、method の呼び出し)を R15 の表の列にする。`Main.mk(5) + 1` が通らないことは、拒否のままにする入力として置く。
- **段階**: (1) 結果の class の表を作り、instance の `lexical_local_route` と direct の `local_result_class_v1` をその表に替える(MIR の同一)。(2) R1 の gate(`direct_call_static_execution.rs:45-64,87-104,151`)を結果の種類の表で広げ、formal が無いまたは全部 i64 の static の Text/Handle を通す(`oi`、`si`、`sn` の組。§2)。(3) 初期化が static の呼び出しの時の `initializer_class` の枝で、`a.get()` を通す(§9 の未確認 2)。(4) R3 を R1 へ寄せて消す。各段階で、329 + 1,459 本の MIR の同一と、§2 の 384 組の前後比較(通った組が減らないこと)を取る。

### 8.6 案 B の表(§4.2)に足す列

| 観点 | 案 E |
|---|---|
| 内容 | 結果の class の決定を instance と static で共有する。対象の決定と物理の発行は receiver の有無で別 |
| 消せる物 | 道は減らない。結果の class の判定 3 か所(約 90 行)が表 1 つになる。R3 は段階 4 で消せる |
| 足す物 | 結果の class の表、`initializer_class` の static の枝(約 15 行)、R1 の gate の拡張 |
| 公開の追加 | 要らない見込み |
| 危険 | Handle の漏れ(防ぎ方は §8.5) |
| 相談の 7 との関係 | **「static と instance を共通にする」に合う**(結果の契約の行から決める部分を共有する)。案 B の限定版は static の道どうしだけ |
| 段階 | 4 |

## 9. 未確認 1〜3 の答え

### 未確認 1: `Callee::Global` の呼び出しの結果が `Handle`/`OwnedString` の時、物理 ABI と published-lifecycle の検査が受けるか

**読んだ範囲では、受ける。拒む検査は見つからなかった**(実行では確かめていない。静的な Handle の MIR を作る入力が今は通らないため)。
- 呼び出しは `MirInstruction::Invoke { operation: InvokeOperation::Call { call, result } }` の形で、`ordinary_call_site_v1`(`physical_program.rs:307-315`)が `call` と `result`(`InvokeCallResultKind`)を取り出す。callee の種類(Global か instance か)で弾かない(`BirthConstructor` だけを除く)。
- 物理 JSON の `ordinary_call`(`physical_program_json_call_transport.rs:288-330`)は、receiver があれば入れ、無ければ `{target, args, dst: null}` にし、`result` を `OwnedString => "text"`、`Handle => "handle"`、`NullableHandle => "nullable_handle"`、`I64`、`Bool`、`Map`、`Unit` に符号化する(`:305-316`)。static の Text(R3)が既に同じ符号化で通っている(`ordinary_text`)。
- callee 側の物理 function の role は、`OrdinaryHandle`/`OrdinaryText`/`OrdinaryNullableHandle` ほか(`physical_program_role.rs:6-70`)が `receiver_object: Option<CanonicalObjectIdV1>` を持つ。`ordinary_receiver_object`(`physical_program_object_identity.rs:7-21`)は、**instance の key でなければ `Ok(None)`** を返す。role の割り当て(`physical_program_projection.rs:218-265`)は、結果の種類だけで role を選ぶ。`compiled_entry_contract.rs:327-345,425-445` の検査も、role の列挙に `OrdinaryHandle` を含む。
- 残る未確認: (a) C shim の返却起源の検査(`null / fresh / owned_call`)が、static の callee の `return new` でも同じに働くか(callee 側の Return だけを見るので、変わらないと読んだが、C のコードは読んでいない)。(b) `ordinary_calls`(`physical_program.rs:561` ほか)の期待の列が、R1 の `MirCall`(`Callee::Global`、`Invoke` の包み無し)をどう受けるか。p5(§10)の `instruction-unsupported` は、**`Invoke` で包まれていない生の `MirInstruction::Call`** を `ordinary_i64` の function が持っている場合に出る(`physical_program.rs:663-696`)。R1 の `materialize_call` が作るのは `MirCall` で、`Invoke` の包みは別の writer(`LocalRouteWriterV1::DirectStatic`)が付けると読んだが、確かめていない。

### 未確認 2: 受けた Home を `a.get()` で読む判定(`ordinary_new_root_instance_call.rs:112-118,209`)が何を手がかりにしているか

- `issue_root_instance_call_dispositions`(`:91-`)は、terminal の `Call` 関係ごとに、(i) **`borrowed_terminal_arguments_v1(owner, return_site)` が `Some` なら `continue`**(`:112-119`。「lexical の発行が後で裏づける。root の行を 2 つ作らない」)、(ii) `selected_static_local_source_v1` が `Some` なら `continue`(`:122-129`)、(iii) それ以外は receiver の Home を探し、その初期化の **`new` の claim が無ければ `root-instance-call-new-missing`**(`:200-210`)。
- instance の `return a.get()` が通るのは、(i) の `borrowed_terminal_arguments_v1` が `Some` を返すから。その条件は `terminal.source_scalar()`(`ordinary_new_borrowed_formal_result.rs:338-344`)。`source_scalar` は、`issue_borrowed_i64_terminal_call`(`resolved_semantics/home_local_call_flow.rs:591-640`)が `source_scalar_call::demand_v1`(`home_local_source_scalar_call.rs:113-138`)で package から**貸してもらう** `BorrowedScalarResultSourceLoanV1`。貸す側は `source_scalar_result_arguments_v1`(`ordinary_new_borrowed_formal_result_scalar_loan.rs:326-376`)で、`targets`(lexical の対象の表。site が `a.get()` の行があるか)を探す。
- つまり**手がかりは「`a.get()` の site が lexical の対象の表に載っているか」**で、載る条件が §8.1 の (iv): `a` の初期化が `recv.m(..)` で `recv` が lexical な local か、`new`/`move`/`me.f` であること(class が決まること)。static の `Main.mk(5)` を初期化にする `a` は class が決まらず、表に載らない。載らなければ (i) は `None`、(iii) で `new` の claim が無く止まる。
- R1 や案 E の結果でも働くには、**`initializer_class` に static の呼び出しの枝を足す**こと(§8.5 の段階 3)。静的な callee の結果の class は `callable_result_classes`(`OrdinaryNewResultClassV1::Object`)に既にある(前の series の測定で、static の `mk` にも `Object(Leaf)` の claim が出ている)。

### 未確認 3: `source-not-i64` の `Err` を読んで拒否する消費者

- **作る所**: `ordinary_new_borrowed_formal_result_composition.rs:607`(proof の phase が `IngressVerified` でない、または `grounded` に owner が無い時に `*proof = Err(freeze("borrowed-result/source-not-i64"))`)と、`ordinary_new_borrowed_formal_result_pending.rs:572,581,635`(field/call の row が source を持たない時)。
- **読んで拒否する所**: `ScalarSourceValidationV1::demand`(`ordinary_new_borrowed_formal_result_scalar_loan.rs:72-80`: `results.get(&target.callee_owner())...as_ref().map_err(Clone::clone)?`)。これを `issue_source_scalar_result_loan_v1`(`:111-121`)が呼び、`source_scalar_result_arguments_v1`(`:326-376`、`:372`)から `ordinary_new_coseal_issue_call_arguments.rs:345-358` が `OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue }` に包む。`NYASH_BUILDER_DEBUG=1` で確かめた拒否の site は、`q11` が `a.get()`(`[Body(2), Initializer(0), Lhs]`)、`q14` が `return a.get()`(`[Body(2), Value]`)、`q04` が `local a = m.mk()`(`[Body(1), Initializer(0)]`)。
- **instance の道の狭さ(§3)を広げる時に変える所**: (1) `grounded_source_results_with_ingress_v1`(`..._scalar_loan.rs`)の grounding の条件、(2) `source_scalar_seed_target_v1`/`selected_source_scalar_target_v1`(`:249-`)の seed の条件。**どの callee の proof が grounded にならないか、が proof ごとに違い、この読みでは特定できていない**(未確認)。`composition.rs:607` は、通る入力でも実行される(§1 の計器)ので、拒否の原因はこの `Err` を**どの target が demand するか**で決まる。

## 10. main の文脈で道が変わる理由(p1〜p5)

入力は `/mnt/workdisk/claude-scratch/static-route-audit/in/`(`hakorune.main` で流した。結果は Opus の記録と同じ)。

| 入力 | 形 | 結果 | 理由(file:line) |
|---|---|---|---|
| `p1` | `f(v: i64): i64` を `local a = Main.f(5); return a` で受ける | `qualified-route-retired` | main は `plain_main_integer_result_v1`(`ordinary_new_coseal_issue_walk_triggers.rs:177`)が `has_static_source_local_v1` で偽になり、plain の道を外れる。R1 の `join_static_execution_v1` の `context`(`direct_call_static_execution.rs:124-141`)に main の owner が入らない(呼び出し関係が無く、`new` も `print` も無い)ので、`has_static_execution_for_owner_v1`(`direct_call_loan.rs:346`)も `has_routed_static_local_for_owner_v1`(`packet_entry.rs:451`)も偽。最後に `main_root.rs:161-177` が、arity 0 の main で `static_local_route` が偽かつ qualified の呼び出しがあると、**廃止された qualified の経路の代わりに名前つきで拒否**する |
| `p1u` | 同 `f(v: i64)`(宣言なし) | `selected-header-missing` | 同じく `context` に入らない。さらに宣言が無い `f(v)` は結果 `None` で、物理 header の行が作られない(`physical_header.rs`: header は `result: Some(abi)` の callee だけ)。`issue_app_main_qualified_receiver_catalog_relation`(`model.rs:260-372`)が、`static_local_route`/`typed_static_route` が偽の時に callee の header を探して無く、`model.rs:357-367` で拒否。main_root に着く前に落ちる |
| `p2` | `p1` に `local z = new Ctx()` | 通る(`6`) | `requires_new_context_v1`(`ordinary_new_ledger.rs:9-11`)が真になり、`context` に main が入る(`direct_call_static_execution.rs:135`)。R1 が `additions.push`(`:180`)で `Main.f` の呼び出しを加え、`typed_static_route` が真になる |
| `p3` | `return Main.f(5)` | `main-arguments/root-completion` | terminal が `Call`。`retain_main_arguments_v1`(`ordinary_new_main_arguments.rs:35-41`)が `root_completion` を要るが、**main の root completion が verified にならない**(`root_completion` が `Err`)ので `fault("root-completion")`。R1 の対象は `local_binding().is_some()` の呼び出し(`direct_call_static_execution.rs:149-151`)なので、`return` の直接の転送は R1 に入らない |
| `p4` | `local a = Main.f(5); print(a); return 0` | 通る | `print` が `flow.lifecycle_prefixes` に `Call` 以外の prefix を作る(`direct_call_static_execution.rs:136-141`)ので `context` に入る |
| `p5` | 別の static `g` の中の `local a = Main.f(v); return a + 1` を `print(Main.g(5))` で呼ぶ | `instruction-unsupported`(function=`Main.g/1` role=`ordinary_i64`) | `g` は呼び出し関係も `new` も無く `context` に入らないので、R1 の対象にならず、`Main.f(v)` が**生の `MirInstruction::Call`(`Callee::Global`)**のまま MIR に残る。物理側 `validate_instruction_with_context`(`physical_program.rs:600-696`)は、`ordinary` な呼び出しを `Invoke { Call }` の形(`ordinary_call_site_v1`)でだけ受けるので、生の `Call` を `instruction-unsupported`(`:696`)で拒否する |

**何も無い `main()` の通る pipeline**: `plain_main_integer_result_v1`(`ordinary_new_coseal_issue_walk_triggers.rs:177-195`)が真になる main(`is_main` かつ site が無く、map も owner loan も無く、`has_static_source_local_v1` が偽で、receiver も capture も無い)は、`app_main_integer`(`OwnershipRouteV1::decide` の `PlainIssuanceV1.app_main_integer`。`ordinary_new_coseal_issue_route.rs`)で、整数の結果の completion を root に置くだけの道を通る。static の source local を持つ main はこの道を外れ、walk に入る。walk に入っても、R1 の `context` に入らなければ、qualified の呼び出しを受ける道が無い。

**案 B と案 E での見込み**:

| 入力 | 案 B(R1 に結果の種類) | 案 E(結果の class を共有) |
|---|---|---|
| `p1` | 変わらない。`context` の条件が残るので、文脈なしの main の qualified 呼び出しは `qualified-route-retired` のまま | 同じ |
| `p1u` | 変わらない(header の行が無い) | 同じ |
| `p2`、`p4` | 通ったまま | 通ったまま |
| `p3` | 変わらない(root completion の問題) | 同じ |
| `p5` | 変わらない。`g` を `context` に入れる条件(§10 の p5)を変えない限り、生の `Call` が残る | 同じ |

つまり **p1〜p5 の拒否は、案 B にも案 E にも直らない**。直すには `context` の条件(`direct_call_static_execution.rs:124-141`)を、「qualified の static 呼び出しを持つ owner は常に入れる」に変える別の判断が要る。それは `pure scalar component retains its original scalar protocol`(同 file の先頭の comment)という設計の変更で、**ユーザーの決定が要る**。

## 11. 勧めの書き直し

§5 は案 B の限定版を勧めた。2 回目の読みで、次が変わった。

1. 案 B の限定版は **R1 の `context` の条件に縛られる**(§10)。object の結果を R1 に載せても、文脈なしの `main()` からの qualified の呼び出しは通らない。実アプリは `print` や `new` を持つので多くは `context` に入るが、「local の呼び出しだけを持つ小さな main」は通らない。
2. 未確認 1 は、**物理側には拒む検査が見つからない**(§9)。案 B の最大の不安は小さくなった。
3. 未確認 2 は、`a.get()` を通す変更の場所が分かった(`initializer_class` の枝、約 15 行)。案 B でも案 E でも、**この枝は必要**。
4. 案 E は、道を減らさないが、結果の class の判定 3 か所(instance の `lexical_local_route`、direct の `local_result_class_v1`、R1〜R5 の 39 か所)を表 1 つにし、**相談の 7「static と instance を共通にする」に合う**。

**勧め: 案 E と案 B の限定版を、順に 1 本の流れで進める。** 案 E の段階 1(結果の class の表。instance と direct を表に替える。MIR の同一)から始める。これは受理を変えず、判定を 1 つにするだけで、後のどの案でも要る。続いて案 B の限定版の R1 の gate の拡張(段階 2)と `initializer_class` の枝(段階 3)。R3 を消すのは最後(段階 4)。

理由:
- 案 E の段階 1 は**受理を変えない置き換え**で、証拠(329 + 1,459 本の MIR の同一)を取りやすく、危険が最も小さい。
- 残りは、案 B の限定版が必要とする変更と同じ物を、結果の class の表の上に載せる形になる。表が先にあれば、39 か所の `is_some()` を触らずに済む。
- 案 A(direct に載せる)は、通る形が 1 組(§4.3)なので、勧めない。案 C は形ごとの finish を足すので勧めない。

**勧めが変わる条件**:
- 結果の class の表を作る段階 1 で、instance の `lexical_local_route` と direct の `local_result_class_v1` の判定が**同じ表に収まらない**(条件が食い違う)と分かった時は、案 E を止めて案 B の限定版だけにする。
- C 側の返却起源の検査が、static の callee の `return new` を拒むと分かった時(§9 の未確認 1 の (a))は、公開の追加が要るので止めてユーザーに聞く。
- ユーザーが「文脈なしの `main()` の qualified 呼び出し(p1)も通す」と決めた時は、`context` の条件の変更が先に要り、その後で上の流れ。

**未確認の所**(§5 の未確認のうち残る物と、この節で増えた物):
1. C shim の返却起源の検査(`null / fresh / owned_call`)が static の callee の `return new` でも同じに働くか。
2. R1 の `materialize_call` の `MirCall` に `Invoke` の包みを付ける writer が、Handle/Text の結果でも同じ形を作るか。
3. `source-not-i64` が、`q04`/`q07`/`q11`/`q14` の各入力で、どの target の proof の `Err` を demand して出るか(各 proof の grounding の失敗の理由)。
4. `qualified_scalar_result_v1()` の 39 か所のうち、Handle を通してよい所の数。
5. R5(式の値の引数枠)を String/object の結果に広げられるか。
6. 別の static box が複数ある時の `Other.f`。

## 12. 記録: `ordinary_new_borrowed_static_packet_entry.rs`(844 行)

前から 800 行を超えている。責任で分けるなら、(a) **route の選択と co-seal**: `is_static_zero_packet_cohort_v1`(`:175`)、`select_static_zero_local_routes_v1`(`:188`)、`select_static_scalar_local_routes_v1`(`:212`)、`selected_static_local_source_v1`(`:235`)、`co_seal_static_local_routes_v1`(`:343`)、`has_routed_static_local_for_owner_v1`(`:451`)の約 380 行を `ordinary_new_borrowed_static_route_selection.rs` へ。残り(入力の source と scalar の packet、provider の packet、`verify_original_static_*`、`borrowed_static_packet_actuals*`、`with_qualified_static_packet_actuals_v1`)の約 460 行は今の file に残る。直さない(記録だけ)。

## 13. 入れた計器と時間

この節では計器を入れていない(`NYASH_BUILDER_DEBUG=1` の既存の出力だけ)。`git status` は文書だけの変更。読みきれなかった所は §11 の未確認に列挙した。
