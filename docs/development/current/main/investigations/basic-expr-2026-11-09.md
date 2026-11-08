# 基本式と格納しないtakeの接続

基点: main `41cd71f719`。今の `scan_statement_flow` と既存のchecked評価・cleanupを使う。e17（String fieldをlocalへ借りる公開契約）、値の`&&`、edit、CFG走査の変更は対象外。

## 原因の表（実装前の調査）

| 群・入力 | baseの拒否と元の判定（src/mir/以下） | 共通の不足責務 |
|---|---|---|
| G1 e01/e03/e15 | print-expression/source-unavailable、PrefixNotCovered。resolved_semantics/home_new_prefix_initializer_expression.rs:77 は算術initializerをLoopか別consumerからの要求に限定する。home_new_prefix_local_statement.rs:542 は別のscalar評価へ落とす。 | 既に法則の決まった算術のchecked評価を、initializerの行き先へ渡す接続。 |
| G1 e02/e04/e16/e20 | main-arguments/root-completion（normal_callable_semantic_package/ordinary_new_main_arguments.rs:41 は出口の再包装）。home_terminal_returned_value.rs:642,663 はreturnの式形とAdd内Call有無で入口を制限する。 | 同じ式の値と順序をreturnへ渡す。未完了prefixの原因とreturnの未選択を区別する。 |
| G2 e07 | root-instance-call-new-missing。ordinary_new_root_instance_call.rs:216,225 はreceiverをnew由来のuser objectに限定する。home_terminal_returned_value.rs:642 はmethod根を完全return評価へ選ばない。 | Stringの既存CoreMethod結果を、object-callの再分類をせず共通評価からreturnへ。 |
| G3 e10/e11 | array-holder/append-source CandidateSource（resolved_semantics/named_array_append_only_source.rs:265）。同file:198 は引数1個、:281 はgetを引数位置に限定、:304 はlength等を除外する。e11のreturnは上記の未選択も持つ。 | 既存Arrayのcandidate・read・length契約とEndを式の結果へ接続する。 |
| G3 e12 | main-arguments/root-completion。returnの共通入口制限に加え、get結果の正確な型・不在・cleanupをMapの元契約で閉じる必要がある。 | Mapの既存source/格納/結果契約から値を借り、MIRの見た目で整数と推測しない。 |
| G3 e13 | array-holder/append-source HolderEscape（named_array_append_only_source.rs:325）。 | 測定のみ。types.mdのdisplay表はString加算の契約であり、Array自体のprint書式を決めない。 |
| G4 e18 | local-commit/artifact-source-unavailable。home_new_prefix_condition.rs:488 はfield seedなしでleaf準備する。一方initializer/printはhome_new_prefix_field_read.rs:540のText field proofを使う。 | 即時式のText field読取と比較を同じ取得・Normal/Fault終了の評価へ渡す。 |
| G5 t01 | TakeBirthSourceUnconnected（ordinary_new_candidate.rs:290）。instance_construction.rs:55 のstore RHSとinstance_construction_body.rs:8の手順はfield格納中心。candidate.rs:307 はtake未格納を拒否する。 | calleeが受領したHomeの読取・移転・未格納Endを同じcleanup責任で閉じる。 |
| G5 t02 | borrowed-entry/finite-domain-unclosed（ordinary_new_borrowed_formal_entry_finite.rs:117）。 | 所有を受け取るformalの元型と全incomingを既存source domainへ接続する。 |
| G5 t03/t06 | borrowed-entry/source-use-completion-pending（ordinary_new_borrowed_formal_entry_contract.rs:651）。 | 受領後の全use/出口の契約とEndを閉じる。 |
| G5 t04/t05 | qualified-route-retired。callee入口・fresh actualの受領と終了が閉じず、正規qualified callが未選択。 | 無使用でも受領は消えず、fresh値も既存の受領契約へ渡す。 |

型・操作・責任が既に決まる部分で、利用先ごとに別の選択が置かれている点は相談§3の診断と一致する。ただしcontainerの型/不在とtakeの受領終了は実際の契約を閉じる必要があり、拒否名だけから同じ修正で全て通るとは扱わない。

仕様根拠: docs/reference/language/types.md §4、dynamic-operators.md、strings.mdのfieldの即時評価、ownership.md §0のmove/parameter/field規則とNormal/Fault責任。仕様にない表示や公開借用は推測しない。

## 規則（実装より先）

B1. 式のsource・順序・型・操作法則・取得責任は既存checked評価で一度確定し、local・print・return・引数・条件は結果の行き先と制御辺を指定する。
B2. 既に通る入力の既存選択と発行順を保存し、適用可能性は効果より前に決め、選択した後の失敗で別評価へretryしない。
B3. 正確なi64算術は共通operator契約と元のoperand証拠で認め、consumerの要求有無やLoop内外を算術の意味にしない。
B4. scalarのlocalと再代入は同じchecked producerを保持し、aliasや利用先から値を再計算せず、元bindingと現在のsource位置を照合する。
B5. returnのscalarとCoreMethod結果は同じchecked評価の結果を既存Return/cleanupへ渡し、Stringのreceiverをuser objectのnew由来へ偽装しない。
B6. Text fieldの即時読取は元fieldの証拠から独立copyを一度取得し、比較の両経路とFaultで同じ評価が終え、e17の借用local契約を追加しない。
B7. Array/Mapの候補・read・countは既存のcontainer契約と型/不在証拠に基づいて評価し、containerの所有・候補の取得・Normal/Fault Endを既存ownerへ接続する。
B8. take受領前Faultはcaller、受領後はcalleeが所有し、fieldへ移すかどうかと独立に全Normal/Fault出口で一度移転またはEndし、move後の利用拒否を保存する。
B9. 置換したconsumer別の判定と引渡しは同じ責務の変更で削除し、source適合と物理対応と生成後の独立検証は残す。
B10. 算術のdomain・除数の証明・overflow/Fault・Text変換は既存法則を保持し、未証明値をDynamicへ落として通さない。
B11. 今のsource walkと既存CFGの責任を維持し、短絡の未実行枝に取得や効果を発行せず、値の短絡とeditのCFG設計を先取りしない。
B12. 公開MIR/ABI等または走査/意味契約の変更が必要な群は、試した接続とfile:lineと不足物を記録してその群だけ止め、他群を進める。

B13. 元のscalar appendが既に決まるinitializerはその発行を保持し、checkedな元演算の結果をCurrentScalarReadへ貸し、共通式consumerのためにinitializerを再発行しない。
B14. 元のi64演算は全operandのsource証拠と共通演算法則を保持し、演算結果の読取は元appendとlocal束縛の対応を照合し、型や同形のMIR検索からproducerを捏造しない。

初回の算術initializer選択拡大はe01/e03/e15がEXE成功したが、既存21本のMIRが変わったため不採用。古いscalar appendを新しいframe発行で置換したことが原因（既存再帰のID/発行順、既存Callの経路が変化）。試作はpatchへ保存して戻し、B13の元producer貸出へ切り替える。

B15. 比較の入力scopeは同じchecked評価のfield読取も含み、そのreceiverのIdentityは値の取得として再評価せず、field copyの取得とNormal/Fault Endを同じ入力順序に保持する。

B16. Core結果を新しい利用先へ渡す時は元Core loanで結果のclassを効果前に選び、同じloanを一度だけ葉の準備へ渡し、準備済みの葉を共通の値評価が消費して再分類・再発行しない。
