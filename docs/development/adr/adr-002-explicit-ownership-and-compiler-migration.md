# ADR-002: 明示する所有操作と共通ownership passへの移行

Status: 設計方向を採用。2026-10-30に表面契約を決定（[ownership.md §0](../../reference/language/ownership.md)）。
2026-10-31にShared・`sync box`・threadの実装方向を決定。step 1は限定的なlocal `move`のEXEと独立MIR検証、旧local planner退役まで着地。Sharedと一般ABIは未完、切替は未着手。
Date: 2026-10-30（改訂 2026-10-31）
Scope: 次期所有モデルの判断・未決点・移行タスクをここへ集約する。

現在の言語意味論は[ownership.md](../../reference/language/ownership.md)、
運用は[RULES.md](../RULES.md)、実行対象は
[CURRENT_STATE](../current/main/CURRENT_STATE.toml)が所有する。
このADRは現行アプリの完了条件を増やさず、parser対応や性能を主張しない。
Array carrier格納はownership.mdの独立した採用済みtargetとして先に接続する。

## 採用する方向

所有操作を局所的に読めるようにし、単独所有を基本経路にする。
全Boxへの暗黙RC、最終使用位置による暗黙move、推論失敗時のDynamic化は採用しない。

1. `=` は右辺の結果を左辺へ格納する。
2. 所有値を手放す操作は `move`。複製はcopy契約を宣言した型だけ（`copy x`を`.copy()`へ綴り替えるかは保留）。
   共有は `Shared(move x)` / `Shared(新しい値)` で作る。`share x`前置形と`.share()`は作らない。
3. 新しい所有値を返す式は直接渡せる（`cache.put(Image.load(p))`）。数値・Bool・Shared handleは暗黙コピーの値カテゴリで、サイズでは決めない。
   Shared handleをtake引数へ渡すとhandleが一つ増え、`move shared`は増やさず移す。Sharedは当面読み取り専用で、共有変更とthread越えは別契約。
4. 引数は既定 `read`（書かない）。`mut` と `take` は定義側で宣言する。receiverのread/mut借用も宣言から決まり、呼出し側に書かない。
   receiverの消費だけ呼出し側で明示する（`(move image).into_bytes()`）。
5. `return`は所有ローカル全体と新しい所有値に限りmove不要。借用値は、定義側で支持元を宣言した
   `from me` / `from <引数>` の結果だけ返せる（2026-10-30改訂。制約はownership.md §0）。
   その他の場所では最終使用でもmoveを明記し、ブロック末尾やbreakにも省略を広げない。
6. 部分moveは禁止し、要素を取り出す操作は契約を持つ `remove` / `replace` に限定する。

```hako
// 次期構文の説明例。今日のparser-liveの実行例ではない。
local p = Point(0, 0)
local q = move p
local r = copy q
arr.push(move r)
arr.push(Point(1, 2))
translate(q, 10)        // mutは定義側。呼出し側の印は未決（案: 書かない）
return q
```

裸の `local b = a` は、所有値なら暗黙alias/move/copyにせず診断する方向。field読み `local x = me.field` は注釈なしの `from me` 読み借用で、借用結果と同じ検査器が扱う。
`return q`の省略は所有可能なローカル全体に限り、借用引数やfieldからの部分moveを許可しない。
明示Dynamicは独立した契約として残す。静的検査を基本にすることと、Dynamicの禁止は別である。

## 切替前に確定する境界

