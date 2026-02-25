# JoinIR — 関数正規化 IR（関数と継続だけで制御を表現する層）

目的
- Hakorune の制御構造（`if` / `loop` / `break` / `continue` / `return`）を、**関数呼び出し＋継続（continuation）だけに正規化する IR 層**として設計する。
- これにより:
  - φ ノード = 関数の引数
  - merge ブロック = join 関数
  - ループ = 再帰関数（loop_step）＋ exit 継続（k_exit）
  - break / continue = 適切な関数呼び出し
  という形に落とし込み、LoopForm v2 / Exit PHI / BodyLocal / ExitLiveness の負担を**構造的に軽くする**。

前提・方針
- LoopForm / ControlForm は「構造の前段」として残す（削除しない）。
- JoinIR は「PHI/SSA の実装負担を肩代わりする層」として導入し、ヘッダ/exit φ や BodyLocal の扱いを **関数の引数と継続** に吸収していく。
- PHI 専用の箱（HeaderPhiBuilder / ExitPhiBuilder / BodyLocalPhiBuilder など）は、最終的には JoinIR 降ろしの補助に縮退させることを目標にする。

### JoinIR ロワーが「やらないこと」チェックリスト（暴走防止用）

JoinIR への変換はあくまで「LoopForm で正規化された形」を前提にした **薄い汎用ロワー** に寄せる。  
このため、以下は **JoinIR ロワーでは絶対にやらない**こととして明示しておく。

- 条件式の **中身** を解析しない（`i < n` か `flag && x != 0` かを理解しない）
  - 見るのは「header ブロックの succ が 2 本あって、LoopForm が body/exit を教えてくれるか」だけ。
- 多重ヘッダ・ネストループを自力で扱おうとしない
  - LoopForm 側で「単一 header / 単一 latch のループ」として正規化できないものは、JoinIR 対象外（フォールバック）。
- LoopForm/VarClass で判定できない BodyLocal/ExitLiveness を、JoinIR 側で推測しない
  - pinned/carrier/exit 値は LoopVarClassBox / LoopExitLivenessBox からだけ受け取り、独自解析はしない。
- 各ループ用に「特別な lowering 分岐」を増やさない
  - `skip_ws` / `trim` / `stageb_*` / `stage1_using_resolver` 向けの per-loop lowering は Phase 27.x の実験用足場であり、最終的には Case A/B/D 向けの汎用ロワーに吸収する。

> 要するに: JoinIR は「LoopForm＋変数分類の結果」だけを入力にして、  
> ループ/if の **構造（続行 or exit の2択＋持ち回り変数）** を関数呼び出しに写す箱だよ。  
> 構文パターンや条件式そのものを全網羅で理解しに行くのは、この層の責務から外す。

Phase 28 メモ: generic_case_a ロワーは LoopForm / LoopVarClassBox / LoopExitLivenessBox から実データを読み、minimal_skip_ws の JoinIR を組み立てるステップに進化中。Case A/B/D を汎用ロワーに畳み込む足場として扱う。

位置づけ
- 変換パイプラインにおける位置:

```text
AST  →  MIR（+LoopForm v2）  →  JoinIR  →  VM / LLVM
```

- AST / MIR では従来どおり Nyash 構文（if / loop 等）を扱い、
  JoinIR 以降では **関数と値（Box＋Primitive）だけ**を見る。

---

## 1. JoinIR のコアアイデア

### 1-1. 「ループ = 関数を何回も呼ぶこと」

通常の while/loop は:

```hako
loop(i < n) {
  if i >= n { break }
  i = i + 1
}
return i
```

JoinIR 的に見ると:

```text
fn main(k_exit) {
    loop_step(0, k_exit)
}

fn loop_step(i, k_exit) {
    if i >= n {
        // break
        k_exit(i)
    } else {
        // continue
        loop_step(i + 1, k_exit)
    }
}
```

