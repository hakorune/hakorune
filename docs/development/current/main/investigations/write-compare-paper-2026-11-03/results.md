# ADR-002 selfhost 入口ゲートの紙上書き比べ 結果

作成日: 2026-11-03。対象規則: `docs/reference/language/ownership.md` §0、`docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md`、`EBNF.md`、`types.md`。

## 0. この結果の扱い

- **Hakorune のコードは一度もコンパイルも実行もしていない。** 受理・拒否の判定はすべて、書かれた規則を私が読んだ結果である。`share`、式の `take`、`mut` 宣言、`T?`、`from a, b`、`match move` にはパーサがない。
- 速度とメモリの主張はしない。「全ノードを作り直す」と書く箇所は構造の事実であり、性能の評価ではない。
- Rust は `rustc 1.89.0` でコンパイルと実行を確認した。9 ファイルとも最初の rustc 実行で通り、後から足した T2、T5 の追加分も一度で通った。
- ADR が比較項目に挙げる「完成までの修正回数」「診断の分かりやすさ」は、Hakorune 側を測れない。
- 両言語とも書き手は私一人である。自然な形の選び方には書き手の偏りがある。
- 読み直しで自分の誤りを 4 件見つけた。box 初期化のカンマ抜け、借用が生きている間の `r.leave()`、`me.span = span` の `copy` 抜け、T4b V3 の仮引数の隠蔽である。コンパイラなしでは、他にも見落としがある前提で読んでほしい。

### 記号

強制された書き換えの分類は次のとおり。

- **H**: 補助変数
- **C**: コピー
- **S**: Shared 化
- **F**: 関数分割
- **N**: 余分な nullable
- **R**: 拒否（別の書き方が必要）

## 1. ゲート判定

| ゲートの自然な形 | 判定 | 根拠 |
| --- | --- | --- |
| AST を `match` して子を借用で返す | **通過** | T1。H/C/S/F/N すべて 0。ただし `match me` の形は書けず、enum を受ける static helper になる |
| 借用した子から孫を読む | **必須 child は通過、nullable は不通過** | T2。必須は 2 呼出しの連鎖で補助変数なし。nullable は段ごとに H と null 検査が要る |
| Map の要素を短い区間だけ更新する | **更新だけなら通過、記録を伴うと決定規則では不通過** | T3。捕捉なしは H1、F1、検索 2 回。捕捉ありは自然に書けるが未決定で綴りもない |
| `node = fold(move node)` で木を書き換える | **nullable は通過、必須 child、`mut` 経由、Array は不通過** | T4a は通過。T4b と T4c は C か F が残る。T4a の Array 子は C が残る |

## 2. 計数

計数対象はコメントと文字列を除いたコード。6 種のマーカーは `move`, `take`, `share`, `from`, `mut`, 型の `?`。スクリプトは `count_markers.py`、節ごとの全表は `counts.md`。式位置の Result `?` は一度も使っていない。

| タスク | Hako LOC | move | take 宣言 | take 式 | share | from | mut | `?` | 6 種計 | 100 行あたり | copy | 綴りなし | Rust LOC |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| T1 | 66 | 0 | 0 | 0 | 0 | 4 | 0 | 4 | 8 | 12.1 | 0 | 0 | 38 |
| T2 | 91 | 2 | 0 | 0 | 0 | 5 | 0 | 3 | 10 | 11.0 | 0 | 0 | 60 |
| T3 | 51 | 0 | 0 | 0 | 0 | 0 | 2 | 0 | 2 | 3.9 | 0 | 3 | 42 |
| T4a | 93 | 9 | 4 | 4 | 0 | 0 | 1 | 4 | 22 | 23.7 | 1 | 0 | 71 |
| T4b | 203 | 38 | 12 | 2 | 0 | 1 | 0 | 13 | 66 | 32.5 | 10 | 0 | 94 |
| T4c | 69 | 1 | 2 | 1 | 0 | 0 | 6 | 3 | 13 | 18.8 | 0 | 0 | 57 |
| T5 | 99 | 2 | 2 | 3 | 0 | 4 | 0 | 7 | 18 | 18.2 | 0 | 3 | 84 |
| T6 | 81 | 8 | 4 | 1 | 0 | 0 | 0 | 1 | 14 | 17.3 | 3 | 0 | 30 |
| T7 | 58 | 2 | 1 | 0 | 6 | 1 | 0 | 0 | 10 | 17.2 | 0 | 0 | 51 |
| **計** | **811** | **62** | **25** | **11** | **6** | **15** | **9** | **35** | **163** | **20.1** | **14** | **6** | **527** |

