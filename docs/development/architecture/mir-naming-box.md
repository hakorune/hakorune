# MIR NamingBox — static box naming rules

目的: Builder と VM で「static box メソッドの名前」を一箇所で定義し、`main._nop/0` などのケースズレを防ぐ。

## 役割
- `canonical_box_name(raw)`:
  - `main` → `Main` に正規化（最小限の補正）
  - それ以外はそのまま返す（仕様拡張は別フェーズで検討）
- `encode_static_method(box, method, arity)`:
  - `Box.method/arity` 形式にまとめる（Builder 側で defs→MIR 関数化時に使用）
- `normalize_static_global_name(func_name)`:
  - `main._nop/0` のような global 呼び出しを `Main._nop/0` に正規化（VM 側で実行前に使用）

## 呼び出し箇所
- Builder (`src/mir/builder/decls.rs`):
  - `build_static_main_box` が static メソッドを関数化する際に `encode_static_method` を使用。
  - `Main._nop/0` のような名前がここで確定する。
- VM (`src/backend/mir_interpreter/handlers/calls/global.rs`):
  - `execute_global_function` が `normalize_static_global_name` を通してから function table を検索。
  - canonical 名（例: `Main._nop/0`）→元の名（互換用）の順に探す。

## 追加テスト
- `src/tests/mir_static_box_naming.rs`:
  - `Main._nop/0` の defs が MIR module に存在することを確認。
  - `me._nop()` 呼び出しが Global call として `_nop/0` を指していることを観測。
  - `NYASH_TO_I64_FORCE_ZERO=1` 下で `apps/tests/minimal_to_i64_void.hako` を VM 実行し、静的メソッド呼び出し経路が通ることを確認。

## Phase 21.7 との関係
- Phase 21.7（Methodize Static Boxes）では `Global("Box.method")` を「単一インスタンスを持つ Method 呼び出し」に寄せる予定。
- NamingBox はその前段として「名前の正規化」を共有化する箱。Method 化するときもこのルールを踏襲し、Box 名のゆらぎを防ぐ。
