# 所有の String の結果(宣言の `: String` を結果の契約で受ける規則)

Status: 実装より前に書く規則の文書です。authority ではありません。branch `owned-string`(main `b4078a5106` から)。根拠: `result-contract-consult-2026-11-06.md` の回答 4〜9、Bool の規則 `mirbuilder-result-contract-bool-2026-11-07.md`(R1〜R18)。実装はこの文書の文を写します。文に無い分岐が要る時は、先に行を足して理由を書きます。

## 1. 範囲

- 今回つなぐ結果の型は、Unit、I64、Bool(着地済み)に、所有の String を足す。
- 範囲は static の呼び出し。instance と `me` の呼び出しは、同じ契約で通るなら入れ、別の道が要るなら止めて理由を書く。
- 範囲外で、今の拒否の名前のまま残す: `String?`(`has_nullable_text_result_v1` は `Unannotated` のまま)、借用の `String from me`、Text の formal から作る本体(`x + "d"`。宣言の有無によらず `borrowed-static/source-actual-unavailable`)、object と nullable の結果。
- 公開 MIR、opcode、ABI、descriptor、C shim は足さない。公開の表現は既存の `OwnedString` と `ordinary_text` を使う。足す必要が見つかった部分は止めて報告する。

## 2. 規則(1 行 1 文。実装はこの写し)

| # | 場面 | 規則 |
|---|---|---|
| S1 | `f(): String` | 署名の `String` は結果の契約の行の種類(所有の Text)になる。宣言の綴りを読む表は、Bool で作った `ExactTrivialScalarAbiV1::classify_result` の 1 つで、`String` の行をその表に足す。String 専用の照合の関数は足さない。 |
| S2 | 宣言の有無 | 宣言の有る String の結果と、宣言の無い同じ本体の結果は、同じ契約の行を通る(Bool の R9)。今の `Unannotated` だけを受ける Text の判定は、「宣言が無いか、`String`」を答える 1 つの投影に置き換えて消す。 |
| S3 | `return e`(宣言 `String`) | 本体の Normal の出口は全部 Text に照らす。`e` が Text でなければ、`DeclaredResultDisagrees` で拒否する(`: String` で `return 1`)。 |
| S4 | `return "a"`(宣言 `i64` または `Bool`) | 今の `DeclaredResultDisagrees` のまま拒否する。 |
| S5 | 責任が移る時点 | 返す前の cleanup が Fault なら、結果を公開せず callee が片付ける。`Return` の成功と `Call` の Normal で、結果の責任は caller へ移る。caller の後の評価が Fault なら、caller が片付ける。 |
| S6 | 名前の無い一時値 | `print(Main.t())` のように結果が名前に束縛されない時も、結果の責任を caller が持ち、使い終えたら End する。 |
| S7 | 受け取った側の所有 | 受け取った側の所有は、歩 B の遷移の操作(`acquire_*`)と歩 A の計画の型(`FaultCleanupPlanV1`)を使う。新しい遷移の操作と、計画を手で作る所を足さない。 |
| S8 | 引数の後片付け | `text_argument_cleanup` は結果の型と別である。結果が `OwnedString` の時に限る制限を、新しい契約へ写さない。 |
| S9 | 診断 | 範囲外の形は、今の拒否の名前のまま拒否する。名前を変えない。 |

## 3. 今 String の結果を決めている所(`b4078a5106`)

調べ方: `Unannotated`、`has_complete_text_result_v1`、`ExactString`、`classify_result`、`OwnedString`、`ordinary_text` を grep し、読んだ。**読んだ範囲に限る**(未確認は「未確認」と書く)。

| # | file:line | 何を決めるか | 分類 |
|---|---|---|---|
| T1 | `exact_trivial_scalar_abi.rs:26` `classify_result` | 宣言の綴りから結果の種類(I64、Bool)を読む表 | 宣言の照合。S1 で String の行を足す |
| T2 | `physical_header.rs:189-205` `validate_result` | 宣言を `classify_result` で読み、読めなければ `UnsupportedResultAnnotation`。`Unannotated` と `Void` は `None` | 宣言の照合 |
| T3 | `physical_header.rs:136-155` `declared_i64_source_agrees_v1`、`declared_bool_source_agrees_v1` | 宣言と scalar の証明が一致するか。`(None, Unannotated) => true` | 宣言の照合(scalar)。String の宣言では false のまま(S3 の拒否) |
| T4 | `physical_header.rs:158-174` `require_declared_result_agrees_with_exits_v1` | 出口の scalar class が宣言と違えば `DeclaredResultDisagrees` | 宣言の照合 |
| T5 | `result_contract.rs:192-215` `has_complete_text_result_v1` | 宣言が `Unannotated` で、全出口が Text | `Unannotated` 限定の判定。S2 で置き換え |
| T6 | `result_contract.rs:220-250` `has_nullable_text_result_v1` | 同じく `Unannotated` で {Text, null} | 範囲外(S1)。そのまま |
| T7 | `result_contract.rs:261,278,290,785` | T5 を呼ぶ側(static と instance の Text source の照合、finalized) | 呼ぶ側 |
| T8 | `ordinary_new_local_commit/finalized_root_cleanup.rs:23` | T5 を呼ぶ | 呼ぶ側 |
| T9 | `callable_result_representation/function_proof.rs:56-92,117-119` | 宣言を読み、`Some(None)` を `DeclaredNonI64Result` で早期に拒否。`declared_bool` なら本体を証明して `ExactBool` でなければ `DeclaredResultDisagrees` | 宣言の照合 |
| T10 | `callable_result_representation/function_proof.rs:417-459` | 本体の return が Text なら `ExactString` | 本体の証明(宣言なしで既に通る) |
| T11 | `resolved_semantics/callable_index.rs:414-420` | 宣言を `classify_result` で読み、読めなければ `ReturnTypeOutsideProfile` | 宣言の照合 |
| T12 | `ordinary_new_source_claim_preparation.rs:105-127` | 借用の宣言の照合 | 借用。範囲外 |
| T13 | `ordinary_new_local_commit/finalized_callable_participants.rs:67,180,262` | `InvokeCallResultKind::OwnedString` の結果 | 呼ぶ側の受け取り |
| T14 | `physical_program_json_call_transport.rs:287-319` | 物理 ABI の JSON の `text_argument_cleanup`。結果が `OwnedString` の時に限る | S8。写さない |
| T15 | `resolved_control_flow/function_control.rs:316-324` | 宣言の綴りごとの `ReturnExitRelationV1` | 宣言(`Annotated` は `DeclaredContractDeferred`) |

## 4. 完了の条件の各形が決まる行

| 形 | 行 |
|---|---|
| `t(): String { return "abc" }`、`local s = Main.t()`、`print(s)` | S1、S2、S5 |
| 宣言の無い同じ形 | S2 |
| `print(Main.t())` | S6 |
| `local s = Main.t()` の後に Fault しうる呼び出し | S5、S7 |
| `u(): String { return Main.t() }` | S5(Return の成功で責任が移る)。**I64 でも基準で拒否される形**(Bool の規則 §7)なので、通らない時は止めて理由を書く |
| `t(): String { return 1 }`、`t(): i64 { return "a" }` | S3、S4 |
| `t(x: String): String { return x + "d" }` | S9(今の名前のまま) |
