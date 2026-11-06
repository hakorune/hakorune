# 関数の結果の契約の相談と決定

Exception: 外部 AI の回答と実測の結果は再生成できないので残します(RULES §7「`/tmp` だけを永続証拠にしない」)。

ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

Status: 決定の根拠の記録です。authority ではありません。順番の正本は ADR-002 の 3a の行と[整理の順番](mirbuilder-structure-refactor-queue-d0-2026-08-23.md)です。

## 決定(2026-11-06、ユーザー決定)

結果の契約は、Bool から S3b の前に入れます。

| 順 | 内容 |
|---|---|
| 1 | Local の初期値の呼び出しの連鎖を消す(作業中) |
| 2 | 警告の整理(embedded snapshot と環境変数の競合の 2 件を含む) |
| 3 | 結果の契約の土台と、明示の Bool の結果(下の回答の 9) |
| 4 | S3b(確定した結果の契約を入力にして、所有の状態と Fault の後片付けを CFG の解析へ移す) |
| 5 | 所有の String、所有の object、nullable の結果 |
| 6 | S4、`edit`(2026-11-08 以降)、3b と旧経路の退役、builder の規模調査、`.hako` parser parity、selfhost |

- 3 で変えるのは内部の header の型だけです。Bool は既存の `Bool`/`ordinary_bool` を使うので、新しい公開 MIR、opcode、ABI、descriptor は足しません。足す必要が見つかったら、その部分を止めてユーザーに聞きます。
- 借用の String(`String from me`)は、借用の Text の公開の結果契約が要るのでユーザー判断です。5 の後で決めます。それまでは名前つきの拒否のままです。

## 相談した事(2026-11-06)

S3a で、`: Bool`、`: String`、`: P` の結果の宣言が `UnsupportedResultAnnotation` で止まりました。「結果の種類」を表す型が 9 つあるので、ChatGPT Pro に、結果の契約を 1 つにする設計を聞きました。聞いた事は 5 つです。

1. 結果の契約を 1 つにする形(軸)と、今の 9 つのうち残す物、投影にする物、消す物
2. 意味の型、所有の区分、物理の表現を分ける線。`ExactTrivialScalarAbiV1` に Bool を足すだけでよいか
3. 宣言がある時と無い時(推論)を同じ道にしてよいか
4. 返した String と object の持ち主と、S3b との分担
5. Bool → String → object → nullable の順で入れる時の土台と、公開 MIR・ABI の最小の変更

## 実測(2026-11-06)

main(`67185dc817`)の最適化 binary で、EXE まで作って試しました。

| 形 | 結果 |
|---|---|
| `t(x: i64): Bool { return x > 1 }` → `local b = Main.t(4)` | 拒否 `UnsupportedResultAnnotation { name: "Bool" }` |
| 宣言なし `t(x: i64) { return x > 1 }` → `local b = Main.t(4)`、`print(Main.t(4))`、`if Main.t(4)` | 拒否 `borrowed-static/source-unobserved` |
| 宣言なしの instance の Bool の結果 `local b = c.t(4)` | 拒否 `local-commit/artifact-source-unavailable` |
| `t(x: i64): String { return "ab" }` | 拒否 `UnsupportedResultAnnotation { name: "String" }` |
| 宣言なしの static の String の結果 | 拒否 `main-import-view/selected-header-missing` |
| 宣言なしの instance の String の結果 | 拒否 `artifact-source-unavailable` |
| `get(): String from me { return me.f }` | 拒否 `UnsupportedResultAnnotation { name: "String" }` |
| `make(x: i64): P { return new P(x) }` | 拒否 `UnsupportedResultAnnotation { name: "P" }` |
| 宣言なしの static `make(x: i64) { return new P(x) }` | 拒否 `selected-header-missing` |
| 宣言なしの instance の `f.make(3)`(object を返す) | 通る、出力 3 |
| `me.make(4)` を本体の中で受けて `p.v` を返す | 拒否 `borrowed-result/source-not-i64` |
| 宣言なしの nullable(`return new P(x)` か `return null`) | 拒否 `selected-header-missing` |
| `add(x: i64): i64 { local y = x return y }` | 通る(S3a) |

