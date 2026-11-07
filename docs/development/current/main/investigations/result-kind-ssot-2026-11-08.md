# Callee result kind SSOT（受理不変、2026-11-08）

U1. 決定の正本は既存source証拠を合成する単一callee-result productとし、instance・bare・staticは同じcallee identityを索引にして読む。
U2. scalarとTextの証拠は既存CallableResultDisposition、objectとnullableおよびforwardの証拠は既存SourceCallableResultFactsから取り、欠落を別の結果種類として推測しない。
U3. ExactNominalBoxはobject結果の第二の決定にはせず同じobject証拠への投影とし、二つの証拠が異なるclassまたはkindを主張する場合は勝手に優先順位を付けず入力とcalleeを報告する。
U4. construction_result_calleeの全return同一構築検査とnullable_exits_holdの全return Null/構築検査は独立した受理検証としてのみ残せ、決定されたclass/kindを生成しない。
U5. 構築・forward・借用・Normal/Fault・所有と入力ABIの検証はkindの決定と分けて保持し、同じkindであっても既存の実行証拠が欠ける経路へ受理を広げない。
U6. is_normal_i64_result_v1は元returnとNormal-only整数の独立した検証に限定し、単一productと異なるkindを新たに決めない。
U7. MapとUnitも既存の検証済みsource/Completion証拠から単一productへ保持し、宣言名・MIR型・任意のMap式の存在だけから結果種類を決めない。
U8. kindから物理結果への写しはLocalCallResultClassV1::invoke_resultに一つとし、ordinary object claimの写しもそこへ委譲する。
U9. K13のgroup記録はkindではなく各siteの元評価groupと物理実行責任を照合してそろえ、同一にできない元証拠の差があれば段2を止める。
U10. K14の箱classとconstructor入力のi64要求は別の事実として保持し、要求が未証明でもclassをUnavailableへ降格せず既存の入力受理検証で拒否する。
U11. 各段で基準329入力と1459 dumpのexit・診断・MIRが完全一致することを確認し、違いが出た段を戻して基準へ合わせる修正はせず門を報告する。
U12. 仮比較は決定を変更しない観測だけに限定し、比較codeを残さず元sourceへ戻して再buildしてから最終検証する。
U13. 新たな分岐や責務境界が要る時は先にこの文書へ理由を追記し、公開MIR/opcode/ABI/descriptor/C shim・既存の検証・入力sourceを変更しない。

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
