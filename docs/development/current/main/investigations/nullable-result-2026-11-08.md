# Nullable object result（2026-11-08）

N1. 結果kindはCalleeResultKindCatalogV1のNullable(class)を読み、元callee identity・宣言・全return証拠を照合し、別の分類器を作らない。
N2. 所有nullableのlocalは同じ元Call結果を保持し、非nullのNormal取得だけがHomeを作り、各Normal/Fault出口は既存の条件付きEndで一度だけ消費する。
N3. nullのwire値は0であり、既存HomeReleaseIfLive/ObjectEndIfOwnedは元結果outcomeのNullを確認してkernelを呼ばず、kernelの通常release自体がnull-safeであるとは仮定しない。
N4. null比較は元nullable localの正規readとConst NullのCompare Eq/Neで表し、SSA binding・元Call・canonical objectと比較siteの対応を独立検証する。
N5. nullable receiverのCallは実行前にnonnullを検査し、nullではTypeErrorを同じCallのFault辺へ送り、引数評価と取得済みHomeのcleanupを保持し、nullをcalleeへ渡さない。
N6. N5は既存Invoke Callのchecked実行とFault辺で表せる範囲だけを実装し、Cの既存op受理拡張では公開キー/opcode/ABI/descriptorを追加せず、C/Rust双方の生成後検証を保持する。
N7. 早期Return後に残る経路のHome/read/groupは共通CFGの支配関係で保持し、nullable専用の形判定を作らず非nullの同じ入力も同じ規則で扱う。
N8. 宣言P?はcanonicalなPの結果検査であり、全returnがPまたはNullであれば片側だけでも許し、nullだけから未宣言のPを推論しない。
N9. 宣言のPはnull不可のまま、String?/scalar nullable・alias/rebind/forward/bare/配列保存は今回開かず、同じ既存の名前付き拒否を維持する。
N10. 各段は元source証拠から最終consumerまで接続し、既存成功入力のMIR/出力が変わればその箇所を止め、未対応を名前や失敗後のfallbackで埋めない。
N11. nullableの根拠を読む時はcatalog発行後であることを確認し、KnownValueClassの受理投影はadmission_factを使う。
N12. 受理不変の責務分割は別commitでMIR一致を確認し、新しい契約の証拠だけを既存test familyへ統合する。

N13. receiverの名義classは元Call取得とcatalogへ結び、nullableのclassが既知であることをnonnullや所有権の証明へ読み替えず、既存の元Call commitを物理readの検証でも使う。
N14. 元結果classからcanonical objectを照合する処理は共通queryへ寄せ、同classの全claimと移管済みResult commitが同じobjectであることを保持する。

## 段0: ownerと門

| 入力 | 現在の門 | 次の責務 |
|---|---|---|
| n01 | local_commit/root_call_entry/print_expression_projection.rs:204 source-unavailable | nullable localからreceiver/read/print frameへの共通source接続 |
| n02/n03/n04/n05 | ordinary_new_root_instance_call.rs:209 new-missing | 元Call取得とcatalogのclassをreceiver証拠へ、独立Call検証、null Fault |
| n08 | nullable local二つのread/receiver経路 | 同じCall identityごとのHome/End、別siteを混同しない |
| t02 | local_commit/root_call_entry.rs:385 binding-contracted | 早期Returnの元groupを共通CFG投影で保持 |
| n12 | main-import-view/selected-header-missing | static入力protocolとNullable結果を接続 |
| n11/n14/n21 | callable_parameter_contract/issuer.rs:66 UnsupportedResultType | 元宣言P?とcatalog/全return/physical結果の一致検証 |

pathの省略部分はsrc/mir/normal_callable_semantic_package配下。
null cleanupの実装根拠はlang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_v4_emit.inc:534–565（Nullではreleaseを呼ばない）であり、kernelのcrates/nyash_kernel/src/exports/fault_checked_object.rs:243–281は有効handleだけをreclaimする。
null receiverの既存C ordinary_callは同emit.inc:485以降がcalleeを直接呼び、indexed_flow.inc:161–166がnonnull receiverを要求するため、既存opのchecked受理と独立検証の両方を接続する必要がある。
N5/N6を既存公開契約で表せないと判明した時はその門を明記して保留し、他の段は続ける。