- ループ本体 = `loop_step(i, k_exit)` という関数。
- `i` は LoopCarried（carrier）変数 → 関数の引数で表現。
- break = exit 継続 `k_exit` の呼び出し。
- continue = `loop_step` をもう一度呼ぶこと。

### 1-2. 「if の merge = join 関数」

ソース:

```hako
if cond {
  x = 1
} else {
  x = 2
}
print(x)
```

JoinIR 的には:

```text
fn main(k_exit) {
    if cond {
        then_branch(k_exit)
    } else {
        else_branch(k_exit)
    }
}

fn then_branch(k_exit) {
    x = 1
    join_after_if(x, k_exit)
}

fn else_branch(k_exit) {
    x = 2
    join_after_if(x, k_exit)
}

fn join_after_if(x, k_exit) {
    print(x)
    k_exit(0)
}
```

- φ ノード = `join_after_if` の引数 `x`。
- merge ブロック = `join_after_if` 関数。
- 「どのブランチから来たか」の情報は、**関数呼び出し**に吸収される。

---

## 2. JoinIR の型イメージ

※ Phase 26-H 時点では「設計のみ」を置いておき、実装は最小ケースから。

### 2-1. 関数と継続

```rust
/// JoinIR 関数ID（MIR 関数とは別 ID でもよい）
struct JoinFuncId(u32);

/// 継続（join / ループ step / exit continuation）を識別するID
struct JoinContId(u32);

/// JoinIR 関数
struct JoinFunction {
    id: JoinFuncId,
    params: Vec<VarId>,        // 引数（φ に相当）
    body: Vec<JoinInst>,       // 命令列
    exit_cont: Option<JoinContId>, // 呼び出し元に返す継続（ルートは None）
}
```

### 2-2. 命令セット（最小）

```rust
enum JoinInst {
    /// 通常の関数呼び出し: f(args..., k_next)
    Call {
        func: JoinFuncId,
        args: Vec<VarId>,
        k_next: JoinContId,
    },

    /// 継続呼び出し（join / exit 継続など）
    Jump {
        cont: JoinContId,
        args: Vec<VarId>,
    },

    /// ルート関数 or 上位への戻り
    Ret {
        value: Option<VarId>,
    },

    /// それ以外の演算は、現行 MIR の算術/比較/boxcall を再利用
    Compute(MirLikeInst),
}
```

### 2-3. 継続の分類

- join 継続:
  - if の merge や、ループ body 終了後の「次の場所」を表す。
- loop_step:
  - ループ 1ステップ分の処理。
- exit 継続:
  - ループを抜けた後の処理（`k_exit`）。
- これらはすべて「関数 or 継続 ID」と「引数」で表現される。

---

## 3. MIR/LoopForm から JoinIR への変換ルール（v0 草案）

ここでは Phase 26-H で扱う最小のルールだけを書く。

### 3-1. ループ（LoopForm v2）→ loop_step + k_exit

前提: LoopForm v2 が提供する情報
- preheader / header / body / latch / exit / continue_merge / break ブロック集合。
- LoopVarClassBox による分類（Pinned / Carrier / BodyLocalExit / BodyLocalInternal）。

変換方針（概略）:

1. LoopCarried 変数の集合 `C = {v1, v2, ...}` を求める。
2. 「ループを抜けた後で使う変数」の集合 `E` を ExitLiveness から得る（長期的には MirScanExitLiveness）。
3. JoinIR 上で:
   - `fn loop_step(C, k_exit)` を新設。
   - ループ前の値から `loop_step(C0, k_exit0)` を呼び出す。
   - body 内の `break` は `k_exit(E)` に変換。
   - body 内の `continue` は `loop_step(C_next, k_exit)` に変換。

今の LoopForm / PhiBuilderBox がやっている仕事（header φ / exit φ 生成）は、
最終的には「`C` と `E` を決める補助」に寄せていくイメージになる。