「綴りなし」は `/*mut me*/` などのプレースホルダで、決まった綴りがない構文の数。6 種の計数には入れていない。

変種別（Hakorune と、対応する Rust の節）:

| 変種 | Hako LOC | 6 種 | 100 行あたり | copy | Rust LOC | Rust の所有トークン | 100 行あたり |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| T3 捕捉あり | 19 | 1 | 5.3 | 0 | 42（全体） | 2 | 4.8 |
| T3 捕捉なし | 20 | 1 | 5.0 | 0 | 同上 | 同上 | 同上 |
| T4a nullable | 43 | 15 | 34.9 | 0 | 36 | 14 | 38.9 |
| T4a Array 子 | 26 | 2 | 7.7 | 1 | 7 | 2 | 28.6 |
| T4b V1 候補 | 42 | 18 | 42.9 | 2 | 52 | 19 | 36.5 |
| T4b V2 作り直し | 44 | 4 | 9.1 | 5 | 対応なし | | |
| T4b V3 `match move` | 47 | 37 | 78.7 | 0 | 24 | 16 | 66.7 |
| T5 V1 `from a, b` | 16 | 2 | 12.5 | 0 | 9 | 12 | 133 |
| T5 V2 所有者の鎖 | 41 | 14 | 34.1 | 0 | 37 | 21 | 56.8 |
| T5 V3 所有者 2 つ | 18 | 0 | 0 | 0 | 対応なし | | |
| T6 V1a box 必須 | 19 | 2 | 10.5 | 2 | 30（全体） | 5 | 16.7 |
| T6 V1b box nullable | 16 | 4 | 25.0 | 1 | 同上 | 同上 | 同上 |
| T6 V2 record（B 案） | 14 | 4 | 28.6 | 0 | 同上 | 同上 | 同上 |
| T6 V3 一変種 enum | 13 | 4 | 30.8 | 0 | 同上 | 同上 | 同上 |

Rust のトークンは `&`, `&mut`, `Box`, `*`, `Option<`, `.take()`, `.clone()`, `mem::replace`, lifetime を正規表現で数えた概数で、種類も数え方も Hakorune とは異なる。比較は目安である。

## 3. タスク別の記録

AST の形について。ADR:50 が「子を所有するデータ（AST ノード…）は Box」と定めるため、ノードは Box とした。和型を作れるのは enum だけなので、`enum Expr { Neg(NegNode) ... }` の二段構成である（T4b V3 だけ enum に直接子を持たせた）。box 初期化 `new X { f: move y }`（EBNF.md:589-596）を主に使ったので、`birth(take x)` より印は少ない。`birth(take x)` なら所有フィールド 1 つあたり `take` と `move` が 1 つずつ増える。

### T1 `match` して子を借用で返す

強制された書き換え（所有規則によるものは 0）:

- **R**: ADR:55 の `return match me { … => payload }` は書けない。`enum_decl` にメソッドがなく（EBNF.md:1032-1036）、Box には variant がない。enum を受ける static helper で書いた。関数分割ではなく置き場所の問題である。
- 所有以外の制約: `_ => null` は網羅性を満たさない（EBNF.md:386）ので、`else_of` は `guard let` で書いた。

未解決: G7（`T?` の文法行）、G9（enum の所有 payload）、G10。

### T2 孫を読む

- 必須 child: `me.entry_ref().body_ref()` が補助変数なしで書ける（ADR:55 連鎖借用の第一類）。
- **H**: `first_grandchild` の `c`。`T?` の値を `T` の引数へ渡せず、連鎖の構文がない。`?` は Result 専用（grammar-contract.md:56、LANGUAGE_REFERENCE_2025.md:885-886）。Rust は `first_child(first_child(e)?)` の 1 行。
- **H**: `grandchild_value` の `g`。`null` と enum の variant を同じ `match` で扱う規則がなく、先に null を除く形で書いた。Rust は `Some(Expr::Num(v))` と入れ子のパターンで書ける。
- **H**: `value_of_fresh` の `t`。G16 による安全側の読み。