| 項目 | 決めること |
| --- | --- |
| record / box | recordの値意味論とboxのidentityを区別。Boxまで一律MVSへ変える決定はしない。 |
| copy | 決定: copy契約を宣言した型だけ。残り: Boxの新identity、入れ子の所有値・Shared・弱参照・失敗時の扱い、`.copy()`への綴り。 |
| Shared / mut | 決定: `Shared<通常Box>`は読み取り専用（readメソッドとfield読みは可、mut/take/field代入は不可）。Sharedだけでは安全性を発行しない。表現は**Shared専用の非原子RC**を物理表現を隠すopaque境界の内側に置く（Unique Boxにheaderを要求しない、Rust `Arc`を境界に出さない）。操作契約: 構築成功＝handle 1でpayloadの所有・End義務を一度だけ取り込む／暗黙複製・take通常渡し＝+1／`move shared`＝0／read・借用＝0（支持元Homeに依存）／handleのEnd＝−1、`1→0`だけがpayload終了へ進む（wrap・0からの復活・Fault後のEnd再試行は禁止）。独立MIR検証には型付きの複製・移転・解放、借用の支持元、実CFG辺との対応を残す（生のinc/decやHome記録だけの検証にしない）。cycle: 構築中の自己参照・可変alias経由の再接続・証明不能な所有循環は公開前に拒否、weakは後送り（strong＝0以降のupgradeは失敗）。決定（2026-11-02）: `Shared<T>`は組込みの型構築子（field・引数・戻り値に明記可、局所推論も可、汎用generics不要）。fieldへの`local b = me.sharedField`は+1（§0の「field読みは借用」へのShared限定の例外）、field経由のreadとread引数渡しは借用、通常Boxを暗黙にSharedへ変換しない。field/要素は終了義務handle一つを持ち、置換は候補準備→slot検査→commit→旧handle Release（commit前Faultと旧Release中Faultを区別し後者で新slotを戻さない）、外側のEndはhandleだけをReleaseしpayloadを直接Endしない。保存・返却は引数契約・結果契約・field/要素の所有形態・物理表現（意味契約と分離）を型情報として保持する。物理追加（新MIR命令）が必要と分かったら止めてユーザー判断。残り: header配置・inline化、weak。 |
| 共有変更 / sync box | 決定: 基本は単独owner＋必要時`mut`借用、外へ渡すのはID・snapshot・独立所有結果。共有変更は既存`sync box`（[boundary-model](../../reference/concurrency/boundary-model.md)）へ集約し、`Shared<sync box>`は宣言された同期メソッドへの入場だけを許して区間内でのみ排他的な`me`を得る。区間内はsuspend・待ち・task dispatch・event pump・暗黙yield（現`safepoint_and_poll`のscheduler pollを含む）をしない。cleanup/fini（暗黙Endと呼び先を含む）から新規入場しない。`me.helper()`は同じreceiverの生存guardを借りる証拠で許可し、callback・別handle経由は内部呼出しと認定しない。外部再入はFault、再入可能ロックは採らない。同一thread版は別objectへの入れ子を許し同一objectはFault（boundary-modelの現行拒否を切替前に明示改訂）。guardは公開値にせず全終了辺で一度だけ退場。Faultで退出した入場済み区間をpoison（解錠は必ず、最初のFaultを保持、`Result::Err`では付けない）。区間内の効果（suspend・待ち・task実行・取得lock順位・escape上限）は関数契約に持ち、独立verifierが実CFGと呼出しグラフ（挿入End/fini含む）から再集計。未知Dynamic/FFIは区間内で拒否。残り: 条件待ち`wait`（区間を一度終える操作）、poison回復API。 |
| thread | 決定: `move`は所有移転で、越境許可は別（既存`HakoSend`/`HakoShare`/`ThreadRoot` descriptorを検査へ育てる。Dynamicは既定で越境不可）。非原子Sharedと越境用Sharedは型・ABIで別物にし、build設定で同じ型の意味を切り替えない。非原子Sharedを含むtaskはthread固定。thread版`sync box`は型/lock class単位の厳密な昇順＋try取得のみ（guard保持中は待たない）。同一thread版で受理した入れ子をthread版で自動受理しない。チャネルは送信→queue→受信の所有commitと、失敗・cancel・未受信終了のcleanup担当を一意にする。atomic RCは中身のthread安全を意味しない。残り: atomicのmemory ordering、順位付きblocking取得、公平性。整理: thread gateの`thread_registry_gc_roots_enabled`と`safepoint_and_poll`の`gc.safepoint()`は、GCなし方針の「workerが保持する所有・handleと終了責任」要件へ置き換える（名前だけで検査を省かない）。 |
| 構文と借用 | 決定: 宣言側read（省略）/mut/take、呼出し側は所有を失う所だけ`move`、借用結果は`from`宣言付きでlocal・分岐・read引数まで（field/global保存・escape closure・保持FFI・suspend越えは不可）。決定（2026-11-02）: 呼出し側の通常引数に`mut`は書かない。composite anchored results（fresh outerが借用childを保存）は切替で拒否し、所有childは`move`で渡す。`from a, b`は返値が由来してよい支持元の集合として意味を固定し、各returnとPHI入力が集合内にあることを検証、呼出し側は全候補を借用終了まで生存させる（片方だけ生きていても不可、実行時の選択追跡は後送り）。受理開始はselfhostが必要とするAPIより前で、それまでは名前付き拒否。`from`は維持する（2026-11-02）: 説明は「`from x`は結果が`x`の中身を借りて読むものなので、使っている間は`x`と参照先を有効に保つ宣言」、関数内ではコンパイラが借り元を追え関数の外へ返す時は呼出し側にも分かるよう宣言に借り元を書く、で統一し（`local x = me.field`と`get(): T from me { return me.field }`は同じ借用で後者だけが関係を公開する。Sharedの所有local化は既定どおりhandle複製の例外）、短縮構文（`peek`等の第二の構文）は増やさず補完・宣言テンプレート・修正候補で手間を減らす。借り元の宣言を本体の形や名前（`get`等）から自動発行しない。規則は全returnが対象: if/else各枝・loop内の途中return・`match`各arm・合流（PHI）の全incomingが宣言した支持元からの借用であること（fresh値や別の支持元の枝は拒否、null枝は支持payloadを持たず正常）、借用中に支持元のEnd/move/置換/mut重複がないことを全経路で検証。`return match me { … => payload }`は宣言に`from me`があれば受理する設計（各armのpayload projectionを元の`me`とvariant・slotへ結び付け、return/PHIの全incomingで確認）。`match move e`は全体消費で、取り出した所有payloadを値として返すことはできるが消費される`e`を借り元にして返さない。Shared payloadを所有localとして複製したhandleを終了させてその借用だけ返す形は不可（`from me`で返すなら元slotからの借用projectionを維持し、Sharedのpattern束縛を借用に切り替える場合は明示的な追加規則とし既定の+1を無言で消さない）。連鎖借用は三つに分ける: 生存中のaからの借用結果の再借用（支持関係と中間slotの有効性を合成。一律禁止にしない、nullableならnull確認も別）、`makeTree()`等の所有一時値を根にした借用（一時値をいつまで保持するかの別問題で後送り）、途中で所有Shared handleを返す連鎖（新しい一時ownerとして区別）。`: Shared<T>`は常に+1ではない（借用fieldから独立handleを返す時は+1、所有local全体やfresh handleは責任を移すだけ）。選択指針: 取得元と切り離して保持させるなら`: Shared<T>`、取得元に依存したまま読むなら`: Shared<T> from x`。診断は内部用語でなく「この値は誰が持っているか」で説明し、全返却経路を確認できた場合に限り`from me`の追加を修正候補にする（`remove`への変更は契約確認済みのAPIだけを別案として示し自動修正にしない、nullable借用結果のnull枝は書きすぎ扱いにしない、宣言と実際の借り元が不一致なら実際の借り元を示す）。現行MIRの支持関係は単一formalまたは特定のreceiver slotに限られ、任意のenum projectionや借用PHIは受理済みの証拠がないため、共通の支持関係・区間検査を広げる（getter形別producerや借用PHI拒否の単純解除は避ける）。descriptor/ABI拡張が要るなら物理追加として止めてユーザー判断。published/serializationの一般化が要り、新しいトップレベルMIR命令は必然ではないがdescriptor/ABI変更は必要。 |
| Dynamic carrierのread | 生存中の`SelfContainedDynamicCarrier`を通常callableの`read`入力へ渡す時、callerがcarrierを保持してcall終了後に一度だけEndし、calleeは保持・escape・returnしない。`take`/`share`や追加ownerを発行しない。このsource/ABI/lifecycle接続を明示して検証する。 |
| 型検査 | 決定（2026-11-02）: 宣言境界で型と所有契約を確定し、関数内は推論する。注釈必須: 名前付き関数/メソッドの引数（`take`は型必須、実装済み）、field、**名前付き関数/メソッドの戻り値（公私とも最初は全て明示。無値は`: void`。birthはUnit固定で不要）**。推論可: ローカル、closureの引数/結果（期待関数型から。所有修飾子・`from`・効果は推論しない）。`from`と所有需要は将来も推論しない。呼出し箇所から引数型を推測する方式は採らない。推論は1関数内で各bindingの型を一つに決める制約解決（分岐は同型/既存のnullable関係へ合流、無関係な型を自動union/Dynamicへ広げない、loopは入口とbackedgeで収束、再代入でbinding型は変わらない）。整数literalは期待型があれば範囲検査、なければi64。空Array/Mapは期待型か関数内の操作で決め、決まらなければ注釈要求。決まらない場合は名前付き拒否（型名不明/注釈が必要/型不一致/契約は既知だが未実装を分ける）。**nullable: 既定は非null、nullを許す時だけ`T?`（公開構文、内部は`Nullable(T)`、式位置のResult伝播`?`とは別構文）。`i64`/`Bool`/record/通常Box/`Shared`/`String`の裸の型は非nullable。`Node`・`Node?`・`Option<Node>`（null-free enum）・`Shared<Node>?`・`Shared<Node?>`・未初期化slotは全て別物で暗黙変換しない。欠損をnullで返すAPIは`get(...): Node? from me`の形（ヒット時はholderとslotに支持された借用）。`take x: Node?`はnullまたは所有Node一つを受領、`move x`は中身がnullでも元bindingを使用不可にするがnullにHome/Endを作らず、literal nullはそのまま渡せ`move null`は拒否、nullable owned fieldのEndはnullなら子Endなし、非null判定は型を絞るだけで所有/mut権限は増やさずslot更新で根拠が失効することも検査。既知のnullを非nullable型へ代入は静的型エラー、Dynamicから非nullableへのchecked castでnullはFault（nullableならnull受理）。既存APIの範囲外Fault等をnullへ変更しない。** 明示Dynamic: 綴りは`: Dynamic`、省略や推論失敗からDynamicを発行しない。静的型へ戻すのは対象型がコンパイル時に確定する明示checked cast（不一致はFault、成功しても所有/mut/Sharedは発行しない、Boxは最初は元carrier支持のread借用）。`take d: Dynamic`/`move d`/`Shared<Dynamic>`はcarrier移転契約の接続まで名前付き拒否。Dynamicはread-only/非suspend制約を解除しない。診断フェーズ順（別checker/別CFGは作らない）: 構文・宣言解決→型と呼出し契約の確定→所有と借用の検査（同一CFG/Place/Bindingで収束）→End計画・lower・published独立検証。型または受領契約が不明な間はmove漏れと断定せず、型が壊れた式に依存する所有診断だけ抑止。型表現: parserの型参照（Name/Apply+位置）→resolverの正規型（primitive、宣言IDのnominal、明示Dynamic、Shared、対応済みコンテナ、Nullable）へ構造化し、意味判断は一度解決した正規型から行い各consumerの再解釈は退役。型変数/Unknown/Errorは解析内部のみ。`Shared<T>`は組込み構築子・引数一つ・payload資格検査で導入し一般genericsは不要。既存の型保証matrix（`type_contracts/guarantee_matrix.rs`）へ統合し台帳は増やさない。構造化は3a中に既存consumerへ接続、意味の強制切替は3b（3aでは省略の意味まで変えない）。現行記録の「注釈省略＝dynamic Any」（`type-contract-status.md`）は3bで新規則へ変わる。closure（最初は同期・non-escaping）: 既定read借用、mut/所有の捕捉は明示、借用期間はclosure生成から利用終了まで、move捕捉は生成時に移転、non-escapingでも複数回呼べ所有値をmoveする本体は最大一回の呼出し契約、`update`は未ヒット0回/ヒット1回で未呼出し・後続引数Faultでも獲得済み捕捉の終了責任を処理、mut捕捉は対象Mapと重ならないPlaceへの排他が証明できる場合だけ、保存/返却/task受渡しのescapeは当面名前付き拒否。関数契約（引数型・read/mut/take・結果型・from・効果・noescape・呼出し回数上限）と、closure実体の捕捉情報（捕捉元Place・mode・支持元・所有捕捉のEnd義務）は分けて保持。enum/record: 通常の`match e`はeを消費せずUnique payloadはeに支持されたread借用（束縛型は確定型とvariant宣言から、Sharedを所有localへ取るならhandle複製）、所有payloadの分解は`match move e`の全体消費として別契約（接続まで名前付き拒否）、`record with`は元recordを変更せず継承Shared fieldはhandle複製・継承Unique fieldは暗黙copy/move不可でcopy契約がなければ拒否・置換対象fieldを元recordからEndしない、`(move r) with {…}`は後送り。aggregateの再帰的な所有分類（recordだからTrivialとはしない）はselfhostのASTを書く前に固定。戻り値推論を将来緩める最小条件: 本体が利用可能で公開/opaque ABI境界でない、自己再帰のない単一関数のSCC、呼び先の結果契約が確定済み、全Normal returnの型が一意、借用結果は対象外。型検査・nullable導入に新MIR命令は本質的に不要の見立てだが、nullableなscalar/record/Shared、型付き捕捉、aggregate消費が既存表現で足りるかは未確認（不足時は止めてユーザー判断、代替はその操作だけ未対応拒否）。 |
| 再代入と失敗 | RHSを評価して成功後にcommit、旧値のEnd。moveの消費時点、copy失敗、Fault時の責任を各辺で保持。決定（2026-11-02）: 呼び先・receiver確定後、実引数を左から右へ一度ずつ評価する。`move x`の評価で元bindingを読み取り・再move不可にするが、所有責任（元Homeの同一性とcleanup順位）は移転待ちとしてcallerが保持し、新しいownerや破棄順位を作らない。fresh値・Shared複製で得たHomeは評価フレームの一時Home（read/mut借用はHomeを増やさない）。引数評価中のFaultはcalleeに入場せず、獲得済み責任を既存cleanup graphで処理（新規一時Homeは逆順、移転待ちは元の順序。`x`は復活しない）。全実引数の準備後、call-entryで`take`実引数を一括移転し、以後はcalleeがNormal/Fault双方で責任を持つ（入場後Faultのcaller再Endは禁止）。cleanup中のFaultは最初のFaultを保持して残りを続行。`Shared(move x)`のFaultでShared runtimeがpayloadをEndしない（NormalでだけcellへMove、Faultはcallerの既存cleanup）。独立検証は元Home・引数ordinal・入場点・各CFG辺の所有状態をsource対応と実CFGで照合する。現birth引数移転（Normalのみ除去）は別契約で、通常`take`の証拠に使わない。`x = f(move x)`: RHS成功後に新しい値をcommitし、代入先に旧所有責任がまだ残っている場合に限り旧値をEndする。`move x`で旧bindingは使用不可、入場前Faultはcaller・入場後Faultはcalleeが旧責任を処理して`x`は復活せず、同じobjectが返っても旧`x`を再Endしない。loopは各backedgeに次世代の所有責任が一つ届くことを検証し、live/emptyが実際に合流する場合だけdrop flag（PHI発行元はPhiTxn）。 |
| 既存互換 | 決定（案A）: 通常Boxの意味（現行SharedV1）は切替まで変えず、切替はselfhostコードを書く前に一度だけ。切替では、所有受領が確定した転送点（通常関数・birth引数、container格納・所有field代入、owned値のlocal別名代入・再代入）へ既存owned bindingを渡す場合に明示`move`を要求する（fresh値と全体owned localのreturnは例外、宣言済みcopyは可）。診断は「move必要」「借用から所有を渡せない」「移転後使用」「未対応契約」を分け、型・受領契約が不明な間はmove漏れと断定しない。固定suiteの該当箇所は現行定義で再列挙して直し（旧「約6箇所/17行」は未確認の計画値）、期待出力・終了責任を基準に全件PASSを保つ。新旧間のsilent fallbackなし: 所有authorityを得られないordinary user Boxがraw側へ進む入口を引数lowering・物理発行前に拒否する（構造で閉じ、smokeは回帰証拠）。`with_source_aliases`は同一所有tokenの別SSA名参照の検証に必要で一括撤去しない。旧経路は切替commitで到達を禁じ、物理削除は到達不能・caller-zero確認後の後続commitで行う。anchored borrowed returnは`from`結果へ置換。 |
| MIRを使う比較 | 挙動不変の整理では旧MIRを診断資料として比較する。所有規則自体を変える移行では採用済みの新source契約と明示migration例を正解とし、旧MIRの挙動を規範にしない。 |

