# static の呼び出しの結果の種類を 1 つの表で決める(規則)

Status: 実装より前に書く規則の文書です。authority ではありません。branch `static-result-kind`(main `03a36ac2d9` から)。根拠: `static-call-result-routes-survey-2026-11-07.md` の §8(案 E)、§9、§11、穴の規則 `static-call-context-rule-2026-11-07.md`(C1〜C8)、object の規則(O1〜O9)、String の規則(S1〜S9)、Bool の規則 R12 と R15。実装はこの文書の文を写します。文に無い分岐が要る時は、先に行を足して理由を書きます。

## 1. 規則(1 行 1 文。実装はこの写し)

| # | 規則 |
|---|---|
| K1 | 呼び出しの結果の種類は、callee の契約の行(`VerifiedCallableResultDispositionV1` の `ExactI64`、`ExactBool`、`ExactString`、`ExactNominalBox`)から、1 つの表で決める。 |
| K2 | 表は、結果の種類(`LocalCallResultClassV1`)から、物理の結果(`InvokeCallResultKind`)と、呼び方(lexical、direct)ごとの local の route(`LocalCallRouteV1`)を返す。 |
| K3 | instance の lexical の route と bare の direct の route は、K2 の同じ表を読む。型つきの静的な実行(R1)も、同じ表の物理の結果を読む。 |
| K4 | 表の行は `I64`、`Bool`、`Text`(`OwnedString`)、`Handle`、`Nullable`(`NullableHandle`)、`Map`、`Unit` を持つ。**`Nullable` は表に行を書くが、この series では static の呼び出しで受けない**(nullable の series)。 |
| K5 | scalar の判定(`qualified_scalar_result_v1()` の 39 か所、`scalar_result_class()`、`required_i64_arguments()`、`scalar_invoke_result()`)は変えない。Text と Handle は、別の accessor で出す。こうして、`Main.mk(5) + 1` が scalar として通らない。 |
| K6 | 使い先は R15 の表に従う。算術と i64 の lane は I64 だけ。`local` は種類を問わない。method の receiver は Handle。 |
| K7 | 型つきの静的な実行が運べる呼び出しの条件(C4)は、formal の条件(全部 i64 か無し)を変えず、結果の条件を「表の I64、Bool、Text、Handle」にする。 |
| K8 | 1 つの site は、ちょうど 1 つの道に入る。今 Text の引数なしの packet(R3)が受ける site は R3 のままで、MIR を変えない(R3 を消すのは次の series)。 |
| K9 | 受け取った Text と Handle の責任は String の S5 と object の O7 のとおり。受け取った側の所有は、歩 B の遷移の操作と歩 A の計画の型を使い、新しい遷移や計画を手で作る所を足さない。 |
| K10 | static の呼び出しで初期化した local の class は、instance の lexical の呼び出しと同じに、callee の結果の class(`callable_result_classes` の `Object(C)`)で決める(`initializer_class`)。 |
| K11 | 段 1(表を 1 つにして instance と direct を替える)は受理を変えない。329 入力と 1,459 本の `--dump-mir` が基準と全部同じである。 |
| K13 | 受け取った Handle は、自分の commit 行(`CallReceived`)が発行した binding を持つ。static の実行は、call-prefix の binding group を I64、Bool の結果にだけ課し、Handle の site は routed の表に入れない。(実装中に足した行。group を足すと、2 回受ける形で fault-unwind の release を 2 回数えて `handle-release-missing` になった。) |
| K14 | 他の関数から呼ぶ static の公開の要求(`project_static_exact_i64_requirement_v1`)は、箱の Handle の結果も、constructor の引数の i64 の要求を持つ行として読む。(実装中に足した行。受ける側が main でない形 `g(v) { local a = Main.mk(v) ... }` で `TargetOnlyDispositionMustBeUnavailable` になった。) |
| K15 | (書き直し)案 B を選ぶ。static の String の結果は、R1 ではなく、static の入力の finish(`StaticInputFinishV1`)が運び、受けた String は instance、R4 と同じ Text residence(initializer の expression frame)が持つ。R1 に Text の install は足さない。(旧 K15「R1 が Text の観測を issuer から得る」は消した。) |
| K16 | (書き直し)header の行を課すのは、R1 の経路だけ。finish の経路(K15)で受ける呼び出しは、main の import view が `static_local_route` で外す(`model.rs:286-300`)ので、header の行を要らない。(旧 K16「header の行を契約の行から作る」は消した。) |
| K17 | (書き直し)受けた String の所有と release は S5 のとおり、Text residence が持つ。R1 の call-prefix の group は課さない(group を持つのは scalar と Handle の R1 の site だけ)。 |
| K18 | (書き直し)引数なしの呼び出しは、引数のある呼び出しと同じ finish の「実引数 0 個」の場合として扱う。引数なし専用の判定(`is_zeroarg_text_v1`、`Zero` の evidence、zero packet の枝)は、この規則の実装の後に残さない。1 つの site は、ちょうど 1 つの道に入る。 |
| K19 | 使い先は R15 の表のまま。Text は算術に入らず、`s + 1` は拒否のまま。 |
| K20 | 引数が i64 でない static(`t(x: String)`)は R4 のまま。R1 は formal が全部 i64 か無しの呼び出しだけ運ぶ(K7)。 |
| K21 | 全部の formal が i64 の static が String を返す呼び出しは、static の入力の finish の qualified の evidence(`StaticInputEvidenceV1::Qualified`)で受ける。borrowed の formal が 1 つも無い callee の証拠 `uses` は無し(`None`)で、`domains` は空である。i64 の実引数の証拠は、今ある `snapshot.ordered_arguments`(claim の `required_i64_arguments` から作る)を使い、新しい証拠を手で組まない。 |
| K22 | 宣言した `x: String` の formal に literal や String の local を渡す形は、宣言の無い `f(x)` と同じ証拠で受ける(段 3)。 |
| K12 | 段 2 と段 3 が規則の範囲で通らない時は、止まった門の file:line と、その門が何を要るかを書いて止める。公開の追加(MIR、opcode、ABI、descriptor、C shim)が要る時は、その部分を止める。 |

