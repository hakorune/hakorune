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