未解決: G16、G18。

### T3 Map 更新と記録

- **R**: `get` の結果へのフィールド書込み。`get` は read 借用で（ADR:58）、mut 借用を返す形は後送り（ADR:99）。
- **R**: 取り出して戻す形。Map 要素は `take` の名前付き拒否（ownership.md:166-167）で、Map 削除は Bool 結果（ADR:63）。
- 捕捉あり V1: 自然に書ける。ただし、ADR:58 が条件付きで認める `mut` 捕捉は 3a の範囲外（ADR:99）で、未決（ADR:101）。
- **H**（V1）: `has` の事前検索。`update` の未ヒットの結果が書かれていない（G6）。
- **H, F**（V2、決定規則のみ）: `was_const`、読み→更新→記録への分割、検索 2 回。判定を更新前に計算できる場合しかこの形にならない（判定が更新後の値に依存する場合は推論であり、書いていない）。

Rust は `get_mut` 一つで済み、補助変数も分割もない（フィールドの分離借用）。

未解決: G1, G2, G6。

### T4a nullable の `take` 書き換え

- `node.lhs = me.fold_opt(take node.lhs)` は Rust の `node.lhs = fold_opt(node.lhs.take())` と 1 対 1 で書ける。
- `root = Folder.fold(move root)` は loop 内でも規則どおり（ownership.md:183-186、ADR:59）。
- **F**: `fold_opt`。`T?` と `T` の差による。Rust にも同じ形があるので所有規則固有ではない。
- 印: `Lit => move node`。`return` の省略は arm 末尾に広がらない（ADR:28）。Rust には対応する印がない。
- 戻し書き `node.lhs = move inner`: 形を見る前に取り出すので必要。Rust も同じ。
- **R, C, F**（Array 子）: `take call.args[i]` は名前付き拒否（ownership.md:166-167）で、所有権を返す pop や remove の契約もない（ownership.md:426-427）。手書きの `clone_term`（8 行）と作り直しが強制された。Rust は `mem::take` と `into_iter().map(fold).collect()` の 1 行。

未解決: G5, G8, G11, G12, G14, G20。

### T4b 必須 child の書き換え

3 種の書き換えで評価した。A: 親を残して子を変える。B: `-(-x) => x`。C: `if(定数) a else b => a`。B と C は親が消える。

| | V1 候補 | V2 決定規則のみ | V3 `match move` |
| --- | --- | --- | --- |
| 必要な未接続の構文 | 候補、`move` の文法拡張、`match move` | なし | `match move` |
| A | コピーなし | 全コピー | コピーなし |
| B | **C**（`clone_expr`） | 全コピー | コピーなし |
| C | **C**（then）、`take`（else） | 全コピー | コピーなし |
| 強制された分割 | `clone_expr`, `clone_opt` | 同左 | `fold_if`, `pick` |
| ADR:50 の Box ノードの指針 | 従う | 従う | 外れる |

V1 の所見:

- **`match move` は候補と独立に必要である。** 所有する `Expr` から所有する `NegNode` を得る手段が他にない。
- 文法: `move` は IDENT しか取らない（EBNF.md:278）。「新しい語は増やさない」としても文法行の変更は要る。
- 親が消える書き換えは「同じ field へ再格納」（ADR:56）を満たさず、**候補では解けない**。then（必須）はコピー、else（nullable）は `take` という非対称が 1 関数の中に出る（`fold_if`）。
- 実コードに出していないが規則から決まる **H**: `fold(move node.body, node.kind)` は拒否（ADR:56）なので、先に `copy` して局所に取る必要がある。Rust はこの形を受理する（`rust_walls/ok_sibling_read.rs`）。候補は兄弟 field の読みに関して Rust より厳しい。
- 実コードに出していないが規則から決まる **F**: `node.a.b = fold(move node.a.b)` は「1 段の直接 field」（ADR:56）に反するので、階層ごとに関数が要る。
- Rust との一致: `node.fold(move node.body)` の拒否と fini 親の拒否は Rust と同じ（E0505、E0509）。`rust_walls/` で確認した。

V2 の所見:

