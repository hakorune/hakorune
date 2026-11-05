# selfhost までの順番の相談と決定

Exception: 外部 AI の回答と実測の結果は再生成できないので残します(RULES §7「`/tmp` だけを永続証拠にしない」)。

ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

Status: 決定の根拠の記録です。authority ではありません。順番の正本は ADR-002 の 3a の行です。

## 決定(2026-11-06、ユーザー決定)

所有の段を先に全部終える順番をやめます。文法の凍結は変えません。

| 順 | 内容 |
|---|---|
| 1 | 型付き呼び出しの合成(入れ子引数、条件、loop、再帰、caller に生きた Home がある状態) |
| 2 | 結果契約(scalar、Bool、nullable、String)、基本の式、必要な container、混在 field、field に入れない `take` |
| 3 | `edit` の公開 Place と ADR の残り(`from`、`take place`、`match`、`update`、`sync box`) |
| 4 | ADR の課題を組み合わせて正規 EXE で確認し、3b と旧経路の退役 |
| 5 | selfhost で使う部分集合の `.hako` parser parity、selfhost |

builder の規模調査は、4 と 5 の間の位置のままです。

未決: `local s = t.name` の公開借用(下の回答の 5(a))は公開 MIR の拡張を伴うのでユーザー判断です。順番の 2 に着いた時に決めます。それまでは名前付きの拒否のままです。

## 実測(2026-11-06)

`form-table` から切った作業 branch の最適化 binary で測りました。production の差は main に対して `cfg(test)` の lock だけです。`main` は `print(r) return 0` の形です。

| 型付きの呼び出し | 結果 |
|---|---|
| `local r = Main.a(1)` | 通る |
| `a` が `local y = Main.b(x)` で `b` を呼ぶ。3 段も同じ | 通る |
| `local s = Main.b(r)` の連続、`print(r + 1)` | 通る |
| `a(q: P): i64` に object を渡す | 通る |
| `h.add` の連続 | 通る |
| `Main.a(Main.b(1))` | 拒否 `borrowed-static/source-unobserved` |
| `if Main.a(1) < 3`、`Main.a(1) + Main.b(2)`、`print(Main.a(1))` | 拒否 `qualified-preflight` |
| loop の中の `local r = Main.a(i)` | 拒否 `direct-scalar/physical-binding` |
| `local p = new P()` の後の `Main.a(1)` | 拒否 `static-execution/local-contract` |
| 自己再帰と相互再帰(`local m = n - 1`、`local f = Main.a(m)`、`return n + f`) | 通る。`a(4)` は 10 |
| 引数に `x - 定数`(`Main.b(x - 1)`。再帰でなくても同じ) | 拒否 `physical-boundary/finished-sequence` |
| callee の `return n * f`、`local g = n * f`(呼び出しがなくても同じ) | 拒否 `static-execution/completion-not-ready` |
| 深さ 100 万の再帰 | 実行で exit 139(Fault にならず落ちる) |
| instance method の中の `local y = x` の後の `return y` | 呼ぶ側で拒否 `artifact-source-unavailable` |
| `a(s: String)` に `"x"` | 拒否 `borrowed-static/source-actual-unavailable` |
| `a(x: i64): bool` | 拒否 `UnsupportedResultAnnotation` |
| 型なしの `a(x)` に `1` | 拒否 `qualified-preflight`(ADR 57 行は型を必須としているので、新しいコードには不要) |

全体の通り具合は、`apps/**` の入口 1307 本のうち 100 本、`lang/src/compiler` などの入口 19 本のうち 1 本です。`apps` は fixture が多く、進み具合の割合ではありません。

## 相談文の言い過ぎの訂正

相談文に書いた次の点は、測り直すと強すぎました。下の回答はこれを前提に読んでください。回答の結論は変わりません。

- 「呼ばれた関数がさらに呼ぶ形は拒否」: `local` で受ければ通ります。止まっていたのは `return Main.a(x) + 1` のような式の中の呼び出しです。
- 「再帰は拒否」: 再帰そのものは通ります。止まっていたのは `x - 定数` の引数と `*` です。回答の 2 節にある、再帰のために宣言契約と本体の証明を分ける話は、この前提で読み直してください。
- 「`local b = a + 1` は拒否」: 生きた Box があって `print` がなければ通ります。計算した local を `print` に渡す形が拒否されます。
- 「`new ArrayBox` だけは拒否」: `return 0` だけの形は `ny-llvmc` が無いだけで、言語の拒否ではありません。`print` と組むと `lexical-call-prior-home-unavailable` です。
- 「`normal-projection-exit` の 101 本」: 既存の拒否を包み直す名前です(`ordinary_new_normal_return_projection.rs:178-183`)。中身は `PrefixNotCovered` 97 本、`TerminalNotCovered` 4 本で、一つの壁ではありません。

