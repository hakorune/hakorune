# Hakorune 開発ルール

Status: SSOT
Date: 2026-10-08
Scope: task selection, implementation, validation, production cutover, retirement, and restart routing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/agent-current-entry-contract-ssot.md (family scheduler, worker, and optional-tool procedures)
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
    (document lifecycle and size rules)

## 0. 目的

正しい意味を、小さく、速く本線に入れる。重複した記録を前進条件にしない。
source authority・意味契約・失敗境界・必要な acceptance は実装着手の条件として保つ。

## 1. 変えない設計原則

- 1つの意味に owner は1つ、実行経路も1つ。
- 意味変更は `source -> Facts -> Recipe -> Verify -> sole physical owner -> publish` の順に流す。
- Fail-Fast。silent fallback、名前によるハードコード、AST rewriteは禁止。
- `.hako` の書き換えでコンパイラの表現力不足を回避しない。
- BoxCount（受理形を増やす）と BoxShape（挙動を変えない整理）を同じsliceに混ぜない。
- FactsはRecipe keyやphysical IDを作らない。Recipe producerがkeyを発行し、物理layoutは意味を再判定しない。

契約の詳細は[Recipe契約](current/main/design/recipe-first-entry-contract-ssot.md)、
[最終pipeline](current/main/design/mirbuilder-final-pipeline-ssot.md)、
[表現力優先ポリシー](current/main/design/compiler-expressivity-first-policy.md)を参照する。

## 2. 作業の入口と単位

- 選択中の作業は `CURRENT_STATE.toml` とそこから指されるactive cardで決める。
- goalの範囲は `finite_product_goal` とその正本の必須完了条件。監査の改善候補を自動で必須化せず、parked laneは勝手に再開しない。
- `CURRENT_STATE.toml.work_mode` が `fast` / `design_stop` / `closeout` の唯一のmode選択元。blocker文言から推測しない。
- `CURRENT_TASK.md` は再開用の薄いroot pointer。各sliceのたびに書き換えず、active cardへ作業範囲と証拠を記録する。
- 1sliceは1責務。コード変更は第6節のpositive/negative検証を満たす。契約変更なら同じsliceでowner README/referenceを更新する。

## 3. 毎回の流れ

1. `CURRENT_STATE.toml`、`CURRENT_TASK.md`、active cardを読み、`git status -sb` と `bash tools/checks/current_state_pointer_guard.sh` を確認する。
2. `work_mode`に従う。`fast`は選択されたbounded sliceだけを実装し、`closeout`は証拠とpointerを整える。
3. `design_stop`ではactive cardの未解決契約だけを扱う。source authorityやcanonical issuerを推測せず、実装・fixture・fallback・production switch・新しいsemantic receiptを先行させない。
4. sliceが要求するfocused positive/negative testsとstable guardを実行する。filter結果が0件なら成功扱いしない。
5. コード・test・その契約を説明するowner docsを同じcommitにまとめる。pointerは選択先が変わる時だけ更新する。
6. commitではsliceに属するパスだけを明示してstageし、既存の無関係な
   staged/unstaged変更を保持する。同じファイルに別作業が混在する場合は
   対象hunkだけを選ぶ。

`tools/checks/dev_gate.sh quick` や重いacceptanceはactive cardが要求する時に実行する。CI結果待ちの間に独立した許可済み作業があれば続け、必須acceptanceの結果は正確なrun/SHAでcloseoutに記録する。

## 4. 建設と旧経路退役の証明タイミング

