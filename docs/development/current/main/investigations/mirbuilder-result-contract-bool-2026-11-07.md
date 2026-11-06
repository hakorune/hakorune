# 結果の契約の土台と、明示の Bool の結果(規則)

Status: 実装より前に書く規則の文書です。authority ではありません。根拠は `result-contract-consult-2026-11-06.md` の「決定」と回答の 3(規則の表)です。型の規則の持ち主は `mirbuilder-value-class-owner-s3a-2026-11-06.md` の §5 で、この文書はその結果の面を足します。
ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

## 1. 範囲

- 今回つなぐ結果の型は Unit、I64、Bool です。String、Nominal、`Nullable(T)` は契約の型の軸に場所だけを用意し、受理しません(今ある名前つきの拒否のまま)。
- 返し方は値で返す物だけです。読み借用(`from`)は、今ある借用の宣言の証拠のままにします。
- 出所(宣言か、移行用の証明か)は診断のためだけに持ちます。lowerer や後段の選択には使いません。

## 2. 規則(1 行 1 文。実装はこの写し)

| # | 構文・場面 | 規則 |
|---|---|---|
| R1 | `f(): T` | 署名の `T` を先に確定する。本体の検証より前に、呼び出す側(再帰を含む)が `T` を読める。 |
| R2 | `return e`(宣言あり) | 本体の Normal の出口は全部 `T` に照らす。`e` の種類が `T` と違えば、名前つきの拒否にする。推論で救わない。 |
| R3 | 整数 literal、`+ - *` など | I64。 |
| R4 | 比較(`== != < <= > >=`)、論理(`&& ||`)、`!` | 正常に完了した時の結果は Bool。Fault の有無とは別に持つ(Bool の型の証明は、Fault が無い事の証明ではない)。 |
| R5 | bool literal | Bool。 |
| R6 | local / formal | local は初期値の種類(S3a の型の規則)、formal は宣言の型。 |
| R7 | call | 結果の種類は callee の契約の行。static、instance、`me` で規則を分けない。呼ぶ側で callee の本体を分類し直さない。 |
| R8 | `if` の条件 | 条件の式の種類が Bool であること。条件に使われた事から、call の結果を Bool にしない。 |
| R9 | 宣言の無い関数 | 今の移行用の入口(ExactBool の証明など)のまま。ただし出口は R1 と同じ契約の行で、後段は契約の行だけを読む。宣言の有無で別の道を選ばない。 |
| R10 | 結果の契約の行 | 完成した行に Unknown、`UnresolvedScalar`、未対応を入れない。 |
| R11 | 物理の形 | header は契約の行を参照し、物理の形は契約からの投影とする。`ExactTrivialScalarAbiV1` の Bool は結果の投影であり、引数の型には使わない。 |

## 3. 完了の条件の各形が決まる行

| 形 | 行 |
|---|---|
| `t(x: i64): Bool { return x > 1 }` | R1、R2、R4 |
| `local b = x > 1  return b` | R2、R6 |
| `u(x): Bool { return Main.t(x) }` | R2、R7 |
| `c.t(4)`、`me.t(4)` | R7 |
| `even(n): Bool` の再帰 | R1(署名を本体より前に確定) |
| 宣言の無い Bool の結果 | R9 |
| `t(x: i64): Bool { return x + 1 }`、`t(): i64 { return 1 > 0 }` の拒否 | R2、R3、R4 |
| 公開の結果の種類の改変の拒否 | 生成後の独立検証(`check_call_result`)。契約の行には依存しない |

## 4. 規則が決めない事(この series の範囲外)

- Bool の結果の使い先(`print`、`local` への格納、`if` の条件)の物理への接続。これらは結果の契約ではなく、使い先の式の規則(R6、R8)と物理の接続の側にある。接続が無い使い先は名前つきの拒否のままで、実測の一覧を報告に書く。
- String、Nominal、`Nullable(T)` の結果。
- 公開 MIR、ABI、descriptor の追加。Bool は既存の `Bool` / `ordinary_bool` だけを使う。足す必要が見つかった部分は止めて報告する。