## 確認

Claude Opus 5.5 が、回答の引用先をコードで読みました。読んだ範囲で全部合っていました。回答に「未確認」とある事項は、確かめていません。

## ChatGPT Pro の回答(原文)

**順番を変えるべきです。通常の呼び出しと式の組合せを3aの中心へ入れ、所有機能を全部終えてから基本機能を確認する進め方はやめることを勧めます。** 文法凍結を変更する必要はありません。変更すべきなのは実装の順序と、受理を広げる単位です。

指定commit **`2b590e937b`** を読みました。コード変更・実行検証はしていません。提示された測定値、過去の受理からの回帰、未pushの修理結果は再確認していません。

先に、前提から導ける結論を二点補正します。

- **`print(r)`付きのprobeは、式単独の検査ではありません。** 空Arrayには構築だけの経路がありますが、Printへの受け渡しが用途制限に当たる可能性があります。根拠：[`ordinary_new_named_array_source_selection.rs:160–183`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/ordinary_new_named_array_source_selection.rs#L160)、[`named_array_append_only_source.rs:312–329`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/resolved_semantics/named_array_append_only_source.rs#L312)。
- **呼び出しが約1万か所あること自体は、実装が届かない証明ではありません。** 問題は、同じ呼び出し規約が引数・条件・loop・returnへ合成できるかです。現在、その合成を妨げる制限はコードで確認できます。

**1．selfhostに本当に必要な部分**

**結論：**「理論上コンパイラを書ける最小言語」と「ADRで約束したselfhost開始条件」を分けてください。後者には書きやすさの条件も含まれます。

| 区分 | 必要なもの |
|---|---|
| 書き直すselfhostでも必要になる基盤 | 型付きの宣言、複数段の呼び出し、式内の呼び出し、算術・比較・分岐・loop・再代入、Stringの読みと構築、可変長データ、fieldアクセス、必要なimport／入出力、Normal／Faultの終了責任 |
| ADRの開始条件として必要 | `match`した子の`from`返却、子から孫への借用、Map要素の短い排他更新、`node = fold(move node)`による必須childを含む木の変換 |
| コンパイラの構造として避けられるもの | 所有循環はowner＋ID、再帰は明示work stack、内部の例外回復は結果値で表現可能。一般Dynamic、逃げるclosure、thread、atomic／weakは初期selfhostに必須ではない |
| 現行488fileを移す場合に追加される仕事 | parser差分、注釈なし宣言、旧暗黙共有、既存の例外処理・API利用の移行 |

通常の再帰は理論上避けられますが、AST処理では自然です。**再帰を避けるためだけに全面的なwork stack化を要求する言語を、書きやすいselfhost基盤とは評価しません。**

また、単独owner中心のコンパイラならSharedやsyncを多用せずに書けます。ただし、承認済みのsync最小例などを、それを理由に黙って省略することはできません。

**根拠：**[ADR-002:102–104](https://github.com/hakorune/hakorune/blob/2b590e937b/docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md#L102)は、全凍結文法の完成を要求する一方的な一覧ではなく、3a／3bの必須条件と、後から閉じる事項を分けています。104行の四課題は、補助変数・コピー・Shared化・関数分割による回避なしで通す条件です。

**未確認：**新selfhostの設計がまだないため、具体的にどのcontainer APIやI/O APIまで必要かは確定できません。現行コードの出現数を、そのまま新実装の必須機能数にはできません。

**2．呼び出しの接続**

**結論：**懸念の方向は正しいです。必要なのは、呼び出しの形を追加し続けることから、**確定した呼出し契約を、共通の式評価と終了処理へ接続すること**への移行です。ただし、全型・全呼出しを一commitで実装する意味ではありません。

`seal_local`の条件は、次のように分けられます。

| 条件 | 判断 |
|---|---|
| target・owner・site・引数数・型・受領契約の一致 | 必須 |
| 全Normal／Fault出口で責任が閉じる | 必須 |
| 公開時点でcallee本体も検証済み | 必須 |
| 結果と引数がI64だけ | 現laneの対応範囲 |
| `prior_homes`が空 | callerの既存責任を扱う接続が限定されている |
| 必ずlocalへ束縛する | 式結果の利用先が限定されている |
| callerも`returns_value()`である | このlaneの限定。Unitを返すcallerを禁止する言語上の理由にはならない |
| call解析前にcalleeのCompletionが完成済み | 再帰を扱うには、宣言契約と本体の証明を分離する必要がある |

根拠：[`direct_call_static_execution.rs:86–106`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/direct_call_static_execution.rs#L86)。

勧める構成は次です。

1. **既存catalogで宣言契約を先に確定する。** 引数・結果の型、read／edit／take、`from`、必要な効果を使います。
2. **各本体を、その契約に適合するか検査する。** 呼び出しは共通の式評価へ入り、引数・条件・loopでも同じ取得／移転規則を使います。
3. **再帰する関数群は契約を相互参照し、公開前に全本体の証明を閉じる。** 宣言があるだけで実行許可を発行してはいけません。
4. **既存cleanup graphへcallerの残存Homeと一時値を渡す。** 入場前Faultはcaller、take受領後はcallee、という責任を保ちます。短絡条件では、実行しなかった枝の取得を発行しません。

現在はHome／Completion生成、結果契約のseal、物理signature、call実行の接続という順序です。また、限定された結果分類器は自己・相互再帰を未確定のまま残すと明記されています。根拠：[`issuer.rs:532–620`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/issuer.rs#L532)、[`ordinary_new_result_class_claim.rs:11–25`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/ordinary_new_result_class_claim.rs#L11)。

多層の検査は、**source適合／物理表現との対応／生成後の独立検証**なら必要です。同じ結果を別々に意味分類し直すことは整理対象です。`source-overlap`はscalarとborrowedの両方を発行したときの拒否なので、検査を外すのでなく、上流の一つの結果契約から排他的に投影すべきです。根拠：[`ordinary_new_callable_result_source.rs:233–243`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/ordinary_new_callable_result_source.rs#L233)。

**未確認：**個々の`completion-not-ready`が依存循環そのものかは、traceなしでは断定できません。また既存Call／Invokeで扱える範囲から始められますが、edit Placeや借用Textなど、未表現の契約まで物理追加なしとは言えません。

**3．基本形が通らない原因**

**結論：**すべてがHomeという考え方の欠陥ではありません。ただし、**型・操作・所有責任を確定した式でも、利用文脈ごとに受理が分断されている**ことは、共通の構造的問題です。

| 対象 | コードから分かることと判断 |
|---|---|
| `local b = a + 1`、`1 + 2` | 算術初期化の選択に「selected loop内」または特定の利用frame要求が必要。演算自体より、初期化式を共通評価へ載せる接続が疑わしい |
| 空Array | 構築のみの経路は存在。取得→Endと、Print／引数／返却で使えるかを分けて見る必要がある |
| 空Map | 提示された入力の停止位置は確定できない。`new MapBox`、literal、Print渡しを同じ原因とは扱えない |
| `h == null` | 共通比較と、nullable object用の比較証拠が別にある。全null比較が不可能なのではなく、取得元・型・文脈をつなぐ範囲の問題が疑われる |
| `normal-projection-exit` | root Home flowの出口にある既存エラーを再包装する地点。名前だけで「object returnの不具合」とは判断できない |
| loop群 | sole familyの選択、Call有無、carrier、子loop境界など別々の制限。winner拒否とV2検証拒否を一つの穴として直せない |

**根拠：**

- 算術の選択制限：[`home_new_prefix_initializer_expression.rs:78–84,196–210`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/resolved_semantics/home_new_prefix_initializer_expression.rs#L78)。
- 比較の対象分類：[`home_expression_operations.rs:27–55`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/resolved_semantics/home_expression_operations.rs#L27)。
- 出口エラーの再包装：[`ordinary_new_normal_return_projection.rs:171–183`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/ordinary_new_normal_return_projection.rs#L171)。
- loopのfamily一致とCall分岐：[`normal_callable_loop_source_route.rs:271–277,335–365`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/builder/normal_callable_loop_source_route.rs#L271)。

既存のchecked expression evaluationを、**値・効果・取得責任の共通表現**として使い、local／条件／引数側は結果の利用先と制御辺を与える方向を勧めます。loopも、選択された一般経路が既に扱える形を専門familyとの重複だけで落とす問題と、実際に不足するPHI・所有証明を分けて直します。

必要なselfhost部分集合の算術・呼び出し・null判定・loopは、3b前に閉じるべきです。使わないTryCatchの形、旧Dynamic互換、全fixtureの受理は後送り可能です。**Faultの伝播とcleanup自体は後送りできません。**

**未確認：**各probeの完全なsourceと停止traceを確認していないため、表の「疑わしい」は原因候補です。拒否名の件数を、そのまま修正で通る件数には換算できません。

**4．順番**

**結論：所有機能を先に全部終える順番は勧めません。** 次の順へ変更する提案です。

| 順 | 完了するまとまり |
|---|---|
| 1 | 型付き呼び出しの合成。多段・入れ子引数・条件・loopを、callerのHomeがある状態でも扱う |
| 2 | scalar／Bool／nullable／Stringの結果契約と基本式、必要なcontainer・混在fieldを同じ規約へ接続。格納しないtakeも終了責任の一般化として扱う |
| 3 | `edit`の公開Placeと、from／take place／match／更新などADR必須部分を接続 |
| 4 | ADRの課題を**組み合わせて**正規EXEで確認し、3b・旧経路退役 |
| 5 | selfhostで使う部分集合のHako parser parityを閉じ、selfhostを実装。既存の規約を変える最適化や広い整理は必要に応じて進める |

Stringの残件は、所有結果の注釈など既存表現で閉じるものと、借用Textの公開契約が要るものに分けます。「String全形を先に完了」を呼び出しの前提にはしません。

builderの規模調査も、一括して最後に置く必要はありません。触るownerの重複と退役対象を接続時に確認し、全体の行数調査を別の巨大な前提作業にしない方がよいです。

**根拠：**[ADR-002:102–104](https://github.com/hakorune/hakorune/blob/2b590e937b/docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md#L102)は、3b前の終了責任、構造整理、sync例、自然な四課題を要求しています。一方、現在の「sibling callは必要なappで再開」という記録は、selfhostを次の受入対象として選ぶなら再開する理由が成立します。[`app-boxtorrent-mini.md:89`](https://github.com/hakorune/hakorune/blob/2b590e937b/docs/development/current/main/investigations/app-boxtorrent-mini.md#L89)。

3b前にコンパイラ本体を書かなくても、AST走査や表更新を行う小さな受入アプリで合成を確認できます。小さい部分集合を選ぶことと、現在の実装都合で基本的な組合せを避けることは区別してください。

**未確認：**115件の赤の個別内容は監査していません。触る所有・借用・Fault・公開MIRの安全性テストは解決が必要ですが、無関係な既知失敗まで全件ゼロにすることを各着地の前提にはしません。未pushの89件という状態は、このcommitの証拠には使えません。

**5．Stringの読みと混在field**

**(a) `local s = t.name`**

**結論：Bの公開借用契約を、Cの既存carrierで実装する組合せを勧めます。** 新しいトップレベルopcodeやruntime structが必須とは確認できませんが、公開MIR上の借用producer・支持関係の拡張は必要です。

確認できた範囲は次のとおりです。

- `FieldMaterialize`はNormalで独立したOwnedStringを取得します。結果にfieldの借用支持関係は残りません。  
  [`invoke_string.rs:70–74`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/instruction/invoke_string.rs#L70)
- 既存`BorrowedText`の検証は、Copyを辿って特定のformalへ戻れることを要求します。field起源を入れるだけでは通りません。  
  [`verification/invoke_string.rs:104–126`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/verification/invoke_string.rs#L104)
- C emitterは既にfieldから借用pairを取り出し、既存String operandを作ってmaterializeへ渡しています。  
  [`hako_llvmc_ffi_lifecycle_v4_string_emit.inc:185–199`](https://github.com/hakorune/hakorune/blob/2b590e937b/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_string_emit.inc#L185)

したがって不足は、bytesやhandleを運ぶ箱そのものより、**保持できる借用結果と、holder／slotへの支持関係**です。

Aで独立検証できるのは、合法なfield読取、コピーの取得、コピーの一度だけのEnd／移転です。元fieldとの依存が消えたMIRから、借用支持元の生存条件までは検証できません。

Bでは、出所・型・支配関係・支持slotの有効性・競合更新・寿命を壊す所有移転／End・escape・全Normal／Fault辺を検査します。借用結果自身にEnd義務を与えません。`copy s`は明示的な独立所有取得として、既存Materializeへ接続できます。

現在の即時式のmaterializeは移行実装として残せますが、**裸のlocal読みまでコピー方式で広げることは勧めません。** コピーの実行時間は未測定でも、読取ごとの取得と終了、割当失敗の可能性が増えることは区別すべきです。

**未確認：**Cの全consumerを網羅していないため、runtime ABI完全無変更とは断言しません。公開契約の追加はメンテナ判断事項です。

**(b) 注釈なし`get() { return me.name }`**

**結論：A。3bまでの移行挙動として範囲を固定し、3bで`from`か明示copyへ移行します。Cの読み直しは採りません。**

現挙動は[`ownership.md:1106–1116`](https://github.com/hakorune/hakorune/blob/2b590e937b/docs/reference/language/ownership.md#L1106)に記録済みです。しかし、backendがコピーを生成したことを理由にsourceのfield読みをfresh値へ変えると、「読むなら借りる」という凍結契約を変えてしまいます。この例は`from`または`copy`で書けるため、凍結変更条件には当たりません。

結果注釈は、次の二つを分けます。

| 結果 | 判断 |
|---|---|
| `: String`の所有結果 | `validate_result`がI64専用分類器へ注釈を渡すことが現在の拒否原因。既存OwnedString結果を再利用する方向で、新runtime表現が必須とは言えない |
| `: String from me` | `validate_declared_result`は宣言owner・型の一致でheaderを通せるが、Textの借用結果ABIを発行してはいない。支持関係を持つ公開結果契約が必要 |

根拠：[`physical_header.rs:148–185`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/physical_header.rs#L148)、[`invoke.rs:243–270`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/instruction/invoke.rs#L243)。後者には`OwnedString`はありますが、borrowed Text resultはありません。

**未確認：**所有String結果の全呼出し経路が既存ABIだけで閉じるかは未検証です。`UnsupportedResultAnnotation`だけから新ABI必須とも、header修正だけで完成とも判断できません。

**(c) String＋型付きobject field**

**結論：object-store配置では、既存layout・操作・End descriptorの合成を先に試すべきです。新layout／ABIが必要という根拠は見つかりません。**

確認済みの表現力があります。

- 宣言モデルはText・owned object・Arrayの終了要件を組み合わせられます。  
  [`instance_constructor_semantic/object_definition.rs:149–184`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/normal_callable_semantic_package/instance_constructor_semantic/object_definition.rs#L149)
- 独立End検証は、DynamicResidenceとObjectFieldを一つの列へ積み、逆宣言順＋最後のReleaseOuterとの完全一致を検査します。  
  [`verification/invoke_object_end.rs:16–42`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/verification/invoke_object_end.rs#L16)
- C側にもdynamic slotとstatic childを同じ逆順walkで終了する実装があります。  
  [`hako_llvmc_ffi_lifecycle_v4_teardown_emit.inc:303–365`](https://github.com/hakorune/hakorune/blob/2b590e937b/lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_teardown_emit.inc#L303)

最小の道は、Textをdynamic residence、Leafをexact owned child、i64をTrivialとして、**fieldごとの格納と部分構築のcleanupを同じ完全End契約へ接続すること**です。狭い`supports_static_no_hook()`条件を外す場合も、単に緩めず、そのchildの完全なEndが得られることを条件にします。

直接配置は別です。[`object_definition.rs:106–119`](https://github.com/hakorune/hakorune/blob/2b590e937b/src/mir/function/object_definition.rs#L106)はi64と選択済みchildに限定されています。Stringのdirect配置まで、今回の混在boxの前提にしない方が小さく進められます。

**未確認：**提示されたNodeのどの`FieldContractUnsupported`が発火したか、partial birthのFaultまで閉じるかは未確認です。Holderの2対2はdynamic経路の証拠ですが、typed childの接続証明にはなりません。Array carrierの規則も、この混在field判断の直接根拠には使いません。

**次に着手する一つは、「型付き呼び出しを、既存Homeがある状態で合成できる小アプリ」です。**

最初はI64入出力と既存Box一つに範囲を絞り、完了条件を次にします。

- helper→helper、`f(g(x))`、条件・loop中の呼出しが同じ規約で正規EXEまで通る。
- callerに生きたBoxがあっても通る。
- Normal、後続引数のFault、callee入場後のFaultで、責任主体・End回数・順序が正しい。
- 改変したMIRの引数契約、所有移転、cleanup欠落を独立検証が拒否する。
- 置換した形別判定・adapterを同seriesで削除し、固定suiteと関連する安全性検証を保つ。

これを終えれば、呼び出しが一つ増えるたびに新しい受理経路が必要なのか、共通契約で組み合わせられるのかを、実装と実行結果で判断できます。
