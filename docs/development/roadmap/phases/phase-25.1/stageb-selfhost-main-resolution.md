# Phase 25.1 — Stage‑B Selfhost main 解決メモ（InstanceBox / Static Box 呼び出し）

目的
- `tools/selfhost/build_stage1.sh` 実行時に発生している  
  `Unknown method 'main' on InstanceBox` の根本原因を整理し、  
  どの Box / メソッド名のズレかを明確にするための観測メモだよ。

## 1. 現状の観測結果

- エラー発生箇所:
  - `src/backend/mir_interpreter/handlers/boxes.rs:105` 付近（InstanceBox 用のメソッドディスパッチ）。
- 症状:
  - VM が **InstanceBox** という generic 型に対して `"main"` を探している。
  - 期待されるのは `StageBDriverBox.main(...)` のような「ユーザー定義 static box 上の main」。
  - Rust の `type_name()` macro がユーザー定義 Box を generic 型名（InstanceBox）として返しているため、  
    ログ上は常に `box_type_name=InstanceBox` となり、「どの box の main を探しているか」が見えづらい。

## 2. StageBDriverBox 側の定義

- 定義ファイル:
  - `lang/src/compiler/entry/compiler_stageb.hako:1141-1367`
- 構造:
  - `static box StageBDriverBox { main(args) { ... } }` という形で main が定義されている。
  - エントリポイント:
    - `Main.main()` から `StageBDriverBox.main()` を呼ぶ導線が 1373-1377 行あたりに存在。
- 直感的には:
  - **`.hako` 側は正しく main を定義している**が、
  - VM 側の `InstanceBox` 呼び出し時に **メソッド名の解決 or Box 名の正規化** でズレている可能性が高い。

## 3. ログ追加の案（型名＋メソッド名の観測）

次のようなログを handlers/boxes.rs の Unknown method 分岐直前に入れておくと、  
どの Box / 関数名の組み合わせが原因かが見やすくなるよ。

```rust
eprintln!(
    "[vm/method-dispatch] Unknown method on Box: box_type_name={}, method_name={}",
    recv_box.type_name(), // InstanceBox 側の type_name
    method,               // 探しにいったメソッド名（例: \"main\" / \"StageBDriverBox.main/0\"）
);
```

※ 実際には `type_name()` は常に `InstanceBox` を返すので、  
　実 box 名は `BoxDeclaration` / `InstanceBox` 内のフィールドから取り出す必要があるかもしれない。

## 4. 次に見ると良い箇所

### 4-1. MIR 側の BoxCall 受け側

- 目的:
  - Stage‑B selfhost の MIR で、「どの global / boxcall 名で StageBDriverBox.main を呼んでいるか」を確認する。
- 手順の目安:
  - Stage‑B ランチャの MIR を `--dump-mir` 等でダンプ。
  - `call_global "StageBDriverBox.main/..."` のような形になっているかを見る。

### 4-2. MirBuilder の receiver / callee 解決

- ファイル候補:
  - `src/mir/builder/calls/unified_emitter.rs`
  - `src/backend/mir_interpreter/handlers/global.rs`
- 見たいこと:
  - StaticMethodId / NamingBox を通った後に、  
    `"StageBDriverBox.main/arity"` がどのように解決されているか。
  - InstanceBox に対するメソッド呼び出しのときに、  
    メソッド名から Box 名が正しく分離されているか。

## 5. 方針メモ

- joinIR ラインとは独立して、この問題は「Stage‑B driver / InstanceBox / naming」の層で解決する。
- まずは:
  - どの InstanceBox／関数名の組み合わせで Unknown が出ているかをログで観測。
  - `.hako` 側の StageBDriverBox 定義と、Rust 側のメソッド名解決ロジックの差分を見る。
- 修正自体（命名・正規化の調整）は Phase 25.1 の別サブフェーズで扱う前提として、  
  このファイルは「現状の観測結果と差分の候補」を残しておくためのメモだよ。