既存Mapの削除はBool結果である。上のremove/replace方針から、既存APIを所有値返却へ
自動変更しない。APIごとの取り出し契約は移行時に明記する。

## コンパイラの責務

```text
parse / resolve / type + source intent
  -> 同一のsemantic CFGでownership・borrowを解析
  -> scope / Normal / FaultごとのEnd計画
  -> sole lower / MIR
  -> 変換後MIR・ABIの独立検証
  -> 最適化 / LLVM
```

MirBuilderはキーワードだけでなく、正確なPlace・Binding・型・引数契約・評価順・
Normal/Fault辺を保持する。unknownをTrivialにする既定値で新モデルを通さない。
検査は実際に獲得された所有責任について、全経路で移転またはEndがちょうど一回と証明する。
End挿入前は未消費を破棄義務として計画し、挿入後の検証で実際の消費を再確認する。
maybe状態はCFGの必要性がある場合だけdrop flagにし、loop世代とFault前後を区別する。

既存[Ownership SSA](../../../src/mir/ownership_ssa/README.md)はPHI辺移転・二重消費・
終了時の未処理を検査する狭い土台。本番callerはなく、managed-call ABIと一般の借用検査は未対応。
既存[local planner](../../../src/mir/builder/resolved_lowering/ownership/README.md)も未接続で、
field/index/capture・一般Placeは対象外。借用の暗黙コピー処理は新規則へそのまま流用しない。
現在のMIR `CopyOwned`は同じobjectの独立owner tokenを作る命令で、source `copy`の複製ではない。
`DestroyOwned`の共通化には、型別終了処理・Fault・順序を残す。KeepAliveの生存支持や
ReleaseStrongの既存callerを名前変更だけで削除しない。
LLVM noaliasはmutの綴りから発行せず、実際の排他区間と到達可能なaliasを証明する。