## 2. 結果の種類を決める所の一覧(`03a36ac2d9`)

調べ方: `scalar_invoke_result`、`LocalCallResultClassV1`、`DirectLocalResultV1`、`lexical_local_route`、`LocalCallRouteV1`、`InvokeCallResultKind` を grep して読んだ。**読んだ範囲に限る**。

| # | file:line | 何を決めるか | 種別 |
|---|---|---|---|
| D1 | `ordinary_new_coseal_issue_lexical.rs:162-191` `lexical_local_route` | instance の lexical の route: claim が `NullableObject` なら `LexicalNullable`、全 formal が exact i64 かつ `construction_result_callee` なら `LexicalHandle`、claim が無く正常な i64 の結果なら `LexicalI64` | 結果の種類 → route |
| D2 | `direct_call_lifecycle.rs:455-472` `local_result_class_v1` | direct の結果の class: 宣言の結果が `I64` なら `I64`、`Bool` なら `None`、`None` なら formal と body から `Map` か `Handle` | callee → 結果の種類 |
| D3 | `ordinary_new_coseal_issue_local_route.rs:42-56` | direct の class を route(`DirectI64`/`DirectMap`/`DirectHandle`)に写す | 結果の種類 → route |
| D4 | `direct_call_lifecycle.rs:608-614` | direct の lifecycle の seal で、期待する結果の class を宣言の結果と body から決める(`I64`、`Bool` は拒否、`Handle`、`Map`) | callee → 結果の種類 |
| D5 | `home_local_source_scalar_call.rs:33-43` `scalar_invoke_result` | I64、Bool だけを物理の結果に写す(R15 の表) | 結果の種類 → 物理 |
| D6 | `home_local_call_route.rs:28-40` `LocalCallRouteV1::result` | route を結果の class に戻す。`Static(claim)` は claim の結果 | route → 結果の種類 |
| D7 | `home_local_call_route.rs:105-125` `install_routed_local_call` | 結果の class ごとに local を install する(Text の行は無く `unreachable!`) | install の表 |
| D8 | `qualified_static_call_claim.rs:154-171` | claim の索引は callee の契約の行の `scalar_result_class()` と `required_i64_arguments()` を要る。`ExactString`、`ExactNominalBox` は `None` なので索引に入らない | callee → claim |
| D9 | `callable_result_representation/disposition.rs`(`scalar_result_class`、`required_i64_arguments`)と `expression_proof.rs:371-375` | `ExactNominalBox` は引数の要求を持たず(`New` の引数の proof を捨てる) | callee の契約の行 |
| D10 | `direct_call_static_execution.rs:45-66`(`validate`、`materialize_call`)、`:87-104`(`seal_local`)、`:151`(`scalar_invoke_result()`) | 型つきの静的な実行(R1)が運ぶ結果を I64、Bool に限る。formal は全部 i64 | 道の gate |
| D11 | `direct_call_lifecycle.rs:329-343` `local_call_gate` | lifecycle の local の呼び出しは `scalar_invoke_result()` が `Some` | 道の gate |
| D12 | `ordinary_new_local_commit/root_call_entry/direct_scalar_call.rs:41-60,236` | R1 の物理の発行(`DirectScalarCallProjectionV1`)が `scalar_invoke_result()` を要る。`lexical_i64_call_source` を探す | 物理の発行 |
| D13 | `ordinary_new_lexical_instance_call_provenance.rs:391-538` `initializer_class` | `local a = <初期化>` の class: `new C`、`move`、`me.f`、lexical な local の method の呼び出し。static の呼び出しは `variable_ref(object)` が local でなく `None`(`:508-533`) | 初期化 → class |
| D14 | `home_local_source_scalar_call.rs:553-559`、`ordinary_new_borrowed_call_result_source.rs:277,422`、`result_contract.rs:277` | Text の静的な local は `LocalCallResultSourceV1::Ordinary(loan)` の別の道(R3、R4) | Text の道 |