- 決定規則だけで 3 種とも書ける。`root = FolderRebuild.fold(root)` と書け、`move` は局所の 2 個だけ。代わりに全ノードを作り直し、`copy` が 5 + 3 個と手書きの `clone_expr` が要る。
- 成り立つのはノードの中身がすべて複製できる場合だけである。copy 契約のない所有値を持つノードは作り直せない（ADR:51）。

V3 の所見:

- 3 種ともコピーなしで書けた唯一の形である。Rust の V3 と構造がほぼ同じ。
- **F**: レコード変種のブロック arm が現行の範囲外（EBNF.md:1047、types.md:310）で `fold_if` が強制される。ブロック式の中で `return` もできない（block-expressions-and-map-literals.md:83-85）。
- **作り直しの強制**: `match move x` で `x` が消えるので、変更なしの arm も field ごとに組み立て直す。Rust の `other => E::Neg(Box::new(other))` に当たる形がない。パターンの IDENT 単独は unit 変種の意味（EBNF.md:343）。`fold_neg` と `fold_if` で計 3 arm。
- **実際に踏んだ誤り**: レコードパターンに rename がなく（EBNF.md:344）、初稿の `fold_if(take cond, take then_e, take else_e)` は最後の arm の束縛が仮引数を隠した。隠された所有値は scope 終了で黙って End される。文法か診断で扱う必要がある。

未解決: G3, G4, G9, G13, G19。

### T4c `mut` 経由の書き換え

- nullable: `s.hint = me.fold_opt(take s.hint)` で通る（ownership.md:162-164）。
- **R ×3**（必須 child）: `move s.init` は文法外、候補は `mut` 親を除外（ADR:56）、`take` は `T` を拒否（ownership.md:165）。
- **C**: 残るのは読み→新規の作り直しか、`mut` 引数での中身の更新だけ。後者は identity も variant も変えられない。
- Rust も同じ壁に当たる（E0507、`rust_walls/wall_mut_parent.rs`）。ただし `mem::replace` とダミーで抜けられる。Hakorune には `T` の旧値を受け取る手段がなく、ダミーを使っても書けない。
- **R**（enum 子）: arm 束縛は常に read 借用（ADR:58）で、`mut` を継承する規則がない。Rust は `match e { Expr::Neg(b) => visit(b) }` と書ける。**enum を挟むと `mut` で降りられないのは Rust より狭い。**

未解決: G2, G3, G13。

### T5 スコープ連鎖

- V1 `from a, b`: 1 行で書ける。受理は未開始の名前付き拒否（ownership.md:187-190）で、現行のヘッダは 1 支持元のみ（ownership.md:660-664）。
- V2 所有者の鎖（`from me`、決定規則のみ）: `lookup` は書ける。
  - **R**: 子は親を所有するしかない。借用の保存は拒否（ownership.md:127）。Rust の第一候補 `parent: Option<&Scope>` は書けない。
  - **N**: `cur: Scope?`。`mut` receiver の下で必須 `cur` を新スコープへ移せない（ownership.md:165、ADR:56）。Rust は `Default` の空スコープをダミーにして `mem::take` で書いた。
  - null 検査がアクセスのたびに増える。
  - `current_mut()` のような切り出しは書けない（ADR:99）。パスを毎回全部書く。
- V3 所有者 2 つ（決定規則のみ）:
  - **R**: 局所の合流 `if s == null { s = outer.get(name) }` は、異なる支持元の合流として拒否（ADR:99）。
  - **F**: 続きの処理を両方の枝に重複して書く。別解は ID を返す形（ADR:50）。

未解決: G1, G13, G8（`name` の Map キー格納）。

### T6 値の束

- V1a（box、必須 field）: **R**。束をほどけない。`move p.expr` は文法外かつ部分 move（ADR:29）、`take` は `T` を拒否（ownership.md:165）。候補は同じ field へ戻さないので効かない。残るのは **C** だけ。読むだけなら問題ない。
- V1b（box、nullable）: **N**。生成後は決して null でない field を `T?` にし、到達しない `panic` の枝が付く。ゲートが禁じる「全部 nullable にして合格」に近い。
- V2（record が box を持つ、B 案）: B は「持ってよい」許可を足すだけである。取り出しには `match move` が要り、さらに record のパターンが文法にない（EBNF.md:344 は known-enum のみ）。
- V3（一変種 enum）: **V2 と本体が同じで、新しい許可が要らない。** 現行の文面では enum の payload は Unique を持てる（ADR:58）し、A 案の制限は record だけ（ADR:50）。enum 側の分類が未決なので、この抜け道が残るかは G9 次第である。B 案が V3 より得をするのは `p.span` と直接読めることだけである。