- 建設の入口は二点で決める。①選ぶsource authority・canonical issuerと、このsliceが受け持つ意味／失敗境界が固定されている。②置き換えるproduction callerと旧責務を名前で指し、新経路を判定するfocused positive/negative acceptanceが決まっている。
- この二点が閉じたら建設を始める。建設後のgreen、caller-zero、別callerの全数調査、CI完走を着手条件にしない。テスト失敗は選んだmapping内で直し、受理形やauthorityを広げる必要が出た時だけ設計へ戻る。
- `NoSafeSlice`はdesign stopのまま扱う。候補・拒否・空receipt・fallbackへ変換しない。
- production cutoverでは、同じsource authorityからselected consumerへ通す。失敗後に旧経路へretryしない。
- 旧edgeを物理削除する前に、影響callerが同じbounded series内で切替または停止済みであり、対象acceptanceとguardが通り、そのedgeのcallerが0であることを確認する。削除集合は選択済み範囲内とし、共有ownerと未選択callerは残す。

## 5. 詰まりと設計停止

- 時間経過や同じ理由での再停止だけを理由に、必須のactive objectiveから自動的に別作業へ移らない。
- mappingが未確定なら短いdecision briefをactive cardに記録する。`CURRENT_TASK.md`を履歴帳にしない。

```text
Decision:
Source authority + canonical issuer:
Non-authority:
Fail-fast boundary:
Smallest next slice:
Non-claims:
```