i64 以外の結果で通るのは、宣言なしの instance の object の結果だけでした。

## 相談文の誤りの訂正

- 相談文に「物理 program の JSON の scalar には、既に `i64`、`bool`、`f64` がある」と書きました。これは array へ書く要素の表現(`physical_program_json_operation.rs:268-273`)で、関数の結果の表現ではありません。回答の指摘どおりです。
- `get(): String from me` の拒否は、I64 だけを分類する `validate_result` では説明できません。`validate_declared_result`(`physical_header.rs:174-189`)は、借用の宣言があれば I64 の分類を通りません。同じ名前の拒否を作る所はもう 1 つあり、`validate_declared_borrowed_source_v1`(`ordinary_new_source_claim_preparation.rs:105-129`)が、receiver の支持元の根が宣言の型を支えない時に返します。今回の拒否はここから出た可能性が高いです。trace では確かめていません。

## 確認

Claude Opus 5.5 が、回答の引用先のうち次をコードで読みました。読んだ所は全部合っていました。

- ADR-002:61(名前付き関数とメソッドの結果の型は、公私とも明示が必須)
- `physical_header.rs` の 1 行目(runtime ABI ではない)、49 行目(header の行が `ExactTrivialScalarAbiV1` を持つ)、108 行目、148-189 行目
- `result_contract.rs` の 48 行目(`VerifiedCallableResultContractRowV1`)、224 行目(nullable の Text の証明は、Text と null の両方の出口を要る)
- `callable_semantic_batch/model.rs:65`(`declared_result_type: Option<TypeReferenceV1>`)
- `instruction/invoke.rs:268`(`MixedHandle` は非 null でも所有ではない)
- `exact_trivial_scalar_abi.rs` の 1 行目(この型だけでは関数を受理しない)
- `physical_program_json_call_transport.rs:317-318`(`text_argument_cleanup` は結果が `OwnedString` の時だけ)

読んでいない引用先: C の `.inc` の 2 file、`invoke_borrowed_calls.rs:44`、`root_home_cleanup_order.rs:411`、`ordinary_new_result_class_claim.rs:125` と `product.rs:9`、`types.md` の 400 行目と 543 行目。回答が「未確認」とした事項も確かめていません。

## 回答の要点

1. **結果の意味を決める契約は 1 つにします。** 既存の `VerifiedCallableResultContractRowV1` を育て、別の台帳は作りません。軸は 3 つです。
   - 型: Unit、I64、Bool、String、Nominal(宣言の ID)、`Nullable(T)`。nullable は型の軸で、所有の nullable と借用の nullable に別の分類器を作りません。
   - 返し方: 値で返す(Bool は終わらせる責任なし、所有の String や object は責任を移す。違いは型の所有の分類から導く)、または読み借用で返す(支持元は receiver か formal)。
   - 出所: 宣言か、移行用の証明か。診断と移行のための情報で、同じ意味の結果に別の lowerer を選ぶ材料にしません。
   - Unknown、`UnresolvedScalar`、未対応は解析の途中の状態で、完成した契約には入れません。
   - 移行中の `MixedHandle` は、証拠つきの互換の契約として同じ owner の中だけで持ちます。新しい宣言の検査に落ちた物を、ここへ落としてはいけません。