数: 前の結果の種類を決める所は、**route/class を作る 7(D1〜D4、D6〜D8)、物理に写す 3(D5、D10〜D12)、class を決める 1(D13)**。実装後の数は報告に書く。

## 3. 止めた門(段 2 の Text)

引数つきの static が返す Text(`t(x: i64): String`、k3n、k3un)は、段 2 で止めた。型つきの静的な実行(R1)は、claim の索引に `ExactString` の行(arity が 1 以上)を足すと次の門に届く。

- 門 1: `resolved_semantics/home_local_call_flow.rs:158`(`LocalCallObservationV1::issue`)は `LocalCallResultClassV1::Text` を `assert!` で拒否し、Text の観測は「専用の source loan の issuer」(`LocalCallResultSourceV1::Ordinary(loan)`、D14)だけが作る。R1 にはその issuer を呼ぶ入口が無く、足すと panic になる(基準は `selected-header-missing` の名前つきの拒否)。
- 門 2: `normal_callable_semantic_package/physical_header.rs:91-110` は、結果が scalar の callee にしか header の行を作らない。`model.rs:357-367` の `selected-header-missing` がそこで出る。
- 必要な物: R4(Text の引数の式の入力)の loan と、受け取った Text の所有(S5)を R1 に通す入口。これは Text の series が要る。この series では K12 に従って足さず、claim の索引に Text の行を入れていない。基準と同じ拒否の名前のまま。

## Opus の確認(2026-11-08)

Opus が branch `static-result-kind`(`d54780a8aa`)を監査し、main へ fast-forward した。