未解決: G4, G9。

### T7 Shared 設定

- 強制された書き換えなし（H/C/F/N すべて 0）。S は本タスクの前提である。
- `share` は保持者 1 つにつき 1 個で、Rust の `Rc::clone` と同数（6 対 5。差は `share new`）。最後の保持者は `move` でも `share` でも合法で、余分な handle は診断されない。
- 所有 local の裸渡しの拒否は規則どおり（ownership.md:144-145）。

未解決: G15。

## 4. 規則の穴と矛盾

書けないものから順に並べた。

| ID | 内容 | 根拠 | 出たタスク |
| --- | --- | --- | --- |
| G1 | **`mut` receiver の宣言の綴りがない。** rule 2 は「宣言から決まる」と書くが、method_decl に置き場所がなく、パーサにも行がない。types.md の `Counter.inc` は印なしで `me` を変更しており、ADR の「本体から入力契約を推論しない」と矛盾する | ownership.md:95-98、ADR:24、ADR:55、EBNF.md:928、types.md:117-119 | T3, T5 |
| G2 | `mut` 引数、closure の `mut` 引数、`mut` 捕捉の綴りがない。`param` は `IDENT (':' TYPE_REF)?` のみ。closure は所有修飾子を推論しないので綴りが要る | EBNF.md:105, 944、ADR:58 | T3, T4c |
| G3 | **arm 束縛が常に read で、`mut` を継承しない。** enum の payload を `mut` 経由で変更できず、`mut` で enum を越えて降りられない | ADR:58 | T4c |
| G4 | **Box の必須 field から所有を取り出す手段が一切ない。** rule 2 の例 `(move image).into_bytes()` も、`bytes` が所有する必須 field なら本体を書けない。消費 receiver の宣言の綴りもない。「AST ノードは Box」の指針と部分 move 禁止を合わせると、必須の Box 子は親から出られない | ownership.md:98、ownership.md:165、ADR:29、ADR:50、EBNF.md:98-100 | T4b, T6 |
| G5 | Array 要素の所有の取り出し（pop、remove）に契約がない。文の並びの書き換えが作り直しになる | ownership.md:166-167, 426-427、ADR:29 | T4a |
| G6 | `update` に署名がない。未ヒットの結果、closure の結果、「明示 scalar 入力」の渡し方が書かれていない。`Map<K,V>` は設計文書にあるが GEN-002 の一覧にない | ADR:58, 99、EBNF.md:771-777 | T3 |
| G7 | 決定済みの構文に文法行がない: 型の `?`（TYPE_REF）、`from` 節、`copy`、`match move` の意味、`(move x).m()`、`move` の field 被演算子。文書も `static box`（EBNF.md:891）を持たず `function_decl`（EBNF.md:130）が残る | EBNF.md:278, 945, 98-100、ownership.md:660-664 | 全部 |
| G8 | 暗黙 copy の範囲が String、unit enum、数値だけの record で未定。rule 7 は数値と Bool のみ。`copy` の文法も、copy 契約の宣言の綴りもない。record は ADR の例に従い `copy` と書いた（計 14 箇所。うち 13 が record の `Span`、1 が unit enum） | ownership.md:155-158、ADR:33-35, 50-51 | T4a, T4b, T5, T6 |
| G9 | enum が Unique を持てるかの分類が未決なのに、T4b V3 と T6 V3 はこれに依存する。record は A 案で禁じられ enum は許される非対称があり、一変種 enum が B 案と同じになる | ADR:50, 58, 101 | T1, T4b, T6 |
| G10 | ADR の `return match me` は書けない形を前提にしている（enum にメソッドなし） | ADR:55、EBNF.md:1032-1036 | T1 |
| G11 | `T?` を絞った値を `move` したときの型が書かれていない（`take x: T` へ渡せるか）。絞り込みが効く形（`!= null`、`guard`、`&&`）の一覧もない | ADR:58 | T4a, T5, T6 |
| G12 | 借用の「生きている」の定義がない。last-use で読まないと T4a の `simplify_add`、T4b V1 の `inner` が通らない。`match node.op` の scrutinee 借用が束縛なしの arm の `move node` と競合するかも未定 | ownership.md:125-126、variables-and-scope.md:20-21 | T4a, T4b |
| G13 | `mut` 引数への代入と借用 local の再代入に意味がない。再束縛か、書き込みか、拒否かが未定で、§0 が閉じていない | ownership.md:1001-1012、variables-and-scope.md:16-19 | T4c, T5 |
| G14 | `Array<T>.get` の結果が `T?` か `T` か定まらない。範囲外を null に変えない規則と「nullable handle を貸す」が並ぶ | ownership.md:404-406、ADR:58 | T4a |
| G15 | `Shared<T>` を plain な read 引数 `T` に渡せるか書かれていない。receiver には payload の read があるのに、引数の形がない | ownership.md:119, 136-137, 139-140 | T7 |
| G16 | `match` が scrutinee の一時値を arm の間生かすか書かれていない。`match me.next_token()` が補助変数なしで書けるかが決まる | ADR:55、ADR:58、ownership.md:1091-1093 | T2 |
| G17 | `T` field の確定初期化の規則がない。`new IfNode { ... }` が必須 field を省いたとき未定。`EmptyField` は読むと Fault になり、非 null の型と食い違う | ownership.md:933-938、EBNF.md:605、lifecycle.md:160-164 | T1, T4b |
| G18 | rule 4 の「支持元は callee で死ぬ local ではない」は、ADR の連鎖借用の合成で読まないと T2 の `first_child(c)` を拒否してしまう | ownership.md:126-127、ADR:55 | T2 |
| G19 | `match move` の詳細が未定: 変更なしの arm の書き方（全体束縛がない）、`_` arm の扱い、レコードパターンの rename、隠された所有値の診断 | ADR:58、EBNF.md:340-347, 386, 1046 | T4b |
| G20 | `take` 引数が rule 1 の「whole owned local」に入るか明記がない（`release` は local と parameter を分けて書く） | ownership.md:91-94, 786-787、ADR:43 | T4a |

