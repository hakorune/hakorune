# Hakorune 開発ルール

Status: SSOT
Date: 2026-10-21
Scope: 作業選択・実装・検証・退役・再開の唯一の運用ルール。
Related: [現在地](current/main/CURRENT_STATE.toml)、[最終pipeline](current/main/design/mirbuilder-final-pipeline-ssot.md)。

## 0. 目的

受け入れアプリが実際に動く状態へ進める。内部準備・文書・検証資産の増加を製品進捗に数えない。

## 1. 変えない設計原則

- 一つの意味にownerと実行経路は一つ。`source -> Facts -> Recipe -> Verify -> sole physical owner -> publish`を守る。
- Fail-Fast。silent fallback、失敗後のlegacy retry、名前・MIR型・backend成功からの意味推測、AST rewriteは禁止。
- `.hako`の書き換えでコンパイラの表現力不足を回避しない。
- 受理形の追加と挙動不変の整理はcommitで区別する。FactsはRecipe keyやphysical IDを発行しない。
- 仕様・所有権・Normal/Fault cleanup・独立した最終検証を保持する。詳細は既存owner契約を必要箇所だけ参照する。

## 2. 作業の入口と単位

- 毎回読むのはAGENTS、RULES、`CURRENT_STATE.toml`、その`latest_card_path`。合計400行以内を目標とする。
- `CURRENT_TASK.md`は任意の薄い入口。workstream・north-star・policy・過去cardは必要時の参照とし、毎回の必読にしない。
- 固定製品acceptanceまでは一つのアプリに集中する。選択rowは`APP-<app-name>`。責務ごとのrow/cardやD0/S0等の接尾辞を増やさない。
- 内部の責務分割はコードとcommitで行う。必要なentry・引数・結果・cleanup・publicationを最終consumerまで接続し、アプリが通るまで追う。
- 別アプリの発見は既存cardに一行残す。別アプリの実装を並行選択しない。共有の不具合は現在のアプリに必要な範囲で直す。
- `finite_product_goal`の必須条件とparked laneの再開条件は維持する。新しい監査候補や言語機能を自動で必須化しない。
- modeの正本は`CURRENT_STATE.toml.work_mode`。`fast`は実装、`design_stop`は実際に未決の意味契約の解決、`closeout`は着地作業。

## 3. 毎回の流れ

1. 現在地と`git status -sb`を確認し、既存pointer guardを通す。
2. 使用binaryとruntimeを明示し、manifestに書かれた選択アプリのsmokeを実行する。古いrelease binaryを暗黙に使わない。
3. 最初の失敗をsource行・実行経路へ結び付ける。precheck、コンパイル、リンク、実行、timeout、SKIPを区別する。
4. 必要箇所の共通ownerを直し、既存focused検証と同じsmokeで確認する。同じ停止点なら既存調査を再利用して欠けた接続を直す。
5. 検証済みの内部commitはアプリ完走前でもよい。対象パスだけstageし、無関係なWIPを保持する。
6. アプリが通ったら既存cardの状態を短く更新し、次のアプリへpointerを切り替える。

commitは短い件名と必要な変更・検証・残件だけにする。通常の自動継続・引継ぎではcheckpoint専用commitを作らず、現在地の更新は次の検証済み実装commitに含める。
ユーザーが指定した停止・文書整理のための記録は独立commitでよい。

## 4. 建設と旧経路退役の証明タイミング

- 着手時に元source authority、canonical issuer、production consumerと必要な失敗境界を特定する。完成済みpacketや全repo censusを着手条件にしない。
- 旧Compatibility経路のMIRは移植の調査資料として使う。仕様・既存期待出力・終了値・Fault/所有権契約を正しさの基準にし、生JSON一致を要求しない。
- 参照経路は明示的な別実行に限る。新経路の失敗後に旧経路へretryせず、旧経路の成功をcanonical実行の証拠にしない。
- 置換した専有edge・一時adapterは、caller切替または停止、caller-zero、必要検証が成立した同じ変更seriesで削除する。局所退役を全アプリ成功まで延期しない。
- 共有Compatibility・schema・backendの大規模退役は、その既存callerと契約が閉じた後に行う。固定suite成功だけで共有ownerや未接続資産を一括削除しない。
- 現在のアプリに不要なwarning整理・全repo削減・caller-zero資産の追加作業は選ばない。既存の恒久的な安全性検証は残す。

## 5. 詰まりと設計停止

- stopごとの文書・census・分類表を作らない。既存app cardへ不足契約・採用判断・次の接続を必要な数行だけ書く。
- `NoSafeSlice`という内部状態や準備不足だけで長期停止しない。元sourceと旧実装を読み、現在のアプリに必要な共通責務を実装する。
- 本当に未決の意味契約があれば、source authorityやcanonical issuerを推測せず、同じcardで決着する。新しいsemantic receiptを先行させない。
- アプリ開始時の難しい接続、新しい責務境界、同じ停止の反復ではread-only workerを使う。既存調査を再利用し、毎小修正の全体監査にしない。
- workerは共有checkoutを編集せず、Cargoを起動しない。主担当が判断を既存cardへ集約する。
- 時間経過だけで別アプリへ逃げない。ユーザーの停止・pauseを優先し、許可範囲だけ再開する。
- goalのblockedは同じ外部阻害が3goal turn連続し、内部で解ける手がなく、ユーザー判断か外部変化が必要な場合だけ。未実装は実装で解く。

## 6. テストと計算資源

