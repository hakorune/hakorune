# Phase 287 P4: Quick 残failの根治（core回帰の修正 + テストの仕様合わせ）

## 目的

- `tools/smokes/v2/run.sh --profile quick` を **fail=0** で安定させる。
- P3で「責務外」を integration に寄せたあとに残った **core回帰**（意味論バグ）を直す。
- REPL はまず `hakorune --repl` の **明示起動**から開始する（仕様は `docs/reference/language/repl.md` をSSOT）。

---

## 優先度1: String mixed `+` の SSOT へ戻す（暗黙 stringify を入れない）

### 症状

`"" + a.length()` が `unsupported binop Add on String("") and Integer(0)` で落ちる。

この挙動自体は **SSOT（Phase 275 C2 / `types.md`）では正しい**。問題は「テスト/ドキュメントが旧挙動を期待している」こと。

### 仕様（SSOT）

- `docs/reference/language/types.md` の `+`（C2: String+String only）に従う。
- `docs/reference/language/quick-reference.md` も SSOT に合わせて修正する（String mixed は `TypeError`）。

### 実装方針（構造的）

- 実装（VM/LLVM）に自動文字列化の分岐を追加しない（Phase 287 で仕様拡張をしない）。
- 連結したいテストは明示的に `x.toString()` を使う（例: `"len=" + a.length().toString()`）。

### 受け入れ（smokeで固定）

この整理により、少なくとも以下の失敗群がまとめて解消する可能性が高い（要再計測）:
- `array_length_vm`
- `array_oob_get_tag_vm`
- `index_substring_vm`
- `map_len_set_get_vm`
- `map_values_sum_vm`
- `string_size_alias`

---

## 優先度2: JoinIR ループ非対応パターンの扱い（quick から外す）

### 症状

`[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern`

### 方針

- これは「言語機能の最低ゲート」ではなく「JoinIRループ網羅/拡張」の範囲なので、Phase 287 では quick に残さない。
- 対象テストは integration へ移動し、必要なら `[SKIP:joinir]`（理由固定）にする。

---

## 優先度3: Top-level local（file mode禁止 / REPL許可）へテストを合わせる

### 仕様（決定）

- file mode は **top-level 実行文禁止**（宣言のみ）。top-level `local` も禁止。
- REPL は `hakorune --repl` で明示起動し、暗黙localを許可する（`docs/reference/language/repl.md`）。

### 具体対応

- `run_nyash_vm -c 'local ...; ...'` のように top-level 実行文を前提にしたテストは、quick からは外すか、次のいずれかに書き換える:
  - `function main(){ ... }` に包む（entryは `NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1` でOK）
  - `static box Main { main(){ ... } }` に包む
- 「top-level local が壊れている」方向へ直すのは、REPL/file mode の仕様を汚しやすいので今はやらない（WIPブランチへ隔離済み）。

---

## 優先度4: 残りの個別（環境依存 / 揺れ）を quick のSSOTへ寄せる

- `filebox_basic.sh` のように host依存が避けられないものは `[SKIP:env]` で理由固定。
- `async_await.sh` / `gc_mode_off.sh` は “環境依存” と断定しない。まずログを見て:
  - core回帰なら修正（quickに残す）
  - 統合寄り/重いなら integration へ移す

---

## 作業手順（推奨）

1) `--format json` で fail をSSOT化（pathとdurationでソート）
2) まず優先度1（String+Integer）を修正 → quick を再実行
3) JoinIR freeze のテストは integration へ移動（必要ならSKIP理由固定）
4) top-level 実行文前提のテストは「mainに包む」か integration へ
5) 残った fail を A/B/C（env/統合/コア）で片付け、quick fail=0 で締める
