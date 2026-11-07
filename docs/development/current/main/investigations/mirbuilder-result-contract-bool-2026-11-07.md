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
| R12 | claim | 静的な scalar の呼び出しの claim は、callee の契約の行から結果の種類（I64 か Bool）を持つ。claim、route、flow、install は 1 つで、種類を引数に持つ。Bool 専用の family、型、述語を足さない。 |
| R13 | 引数の要求 | 本体の証明が、ある formal を exact-i64 の operand として使うなら、その formal は「要求」になる。要求は scalar の証明が持つ物で、I64 という種類が持つ物ではない。比較と論理の演算子は、operand の要求を合わせて残し、種類だけを Bool にする。 |
| R14 | 呼ぶ側の引数 | 呼ぶ側は、要求された位置の実引数が exact-i64 であることを、今と同じ道で示す。結果の種類で、この確認を変えない。 |
| R15 | 使い先 | 呼び出しの結果は、種類の 1 つの表で使い先に入る。算術と i64 の lane は I64 だけ、`if` の条件は Bool だけ（R8）、`local` はどちらも（S3a）、`return` は呼ぶ側の関数自身の契約（R2）に照らす。どの使い先も callee の本体を読み直さない。 |
| R16 | 終端 | 比較か論理の式を返す出口は、その式の種類を決める表と同じ表で、Bool の scalar の relation になる。出口の種類の表は `TerminalRelationV1::scalar_class()` の 1 つのまま。 |
| R17 | 再帰 | header と import view は、契約の行から結果の種類を読む。I64 であることを前提にしない。 |
| R18 | instance と `me` | instance と `me` の呼び出しも、R12 と同じ「種類を持つ claim」を使う（R7）。lexical の route に別の変更が要る時も、同じ種類の欄を使う。 |

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

## 5. 実測の結果(2026-11-07)

- 宣言の `: Bool` は、宣言の無い同じ関数と同じ道を通り、同じ名前つきの結果になる(R9)。通る形は Text 入力の static / instance の Bool(`json-stream-aggregator` の `boolField(line, key): Bool` と `ingestLine(line): Bool`、`local b = P.t(line)  if b`)。
- 宣言の無い Bool の結果が今も繋がっていない形は、宣言があっても同じ名前で拒否される。比較を返す本体(`return x > 1`、`return s == "a"`)、`if f()` の条件に直接置く形、Bool を返す関数を返す形(`return P.t(s)`)、instance の `c.t(..)`、再帰、`print(f())`、exact-i64 入力の static の Bool(`local b = Main.t(4)`)。最後の形に Bool の経路が無いのは、静的な scalar の呼び出しの claim が `ExactI64` の結果だけを前提にしているためで、結果の契約の行(R7)ではなく、使い先の経路の側(§4)にある。
- 本体が宣言と違う scalar を返す形は `PhysicalHeader DeclaredResultDisagrees` で拒否される(R2)。
