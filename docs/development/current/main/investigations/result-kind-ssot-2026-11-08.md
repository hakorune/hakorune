# Callee result kind SSOT（受理不変、2026-11-08）

U1. 決定の正本は既存source証拠を合成する単一callee-result productとし、instance・bare・staticは同じcallee identityを索引にして読む。
U2. staticのscalar/Text/構築式の証拠は既存CallableResultDisposition、instanceのscalarは元return結果の証拠、object/nullable/forwardはSourceCallableResultFactsから同じproductへ集め、証拠の欠落を否定や別のkindとして推測しない。
U3. ExactNominalBoxとordinary objectのclass投影は単一productの同じclass/kindを読み、ordinaryの範囲外のnominal証拠を欠落だけで消さず、異なるclassまたはkindを主張する正の証拠は勝手に優先順位を付けず入力とcalleeを報告する。
U4. construction_result_calleeの全return同一構築検査とnullable_exits_holdの全return Null/構築検査は独立した受理検証としてのみ残せ、決定されたclass/kindを生成しない。
U5. 構築・forward・借用・Normal/Fault・所有と入力ABIの検証はkindの決定と分けて保持し、同じkindであっても既存の実行証拠が欠ける経路へ受理を広げない。
U6. is_normal_i64_result_v1の元return整数証拠は単一productへ一度取り込み、consumerの整数判定は同productを読み、全returnとNormal-onlyの独立した検証は種類を作り直さず保持する。
U7. MapとUnitも既存の検証済みsource/Completion証拠から単一productへ保持し、宣言名・MIR型・任意のMap式の存在だけから結果種類を決めない。
U8. kindから物理結果への写しはLocalCallResultClassV1::invoke_resultに一つとし、ordinary object claimの写しもそこへ委譲する。
U9. K13のgroup記録はkindではなく各siteの元評価groupと物理実行責任を照合してそろえ、同一にできない元証拠の差があれば段2を止める。
U10. K14の箱classとconstructor入力のi64要求は別の事実として保持し、要求が未証明でもclassをUnavailableへ降格せず既存の入力受理検証で拒否する。
U11. 各段で基準329入力と1459 dumpのexit・診断・MIRが完全一致することを確認し、違いが出た段を戻して基準へ合わせる修正はせず門を報告する。
U12. 仮比較は決定を変更しない観測だけに限定し、比較codeを残さず元sourceへ戻して再buildしてから最終検証する。
U13. 新たな分岐や責務境界が要る時は先にこの文書へ理由を追記し、公開MIR/opcode/ABI/descriptor/C shim・既存の検証・入力sourceを変更しない。

U14. static契約catalogはstatic宣言だけを対象としておりinstanceのNormal-i64正例550組には契約行が無かったため、U2/U6は元のinstance結果証拠を捨てず同じkind決定へ取り込む。
U15. 先に必要なstatic入力証拠と後で揃うresolved結果証拠は単一productの段階的な発行で扱い、既知のkind/classを書き換えず追加の証拠を照合する。
U16. kindの一致だけでは実行を認めず各経路の元の入力・ownership・group・publication証拠と拒否のownerを保持する。

U17. 758行のdirect lifecycleへkind集約を接続する前にMap/構築returnの元検証を責務moduleへ意味不変で移し、独立commitのbinaryで両集合のMIR一致を確認する。

## 段0のownerと読む所（基点3f97209c9d）

| 証拠/決定 | 元owner | 直接の読む所 |
|---|---|---|
| scalar/Text/nominal契約 | callable_result_representation/disposition.rs:38 | qualified_static_call_claim.rs:154 と同:219、direct_call_static_execution.rs、physical_header.rs |
| object/nullable/forward | ordinary_new_result_class_claim.rs:91、同product.rs:48 | source claim/lexical/direct/object-return/field/borrow各consumer（型・API別の全検索一覧は実測報告に添付） |
| 同一class構築return | direct_call_lifecycle.rs:111 | 同:480、同:525、ordinary_new_coseal_issue_lexical.rs:186 |
| Null/構築return | ordinary_new_coseal_issue_lexical.rs:203 | 同:183 |
| Normal-only i64 | ordinary_new_borrowed_formal_result_original.rs:472 | ordinary_new_coseal_issue_lexical.rs:192 とborrowed結果の検証consumer |
| 物理の写し | home_local_source_scalar_call.rs:38、ordinary_new_result_class_claim.rs:106 | local Call route/emission、object/nullable publication |
| Map/Unit既存の門 | direct_call_lifecycle.rs:49,68、result_contract.rs | direct/local/result publication（kindと受理条件を混ぜず調べる） |

段0の一時比較では上の1〜4とNormal-i64を同じ元callee identityで記録し、未到達入力とUnavailable・異なるkind/class・検証による拒否を分けて数える。

## 段0実測（比較専用codeは撤去済み、再build中）

329入力と1459入力のunion1788件を基準と一時観測binaryへ通し、両集合でexitとdump本文の差は0、timeoutも0だった。
比較点に達した入力は329中320、1459中1341であり、それより前に拒否する入力を判定一致の証拠に数えない。
観測した入力/calleeは7497組で、1と2が正の異なるclassを示す例、Normal-i64が正で別の正のkindを示す例は無かった。
object claimと直接構築検証の範囲差、nullable claimとNull/直接構築検証の範囲差、ordinary外nominalの範囲差は合計35行あり、入力とcalleeの全件は実測報告へ添付する。
追加の決定/投影としてdirect_call_result.rs:13のterminal-result分類と同:61の全exit分類があり、Map/Unitと所有ABIの証拠を調べる必要がある。