## 5. 未決事項への推奨

### (a) `node.body = fold(move node.body)`: 必須にしない。`match move` を先に接続する

- 候補だけでは足りない。V1 は `match move` がなければ書けない。
- 3 種の書き換えのうち、コピーなしで解けるのは親を残す A だけ。B と C はコピーが残る。
- T4c は候補の文面自身が除外している。
- `match move`（V3）は 3 種すべてをコピーなしで解いた。
- 条件: V3 は G9（enum の所有 payload）に依存し、ADR:50 の Box ノードの指針から外れる。`FnDecl.body: Block` のように和型でない必須 Box 子が多ければ、候補の価値が残る。
- 比較すべき別案（私の推論で、物理面は未検証）: ADR:56 の「取り出した field を除く残存 field と親領域を終了する」は、再格納しない B、C や T6 の取り出しでも同じ処理に見える。候補を採るなら、再格納の条件を外した「単独所有で fini なしの親からの field 取り出し」と比べるべきである。こちらは新しい語が要らず、rule 2 の `into_bytes` も書けるようになる。

### (b) B 案（record が Box を持つ）: 不要

- T6 V2 と V3 は本体が同じで、V3 は新しい許可が要らない。
- B でも `match move` は要る。加えて record パターンという文法追加が要る。
- `match move` が来なければ、B があっても束はほどけない。
- 利点は `p.span` と直接読めることだけである。

### (c) `update` の `mut` 捕捉: 必要

- 記録を伴う T3 は、決定規則のみだと H1、F1、検索 2 回になる。ゲートの判定を通すには捕捉が要る。
- 最小の範囲: Map と重ならない Place への捕捉だけ。ADR:58 がすでに認めている条件である。
- 同時に決めるもの: 捕捉の綴り、closure 引数の `mut`、未ヒットの結果。未ヒットの結果は捕捉の有無によらず必要である（T3 V1 の事前 `has` が消える）。
- 捕捉を遅らせるなら、代わりに closure が scalar を返せる形を決めれば、V2 の分割の多くは減る（推論）。