- binary: `final.commit` は `d54780a8aa`、`find src -newer hakorune.final` は空、`target/quick/hakorune` と `cmp` で一致。
- 規則の順: K1〜K12 の初版(`b4f4a579dc`)は実装より前。K13、K14 は実装の後に足された(報告にも書かれている)。
- 全 lib: 基準の赤 231 の名前に対し、新しい赤は `raw_invocation_port_preserves_async_qmark_check_collection_and_print_children` の 1 本。この名前は owned-object、owned-string、s3b-route の赤の一覧にもあり、順序で出る物。最後の test binary で単独で 2 回流し、2 回とも通った。
- MIR: 1,459 本の違い 5 本と、329 入力の違い 2 本を数え直し、報告の分類と一致した。(c)(前も今も通って MIR が変わった)は 0。段 1 の binary は、329 入力と 1,459 本の MIR が基準と全部同じ。329 入力の 2 本は、使わない box を置いた入力で、物理の検査(`check-uncovered-function`)か `handle-result-terminal-mismatch` で拒否される。
- 384 組: 前に通る 121 組は、結果の行が全部同じ。新しく通る 21 組のうち 2 組の出力を source の計算と比べ、合っていた。
- Opus が書いた 30 本(指示書に無い形): 間違った出力は 0。
  - 通るようになった形: callee の `if` の 2 つの `return`(`50`)、`a.set(9)` の後の `a.get()`(`9`)、`a.get() + a.get()`(`10`)、2 つの class を受ける(`5`)、受けて使わない 2 本目(`5`)、static で受けた object と `new` の object(`12`)、2 引数の `mk(x, y): Pair`(`7`)。
  - 受けて使わない 2 本目の形の MIR は、2 つの Handle を通常の出口と各 fault の出口で 1 回ずつ release する。
- 拒否のまま(全部名前つき):
  - 引数が式の形 `Main.mk(n + 1)`、`Main.mk(Main.f(2))`、`Maker.mk(v + 1)`: `borrowed-static/source-route-identity`。i64 の結果の `Main.f(n + 1)` は通るので、Handle の結果だけが狭い。
  - `local b = a`、`if` の中で受ける形: `root-instance-call-new-missing`、`root-call-binding-contracted`。instance の同じ形も同じ名前で拒否されるので、instance と static の振る舞いがそろった。
  - `a = Main.mk(6)`、`print(a)`、`a.value`: `static-execution/completion-not-ready`。
  - `return Main.mk(5).get()`、`print(Main.mk(5))`、引数に型の無い `mk(v)`: `qualified-route-retired`。
  - callee の `local x = new Leaf(v); return x`: `handle-result-terminal-mismatch`。`g(v): Leaf` が受けた object を返す形: `static-execution/result-source`。
  - `loop` の中で受ける形: `callable-loop/route-not-front-selected`(S4)。
- 設計で見ておく事:
  - K14 で、box の結果の契約の行が constructor の引数の i64 の要求を持つ。引数が i64 と示せない呼び出しでは、box の結果が `Unavailable` になる(前は引数に依らず box)。閉じる向きの変更で、corpora に退行は無い。ただし box の class が引数の証明に依る形になったので、nullable の series で class と引数の要求を分けるかを見る。
  - K13 で、static の Handle の site は call-prefix の group に入らない。instance の lexical の Handle は `record_lifecycle_local_call_site` を呼ぶので、2 つの道で記録の仕方が違う。段 4 で R1 に寄せる時に、そろえられるかを見る。
- 残り: 引数つきの static が返す Text(§3 の門 1、門 2)、Handle の結果で引数が式の形、段 4(R3 を消す)。

## 4. R3 の所の一覧(`5d90bec5af`、段 4 の前)

調べ方: `is_zeroarg_text_v1`、`is_zero_input_source_v1`、`select_static_zero_local_routes_v1`、`is_static_zero_packet_cohort_v1`、`selected_static_local_source_v1` を grep した。読んだ範囲に限る。