### 3-2. if → join_after_if

前提:
- control_flow / if_form.rs で IfForm（cond / then / else / merge）が取れる。

変換方針:

1. merge ブロックで φ が必要な変数集合 `M = {x1, x2, ...}` を求める。
2. JoinIR 上で:
   - `fn join_after_if(M, k_exit)` を新設。
   - then ブランチの末尾に `join_after_if(x1_then, x2_then, ..., k_exit)` を挿入。
   - else ブランチの末尾に `join_after_if(x1_else, x2_else, ..., k_exit)` を挿入。
3. merge ブロックの本体は `join_after_if` に移す。

これにより、φ ノードは `join_after_if` の引数に吸収される。

---

## 4. フェーズ計画（Phase 26-H 〜 27.x の流れ）

### Phase 26-H（現在フェーズ）— 設計＋ミニ実験

- スコープ:
  - JoinIR の設計ドキュメント（このファイル＋ phase-26-H/README）。
  - `src/mir/join_ir.rs` に型だけ入れる（変換ロジックは最小）。
  - 1 ケース限定の変換実験:
    - 例: `apps/tests/joinir_min_loop.hako` → MIR → JoinIR → JoinIR のダンプを Rust テストで確認。
- ゴール:
  - 「関数正規化 IR（JoinIR）が箱理論／LoopForm v2 と矛盾しない」ことを確認する。
  - 大規模リプレースに進む価値があるか、感触を掴む。

### Phase 27.x（仮）— 段階的な採用案（まだ構想段階）

この段階はまだ「構想メモ」として置いておく。

- 27.1: LoopForm v2 → JoinIR の変換を部分的に実装
  - まずは「break なしの loop + if」だけを対象にする。
  - Exit φ / header φ の一部を JoinIR の引数に置き換える。
- 27.2: break / continue / BodyLocalExit を JoinIR で扱う
  - BodyLocalPhiBuilder / LoopExitLiveness の「決定」を JoinIR 引数設計に寄せる。
  - ExitLiveness は JoinIR 関数の use/def から算出可能かを検証する。
- 27.3: SSA/PHI との役割調整
  - LoopForm v2 / PhiBuilderBox は「JoinIR 生成補助」の位置付けに縮退。
  - MirVerifier は JoinIR ベースのチェック（又は MIR/JoinIR 並列）に切り替え検討。

※これら 27.x フェーズは、26-F / 26-G で現行ラインの根治が一段落してから進める前提。

---

## 5. 他ドキュメントとの関係

- `docs/private/roadmap2/phases/phase-26-H/README.md`
  - Phase 26-H のスコープ（設計＋ミニ実験）、他フェーズ（25.1 / 26-F / 26-G）との関係を定義。
- `docs/development/architecture/loops/loopform_ssot.md`
  - LoopForm v2 / Exit PHI / 4箱構成（LoopVarClassBox / LoopExitLivenessBox / BodyLocalPhiBuilder / PhiInvariantsBox）の設計ノート。
  - 将来、JoinIR を導入するときは「LoopForm v2 → JoinIR 変換の前段」としてこの SSOT を活かす。

このファイルは、JoinIR を「全部関数な言語」のコアとして扱うための設計メモだよ。  
実際の導入は小さな実験（Phase 26-H）から始めて、問題なければ 27.x 以降で段階的に進める想定にしておく。

## 6. φ と merge の対応表（抜粋）

| 構文/概念 | JoinIR での表現 | φ/合流の扱い |
|-----------|-----------------|--------------|
| if/merge  | join 関数呼び出し | join 関数の引数が φ 相当 |

---

## 7. 汎用 LoopForm→JoinIR ロワー設計（Case A/B ベース案）

Phase 28 以降は、`skip_ws` や `trim` のような「ループごとの lowering」を増やすのではなく、  
LoopForm v2 と変数分類箱を入力にした **汎用ロワー** で Case A/B 型ループをまとめて JoinIR に落とすのがゴールになる。

