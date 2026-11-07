# S3b の設計の相談(所有の状態と Fault の後片付けを CFG の解析へ移す)

Exception: 外部 AI の回答は再生成できないので残します(RULES §7「`/tmp` だけを永続証拠にしない」)。

ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

Status: 相談の文と回答の記録です。authority ではありません。順番の正本は[整理の順番](mirbuilder-structure-refactor-queue-d0-2026-08-23.md)です。

## 相談の文(2026-11-07、ChatGPT Pro へ。下の線から最後までを貼る)

---

自作言語 Hakorune のコンパイラについて、独立した意見をください。所有の状態と Fault の後片付けを、「source の意図を保持した CFG の上の解析」へ移す最初の段階(S3b)の作り方です。結論を誰かに合わせる必要はありません。下の案より良い作りがあれば、それを勧めてください。

リポジトリは github.com/hakorune/hakorune の main、commit `ec9cd19681` です。根拠は file:line で示してください。読んで確かめられなかったことは、その旨を書いてください。私の前提が間違っていたら、先に指摘してください。

## 背景

- Box 中心の言語で、GC はなく、所有は明示します(`move`、`share`、`take`、`from` など)。文法は ADR-002 で凍結済みです(`docs/development/adr/adr-002-explicit-ownership-and-compiler-migration.md`)。ADR-002 の 73 行目には「同一の semantic CFG で ownership・borrow を解析」とあります。
- 流れ: source → parser → resolver → 意味の package(`src/mir/normal_callable_semantic_package`、`src/mir/resolved_semantics`)→ MIR builder(`src/mir/builder`)→ 生成後の独立検証 → 物理 ABI の JSON → C shim → LLVM → EXE。
- 守る物:
  - すべての呼び出しと演算で、Normal と Fault の両方の End(後始末)の責任を閉じます。不変条件は「その関数の責任を、End か正当な移転で一度だけ閉じる」です。
  - 生成後の独立検証は弱めません。
  - 形ごとの専用 producer、silent fallback、AST の書き換えは使いません。
  - 公開 MIR、opcode、ABI、descriptor の追加はメンテナが決めます。
- 進め方: 稼働中のコンパイラを責務ごとに入れ替えます。並行の別コンパイラは作りません(`docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md`)。

## これまでに決めた事

- 2026-11-06 の相談(経路の選び方)で、目標の流れを決めました。記録は `docs/development/current/main/investigations/mirbuilder-route-selection-survey-2026-11-06.md` と、`docs/development/current/main/investigations/mirbuilder-structure-refactor-queue-d0-2026-08-23.md` の節「流れの作り直し」です。流れは次の 5 つです。
  1. 型と宣言の契約を確定する。
  2. source の意図を保持した、未公開の関数の CFG を作る。保持する物は、明示の move/share/take/copy、Binding・Place・source site、スコープ、cleanup の登録順、評価順、commit の契約です。
  3. その CFG の上の dataflow(loop は不動点)で、使える binding、生きた所有の責任・移転待ち・移転済み、借用、各 Normal/Fault 辺の End を求める。End は既存の cleanup の部品で具体化する。
  4. cleanup の Fault 辺まで含めて、既存の CFG と PHI の持ち主(`CanonicalCfgSessionV1`、`PhiTxn`)で確定する。seal を先に閉じない。
  5. source との対応の検査と、公開 MIR の独立検証。独立の verifier を End の生成器と兼ねない。
- 段階 S3 の完了の条件は、「新しい使い先をつないだ時に、新しい分類器と cleanup の持ち主が増えない」です。
  - S3a(値の種類の規則を 1 つ、演算子の表を 1 つ)は着地しました。
  - S3b は「呼び出しと分岐の所有の状態と Fault の cleanup を、意図を保持した CFG の解析へ移す、最初の責務を 1 つ」です。
- 結果の契約の相談(`docs/development/current/main/investigations/result-contract-consult-2026-11-06.md` の回答の 4)の決定: 結果の契約は責任の種類を決め、S3b の CFG の解析は責任が移る時点を決めます。S3b は結果の型を推論し直しません。
  - 結果の契約のうち Unit、I64、Bool は着地しました(`docs/development/current/main/investigations/mirbuilder-result-contract-bool-2026-11-07.md`)。
