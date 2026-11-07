# 所有の object の結果(宣言の `: P` を結果の契約で受ける規則)

Status: 規則の文書です。authority ではありません。branch `owned-object`(main `691bd3da89` から)。根拠: `result-contract-consult-2026-11-06.md` の回答 4〜9、String の規則 `mirbuilder-result-contract-string-2026-11-07.md`(S1〜S9)、Bool の規則 R9。**この文書は、段階 2 の実装を試しながら書いた(指示は実装より前)。規則の文が実装を写す形になるよう、実装の後で全文を見直し、食い違う行は直した。** 行番号は `691bd3da89` のものを基準にし、実装後の変更は「実装の対応」に書く。

## 1. 範囲

- 今回つなぐ結果の型は、所有の object(`: P`。`P` は利用者の box の名前)。nullable(`P?`)は範囲外で、今の `ParameterContract UnsupportedResultType` のまま。
- 範囲は instance の呼び出しと `me` の呼び出し。**static の呼び出しは、object を返す static を受ける入口が基準に無い**ので、この series では止める(§5 の証拠)。
- 公開 MIR、opcode、ABI、descriptor、C shim は足さない。公開の表現は既存の `Handle`(`InvokeCallResultKind::Handle`)を使う。

## 2. 規則(1 行 1 文。実装はこの写し)

| # | 場面 | 規則 |
|---|---|---|
| O1 | `f(): P` | 宣言の `P` が利用者の box の名前かどうかは、名前解決の答え(`ParserOrdinaryBoxSourceCoverageV1::row_for`)で決め、綴りで決めない。宣言の行の `declared_result_box` がその答えで、1 か所(batch の発行)で作る。 |
| O1a | 宣言の表 | 宣言の綴りを読む表は `DeclaredResultKindV1` の 1 つで、所有の object の行(`OwnedObject`)を足す。`classify_resolved(name, declared_box)` は、表に無い名前が名前解決で box と答えられた時だけ `OwnedObject` を返す。 |
| O2 | 宣言と本体の一致 | 宣言の有る object の結果と、宣言の無い同じ本体の結果は、同じ契約の行を通る(Bool の R9)。宣言が無いか、本体が返す box の名前と同じ名前を宣言する(null を返さない)ことを、`DeclaredFunctionResultContractV1::agrees_with_object_result` の 1 つの投影が答える。`Unannotated` だけを受ける object の判定は、この投影に置き換える。 |
| O3 | 本体が返す class | 宣言が box を名指す時、その box が結果のクラス claim を決める。本体の全出口が `new P(..)`(null なし)でなければ、claim は発行されない(拒否)。宣言が無い時は、今のとおり本体の出口から claim を決める。 |
| O4 | 宣言と違う結果 | `: P` で `return 1`、`return null`、別の box の `return new Other(..)`、`: i64` で `return new P(..)` は、名前つきで拒否する(`result-contract-mismatch`、`handle-result-mismatch`、`artifact-source-unavailable`、`result-cohort-result-contract` など、各層の今の名前)。 |
| O5 | 返せる値 | `return new P(..)` と、`new P` を持つ local の返却(`local a = new P(..); return a`)を返す。呼び出し結果の転送(`return m.mk()`)は、基準で宣言の無い形が拒否される(`published-lifecycle/admission-function-not-birth`)ので、同じ名前のまま残す。 |
| O6 | 範囲外の返却 | `take` の formal の返却、field の返却 `return me.f`、借用の `P from me` は、今の拒否の名前のまま残す。公開契約(返却起源 `null / fresh / owned_call`)の追加はしない。 |
| O7 | 責任が移る時点 | 返す前の cleanup が Fault なら結果を公開せず callee が片付ける。`Return` の成功と `Call` の Normal で、結果の責任は caller へ移る。caller の後の評価が Fault なら、caller が片付ける(String の S5 と同じ)。 |
| O8 | 受け取った側の所有 | 受け取った側の所有は、歩 B の遷移の操作と歩 A の計画の型を使う。新しい遷移の操作も、計画を手で作る所も足さない。 |
| O9 | 診断 | 範囲外の形は、今の拒否の名前のまま拒否する。 |

## 3. 今 object の結果を決めている所(`691bd3da89`)

調べ方: `Unannotated`、`ExactNominalBox`、`OrdinaryNewResultClassV1`、`Handle`、`return new` を grep して読んだ。**読んだ範囲に限る**。

