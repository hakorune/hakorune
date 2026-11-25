# ADR-006: static box は、static method を名前でまとめるだけの箱

Status: 採用(2026-11-26、ユーザー決定)。実装は、まだ無い。
Date: 2026-11-26
Scope: static box に書ける物、static box の `new`、static box の header の `from`、`flow` の宣言、macro の書き方、移る順。

[ADR-005](adr-005-file-functions-live-in-static-boxes.md) で、file の関数は static box の中だけになった。この ADR は、その static box に書ける物を決める。static box は、static method を名前でまとめるだけの箱である。状態を持ちたい時は、ふつうの `box` を作る。

## なぜ決めたか

- EBNF に、今の parser が読む static box の形を写した時(2026-11-26、[EBNF.md](../../reference/language/EBNF.md) の「Static box declaration」)、6 つのずれが見つかった。
  1. `new`: static box の `new` は拒否されるが、無い名前の `new` と同じ拒否だった。
  2. field・`init { }`・`static { }`: parser は読むが、compile は内部の誤りで止まった。持てるかどうかが、文書の間でも違っていた。
  3. 型つきの field・初期値・`gate` の member: `box` では書けるが、static box では parse error だった。
  4. header の `from`: static box では受け付けて compile も通るが、`box` では拒否される。
  5. `flow`: code の注記は「static box の糖衣」と言うが、ふつうの `box` として読まれていた。
  6. macro: guide は箱の中の `static function` を例に書くが、parser は拒否する。
- 相談の答え(2026-11-26)は、static box を「static method を名前でまとめる箱」と決める事を勧めた。
  - 共有の見えない実体を足すと、初期化の順、Fault の後の初期化のやり直し、並行の access、最後の解放を、新しく決める必要がある。
  - ふつうの `box` なら、値を持つ実体・持ち主・終わる時点を、ADR-003 の決まりで示せる。
- メンテナは、6 つのそれぞれに印を付けて、この形を採った。

## 決めた事(ユーザー決定 2026-11-26)

1. **static box の `new` は、専用の名前 `call/new-static-box` で拒否する。**
   - 無い名前の `new` とは分ける。
   - `static box MapBox {}` のように組み込みの名前を覆う時も同じで、組み込みに戻さない。
2. **static box の field・`init { }`・`static { }` は禁止。**
   - 宣言の位置で、compile error にする。案内は「static box は状態・初期化を持てない。状態はふつうの box へ」。
   - 名前の案は `decl/static-box-state`。
3. **型つきの field・初期値(`x: T`、`x = e`、`x: T = e`)も、2 と同じ。**
   - method を選ぶ `gate` は、将来許す方針。今は作らない。
   - file 直下の `static const` の表は別の宣言で、変えない。
4. **`static box S from P` も拒否し、ふつうの `box` と揃える。**
   - 親の method は `P.f()` と呼ぶ。
   - 所有・戻り値の契約の `from`(`pick(a: P, b: P): P from a`)は別の物で、この ADR の対象外。
5. **`flow` という宣言の綴りは、退役する。** 名前で呼ぶ関数の集まりは `static box` に、状態を持つ物は `box` に移す。
6. **macro も `static box MacroBoxSpec { expand(json, ctx) { ... } }` に揃える。**
   - 箱の中の `static function` は作らない。
   - 箱の外の `static function MacroBoxSpec.expand` は、ADR-005 の compile error に入る。
   - `NYASH_MACRO_TOPLEVEL_ALLOW` と `--macro-top-level-allow` の特例は、退役する。

## 書けなくなる物(before / after)

static box の `new`:

```hako
// before: 書けなくなる(拒否の名前は call/new-static-box になる)
static box Counter {
    next(n) { return n + 1 }
}
static box Main {
    main() {
        local c = new Counter()
        return c.next(1)
    }
}

// after: 名前で呼ぶ
static box Main {
    main() { return Counter.next(1) }
}
```

static box の状態(field・`init { }`・`static { }`・型つきの field):