ここでは、その v1 として **単一ヘッダの Case A/B ループ** を対象にした設計をまとめておく。

### 7-1. 入力と前提条件

汎用ロワーが見るのは、次の 4つだけに限定する。

- `LoopForm` / `ControlForm`（構造）
  - `preheader`, `header`, `body`, `latch`, `exit`, `continue_merge` の各ブロック ID
  - `header` ブロックの succ がちょうど 2 本であること
    - LoopForm が「どちらが body 側 / exit 側か」を与えてくれていること
- `LoopVarClassBox`（変数分類）
  - pinned: ループ中で不変な変数集合
  - carriers: ループごとに更新される LoopCarried 変数集合
  - body-local: ループ内部だけで完結する変数集合（基本は JoinIR では引数にしない）
- `LoopExitLivenessBox`（Exit 後で必要な変数）
  - exit ブロック以降で実際に参照される変数集合 `E`
- 条件式の形そのもの（`i < n` か `1 == 1` か等）は **見ない**
  - 汎用ロワーが使うのは「header の succ が body/exit に分かれている」という事実だけ。

前提条件として、v1 では次のようなループだけを対象にする。

- LoopForm が「単一 header / 単一 latch の loop」として構築できていること。
- header の succ が 2 本で、ControlForm が `(cond → {body, exit})` を一意に教えてくれること。
- break/continue が LoopForm の設計どおり `exit` / `latch` に正規化されていること。

これを満たさないループ（多重ヘッダ・ネスト・例外的な jump 等）は **JoinIR 対象外（フォールバック）** とする。

### 7-2. 出力の形（ループごとの JoinFunction セット）

Case A/B 型の単一ヘッダ loop について、汎用ロワーは原則として次の 2 関数を生成する。

- `loop_step(pinned..., carriers..., k_exit)`  
  - 引数:
    - pinned: LoopVarClassBox の pinned 変数（ループ外で初期化され、ループ中不変）
    - carriers: LoopVarClassBox の carriers（ループをまたいで値を持ち回る）
    - `k_exit`: ループを抜けたあとの処理を表す継続
  - 本体:
    - header 条件の判定（`header` ブロック）
    - body/latch の処理（`body`/`latch` ブロックから拾った Compute/BoxCall 等）
    - break/continue の分岐を、それぞれ `k_exit(exit_args...)` / `loop_step(next_carriers..., k_exit)` に変換したもの
- `k_exit` 相当の関数（もしくは呼び出し先関数の entry）
  - ExitLiveness が教えてくれる `E`（exit 後で必要な変数）を引数とし、
    exit ブロック以降の処理をそのまま MIR→JoinIR で表現したもの。

これにより:

- header φ: `LoopHeaderShape` / carriers 引数として `loop_step` に吸収される。
- exit φ: `LoopExitShape` / exit 引数として `k_exit` に吸収される。
- LoopCarried 変数は常に `loop_step` の引数経由で再帰されるので、PHI ノードは JoinIR 側では不要になる。

### 7-3. 汎用ロワー v1 のアルゴリズム（Case A/B）

Case A/B を対象にした最初の汎用ロワーは、次のような流れになる。

1. **対象ループかどうかのチェック**
   - LoopForm が単一 header / 単一 latch を持つことを確認。
   - header の succ が 2 本で、ControlForm が body/exit を特定していることを確認。
   - 条件: LoopForm の invariants を満たすループだけを対象にし、それ以外は `None` でフォールバック。

2. **変数セットの決定**
   - pinned 集合 `P` と carriers 集合 `C` を LoopVarClassBox から取得。
   - ExitLiveness から exit 後で必要な変数集合 `E` を取得。
   - `LoopHeaderShape` と `LoopExitShape` を構築しておき、`loop_step` / `k_exit` の引数順を固定。

