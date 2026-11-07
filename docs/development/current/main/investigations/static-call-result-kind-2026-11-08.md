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