```hako
// before: 書けなくなる
static box Main {
    init { console }
    main() {
        me.console = new ConsoleBox()
        me.console.log("hi")
        return 0
    }
}

// after: 状態は、ふつうの box に持たせる
box App {
    console: ConsoleBox = new ConsoleBox()
    run() {
        me.console.log("hi")
        return 0
    }
}
static box Main {
    main() {
        local app = new App()
        return app.run()
    }
}
```

static box の header の `from`:

```hako
// before: 書けなくなる
static box Child from Parent {
    f() { return 1 }
}

// after: 親の method は、名前で呼ぶ
static box Child {
    f() { return Parent.base() }
}
```

`flow` の宣言:

```hako
// before: 書けなくなる
flow RegexFlow {
    digits(text) { ... }
}

// after: 名前で呼ぶ関数の集まりなら static box、状態を持つなら box
static box RegexFlow {
    digits(text) { ... }
}
```

macro:

```hako
// before: 書けなくなる(箱の中の static function、箱の外の static function)
box MacroBoxSpec {
    static function expand(json) { return json }
}
static function MacroBoxSpec.expand(json) { return json }

// after
static box MacroBoxSpec {
    expand(json, ctx) { return json }
}
```

## 移す相手(相談の答えの数え。見込み)

相談の答えが、`fab3a383e8` の追跡している source を、字句と member の深さで数えた物である。compile はしていない。`apps / lang / tools / examples` の順に書く。

- **`new`:** 同じ file に static box があり、同じ名前を `new` する所。
  - 9 / 5 / 2 / 0 か所(9 / 3 / 2 / 0 file。拒否を確かめる入力を含む)。
  - 別の file の static box を `new` する所もある(例: `apps/selfhost-runtime/ops_calls.hako` の `new ConsoleStd()` が 3 か所)。
  - src の Rust の test の文字列に 3 例(1 file)。
- **名前だけの field:** 0 / 1 / 2 / 0 か所。
- **`init { }`:** 14 / 0 / 0 / 4 か所(16 file、空が 6、中身ありが 12)。
- **`static { }`:** 0。
- **Rust の test の文字列:** 名前だけの field が 6 か所(3 file)。ほかに、tools の shell が、名前だけの field 2 個と空の `init` 1 個を作る。
- **型つきの field:** 12 / 5 / 0 / 0 か所。apps の 4 file の 12 個は、どれも初期値つき。lang の 5 個は、GC の 3 file にあり、初期値なし。型の無い `x = e` と、static box の member の `gate` は 0。
- **header の `from`:** 3 / 0 / 0 / 0 宣言。どれも `apps/tests` にある、selfhost の関数の走査(funcscanner)の入力で、tools の smoke が読む。
  - 対象: `phase29bq_selfhost_funcscanner_box_from_min.hako`、`phase29bq_selfhost_funcscanner_method_boundary_from_birth_min.hako`、`phase29cc_selfhost_funcscanner_typed_params_implements_min.hako`。
  - 4 で成功から拒否に変わる見込みの入力である。
- **`flow`:** 0 / 5 / 0 / 0 宣言。どれも `lang/src/compiler/pipeline_v2/` にある: `EmitMirFlow`、`EmitMirFlowMap`、`PipelineV2`、`RegexFlow` と、`pipeline.hako` が古い抽出に使う flow の 1 つ。
  - この 5 つの名前を直接書いた修飾の呼び出しは、17 file・203 か所。同じ名前の `new` は 0。
  - `using` の別名を通した使い手は、数えていない。
- **macro:**
  - `apps/macros/examples` の `static box MacroBoxSpec` は 13 file。箱の中に `static function` を書いているのは、`gensym_example_macro.hako` の 1 file・2 宣言。
  - `apps/tests/macrobox_ny` の、ふつうの `box MacroBoxSpec` に古い形が 3 file・5 宣言。
  - 箱の外の `static function MacroBoxSpec.expand` は 0。
  - code の側: 読み込み(`src/macro/macro_box_ny/loader.rs:66`)と、子の runner(`src/macro/macro_box_ny/child.rs:151`・`:223`)。古い flag の警告の smoke が tools に 1 本ある。