| # | file:line | 何を決めるか | 分類 |
|---|---|---|---|
| T1 | `exact_trivial_scalar_abi.rs`(`DeclaredResultKindV1::classify`) | 宣言の綴りの表(scalar、`String`) | 宣言の照合。O1a で object の行を足す |
| T2 | `physical_header.rs:196-215` `validate_result`、`:226` `validate_declared_result` | 宣言を表で読み、読めなければ `UnsupportedResultAnnotation` | 宣言の照合 |
| T3 | `callable_result_representation/function_proof.rs`(`prove_function`) | 宣言を読み、表に無い名前は `DeclaredNonI64Result` で早期に拒否 | 宣言の照合 |
| T4 | `callable_result_representation/expression_proof.rs:371-375` | `ASTNode::New` が `ExactNominalBox(class)` の事実を作る | **`return new` で callee の型を決める処理(callable の結果の証明側)** |
| T5 | `ordinary_new_result_class_claim/evaluate.rs:196-203` `evaluate_row` | 全出口が `new C` なら claim `Object(C)` を、本体だけから決める | **`return new` で callee の型を決める処理(claim 側)** |
| T6 | `ordinary_new_result_class_claim.rs:573` `observe_function` | 出口の draft(`New`、`Null`、`ForwardCall`…)を作る | T5 の入力 |
| T7 | `ordinary_new_return_result.rs:150-160` | instance の object callee の源が `Unannotated` でなければ `cohort-result-contract` | `Unannotated` 限定の判定 |
| T8 | `ordinary_new_local_commit/finalized_callable_participants.rs:236-246` | object の結果が `Unannotated` でなければ `object-result-agreement` | `Unannotated` 限定の判定 |
| T9 | `ordinary_new_borrowed_formal_result_original.rs:294`、`ordinary_new_mixed_result.rs:92`、`ordinary_new_callable_result_source.rs:56` | Unit の源、nullable の源の `Unannotated` | 範囲外(Unit、nullable)。残す |
| T10 | `callable_semantic_batch/issuer.rs:380-390`、`model.rs`(`declared_result_box`) | 宣言の `TypeReferenceV1` から名前解決した box の名前 | O1。足した |
| T11 | `normal_callable_semantic_package/model.rs:340-372` | main-import-view が qualified static call の callee に物理 header の行を要求し、無ければ `selected-header-missing` | **static の object を受ける入口が無い所(§5)** |
| T12 | `ordinary_new_coseal_issue_local_route.rs`、`resolved_semantics/home_local_call_route.rs:15-40` | local initializer の呼び出しの route。instance は `LexicalHandle`/`ReceiverHandle`、static は `Static(StaticScalarCallClaimV1)`(scalar と Text の kind だけ) | 呼ぶ側の受け取り |
| T13 | `instruction/invoke.rs:259` `InvokeCallResultKind::Handle` | 物理の結果の種類 | 既存の `Handle`。足さない |

## 4. 完了の条件の各形が決まる行

| 形 | 行 |
|---|---|
| instance の `mk(): Leaf { return new Leaf(me.k) }`、`local a = m.mk()`、`a.get()` | O1、O1a、O2、O3、O7 |
| 宣言の無い同じ形 | O2(基準で通る形のまま) |
| `me.mk()` | O2(`ReceiverHandle`) |
| `local a = new Leaf(v); return a` | O5 |
| `return m.mk()` の転送 | O5(基準で拒否の形は拒否のまま) |
| 受け取った後に Fault しうる形 | O7、O8 |
| `: Leaf` で `return 1`、`return new Other()`、`return null`、`: i64` で `return new Leaf(..)` | O4 |
| static の `mk(v: i64): Leaf`、宣言の無い static の同じ形 | **止める(§5)** |

## 5. static の object の結果(止めた物の証拠)

- 基準(main `691bd3da89`)で、宣言の無い `static box Main { mk(v: i64) { return new Leaf(v) }  main() { local a = Main.mk(5)  return a.get() } }` は `[mir/main-import-view/selected-header-missing]` で拒否される。引数の無い `mk()` の形も同じ。
- 原因は T11: qualified static call の callee に物理 header の行を要り、header の行は `result` が scalar の callee にしか作られない(`physical_header.rs:91-110`)。static の `local x = Main.m()` は T12 の `Static(StaticScalarCallClaimV1)`(scalar と Text の kind)が受け、Text は引数の無い形(`is_zeroarg_text_v1`)に限られる。**Text の `t(x: i64): String` も基準で同じ名前で拒否される**(String の series の監査)。
- したがって「static の object の結果」は、結果の契約の問題ではなく、**引数つきの static call が scalar 以外の結果を受ける入口の欠如**で、Text の zero-arg 専用の gate 約 10 か所(`qualified_static_incoming_source.rs`、`static_input_validation.rs`、`ordinary_new_borrowed_static_*` ほか)を、Handle の結果に広げる別の series が要る。形ごとの file を増やさない決まりに反するので、この series では止める。