| file:line | 何か |
|---|---|
| `qualified_static_incoming_source.rs:102-108` | `is_zero_input_source_v1`、`is_zeroarg_text_v1`(arity 0、引数なし、結果 `ExactString`) |
| `ordinary_new_borrowed_static_packet_entry.rs:175-186` | `is_static_zero_packet_cohort_v1`(12 行) |
| 同 `:188-208` | `select_static_zero_local_routes_v1`(21 行)。zero Text の site を `borrowed_static_source_sites` に入れる |
| 同 `:235-` | `selected_static_local_source_v1`。route 表に入った site の source を返す |
| 同 `:302,356,376,409,422,473,530,532,603,734` | zero packet の枝(10 か所) |
| `static_input_validation.rs:92,108,138` | zero input の検証 |
| `ordinary_new_borrowed_static_input_finish.rs:65,82,124,409,461` | zero Text の finish |
| 使い手 | `issuer_object_input_finish.rs:85`、`ordinary_new_root_instance_call.rs:123`、`ordinary_new_static_loop_packet_source.rs:453`、`direct_call_disposition_port.rs:226,296`、`raw_ordinary_new_claim/terminal_call.rs:81`、`source_loop_frames.rs:285`、`lexical_terminal.rs:131,188` |

## 5. 段 4 の段 1 で止めた門(Text を R1 が運ぶ)

K15〜K20 の範囲で、`t(x: i64): String` を受ける形(k3n、k3un、t1、t2)を R1 に載せる事を 3 回試した。source は戻して、実装の commit は無い。分かった門:

1. **R1 の local-call route に Text は載らない**(`resolved_semantics/home_local_call_route.rs:141`、`install_routed_local_call` の `unreachable!`)。claim の索引に `ExactString`(arity 1 以上)の行を足し、`home_local_call_arguments.rs`(`issue_static_scalar_local_call`)で `SourceResultArguments` の loan から観測を作ると(`home_local_call_flow.rs:158` の `assert!` は越える)ここで panic する。Text の local は route ではなく、`home_new_prefix_local_statement.rs:~497` の `initializer_expression::observe`(Normal Text residence、`TextResidencePacketV1`)が受ける。R1 の route に Text の install の arm を足すには、その residence の所有(S5)を R1 の emit(`terminal_call.rs` の `emit_local`、Text は既定の `record_root_local_call_bindings` の枝)に通す入口が要る。
2. **zero-input の protocol は arity 0 と実引数なしを課す**(`ordinary_new_borrowed_static_input_finish.rs:402-406`、`:455-468`、`:497`)。`is_zeroarg_text_v1`(`qualified_static_incoming_source.rs:105`)を「Text で formal が全部 i64」に替えると、header の門(`selected-header-missing`)は越えるが、ここで `static-zero/contract-identity` になる。引数つきの Text には、zero ではない入力の証拠(`StaticInputEvidenceV1::Qualified` の系)を Text の結果と組む新しい finish が要る。
3. **header の行**(`physical_header.rs:91-110`、`model.rs:357-367`)は、main の routed な local があれば越える(`model.rs:286-300`)。それだけでは足りない。

必要な物: R1 の Text 用の install と emit(S5 の residence)、引数つきの Text の入力の finish。`ordinary_new_borrowed_static_*` の zero packet(R3)は、この 2 つが R1 に入るまで消せない。公開の追加は要らない見込みだが、未確認。

読むだけの報告: Handle の結果で引数が式の形(`Main.mk(n + 1)`、h1)は `static_argument_projection.rs:204-206`(`project_pending_static_source_arguments_for_route_v1`)で `borrowed-static/source-route-identity` になる。この関数は qualified の route に `qualified_scalar_result_v1().is_some()` を要るため、Handle の結果(scalar でない)の行は通らない。`qualified_carried_result_v1()`(K5 を守る別の accessor)に替えれば先へ進むが、その先の gate(式の引数の frame が Handle の結果を運ぶか)は未確認。直していない。