- **状態を持っている物(名指し):**
  - `apps/ny-mem-bench/main.hako`: `static box Main` が、`init { }` に field を 10 個並べ、`me.console` などを使う。
  - `lang/src/compiler/pipeline_v2/readonly_map_view.hako`: `static box ReadOnlyMapView` が、自分を `new` して `_m` に値を持つ。
  - この 2 つは、ふつうの `box` と、はっきり渡す状態に移す。
- **古い VM:** `StaticBoxRegistry` に、遅れて作る singleton がある(`src/backend/mir_interpreter/static_box_registry.rs:35-36`)。
  - 今の Canonical の道で失敗する入力が、古い VM でも動いていなかったとは言えない。
  - なので、移す前に、入力ごとに parse・MIR・EXE の出力と終了値、古い VM の結果を、1 本ずつ別々に測る。

## 移る順

ADR-005 と同じ順にする。

- (1) この文書の回。ADR と、言語の文書を直す。実装は変えない。
- (2) 使っている所を書き直す。出力と終了値が変わらない事を確かめる。
  - 状態を持つ static box は、ふつうの `box` に移す。
  - `flow` の 5 宣言は、`using` の別名を含めて、移す前と後で同じ関数へ届く事を確かめる。
  - macro の例と、子の runner の生成元を、`static box MacroBoxSpec { expand(json, ctx) { ... } }` に移す。出力が変わる macro(`upper_string_macro.hako` など)で確かめる。終了値 0 だけでは、何もしない登録と区別できない。
- (3) compile error にする。
  - 4 で成功から拒否に変わる入力(`apps/tests` の funcscanner の 3 本の見込み)は、(3) の統合の前に、メンテナが 1 本ずつ見る。

掃除と selfhost の順(ADR-003 の「作業の順」)の中に置く。この ADR のためだけに、順を前へ出さない。

## まだ決めていない事

- 診断の文。`decl/static-box-state` は名前の案で、決めるのは (3) の回。`call/new-static-box` の文も (3) の回で決める。
- `new` が static box を指す事の見分け方。今の code の `declares_non_ordinary_box` は interface・record も含むので、それだけで `call/new-static-box` を出すと誤る(相談の答え)。宣言の種類で決め、引数を読む前に決める、が案。
- method を選ぶ `gate` の形と時期。方針は、両方の枝を構文で検査し、同じ署名の対を求め、選ばれた method を 1 つ登録する(ふつうの box の LANG-CFG-002 と共有する)。
- `flow` を退役した後、古い宣言の位置で出す案内の文と、拒否の名前。
- macro の子の runner の、正式な入口(引数の受け渡しを保つ形)。ADR-005 の、runner 向けの `function main(args)` と同じ問題である。
- macro に古い形を指定した時、何も変えずに登録が成功したように見える今の動き(相談の答え)の直し方。
- 古い VM の `StaticBoxRegistry` の遅れて作る singleton を、いつ消すか。
- 状態を持つ static box(`ny-mem-bench`、`ReadOnlyMapView`、型つきの field を持つ GC の 3 file など)を移す先の、box の名前と、呼び手の書き直し方。
- static box の header の型の引数と `interface` の一覧。この ADR は変えていない。
- method の名前の前に付ける `method` という語(`method get() { ... }`)。この ADR は変えていない。

## 根拠

- 相談と答えは、公開の repo の外にある(2026-11-26)。要点は、上の「なぜ決めたか」「移す相手」「まだ決めていない事」に入れた。
- 確かめた事(2026-11-26、今の CLI):
  - static box の `new` と、無い名前の `new` は、どちらも `[mir/callable-semantic-package/issue]` の `OrdinaryBoxCoverageMissing` で拒否される。
  - static box の名前だけの field・`init { }`・`static { }` は、parser を通り、compile が `[mir/normal-root/consume] IntegrityInvalid` で止まる。
  - static box の header の `from` は、compile も EXE も通る。
- 確かめていない事: 「移す相手」の数え(相談の答えの数え。compile していない)と、書き直した後の出力と終了値((2) の回で確かめる)。