- S3b の後は、所有の String・object・nullable の結果、S4(loop を共有の CFG と `PhiTxn` に寄せる)、`edit`、3b(旧経路の退役)の順です。

## 今の作り(私が読んだ範囲)

- 所有の状態は、source の上の線形な走査 `scan_new_home_flow`(`src/mir/resolved_semantics/home_new_prefix.rs:377`)の中で、形ごとの観測が持っています。走査は、前の文の結果(生きた Home、move 済み、失敗)を読みます。
- 「元の名前は使えないが、呼んだ側にまだ終了の責任がある」状態は `StoredLocal::MovedOut` です(`src/mir/resolved_semantics/home_call_entry_transfer.rs:14`)。
- 分岐ごとの状態の解析は `src/mir/resolved_semantics/home_new_prefix_branch.rs` にあります(783 行。`walk_branch` は 33 行目、`observe_if_statement` は 235 行目)。
- `CanonicalCfgSessionV1`(`src/mir/builder/resolved_lowering/canonical_cfg/session.rs`)は seal した block を持ちます。seal の後で predecessor が変わった時は拒否します(396 行目の `SealedPredecessorsChanged`、406 行目の `SealedBlockRemoved`)。`PhiTxn` は `src/mir/builder/emission/phi_lifecycle.rs:133` です。
- `&&` と `||`:
  - 条件の位置では、短絡の分岐として書かれます(`src/mir/normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/bool_condition_control.rs`)。
  - 値の位置(`return x > 1 && y < 3`)には、値を書く writer がありません。`static-execution/completion-not-ready`(`src/mir/normal_callable_semantic_package/direct_call_static_execution.rs:98`)で拒否されます。
  - 意味は仕様で短絡と決まっています(`docs/reference/language/EBNF.md` の Notes の「Short-circuit」と、C197 の行)。
- 凍結した文法が受け付ける所有の形 129 個のうち、EXE までつながるのは 34 個です(`docs/reference/language/status-index.md` の 94 行目)。

## 聞きたいこと

1. **最初に移す責務を 1 つ選んでください。** 私の候補は次の 3 つです。
   - (a) 1 つの呼び出しの前後で、取得済みの一時値の Fault の後片付け
   - (b) `if`/`else` の合流での所有の状態
   - (c) 値の位置の `&&` `||`

   どれなら「解析の土台」を最も小さく作れて、後の段階(所有の String・object・nullable の結果、S4 の loop、`edit`)が同じ土台に乗れますか。完了の条件と、同じ series で消せる旧い判定も挙げてください。
2. **意図を保持した未公開の CFG の最小の形を勧めてください。** node と辺、各地点に持たせる物、Fault の辺の表し方、cleanup の登録順と評価順の持ち方です。今の `CanonicalCfgSessionV1` と `PhiTxn` に、どこで、どの順で渡せば、「seal を先に閉じない」を守れますか。
3. **dataflow の状態の領域と、合流の規則を勧めてください。** 状態は、生きた責任、移転待ち、移転済みで caller に責任が残る物、借用、消費済みです。片方の枝だけで move した時は、拒否、drop flag、合流での End のどれにすべきですか。ADR-002 や仕様の文書に決まりが既にあれば、その場所を示してください。
4. **3 つの線をどこで引くか教えてください。** End を決める所(dataflow)、End を具体化する所(既存の cleanup の部品)、独立検証です。今の形ごとの cleanup のうち、そのまま部品として使える物と、作り直す物を、コードを読んで分けてください。
5. **移行の間の範囲の切り方を勧めてください。** 線形の走査 `scan_new_home_flow` と新しい解析が、同じ関数の中で重ならないようにしたいです(関数単位、文の種類単位など)。両方を試す fallback は使いません。最初の series で、既存の受理を 1 つも落とさずに済む切り方はありますか。
6. **値の位置の `&&` `||` の作り方を勧めてください。** 分岐と合流(Phi)で書くとします。右辺の呼び出しが一時値を取得したり Fault したりする時、End を実際に評価した道だけで閉じる方法を知りたいです。条件の位置と値の位置で、writer を 1 つにすべきですか。
7. **S3b の効果を測る物差しを勧めてください。** 「新しい使い先をつないだ時に、新しい分類器と cleanup の持ち主が増えない」を、series の終わりに数で示したいです。何を数えればよいですか。
