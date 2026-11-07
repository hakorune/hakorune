# static の呼び出しを local で受ける関数の道(規則)

Status: 実装より前に書く規則の文書です。authority ではありません。branch `static-call-context`(main `244f0d0c01` から)。根拠: `static-call-result-routes-survey-2026-11-07.md` の §10 と §11、`direct_call_static_execution.rs` の先頭の comment。実装はこの文書の文を写します。文に無い分岐が要る時は、先に行を足して理由を書きます。

## 1. 穴

- 何も無い `main()` が、qualified の static の呼び出しの結果を local に受けると、宣言があっても `qualified-route-retired` で拒否される(`p1`)。同じ main に `local z = new Ctx()` を足す(`p2`)か `print(a)` を足す(`p4`)と通る。
- main の外でも同じで、`print(Main.g(5))` で呼ばれる `g` が `local a = Main.f(v)` を持つと、`g` の `Call` が生のまま残り、物理側が `instruction-unsupported` で拒否する(`p5`)。
- 原因: plain の道(`plain_main_integer_result_v1`)から外れた関数が、型つきの静的な実行の `context`(`direct_call_static_execution.rs:124-141`)に入らないと、結果を受ける道が 1 本も無い。

## 2. 規則(1 行 1 文。実装はこの写し)

| # | 規則 |
|---|---|
| C1 | **pure な component** とは、qualified または current-owner の static の呼び出しの結果を binding に受ける関数を 1 つも持たない owner の集まりである。 |
| C2 | pure な owner は、今の plain の道(`plain_main_integer_result_v1`、または結果の無い関数の `seed`)を通り、型つきの静的な実行の `context` に入らない。 |
| C3 | binding に static の呼び出しの結果を受ける owner は、plain の道を外れる(`ScanRequestV1::StaticSourceLocal`。`has_static_source_local_v1`)。この owner は、**ちょうど 1 つ、型つきの静的な実行の `context` に入る**。 |
| C4 | `context` に入る理由は、1 つの事実で書く: 「その owner が、型つきの静的な実行で運べる qualified/current-owner の static の呼び出しの結果を、binding に受ける」。運べる呼び出しとは、`join_static_execution_v1` の `additions` に加わる呼び出しである(callee が選ばれた static の method で、全 formal が `ExactTrivial(I64)` か formal が無く、結果が I64 か Bool)。 |
| C5 | 理由は `main` かどうかに依らない。main でも、main でない関数(`p5` の `g`)でも、同じ文で `context` に入る。 |
| C6 | `context` の閉包は変えない。caller と callee のどちらかが `context` なら、両方が入る(`direct_call_static_execution.rs:197-208`)。 |
| C7 | 今まで通っていた形は、今と同じ道を通り、MIR も同じである。MIR が変わるのは、今まで拒否だった owner が通る時だけである。前から `context` に入っていた owner の `context` の条件は変えない(追加だけで、削らない)。 |
| C8 | `main_root.rs:161-177` の `qualified-route-retired` と `model.rs:357-367` の `selected-header-missing` は、この規則の後でも届く入力が残る。残す物は報告に書く。届かなくなった拒否を消すかは、報告で提案するだけで、この series では消さない。 |

## 3. plain の道か `context` かを決める所(`244f0d0c01`)

調べ方: `plain_main_integer_result_v1`、`has_static_source_local_v1`、`context`、`has_routed_static_local_for_owner_v1`、`has_static_execution_for_owner_v1` を grep して読んだ。

| # | file:line | 何を決めるか |
|---|---|---|
| P1 | `ordinary_new_coseal_issue_walk_triggers.rs:177-195` `plain_main_integer_result_v1` | main が plain の整数の結果の道に入る条件。`!has_static_source_local_v1` を含む |
| P2 | `ordinary_new_coseal_issue_walk_triggers.rs:136-` `has_static_source_local_v1`、`ordinary_new_coseal_issue.rs:396` | static の source を初期化に持つ local があるか。`ScanRequestV1::StaticSourceLocal` の要求になる |
| P3 | `ordinary_new_coseal_issue_route.rs`(`OwnershipRouteV1::decide`) | 要求があれば walk の道(`VerifiedWalk`)。plain の seed を止める |
| X1 | `direct_call_static_execution.rs:131` | main の `requires_lifecycle()`(`main(args)` の形)で `context` に入る |
| X2 | `:135` | 呼び出し関係を持つ、または `requires_new_context_v1`(`new` を持つ。`ordinary_new_ledger.rs:9-11`) |
| X3 | `:136-141` | map、名前付き配列、`Call` 以外の lifecycle prefix(`print` など)を持つ |
| X4 | `:197-208` | 閉包: 加える辺の caller か callee が `context` なら両方入る |
| X5 | `:209-210`、`:216`、`:223` | `context` に入る owner を持つ辺だけを `seal_local` で封じる(3 か所の `context.contains` の判定) |
| G1 | `normal_callable_semantic_package/model.rs:286-300` | main の import view が、`static_local_route` か `typed_static_route` が真なら qualified の relation を作らない |
| G2 | `builder/normal_callable_semantic_loan_port/main_root.rs:161-177` | arity 0 の main に qualified の呼び出しがあり `static_local_route` が偽なら `qualified-route-retired` |

数: plain/`context` の入口を決める判定は **plain 側 3(P1〜P3)、`context` 側 5(X1〜X3、X4、X5 の 3 か所の filter を 1 と数える)**。実装後の数は報告に書く。

## Opus の確認(2026-11-08)

- 規則の commit `cb6866f9a7`(23:43)は、実装の commit `9c15e3b59c`(23:47)より前。実装は `additions` の owner を `context` に足す 1 文で、規則 C3〜C5 の写し。形を当てる分岐は無い。test の期待 1 つの変更(pure な main が `Values.find()` を受けると `context` に入る)は、規則 C1 が変えた契約そのもの。
- 最後の `hakorune` は `9c15e3b59c` の木の `target/quick/hakorune` と同じ物で、それより新しい source は無い。全 lib の赤 231 は、基準 `owned-object-fix` と名前が同じ。1,459 本の違い 7 本(終了コードが変わったのは 4 本)と、329 入力の差 0 は、log の数と合う。
- 指示書に無い 15 形を流した(`/mnt/workdisk/claude-scratch/static-call-context-audit/in/`)。出力の誤りは 0。前は拒否で今は通る形:
  - 2 回受けて足す(`13`)
  - 引数なしの static を 2 回受ける(`82`)
  - 別の static の中で受けて、その結果を main で受ける(`3`)
  - 別の static box の 2 引数の呼び出し(`5`)
  - 受けて使わない(`0`)
- 前から通っていた 8 形は、`--dump-mir` が前と同じだった(再帰で受ける形、受けた Bool の `if`、入れ直し、`if` の中での代入、static の中で Bool を受ける形を含む)。
- 拒否のまま: `loop` の中で受ける形(`direct-scalar/physical-binding`、前と同じ。S4 の範囲)。String の formal の static を受ける形(`qualified-route-retired`、C8 の残り)。