- 同じ責務で3回続けて `NoSafeSlice` になったら、edge censusを繰り返す前にsemantic unit、全classifier arms、opaque/transferred subtrees、型要件、counterexampleを監査する。難しい独立設計はread-only workerで確認し、主担当が1つのDecisionへ統合する。
- read-only workerは共有checkoutを編集せず、Cargoを起動しない。文書と実態が食い違えば原因を確認し、記載か設計判断を更新する。
- 候補選択・design stopの解決順・停止判断は[family-local scheduler](current/main/design/agent-current-entry-contract-ssot.md#family-local-action-scheduler)を続けて使う。ここに定めたmodeと入口・退役の規則が優先する。
- ユーザーが設計相談中の停止やgoalのpauseを明示したら、それに従う。
- 同じ阻害が3回連続のgoal turnで続き、安全な代替も尽き、ユーザー判断か外部状態の変化が必要ならblockedにする。内部の未実装・設計不足は課題にして解決を続ける。

## 6. テストと計算資源

- 追加前に対象familyの既存test・fixture・guardを確認し、守る意味と失敗境界を指定する。既存ownerへケースを足し、独立した未被覆条件だけ新規testにする。毎sliceの新規test追加は義務にしない。
- 同じ契約の等価ケースは共通fixtureやパラメータでまとめる。異なる失敗境界の診断と必要な独立実行・filter選択を保ち、source意味・独立した最終MIR・外部入力/実行検証を重複扱いで削らない。
- 挙動不変の整理は既存回帰検証を使い、実装を写すだけのtestを足さない。docs-onlyは対象文書のリンク・pointer・契約を検証し、不要なCargo起動をしない。
- guardは既存family ownerを再利用し、作業rowの完了・一時的なコード配置・過去の選択pointerを守る専用guardを増やさない。新規guardは既存ownerで守れない継続的な契約と利用callerがある場合に限る。
- 一時的test/guard/adapter/proofには、owner・正本の置換先・`retire_when`・削除に必要な証拠を既存card/metadataへ記す。条件を満たした専有物は第4節に従い同じbounded seriesで統合・物理退役する。
  永続的な意味・安全性検証を一時負債に含めず、履歴はGitへ残す。退役を証明する新guard・台帳や、一律のtest数上限・全repo棚卸しを着手条件にしない。
- 必要な検証がgreenなら、関連変更・失敗・未解決の懸念・要求gateがない限り再実行や拡張をしない。同じbuildをfilter実行で再利用し、結果は対象revision・実行条件に対応させる。
- 時間を測る際はbuild/link、test実行、shell guard、gateの重複実行を分ける。test名/ファイル数の削減だけで高速化を主張しない。
- 同じcheckoutでtop-level Cargoを同時に複数起動しない。日常のfocused Rust testは`--profile quick`とし、`CARGO_BUILD_JOBS=4`を上限目安にする。
- Cargoを中断した後は、既存の`cargo`/`rustc` processが終了したことを確かめてから次を起動する。
- `--release`、`--nocapture`、`RUSTFLAGS=-Awarnings`はactive cardが必要とする場合だけ使う。warningを隠すために再実行しない。
- 赤は今回の変更・既知baseline・informationalに分類する。未分類の赤はcloseoutしない。
- CIは通知または次のcloseoutで確認する。進行中runを連打dispatchで取り消さず、pending/cancel/silenceをPASSや失敗と数えない。

## 7. 文書とpointer

- 作業モード・スライス・検証・closeoutの正本はこのfile。現在地は`CURRENT_STATE.toml`、root再開pointerは`CURRENT_TASK.md`、置き場所は[DOCS_LAYOUT.md](current/main/DOCS_LAYOUT.md)が所有する。
- `agent-current-entry-contract-ssot.md`はworker consultationとoptional NekoCodeの補助手順を保持する。作業モードや着手・退役条件がこのfileと異なる場合は、このfileを適用する。
- active cardにはscope、acceptance、parked items、non-claims、実行結果を記録する。再開mirrorへ履歴を複製しない。
- active cardは1000行以内。上限前に完了済み経緯をcommit参照へ圧縮し、現在の契約・未解決事項・必要証拠を残す。
- 通常の実装sliceで更新する文書はactive card、選択やpointerが変わる場合の
  `CURRENT_STATE.toml`、契約が変わる場合のowner README/referenceに限る。
  workstream、隣接card、索引、restart mirrorは、それぞれが所有する事実を
  変更するときだけ更新する。
- 実装前に対象パスを絞る。実装で別文書の所有契約が実際に変わると判明した
  場合だけ範囲を追加し、その理由をactive cardに記録する。履歴や見やすさの
  ための横展開は同じsliceに混ぜない。
- 新しい文書には`Status`、`Scope`、`Related`を記し、`DOCS_LAYOUT.md`に従う。文書を縮めるために契約や必要証拠を捨てない。
- 言語仕様の変更は`docs/reference/**`を先に更新する。

## 8. コードの衛生

- source fileは760行で分割を考え、800行をhard stopとする。圧縮で逃げない。
- `allow(dead_code)`は限定owner・理由・再確認期限を記録する。V1/V2の並立には統合条件を持たせる。
- debug logは既定OFF。無条件`eprintln!`は禁止。
- 挙動不変のrefactorは2〜5 commitsにまとめ、受理形の拡張と混ぜない。

## 9. 完了の定義

- **slice closeout:** 選択ownerのpositive/negative evidence、要求guard、赤の分類、owner docsが揃う。sliceが契約整備ならproduction switch完了とは主張しない。
- **MirBuilder migration complete:** 必須workstream rowが全て閉じ、実sourceからcanonical Facts/Recipe/verification、sole physical owner、publicationまで到達する。選択production callerが切替済みで、選択旧edgeが退役し、要求されたend-to-end acceptanceが記録されている。
- 削除予定をtaskboardに残しただけではmigration完了にしない。
- 進捗はproduction callerの切替、重複責務の解消、旧edgeの削除、実sourceの前進と検証で報告する。commit/file/test数だけを根拠にしない。

## 10. local entryと旧文書

- ignored `AGENTS.md` / `CLAUDE.md`はRULES・CURRENT_STATE・owner資料への短い入口とagent固有の言語・口調だけを持ち、独立した実装手順を複製しない。秘密情報を含む可能性があるためignore設定は勝手に外さない。
- `current-docs-update-policy-ssot.md`の作業モード・建設と退役の証明条件はsuperseded。残る文書lifecycle・registry・specialized validation clausesは個別移行まで有効で、競合時はこのfileを優先する。
- `agent-current-entry-contract-ssot.md`のcurrent entry・work-mode規則はsupersededだが、family-local scheduler、worker consultation、optional NekoCodeの補助手順は引き続き参照する。二つの文書の役割は[DOCS_LAYOUT.md](current/main/DOCS_LAYOUT.md#operational-rule-map)に記す。
- ルールを足す時は重複を減らし、本文は150行以内を目安にする。