2. **9 つの扱い。**

   | 型 | 扱い |
   |---|---|
   | `DeclaredFunctionResultContractV1` | 残す。宣言の記録で、後段の判定者にしない |
   | `ExactTrivialScalarAbiV1` | scalar の投影として残す。Bool を足してよい。header の結果の型からは外す |
   | `ResolvedCallableBorrowedResultDeclarationV1` | 宣言の証拠として残す |
   | `ExpressionValueClassV1` | 式の型からの限定の投影にする。call では callee の契約を読む |
   | `BorrowedResultClassV1` | 完成した結果の並列の分類器としては退役。未解決の状態は solver の中へ |
   | `OrdinaryNewResultClassV1` | 結果の型を決める物からは退役。返却元の証拠(`Fresh`、`ForwardFormal` ほか)は残す |
   | `VerifiedCallableResultRepresentationV1` と `...DispositionV1` | 重なる意味の分類を退役し、投影にする |
   | `LocalCallResultClassV1` | 受け取り方の投影にする |
   | `InvokeCallResultKind` | 公開 MIR の契約として残す。独立検証の側は source の型と一体にしない |

3. **header は契約を参照し、物理の形を投影として持ちます。** 内部の header の型を変えることは、公開 ABI の変更ではありません。`ExactTrivialScalarAbiV1` に Bool を足す時は、未接続の Bool の引数まで受理させないようにします。
4. **宣言と推論は入口を分け、確定した後の道を共通にします。** 宣言があれば署名を先に確定し、本体を照らします。宣言が無い物は 3a の移行用の入口で、既存の証明を使います。検査に落ちた物を推論で救いません。署名の確定と本体の照合の完了は別の段階にします(再帰で、宣言した結果の型を本体の検証より前に読めるように)。
   - `if f()` に使われた事から、`f` の結果を Bool にしません。
   - Bool の型の証明は、Fault が無い事の証明ではありません。
   - `: P?` は、null の枝と P の枝の両方を要求しません(今の nullable の Text の証明の「両方の出口がある」という条件を宣言の検査へ持ち込まない)。
5. **結果の契約は責任の種類を決め、S3b の CFG の解析は責任が移る時点を決めます。** 返す前の cleanup が Fault なら結果を公開せず callee が片付ける。Return の成功と Call の Normal で caller へ移る。caller の後の評価が Fault なら caller が片付ける。名前の無い一時値にも責任がある。null の枝は End を作らない。借用の結果は Home を作らない。S3b は結果の型を推論し直しません。
6. **独立検証は 2 つの責務を別に保ちます。** source との対応の検査(`from`、`copy`、`move`、評価順)と、公開 MIR の検証(宣言、Return、Call、Normal と Fault の辺、所有、借用、End)です。`check_call_result` は signature の一致だけなので、callee の本体まで正しいとは言えません。C の返却起源の検査を、生成側の「検証済み」の印で置き換えません。
7. **順番。**

   | 段階 | 完了させる事 | 同時に消す物 |
   |---|---|---|
   | I64 と Bool | 宣言または互換の証明 → 共通の契約 → header → call → 検証 → EXE | Bool だけの宣言の照合と、使い先の再分類 |
   | 所有の String | 宣言つきの String と、Normal での受け取り、返す前と受け取った後の Fault の終了 | `Unannotated` に限った String の結果の受理 |
   | 所有の object | 正規の nominal 型と所有の移転。static と instance を共通にする | `return new` を callee の型を決める物にする処理 |
   | nullable | `Nullable(T)` から null と非 null の型と責任を投影 | 型ごとの nullable の分類器 |
   | 借用の String | 支持元つきの公開の結果契約を、承認の後で接続 | 所有の String への変換で代用しない |

8. **公開 MIR、ABI、descriptor。** Bool は既存の `Bool`/`ordinary_bool`、所有の String は `OwnedString`/`ordinary_text`、所有の P と P? は `Handle`/`NullableHandle` で足ります。`String from me` には借用の Text の公開の結果契約が要り、ユーザー判断です。F64 の結果と nullable の scalar は今回入れません。`text_argument_cleanup` を結果が `OwnedString` の時に限る今の制限は、新しい契約へ写しません(Bool の結果でも所有の Text の引数の後片付けが要る組を確かめる)。
9. **次の一手。** 既存の結果の持ち主を広げる事と、明示の Bool の結果の縦切りを、1 つの完了単位にします。完了の条件は、Bool の local 経由、別の関数からの返却、static と instance の呼び出しが同じ契約を使って EXE で期待の結果を出す事、型の違う return と公開の結果の改変を拒否する事、置き換えた Bool の再分類を同じ series で消す事です。

