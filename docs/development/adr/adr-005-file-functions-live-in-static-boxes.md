# ADR-005: file の関数は static box の中だけに書く

Status: 採用(2026-11-26、ユーザー決定)。(3) の診断を実装。
Date: 2026-11-26
Scope: file に書く関数の置き場所、関数の名前の決まり、呼び方、REPL の関数の定義、移る順。

この ADR は、box の外に書く関数(`function f(...) { ... }` と `static function f(...) { ... }`)を、file のモードの言語から外す。static box に書ける物(static method だけ)は、[ADR-006](adr-006-static-box-holds-only-static-methods.md) が決める。Box の method の名前の決まり(`LANG-BOX-METHOD-NAMESPACE-D0`、[callable-contracts.md](../../reference/language/callable-contracts.md#box-method-namespace-and-collisions))と、`using` の別名の契約([using.md](../../reference/language/using.md))は、そのまま使う。

## なぜ決めたか

- 今の言語には、関数の名前の決まりが 2 つある。Box の method は「箱の中で名前が 1 つ」。box の外の関数(FreeStatic)は `name/arity` が名前で、`f/1` と `f/2` を別に持てる。
- そのため、box の外の関数を箱に移すだけで、名前の決まりが変わる。box の外では並べられた `f(x)` と `f(x, y)` が、箱の中ではぶつかる。
- 関数の名前がどこに属するかを、1 つの決まりで説明したい。関数は全部どこかの箱に属し、名前は箱ごとに 1 つ、とすれば説明が 1 つで済む。
- REPL の気軽さは、境目で保つ。試す間は session が名前を持つ。残す時は、自分で箱に名前を付けて file に書く。
- 相談の最初の答え(2026-11-26)は、box の外の関数を残す方を勧めた。理由は、今使っている所があること、Rust から `.hako` への変換器がこの形を出すこと、`f/1` と `f/2` を分ける契約があること。メンテナは、その後の相談を経て、この ADR の形を採った。答えが挙げた移す費用と直す文書は、下の「移る順」と「移す相手」に入れた。

## 決めた事(ユーザー決定 2026-11-26)

1. **file のモードでは、box の外の関数は compile error。**
   - 相手は、file の top-level の `function f(...) { ... }`、`static function f(...) { ... }`、`static function Box.f(...) { ... }`。top-level の `gate ... { } else { }` の中も同じ。
   - エラーは「static box の中に method として書いて」と案内する。
   - 名前は `decl/function-outside-box`（[設計図](../current/main/design/mirbuilder-blueprint.md) §23.1 A08）。method 本体の入れ子と macro が展開後に作る file 直下の関数にも適用する。
2. **関数の名前は、箱ごとに 1 つ。** 今の Box の method の決まりのまま。同じ名前で引数の数だけが違う関数(`f(x)` と `f(x, y)`)は、1 つの箱に持てない。名前を変えるか、箱を分ける。FreeStatic の `name/arity` の名前の空間は、言語から無くなる。
3. **呼び方は 2 つ。** 同じ箱の中は `me.f(x)`。外からは `Box名.f(x)`。`using ... as S` の別名は、その箱を指す(`using.md` の今の契約のまま)。
   - static box の中の `me` は、その箱自身を指す。Main の `main` でも同じ。Resolve が呼び先を決め、`Box名.f(x)` と同じ経路で、実引数を左から一度ずつ評価する。引数の数が宣言と違うと `call/arity-mismatch` で拒否する。
   - file で束縛の無い裸の `f(x)` は、Resolve が呼び出しの位置で `call/target-missing` にする。同じ static box に `f` が宣言されている時だけ `me.f(x)` を案内する。
   - 次の物は、この ADR では変えない: 言語が決めた特別な形(`panic(message)`)、関数の値の呼び出し(`Call`)、裸の `error`・`now` の拒否。
4. **REPL では、箱なしの `function` を将来の機能として認める。** 下の「REPL」の節。

## 書けなくなる物(before / after)

box の外の `function`:

```hako
// before: 書けなくなる
function add(a, b) { return a + b }
static box Main {
    main() { return add(40, 2) }
}

// after: static box の method にする
static box Main {
    main() { return me.add(40, 2) }
    add(a, b) { return a + b }
}
```

`static function`(`Box.f` の綴りも):

```hako
// before: 書けなくなる
static function Factory.create() { return new Item() }

// after
static box Factory {
    create() { return new Item() }
}
// 呼ぶ側: Factory.create()
```

同じ名前で、引数の数だけが違う関数:

```hako
// before: 書けなくなる(今は f/1 と f/2 として並ぶ)
function area(w) { return w * w }
function area(w, h) { return w * h }

// after: 名前を変える
static box Shapes {
    square_area(w) { return w * w }
    rect_area(w, h) { return w * h }
}
// または、箱を分ける: Square.area(w) と Rect.area(w, h)
```

箱の名前の無い `f(x)`(別の箱から、box の外の関数を呼ぶ):

```hako
// before: 書けなくなる
function emit(value) { print(value); return 0 }
static box ConsoleStd {
    log(x) { return emit(x) }
}

// after: 同じ箱の中なら me.f(x)、外からなら Box名.f(x)
static box ConsoleStd {
    log(x) { return me.emit(x) }
    emit(value) { print(value); return 0 }
}
```

## REPL

- REPL では、`function twice(x) { ... }` を箱なしで、そのまま書ける。session の宣言として登録する。次の入力からも `twice(4)` で呼べる。`.reset` で消える。
- 説明は「REPL を開くと、自分用の見えない箱が 1 つある」。session の関数は、その箱の method と同じ扱いになる。
- 名前は 1 つ、の決まりも同じ。`twice(x)` と `twice(x, y)` を、両方は持てない。
- 試す間は、session が名前を持つ。残す時は、自分で箱に名前を付けて file に書く。
- **これは将来の機能。** 今の REPL には、入力をまたぐ関数の定義は無い。REPL の作業も、今は止めてある([repl.md](../../reference/language/repl.md) の「Implementation Status」)。

## 移す相手(2026-11-26 に数えた)

数え方は、token を読む script。parser は通していない。`a57e7586e7` と `da050b4509` で同じ数だった。

- top-level の `function` がある所:
  - `.hako` の 69 file(`lang/` 1、`apps/`(`apps/tests/` 以外)44、`apps/tests/` 24、`tools/` 0)。
  - src の Rust の test の文字列 101 本(49 の `.rs`)。
- 呼ぶ所:
  - Main の中: 1 file(`apps/tests/corpus/tl/f1.hako`)と、Rust の 7 本。
  - Main でない static box: 1 file(`apps/lib/boxes/console_std.hako`)。
  - box の外(top-level の `main` から): 5 file と、Rust の 1 本。
  - instance box: 0。
  - 残りは、呼ぶ所が無い。宣言そのものを試す test、変換の期待出力、入口の `main` である。
- top-level の `static function`: `apps/tests/corpus/tl/g0.hako`・`g4.hako` の 2 file と、Rust の test の 17 本(12 の `.rs`)。
- 範囲の外: `examples/` に 28 file、`tests/` に 1 file。
- 名指しの所:
  - 変換器の 2 か所: `apps/rust-subset-to-hako/converter_core.hako:226` と `apps/rust-subset-to-hako/convert.py:244`。どちらも `function 名前(...)` を出す。`apps/(tests 以外)` の 44 file のうち 42 は、この変換器の期待出力(`apps/rust-subset-to-hako/examples/*_expected.hako`)。
  - `apps/lib/boxes/console_std.hako:4` の `__console_std_emit`。`apps/selfhost-runtime/selfhost_prelude.hako:7` が読む。
  - `lang/src/vm/mini_vm_if_branch.hako:38` の、runner 向けの `function main(args)`。
  - Rust の本体: macro の子の runner が `function main(args)` を作る(`src/macro/macro_box_ny/child.rs:223`)。

## 移る順

- (1) この文書の回。ADR と、言語の文書を直す。実装は変えない。
- (2) 使っている `.hako` と、変換器の 2 か所を、static box の形に書き直す。出力と終了値が変わらない事を確かめる。
- (3) file の top-level の `function` と `static function` を、compile error にする。
- (4) builder の CallableSingleLoop と、box の外の関数だけが使う部分を消す。FreeStatic の索引には static box の method も載るので、索引そのものには使い手が残る。

掃除と selfhost の順(ADR-003 の「作業の順」)の中に置く。この ADR のためだけに、順を前へ出さない。

## まだ決めていない事

- REPL で、同じ名前の関数を定義し直す時の扱い(置き換えるか、拒否するか)。引数の数が違う定義し直しも含む。
- REPL から file へ移す時の案内(診断の文、session の関数を箱の形で書き出す手助けを作るか)。
- `examples/` の 28 file を移すか、消すか。
- 変換器の箱の名前は決定済み（LB-2d）。関数を出力する module ごとに一つの static box を作る。箱名は `RustSubset_` に module ID（ASCII 英数字はそのまま、`_` は `_u`、`:` は `_c`）を続ける。同じ箱でメソッド名が重なった場合、引数の数にかかわらず変換を拒否する。
- `console_std.hako` の helper の移し方。箱を足すと、`using` の別名の結び付けが変わりうる(公開の static box が 1 つの時と、複数の時で、今の決まりが違う)。
- runner 向けの `function main(args)`(`mini_vm_if_branch.hako`、macro の子の runner)の代わりの入口。macro の top-level の `static function MacroBoxSpec.expand` と `NYASH_MACRO_TOPLEVEL_ALLOW` は、[ADR-006](adr-006-static-box-holds-only-static-methods.md) で決まった(`static box MacroBoxSpec { expand(json, ctx) { ... } }` に揃え、特例は退役する)。子の runner の正式な入口は、まだ決めていない。
- src の Rust の test の文字列 101 本を、(2) と (3) のどちらで直すか。
- 内部の名前 `FreeStatic`(code と MIR の文書で、static box の method の呼び出しにも使う)の呼び分け。(4) の回で、消す範囲と一緒に決める。
- この ADR が直さなかった文書: `docs/reference/language/LANGUAGE_REFERENCE_2025.md`(過去の記録。`static function Factory.create()` の例がある)、`docs/guides/` の macro の頁、`docs/reference/mir/` の FreeStatic の記述。

## 根拠

- 相談と答えは、公開の repo の外にある(2026-11-26)。要点は、上の「なぜ決めたか」と「まだ決めていない事」に入れた。
- 確かめた事: 今の Rust の parser は、file の文の `function` を `FreeFunction` として、`static function` を `FreeStaticFunction` として受け付ける(`src/parser/items/functions.rs:12-62`、`src/parser/items/static_items.rs:13-18`・`:102-174`、`src/parser/initial_callable_program_source/issue.rs:293-311`)。
- 確かめていない事: 書き直した後の出力と終了値。(2) の回で確かめる。