## 6. 段 0: 設計の比較(`40267ba5d9`、`static-text-entry`)

門 1〜3(§5)をどう越えるかで比べた。

| | 案 A: R1 が Text を運ぶ | 案 B: static の入力の finish が運ぶ(選んだ) |
|---|---|---|
| 門 1(Text の install) | R1 の `install_routed_local_call`(`home_local_call_route.rs:141`)と `emit_local`(`terminal_call.rs`)に Text residence の入口を足す。Text の local は route でなく initializer の expression frame が持つ(`home_new_prefix_local_statement.rs:~497`)ので、第 2 の所有の入口になる | 足さない。受けた String は今の Text residence のまま(instance、R3、R4 と同じ) |
| 門 2(`static-zero` の arity 0 と実引数なし。`ordinary_new_borrowed_static_input_finish.rs:402-406`) | 迂回する(R1 は finish を通らない) | evidence の `Qualified` を、borrowed の formal が無い callee にも使う。`uses` を `Option` にして、i64 の formal だけの callee は `None`、`domains` は空 |
| 門 3(header、`physical_header.rs:91-110`) | header の行を Text の結果にも作る(公開の header の ABI に近い) | 要らない。finish の経路は main の import view が外す(`model.rs:286-300`) |
| R3 を消せるか | 消せない(R3 は別の経路のまま) | 消せる。`Zero` を「実引数 0 個」の `Qualified` に畳める |
| nullable(`{Text, null}`) | R1 に Dynamic の入口がまた要る | 同じ finish と Text residence の入口を使える(`completed_instance_nullable_text_source_v1` と同じ residence) |

選ぶ: 案 B。理由は、受けた String の所有を 1 つ(Text residence)に保ち、足す行が `uses` の `Option` 化と 1 つの判定(`is_i64_input_text_v1`)で済み、R3 の判定を消せるから。

## 7. 段 3 で止めた門(宣言した `x: String` の formal)

K22 を試した。宣言した `x: String` の formal は、`CallableParameterContractKindV1::ExactText` で、宣言の無い `x`(`OpaqueHandle`)とは別の物理の lane を持つ。

試した事(戻した):

1. `callable_parameter_contract/model.rs` に `takes_borrowed_actual_v1`(`OpaqueHandle`、`DeclaredObject`、`ExactText`)を足し、`ordinary_new_borrowed_static_source_actuals.rs` と `ordinary_new_argument_expression_lifecycle.rs` の `is_ordinary_borrowed_handle()` の 3 か所をこれに替えた。`source-actual-unavailable` は越え、`source-projection-identity` も越えた。
2. `static_input_selection.rs` の `text_source` が、`ExactText` を宣言から Text の入力として読むようにした。`qualified-route-retired` は越えた。
3. 次の門: `static_input_validation.rs:37-40`(`verify_static_input_header_v1`)の `contract-identity` が、formal を `OpaqueHandle`、i64、`DeclaredObject` に限る。これを広げても、次の `:49-56` の lane の数(`physical_formal_lane_count == arity`)が合わない。`ExactText` は `physical_signature.rs:288-298` で 2 つの lane(`ExactTextSlot` と `ExactTextGeneration`)を持つ。

必要な物: 呼び出し側が、pinned Text residence の slot と generation の 2 つの値を作って渡す emit(`compiler/pinned_text_backend_frame.rs`、`pinned_text_residence_backend_carrier.rs`、`hako.pinned_text_backend_frame@2` の backend の frame 契約と、`runtime/text_formal_residence` の ABI)。instance の `b.t("abc")`(`x: String` 宣言)も、基準で同じ系の `pinned-text-backend-frame-mismatch` で拒否されるので、宣言した String の formal の呼び出し側は、どの経路にもまだ無い。これは「同じ証拠で受ける」問題ではなく、公開の backend の frame 契約に載る呼び出し側の emit を新しく作る事になるので、この series では止める。宣言の無い `f(x)` は、変わらず通る。
