# 紙上の書き比べ 9 課題

Exception: 書き直しの費用が大きい入力と結果を、`/tmp` から移した資料です(RULES §7「`/tmp` だけを永続証拠にしない」)。

ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

Status: 2026-11-03 に書いた紙の上の下書きです。authority ではありません。仕様は ADR-002 と `docs/reference/language/ownership.md` が持ちます。

## 何か

ADR-002 の 102 行にある「9 課題の実行」と、104 行の書き比べに使う入力の下書きです。同じ処理を Hakorune と Rust で書いています。結果は `results.md`、節ごとの数は `counts.md` です。

## 読む時の注意

- **Hakorune 側は一度もコンパイルしていません。** 受理と拒否の判定は、書き手が規則を読んだ結果です。Rust 側だけ `rustc 1.89.0` でコンパイルと実行を確認しています。
- **今の文法と違う綴りがあります。** 書いた後に、`mut` は `edit` に決まり(ADR 55 行)、`record` と `with` は廃止されました(ADR 58 行の④)。実行する時は、今の文法へ書き直してください。
- ADR が求める「完成までの修正回数」「診断の分かりやすさ」「実行時間とメモリ」は、ここでは測っていません。ゲートの実行そのものは未着手です。
- 両言語とも書き手は 1 人です。詳しくは `results.md` の 0 節と 7 節を読んでください。

## テストではありません

どの suite や checker からも実行されません。`.hako` と `.rs` は読むための資料です。テストや fixture として足さないでください(RULES §6)。`count_markers.py` は `counts.md` を作った時の script で、定期実行しません。

## 中身

| file | 課題 |
|---|---|
| `t1_match_borrow` | `match` して子を借用で返す |
| `t2_chain_borrow` | 借用した子から孫を読む |
| `t3_map_update` | Map の要素を短い区間だけ更新する |
| `t4a_take_rewrite` | nullable の子を `take` で書き換える |
| `t4b_required_child` | 必須の子を書き換える |
| `t4c_mut_parent` | 編集の借用を経由して書き換える |
| `t5_scope_lookup` | スコープの連鎖を引く |
| `t6_bundle` | 値の束 |
| `t7_shared_config` | Shared の設定 |
| `rust_walls/` | Rust の E0507、E0509、E0505 の拒否と、兄弟 field 読みの受理の確認 |

Rust の build 出力は入れていません。
