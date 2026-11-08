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

N15. 既存Invoke CallのC投影はreceiverが0なら既存TypeError理由302をfault frameへ記録して同じFault landingへ進み、非0の時だけcalleeへdispatchし、生成後検証は元nullable Callのobject対応・生存とNormal取得を独立に確認する。

N16. 早期Return後のCall群の投影は既存FinishedBindings::bindingsと同じ透明Jump縮約規則を使い、外側Invoke/結果/Frameと全effectの存続を別に要求する。
N17. C emitterの800行境界では既存Callの実引数準備をinvocation emitterへ責務移管し、意味不変の別commitと比較後にreceiver検査を同じ準備へ接続する。

N18. null receiverのFaultはcallee入場前なので、移転引数をcalleeがFaultでも消費する既存契約だけでは表せない呼び出しは拒否し、今回のnullable receiverの追加受理はその移転を持たないCallに限る。

N19. 新しい所有nullable receiverの取得は既存nullable_exits_holdの構築/Null出口の受理検証も共有し、種類/classが既知でもforwardするcalleeを今回の受理へ加えない。

N20. 最終handoffでは投影済みflat storageの座標と不変な元packetの座標を分けて検証し、Root Callへ付くBool等のprefixにも元projectionを渡し、未投影時の検証契約は変えない。

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

## 段1の結果

n01/n02/n03/n04/n05/n08は正規EXEで期待どおり（0/3/1/3/70/3、n01出力3、n05理由302）。非nullのt02/t02dもexit3。n15を含む指定範囲外とArray借用4形は名前付き拒否を保持した。
新規証拠は既存の所有Call-result familyに、fieldを読まないメソッドへのnullable receiver・途中のscalar条件/早期Return・Normal/TypeError/End Faultの解放数・別classへの改変拒否を統合し、既存の構築/cleanup Faultの証拠も保持した。
C emitterの実引数準備は意味不変で分割し、3 EXEと33 dumpの一致を確認してからchecked dispatchを追加した。

N21. Static の元 source proof は同じ nominal class と literal Null の出口を合流し、nullable bit と元の引数要件を保持するが、Null だけから class を作らず、未知・競合する admission を成功へ変えない。
N22. Null は source value-class の不在値の証拠に限り、Unit の Void と区別し、catalog は Null 単独に object kind を発行せず、nominal nullable の証拠と既存の独立した return 検証を照合する。
N23. Static の Nullable 結果は既存の received Home と conditional End に接続し、元 call の source target・canonical object・引数証明・packet 投影を Handle と同じ共通経路で保持する。
N24. 独立 MIR 検証は既存の StaticBoxMethod Call に NullableHandle を認め、canonical target・arity・callee result と公開 return/lease の照合をすべて保持し、free function や別の ABI を同時に開かない。

Stage 2: n12 canonical EXE prints 3 / exit 0; instance n01 and early-return nonnullable t02 retain their results; forward n15 stays refused. Callable result family: 99 pass. One semantic test adds nullable class/input-obligation proof and conflicting-class/null-only/Void refusals to the existing source-proof family; legacy nonnullable expectations now state the nullable bit explicitly.

N25. 所有結果の `P?` は canonical TypeReference の Nullable(Name(P)) と ordinary box coverage から認め、本文の全 return が P または literal Null に合うことを検査し、P だけ・Null だけも同じ nullable 契約に含む。
N26. 宣言済み Null-only の nominal class は元の宣言からだけ供給し、元の constructor object registry の実在 ID を共有して conditional End を計画するが、架空 ID・New claim・非nullの証拠を作らない。
N27. Null-only の公開 return は C verifier が元の全 return の const_null を独立に証明し、null の条件付き解放だけを許し、通常の所有移転・無条件 End・非null借用の証拠として使わない。
N28. P? の本文と宣言の照合は source catalog と元の return witness、最終 MIR/ABI の既存検証を保持し、非nullable P の null、別 class、String? と scalar nullable を受けない。
N29. 検証済み元 return cohort が Null だけなら SUM は Null 単独の事実を保持でき、object class や nonnull/input ABI の証明には使わず、teardown の条件付き性は本文の枝数ではなく catalog の結果契約に合わせる。
N30. 独立検証の到達解析は公開 MIR の canonical Static Call も既存の callable-key owner で辿り、未解決辺は従来どおり保守的に扱い、到達する関数と物理関数の range 検証をすべて保持する。
N31. catalog が owned Nullable と決めた関数は New site が無くても共通 Home/terminal walk を選び、全 Null の検証済み teardown は既存の descriptor=None/nullable=true/unsupported=false で payload 不在を保持し、packet はそれを conditional End だけへ接続する。
N32. scalar-source 観測で construction のない Null cohort に遭遇しても、catalog が owned Nullable と証明していれば Nullable のまま保持し、I64 や別 class へ変換しない。
N33. C の元 return 解析は exact-object と Null-only を別の内部状態で共有し、Null-only は object_id を発行せず、callee の全 Normal return が const_null の時だけ checked receiver の Fault-before-dispatch に使える。
N34. 新しく接続する Root receiver の CallReceived 取得は canonical root-scope Local initializer に限り、分岐内取得の receiver protocol は今回追加せず、既存の direct-local 判定を共有して拒否する。

Stage 3: n11/n14/n21 canonical EXE prints 3 / exit 3 / prints 0; all ten required forms and eleven excluded forms match their expected results. Fresh-only declaration, static Null-only, renamed class, three returns and Null-only literal method also run correctly. Always-null typed-field getter remains a named compiler coverage refusal.
Evidence: callable result family 99 pass, root instance family 4 pass, owned result family 1 pass (new six captures plus unchanged original cases), C owned execution/cleanup/class mismatch and integer-zero-versus-Null mutations pass. Scope-negative is integrated into that existing family; no test or checker file is added. Full-series regression validation follows this commit.

Final review correction (N11/N21): a retained Null class must not erase Unknown/Pending call admission. Normalize admission before checking Pending and exclude only an actual KnownNonI64 Null from nominal return reduction. A qualified-call semantic negative now covers mixed and declared-nullable wrappers around an unavailable unannotated null-only provider. The old parameter-catalog test's Leaf? result refusal is replaced by the approved acceptance, while nullable parameters and String/scalar/unknown/nested/generic nullable result refusals remain checked. The first lib run and its deterministic obsolete-assert failures are retained separately; final validation restarts after this correction.