## 移行タスクと完了条件

| 順 | 選択・作業 | 完了証拠と置換対象 |
| --- | --- | --- |
| 済 | 2026-10-30の決定をownership.md §0とこのADRへ記録 | docsのみ。コンパイル速度（Text End台帳の`SourceLoopCleanupPlanV2`統合、サイトの整数ID化）は別に続ける。 |
| 1 | `move x`の受理と移転後使用の拒否、Shared型の追加 | 既存アプリの入力・挙動は不変。同じ入力で既存SSAを本番consumerへ接続し、未接続passを作らない（consumerはpublished検証側。退役予定のVM interpreterに置かない）。`move`は新MIR命令を足さずbindingの付け替えで表す。SharedはopaqueなShared cell（カウント＋受理済みpayloadの所有・End義務）で上の操作契約をEXEまで一つ実装する（意味の模型検証だけでは完了にしない）。暫定cellの退役条件は、直接配置版が同じ契約を満たした時点。 |
| 2 | 一つのpayload型をsource→ownership検査→End→EXEまで直接メモリ配置で接続（fieldはruntime object-store呼出しでなくload/store） | 規模を上げたbinary-treesのbenchmarkで測定。Sharedは read/moveのみ（RC 0）、同一objectの複製・非最終解放、多数objectへの複製・回収、最終解放と子の連鎖Endを分けて測り、RC単体比較とEXE全体比較を分ける（現host-handle retainはArc複製＋handle割当を含む）。copy後の独立性、mut競合拒否、takeの消費時点、Normal/Faultの責任、宣言なしborrowed returnの拒否。copyの残る境界はこの型で紙上移行して確定。Dynamic carrierのreadはcallerがEnd責任を維持。置換した同一source/ABI専用edge・adapterは同seriesで退役。 |
| 3a | 切替の準備（通常Boxの既定意味は不変） | `take`宣言を、parser発行・parameter契約・引数ABI・callerの移転・calleeのHome・Normal/Fault End・独立検証・EXEまで一つの小例で閉じる（既存smokeに3ケース統合: Normal／後続引数Fault＝calleeへ入らずcallerがEnd／入場後Fault＝calleeがEndしcallerは再Endしない。exitや総数だけでなくEnd計数とMIR検証の拒否ケースで所有者と順序を示す）。`from`は支持元（`me`または一つの引数）・slot生存・支持元のEnd/move/置換とmut重複の拒否・保存/escape/suspend拒否まで（異なる支持元の合流・temporary延長・汎用mut借用返却・借用を保持するaggregateは名前付き拒否で後送り、旧composite形＝fresh outerが借用childを保存する形は切替で拒否）。container更新は`from`（read）と別の区間限定の排他mut貸出しとして、限定版`update(key, fn)`（捕捉なし/明示scalar入力）を先行: Map全体を区間排他、更新対象要素だけをmutで貸す、競合アクセス・構造変更・End・要素のmove/End・escape/suspendを禁止、Normal/Faultで貸出しを終える（Faultでも要素Endや完了更新の巻き戻しはしない）。既存Call・field操作・CFGで独立検証できるかを先に確かめ、published ABI/借用情報へ新たな物理表現（新MIR命令）が要ると分かった時点で停止して報告する（物理追加はユーザー判断）。`sync box`の一例（下記）も本段で接続する。この地点は安全な停止点（通常Boxの意味は旧のまま）。型参照の構造化（上の「型検査」）を既存consumerへ接続する段階を3aの続きに置く（意味は変えない）。`birth(take …)`は通常`take`と同じ入場契約で3aに接続し（入場前Fault=caller、入場後Fault=birthの構築cleanupが未格納引数と初期化済みfieldを重複なくEnd、fieldへ移した引数は再Endしない、callerに残る未公開outer領域のreclaimと子のEndを分離）、既存birthの移行と強制は3bで行う。3a中の既存birthの意味は変えない。 |
| 3b | 切替（selfhostコードより前） | 上の「既存互換」の条件。アプリ修正・compiler強制・仕様更新を一commitに整合させ、失敗時は整合した単位ごと準備地点へ戻す（混在着地・旧経路へのfallback・selfhost開始は不可）。切替前の確認は同一の最適化CLI・runtime・C shimで、固定suite全件PASS（SKIP/timeoutなし）、19形のprogram-owned関数とexit一致、既知失敗名集合一致。3a中の各着地は固定suite全件PASSと既知失敗名集合の一致を基準にし、19形の機械語確認は切替直前に一度まとめて行う（旧経路は切替で撤去し、撤去対象の旧同一性を長く維持しない）。切替の前に、構造整理を二つ入れる: (A) パラメータ契約型へ`PartialEq`を導入して手書きのフィールド比較を集約し`HomeDemandV1`の構築点を一つにする、(B) 同一検査を重複する`main0`ループlowerer三本の共通部を責務別に統合し、3bで触る巨大ファイルは責務で分割する（物理MIR変更なし、検証は弱めない）。非null既定への切替では、nullを使うfield/引数/戻り値（`ownership-typed-null-min`、binary-treesの子ノード等）を`T?`へ直し、注釈省略（旧Any）の箇所は、静的型を確定できる/nullable明示が要る/意図的にDynamic/所有・借用・ABI未対応の4つに分類して直す（一律の`: Dynamic`補完はしない。確認済み契約を入力にする使い捨ての候補diff補助は、AST書換えやbuild時補完にせず、cardにownerと退役条件を書き、対象移行が終わったseriesで削除する）。影響は固定suiteの再列挙に加えて、その宣言を使うlibrary/import/callbackまで既存の診断機構から測る。警告→拒否は3aで予告し3bで一度に切り替える（新規則の検査に失敗した箇所だけ旧Anyへ戻すのは禁止）。cohortの診断理由が意図して変わる場合は対象を説明して移行し、一括baseline更新で隠さない。container更新の対象アプリ: json-streamは`addEvent`のような更新操作、boxtorrentはMap内Chunkの短い排他更新（Mapを唯一の長期ownerとする）。Endの分岐/loop/再代入/Invokeへの拡張（終了順、一度だけの消費、必要時だけdrop flag、caller-zeroの専有HomeRelease等だけ削除）は切替までに閉じる。切替前に同一threadの`sync box`を一例だけsource→独立MIR検証→EXEまで接続（複数ownerからの更新・参照、再入拒否、Fault cleanupとpoison、区間内task非実行）し、証拠を既存smokeへ統合する。 |
| 後続 | selfhostは明示moveで書く（書き始める前に閉じる未決: `from a, b`の受理開始と借用を保持するaggregate/temporary延長/汎用mut借用返却の扱い、`take place_expr`・field move-out・consuming receiverの実装、`update`の捕捉あり形と既存Map APIの取り出し契約、`.copy()`の綴りとcopyの残り境界、Hako側`move`のparity、aggregateの再帰的な所有分類、`(move r) with`、escape closure。thread/atomic/weak/`wait`/poison回復/header配置は後でよい）。selfhostを書き始める前のゲート（2026-11-02）: 自然に書いた次の四つが補助変数・コピー・Shared化・関数分割なしで通ることを確かめる — ASTを`match`して子を借用で返す、借用した子から孫を読む、Mapの要素を短い区間だけ更新する、`node = fold(move node)`で木を書き換える。同じ小さなAST走査・AST変換・シンボル表更新をRustとHakoruneで書き比べ、文字数でなく完成までの修正回数・診断の分かりやすさ・同じ処理と終了責任での実行時間とメモリを比べる（速度は直接配置・静的呼出しを前提にした設計上の見込みで、実測までは主張しない）。回避のための書き直しが頻発したら`from a, b`・`match`の借用・`update`の捕捉の設計を見直す合図とする。Box/container/Sharedへ同じ契約を広げる | 型別identity・slot commit・read支持・Endを各既存ownerで閉じる。未選択機能を現行finite goalへ追加しない。 |
| thread導入 | Send/Share/affinityの実検査、送受信の所有commit、task終了責任 | その後にatomic Sharedとthread版`sync box`を開く。生存中のcellに非原子/原子アクセスを混在させず、表現を変える時はlayout/profileの版と再コンパイルを要求する。 |