### (d) `from a, b`: selfhost 開始の条件にしない

- 本当のスコープ連鎖は 1 支持元の `from me` で書けた（T5 V2）。
- 要るのは独立した所有者 2 つの場合だけで、回避は 1 関数の続きの重複か、ADR が定める ID 形。
- 受理するなら局所の合流（ADR:99 の名前付き拒否）と同じ検査なので同時に開く。
- ADR:101 の「書き始める前に閉じる未決」は、「受理開始は実際の API が要求するまで遅らせる」として閉じられる。

### (e) `replace`: 不要

- ADR:56 のとおり `fold` は解けない。T4c で書けるようになるのはダミー形だけで、ゲートが禁じている。
- 非ダミーで効くのは T5 の `cur: Scope?` を必須にできる 1 箇所だけで、回避の負担は `T?` 1 つと null 検査である。
- selfhost で「決して null でない `T?` の状態 field」が頻発したら再検討する。

### 優先度の総括

書き比べで最も効いたのは 5 つの未決のどれでもない。selfhost の前に閉じる必要があると判断したものは次のとおり。

1. G1, G2 の綴り（mut receiver、`mut` 引数）
2. `match move` と G9（enum の所有 payload）、G19
3. G3（`match` 経由の `mut`）
4. G5（Array 要素の取り出し）
5. G8（暗黙 copy の範囲）

## 6. マーカー密度

- 全体: 811 行で 6 種 163 個（100 行あたり 20.1）。内訳は `move` 62、`take` 36、`?` 35、`from` 15、`mut` 9、`share` 6。別に `copy` 14。
- **読むコードは静か**: T1 12.1、T2 11.0、T3 3.9。Rust は呼出し側にも `&` が要るので、読むコードでは Hakorune の方がトークンが少ない（T1: 8 対 15）。
- **`share` はうるさくなかった**: 6 個で、保持者ごとに 1 個。Rust の `Rc::clone` と同じ情報量である。
- **`take` はうるさくなかった**: 式の 11 箇所は Rust の `.take()` と 1 対 1 に近い。
- **`move` は変種で差が大きい**: V2 は 2 個、V1 は 10 個、V3 は 24 個。V3（100 行あたり 78.7、Rust は 66.7）が最も密で、24 個のうち 7 個は変更なしの arm を組み立て直すためだけのもの、3 個は `match move`。全体束縛のパターンがあれば前者は減る。
- **ADR:28 による印**: `return match` の arm 末尾の `move` は、外から `return` と見えていても必要である。
- **`copy` は規則次第**: 14 個のうち 13 が record の `Span`、1 が unit enum。G8 の決定で全部消える可能性も、残る可能性もある。

## 7. 検証していないこと

- Hakorune のコードは一度もコンパイルしていない。構文も意味も私の読みである。
- パーサ、型検査、所有検査、MIR、EXE はどれも動かしていない。現行コードは ownership.md §0 の多くを実装していない（ownership.md:16-20）。
- 実装上の可否は調べていない。例として、借用 PHI と任意の enum projection は ADR:55 が「受理済みの証拠がない」と書き、連鎖借用の実アクセスは現在の `CURRENT_STATE` で名前付き拒否中である。`match move` と候補の物理契約も未検証である。
- 速度、メモリ、完成までの修正回数、診断の分かりやすさは測っていない。
- 対象はこの 9 タスクだけで、selfhost の実コードの出現頻度は調べていない。「頻発する」という表現はすべて推測である。
- 規則の読みをメンテナにも別の読み手にも確認していない。文書間の参照は grep と通読で確かめたが、見落としはありうる。

## 8. ファイル

所在: このフォルダ(2026-11-06 に作業用の一時フォルダから移した。説明は `README.md`)

- `t1_match_borrow`, `t2_chain_borrow`, `t3_map_update`, `t4a_take_rewrite`, `t4b_required_child`, `t4c_mut_parent`, `t5_scope_lookup`, `t6_bundle`, `t7_shared_config`（各 `.hako` と `.rs`）
- `rust_walls/`: E0507, E0509, E0505 の拒否と、兄弟 field 読みの受理の確認
- `count_markers.py`, `counts.md`