## ChatGPT Pro の回答(原文)

**推奨は、既存の `VerifiedCallableResultContractRowV1` を「結果の意味を決める唯一の契約」に育て、header・呼ぶ側の受領・公開MIRを、その契約からの投影にすることです。** `ExactTrivialScalarAbiV1` に Bool を足すだけでは、String・object・nullable・`from` の分裂が残ります。

指定コミットを読みました。コード変更・実行検証はしていません。以下、コードで確認した事実と設計提案を分けます。

先に、前提を補正します。

- **無注釈の結果推論は、現段階では移行用です。** ADRは、3b後の名前付き関数・メソッドの結果型を公私とも明示必須にしています。runtimeがtag付きであることと、結果契約を暗黙Dynamicにすることは別です。[ADR-002:61](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md#L61)
- JSONの `i64/bool/f64` の引用箇所は、**配列への追加要素の表現**です。関数のF64結果が使える証拠ではありません。[physical_program_json_operation.rs:268](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_operation.rs#L268)
- `String from me` の拒否を、I64限定の `validate_result` だけでは説明できません。`validate_declared_result` は、対応する借用宣言があればI64分類を迂回します。測定ケースがどこで借用宣言を失ったか、どの入口で拒否されたかは未確認です。[physical_header.rs:174](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/physical_header.rs#L174)

**1．結果契約は、意味型・返却方式・出所に分ける案を勧めます。**

確認済みの既存行には、owner・identity・completion・returnごとの対応関係が既にあります。ここへ別の台帳を並べる必要はありません。[result_contract.rs:48](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/result_contract.rs#L48)

既存行のscalar限定部分を、例えば次に置換します。名前は提案です。

```text
CallableResultSemanticsV1
  ty: 正規型への参照
      Unit / I64 / Bool / String / Nominal(宣言ID) / Nullable(T) / …

  delivery:
      Value
      ReadBorrow { supports: Receiver | Formal(index) | … }

  origin:
      Declared
      LegacyProved
```

- `Value` は値を返す契約です。Boolなら終了責任なし、所有String・objectなら所有責任を移します。この違いは型の所有分類から導きます。
- `ReadBorrow` は宣言された支持元に依存する読み借用です。呼出し時にreceiver/formalを実引数のPlaceへ対応付けます。
- **nullableは型の軸です。** 「所有nullable」と「借用nullable」を別々の型分類器にしません。
- `origin` は診断と移行の情報です。同じ意味の結果に別lowererを選ぶ情報にはしません。
- `Unknown`・`UnresolvedScalar`・未対応は解析状態です。完成した結果契約へ混ぜません。

型参照を文字列から毎回解析し直す必要もありません。semantic rowには既に `declared_result_type: Option<TypeReferenceV1>` があります。これを正規型の解決へつなぎます。[model.rs:65](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/callable_semantic_batch/model.rs#L65)

ただし、移行中の `MixedHandle` は例外です。これはfresh・借用・nullを含み、**非nullでも所有とは限りません**。既存の証拠付き互換契約として同じowner内に限定保持し、上の二択へ無理に変換しないでください。新しい宣言の検査失敗から、この互換契約へ落とすことは禁止します。[invoke.rs:268](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/instruction/invoke.rs#L268)

9種類の整理は、次が妥当です。

| 現在の型 | 推奨する扱い |
|---|---|
| `DeclaredFunctionResultContractV1` | **残す。** 原宣言・省略・voidの記録。後段の意味判定者にはしない |
| `ExactTrivialScalarAbiV1` | **scalar投影として残す。** Bool追加は可。header全体の結果型から外す |
| `ResolvedCallableBorrowedResultDeclarationV1` | **宣言の証拠として残す。** 確定契約の支持元へ反映する |
| `ExpressionValueClassV1` | **式型からの限定投影にする。** callではcallee契約を読み、本体を再分類しない |
| `BorrowedResultClassV1` | **完成結果の並列分類器として退役。** 未解決状態はsolver内部へ |
| `OrdinaryNewResultClassV1` | **結果型の決定者から退役。** nominal・nullableは正規型へ |
| `VerifiedCallableResultRepresentationV1`／`DispositionV1` | **重複する意味分類を退役し、投影にする。** 入力要件など別責務の情報は残す |
| `LocalCallResultClassV1` | **受領処理の投影にする。** Home・一時値・借用の扱いを契約から求める |
| `InvokeCallResultKind` | **公開MIRの契約として残す。** 独立検証側までsourceの型と一体化しない |

消す対象は「意味を再判定する処理」です。返却元の証拠まで消してはいけません。実際、`OrdinaryNewResultClassClaimsV1` は既に、各returnの `Fresh / ForwardFormal / BorrowedReceiver / MapHeld / Null` を保持する製品の別名です。[ordinary_new_result_class_claim.rs:125](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/ordinary_new_result_class_claim.rs#L125)、[product.rs:9](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/ordinary_new_result_class_claim/product.rs#L9)

**2．headerは結果契約を参照し、物理表現を別に投影します。**

現状のheaderは `ExactTrivialScalarAbiV1` を直接持ち、結果が `None` の行を発行しません。これが非scalarを共通経路へ載せにくい構造です。[physical_header.rs:49](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/physical_header.rs#L49)、[同:108](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/physical_header.rs#L108)

提案する分担は次です。

| 場所 | 持つ情報 |
|---|---|
| 意味の結果契約 | 正規型、Value／ReadBorrow、支持元 |
| 内部callable header | 上の契約への参照と、選択済みの物理投影 |
| 公開callee宣言・Call descriptor | 型付きの物理結果、所有移転／借用、必要なobject ID・支持元・read権限 |
| 本体の証拠 | 各returnが宣言と一致すること、sourceとの対応 |
| 独立検証 | 公開宣言と実命令・CFGから、結果と責任を再確認 |

`physical_header.rs` 自身は「runtime ABIではない」と明記しています。**内部headerの型変更と、公開ABI変更を同じ承認事項として扱う必要はありません。** [physical_header.rs:1](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/physical_header.rs#L1)

`ExactTrivialScalarAbiV1` にBoolを足すのは、この投影の一部として行えばよいです。ただし、引数側でも使う型なので、追加だけで未接続のBool引数まで受理させないことが必要です。現在も、このhelper単独では関数を受理しない契約です。[exact_trivial_scalar_abi.rs:1](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/exact_trivial_scalar_abi.rs#L1)

**3．宣言と推論は入口を分け、確定後の道を共通にします。**

宣言があれば、先に署名を確定し、本体を照合します。宣言なしは、3aの互換入口として既存の証明を使います。検査失敗を推論で救済してはいけません。

また、**署名の確定と、本体の照合完了は別段階**にしてください。再帰呼出しも宣言済みの結果型を参照でき、本体の検証が揃ってからseal・公開します。「calleeのcompletionが未完だから、宣言済みの結果型まで取得できない」という依存を避けます。

型規則は、次の文で十分に一般化できます。

| 構文・場面 | 規則の文 |
|---|---|
| `f(): T` | 署名のTを先に確定し、すべてのNormal出口をTに照合する |
| `f(): T from x` | 型の照合に加え、全非null結果が宣言したxに支持された読み借用であることを要求する |
| literal | literalと期待型の規則から決める。null単独からnullableの要素型を作らない |
| local／formal | bindingの確定型を使い、読み・移転・借用を別に検査する |
| 単項・二項演算 | operand型と共通の演算子規則から結果型を求める |
| 比較・論理演算 | 正常完了時の結果はBool。短絡評価とFaultの可能性は別に保持する |
| call | calleeの結果契約を使う。static／instance／`me`で型規則を変えない |
| `new P(...)` | 構築のNormal成功で所有Pを得る |
| field／要素読み | field・要素の型を使い、借用なら支持元とslotの有効性を保持する |
| if／matchの合流 | 同型または許されたnullable関係で合流し、所有方式・支持元も照合する |
| loop内のreturn | 他のreturnと同じ結果契約へ照合する |
| `return e` | 型だけでなく、所有を渡せる値か、宣言された借用かを検査する |

この規則なら、Bool literalだけでなく、`return b`、`return otherBool()` も普通に扱えます。

注意点は三つです。

- `if f()` に使われたことから、`f`の結果をBoolにしてはいけません。条件のtruthinessと、結果型Boolは別です。[types.md:400](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/docs/reference/language/types.md#L400)
- Boolの型証明は、無Fault・純粋性・停止性の証明ではありません。[types.md:543](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/docs/reference/language/types.md#L543)
- `: P?` は「null枝とP枝を両方書け」という意味ではありません。Pだけを返す本体も、nullだけを返す本体も宣言への照合で扱います。現在のnullable String証明の「両方の出口がある」という条件を、宣言検査へ持ち込まないでください。[result_contract.rs:224](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/result_contract.rs#L224)

無注釈推論を将来広げるなら、ADRの条件である「内部・非再帰・呼び先契約確定・全Normal結果が一意・借用結果を除外」に限定します。今回、恒久的な全体推論を追加する必要はありません。

**4．結果契約は責任の種類を決め、S3bのCFG解析は責任が移る時点を決めます。**

所有の不変条件は、厳密には「各関数が必ずEndする」ではありません。**その関数の責任を、Endまたは正当な移転で一度だけ閉じること**です。

| 場面 | 責任 |
|---|---|
| calleeが所有結果を計算した直後 | まだcalleeが保持する |
| 返却前cleanupがFault | 結果を公開せず、calleeが結果も片付ける |
| Return成功／Call Normal | callerへ責任が移る |
| callerの後続評価がFault | 取得済みの結果をcallerが片付ける |
| 結果を捨てる／入れ子callへ渡す | local名がなくても一時値の責任がある |
| nullableのnull枝 | 所有token・payloadのEndを作らない |
| 借用結果 | 新しい所有Homeを作らず、caller側の利用終了まで支持元を保つ |

pending returnの取消は既存のcleanup ownerにもあります。ここを省略して、`return`を見た時点で移転済みにしてはいけません。[root_home_cleanup_order.rs:411](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_home_cleanup_order.rs#L411)

S3bは、この確定契約を入力にして、取得・移転・借用期間・合流・Fault cleanupを解析します。結果型を再推論させません。移行中もEndの発行者は既存ownerか置換後のownerの一方にし、二重発行を避けます。

独立検証については、現状の説明に補足が必要です。`verification` 全体がsource非依存なのではなく、source対応の検査も含まれています。例えば `invoke_borrowed_calls.rs` は `root_source()` を読みます。[invoke_borrowed_calls.rs:44](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/verification/invoke_borrowed_calls.rs#L44)

したがって、次を別責務として維持します。

- **source対応検査:** `from`・`copy`・`move`の意味や評価順を、生成が変えていないか。
- **公開MIR検証:** 宣言、実Return、Call、Normal/Fault辺、所有・借用・Endが整合するか。

現在の `check_call_result` はI64／Bool／Unitのsignature一致だけです。これだけでcallee本体まで正しいとは言えません。[invoke_call_edge.rs:160](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/verification/invoke_call_edge.rs#L160)

C側には実Returnと生存状態の検査があります。objectでは、返却起源を実際のproducer・Normal辺・Copy鎖とも照合しています。この検査を、生成側の「検証済み」印で置き換えないことが重要です。[indexed_return.inc:20](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_return.inc#L20)、[indexed_object_return_origin.inc:39](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_indexed_object_return_origin.inc#L39)

**5．最初はBoolで契約全体を接続し、その後に所有結果を広げます。**

| 段階 | 完了させること | 同時に退役させるもの |
|---|---|---|
| I64＋Bool | 宣言／互換証明→共通契約→header→call→検証→EXE | Boolだけの宣言照合と、consumer側の再分類 |
| 所有String | 宣言付きStringと、Normalでの受領・返却前／取得後Faultの終了 | `Unannotated`限定のString結果受理判定 |
| 所有object | 正規nominal型と所有移転。static／instanceを共通化 | `return new`をcalleeの型決定者にする処理 |
| nullable | `Nullable(T)`からnull／非nullの型と責任を投影 | 型ごとの独立nullable分類器 |
| 借用String | 支持元付きの公開結果契約を承認後に接続 | 所有Stringへの変換による代用は作らない |

`Nullable(T)`と借用の軸は最初から契約に用意し、実行受理は対応済みの範囲だけ開きます。

公開変更について、確認できた範囲は次です。

| 対象 | 公開MIR・ABI・descriptorの判断 |
|---|---|
| 内部結果契約／内部header | 内部型の変更。これ自体は公開ABI追加ではない |
| Bool値 | 既存の `Bool`／`ordinary_bool` を利用可能。新opcode・wire kindは不要 |
| 所有String | 既存の `OwnedString`／`ordinary_text` を利用可能 |
| 所有P／P? | 既存の `Handle`／`NullableHandle` を利用可能。ただし返却起源の対応範囲は別途守る |
| `P? from me/arg` | 既存の借用handle契約あり。非nullable保証や新しい支持元まで自動的に対応済みとはしない |
| `String?` | nullable textの搬送はあるが、現在はcallerにopaque Dynamicとして公開する。静的な`String?`を保持できるか追加確認が必要 |
| `String from me/arg` | **借用Textの公開結果契約が必要。ユーザー判断事項** |
| F64結果／nullable scalar | ordinary resultの既存種類だけでは足りない。今回まとめて受理しない |

既存の種類は [invoke.rs:243](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/instruction/invoke.rs#L243)、JSONの対応は [physical_program_json_call_transport.rs:301](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_call_transport.rs#L301) で確認しました。

二点は特に注意してください。

- **`Handle`があることは、任意の所有値を返せる証明ではありません。** Cの返却起源は現在 `null / fresh / owned_call` を検査します。take引数などへ返却起源を広げる際、既存descriptorで表せなければ公開契約の追加として扱います。
- **`BorrowedText` operandは、借用Stringの結果契約ではありません。** `String from me`には支持元・read権限・Return／Call間の対応が必要です。最上位opcode追加は必然ではありませんが、descriptor等の拡張は必要です。runtime ABIまで据え置けるかは未確認です。[invoke_string.rs:7](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/instruction/invoke_string.rs#L7)

さらに、現在は `text_argument_cleanup` があるcallを、結果が `OwnedString` の場合に限定しています。**引数の後片付けと結果型は独立なので、この制限を新しい意味契約へ移植してはいけません。** Bool結果でも所有Text引数のcleanupが必要になる組合せを確認対象にしてください。[physical_program_json_call_transport.rs:317](https://github.com/hakorune/hakorune/blob/67185dc8173c573889ee51d41e9ce11e944749a9/src/mir/compiler/normal_default_pipeline/published_backend_view/physical_program_json_call_transport.rs#L317)

**次の一手は、「既存結果ownerの拡張と、明示Bool結果の縦切り」を一つの完了単位にすることです。** 完了条件は、Boolのlocal経由・別関数からの返却・static／instanceの呼出しが同じ契約を使い、EXEで期待結果を出すこと。加えて、型違いのreturnと公開結果の改変を拒否し、置換したBoolの再分類を同じseriesで削除することです。

実測ケースの再現、既存テストの緑赤、全consumerの置換量、借用Textの最小runtime ABIまでは、今回確認していません。