数値のみのPointがTrivialなら、それだけでは所有移転やEndの証拠にならない。Trivial分類を
確定し、構文/value copyのcanaryと所有payloadを持つ代表入力を区別する。
record/boxの基本区別は[既存SSOT](../current/main/design/record-box-two-surface-one-substrate-ssot.md)と
[types.md](../../reference/language/types.md)で確定済み。準備ではその区別を再設計せず、copy対象の決定へ使う。
新規test/checker/cardは原則増やさず、追加が必要ならRULES §6に従う。
実装選択は既存parked checkpointに従う。現行アプリを未完のまま全面改修へ切り替えない。
4〜5万行削減・Rust相当の速度・期間は未計測であり、達成条件や見積もりには使わない。

## Recipe生成の整理

既存Recipe ownerでlogical keyの採番と元sourceとの対応を同時に構築し、semantic sealを共通化する。
BuilderはValueId/BasicBlockIdや意味を新規推測しない。別の手書きsource対応表を増やさない。
同一不変入力の意味検証は再利用し、lower後MIR/ABIの独立検証は保持する。
fixture専用の固定キー・定数制約とproductionの制約を区別してから置換する。
この整理は現在のアプリの共通ownerを直す機会に限定し、別の全面改修campaignを開始しない。

## 不採用と理由

暗黙copy/move、最終使用での省略、全Boxの暗黙RCは、局所的な意味とコストを曖昧にする。`share x`前置形、`.share()`、サイズで決める暗黙copy、本体からの借用結果推論も同じ理由で採用しない。
source `copy`を既存CopyOwnedへ単純対応させる案は、値の複製と共有を混同する。
毎stopのfixture/ADR作成や時間だけでの保留は採用せず、現行の1アプリ単位のRULESを維持する。