- test/fixture/guardは原則増やさない。既存test・既存acceptanceを使い、positive/negative証拠を新規test追加の要求へ読み替えない。
- 既存ケースの行・分岐・パラメータ追加も増加として扱う。追加は既存証拠で守れない独立した意味・失敗境界に必要な最小差分だけとし、理由をcardへ一行残す。
- 新規testファイル・checker入口はfamily内の統合・削除と組にして原則純増ゼロ。既存安全性検証を帳尻で消さず、この規則用のcheckerや台帳も作らない。
- 実装を写すtest、専用`_for_test`フック、内部配置・エラー全文の固定を追加しない。source→MIR/実行/構造化refusalを優先する。
- Rc同一性等の検証は、それ自体が所有・権限契約で外から検証できない場合だけ。等価fixtureは既存test-supportへ統合し、独立した失敗診断・filterを保つ。
- アプリ修正に不要なfreezeトークンを増やさない。必要な失敗境界は既存診断ownerを優先し、既知の拒否を通すために無効化しない。
- shell/Pythonを含むguardも同じ増加規則。一時test/guard/adapterは既存cardでowner・置換先・`retire_when`を明記し、条件成立時に統合・削除する。
- 同じcheckoutでtop-level Cargoは一つ。稼働中のCargo/rustcが読むRustを編集しない。中断後は終了を確認して次を起動する。
- jobs上限目安4。root libのfocused testは`cargo quick-test-fast`、CLIは`cargo quick-build-fast`、検証runtimeは`cargo quick-build-lifecycle-fast`で同じtarget/quickを再利用する。本体opt0・依存quick opt1・incrementalを保ち、最適化CLIの測定は`cargo quick-build --bin hakorune`で分ける。
- 同じbuildをfilter実行で再利用する。filter0件は成功にしない。必要検証が通れば、新しい変更・失敗・要求gateがない限り繰り返さない。
- `--release`、`--nocapture`、warning抑制は必要性がある場合だけ。docs-onlyではCargoを起動しない。
- 固定suiteはアプリ着地・広い共有変更・製品acceptanceの確認時に実行し、毎内部commitでは回さない。落ちた別アプリを既知baseline扱いで閉じない。
- smokeのnegative precheckと直接EXEを分ける。意図した拒否境界の意味を確認し、単に期待エラーを現在の失敗へ差し替えない。
- `11/11`表示だけで達成にしない。空marker、SKIP、timeoutを区別し、必要な全EXE生成・実行・期待出力/exitと既存negative境界を確認する。
- 検証は対象revision/binary・command・結果を短く残す。build/link時間と実行時間を分け、古いbinaryの結果を現HEADの結果にしない。
- 整理の増減は既存差分・検索・互換binaryの`--list`で確認する。数えるためのbuildや定期checkerを増やさない。
- 赤は今回の変更・既知baseline・informationalを区別する。CIは通知か着地時に確認し、pending/cancelをPASSにせず、連打dispatchしない。

## 7. 文書とpointer

- `CURRENT_STATE.toml.latest_card_summary`は5行以内。アプリ、停止点、次の一手、WIP/未完条件への参照だけを書く。
- 選択app cardは100行以内。scope・現在のstop・必要契約の参照・短い修正履歴・残件を置く。内部作業ごとのcardは作らない。
- 証拠連番、ログpath、SHA一覧をcardへ積まない。必要なcommand/resultと一つの履歴参照に絞り、詳細はGitへ残す。`/tmp`だけを永続証拠にしない。
- 既存の長い設計資料は必要時のreferenceとし、日々追記しない。未解決の意味契約は削除せず、一つのownerへ参照を保つ。
- 通常更新はapp card、アプリ切替時のCSTATE、契約を変えたowner docsだけ。workstream等の鏡へ履歴を横展開しない。
- 置き場所は[DOCS_LAYOUT](current/main/DOCS_LAYOUT.md)。仕様変更は`docs/reference/**`を先に更新する。

## 8. コードの衛生

- production sourceは760行で分割を検討し800行を上限とする。圧縮で逃げず、必要な責務境界で整理する。testは行数だけで分割しない。
- 巨大な汎用基盤・並列の分類器/台帳/receiptを作らず、既存ownerに統合する。
- 形別の再分類・再探索は共通source/論理計画へ寄せ、同じ不変入力の検証結果を再利用する。変換後MIR/ABIの独立検証は保持し、必要なrelease拒否を`debug_assert!`へ落とさない。
- debug logは既定OFF。無条件`eprintln!`は禁止。dead-code許可には理由・利用予定・再判断条件を持たせる。

## 9. 完了の定義

- **app完了:** その既存smokeの全契約と正規EXE生成・実行・期待出力/exitが成功し、変更と必要検証をcommit済み。内部準備だけは完了に数えない。
- **固定製品acceptance:** `tools/smokes/v2/run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary`の必要全契約が成功。空markerやSKIPはEXEの証拠ではない。
- **MirBuilder/Gate1完了:** 固定製品acceptanceに加え、正本の必須source接続・production切替・R6/R7・Loop・物理退役・最終pipeline条件が成立する。後続Gate2〜4/selfhost条件は元の正本に従う。
- 同じsource authorityから最終consumerまで接続し、置換済みの専有edge/adapterが退役していること。削除予定だけを残して移行完了にしない。
- 着地時に一時証拠とadapterの保持/廃棄を判断する。進捗はアプリ成否・停止点の変化・責務統合・削除で報告する。

## 10. local entryと旧文書

- ignored AGENTS/CLAUDEはこのfileと現在地への短い入口。独自のルールや履歴を複製せず、ignore設定を勝手に外さない。
- 旧policy/schedulerの補助手順は必要時だけ参照する。運用が競合すればRULESを優先し、参照文書を追加の作業前提にしない。
- このfileは150行以内を目安とし、ルール変更時は重複を減らす。