3. **`loop_step` 関数の生成**
   - JoinFunction を新規に作成し、`params = P ∪ C`（＋必要なら `k_exit`）とする。
   - header ブロックの Compare/BinOp から「続行 or exit」の判定命令を MirLikeInst として移植。
   - body/latch ブロックの Compute / BoxCall を順に MirLikeInst へ写し、carrier 更新を `C_next` として集約。
   - break:
     - LoopForm/ControlForm が break 経路としてマークしたブロックからは、`Jump { cont: k_exit, args: exit_values }` を生成。
   - continue:
     - latch への backedge 経路からは、`Call { func: loop_step, args: pinned..., carriers_next..., k_next: None/dst=None }` を生成。

4. **エントリ関数からの呼び出し**
   - 元の MIR 関数の entry から、LoopForm が示す preheader までの処理を MirLikeInst として保持。
   - preheader の最後で `loop_step(pinned_init..., carriers_init..., k_exit)` 呼び出しを挿入。

5. **Exit 継続の構築**
   - exit ブロック以降の MIR 命令を、`k_exit` 相当の JoinFunction か、呼び出し先関数の entry に写す。
   - `E` の各変数を引数として受け取り、そのまま下流の処理に流す。

### 7-4. 既存 per-loop lowering との関係

Phase 27.x で実装した以下の lowering は、この汎用ロワーの「見本」として扱う。

- `skip_ws` 系: minimal_ssa_skip_ws（Case B: `loop(1 == 1)` + break）
- `FuncScanner.trim_minimal`: Case D の簡易版（`loop(e > b)` + continue+break）
- `FuncScanner.append_defs_minimal`: Case A（配列走査）
- `Stage1UsingResolver minimal`: Case A/B の混合（Region+next_i 形）
- StageB minimal ループ: Case A の defs/body 抽出

Phase 28 では:

- まず minimal_ssa_skip_ws だけを対象に、`generic_case_a` のような汎用ロワー v1 を実装し、  
  既存の手書き JoinIR と同じ構造が得られることを確認する（実装済み: `lower_case_a_loop_to_joinir_for_minimal_skip_ws`）。
- そのあとで `trim_minimal` / `append_defs_minimal` / Stage‑1 minimal / StageB minimal に順に適用し、  
  per-loop lowering を「汎用ロワーを呼ぶ薄いラッパー」に置き換えていく。
- 最終的には per-loop lowering ファイルは削減され、LoopForm＋変数分類箱から JoinIR へ落とす汎用ロワーが SSOT になることを目指す。

このセクションは「汎用ロワー設計のターゲット像」として置いておき、  
実装は Phase 28-midterm のタスク（generic lowering v1）で少しずつ進めていく。
| loop      | step 再帰 + k_exit 継続 | LoopCarried/Exit の値を引数で渡す |
| break     | k_exit 呼び出し | φ 不要（引数で値を渡す） |
| continue  | step 呼び出し   | φ 不要（引数で値を渡す） |
| return    | 継続または ret  | そのまま値を返す |

## 7. 26-H で増やす実装箱 / 概念ラベル

- 実装として増やす（このフェーズで手を動かす）
  - `join_ir.rs`: `JoinFunction/JoinBlock/JoinInst` の最小定義とダンプ
  - LoopForm→JoinIR のミニ変換関数（1 ケース限定で OK）
  - 実験トグル（例: `NYASH_JOINIR_EXPERIMENT=1`）で JoinIR をダンプするフック

- 概念ラベルのみ（27.x 以降で拡張を検討）
  - MirQueryBox のような MIR ビュー層（reads/writes/succs を trait 化）
  - LoopFnLoweringBox / JoinIRBox の分割（必要になったら分ける）
  - JoinIR 上での最適化や VM/LLVM 統合

このフェーズでは「箱を増やしすぎない」ことを意識し、型定義＋最小変換＋ダンプ確認だけに留める。
