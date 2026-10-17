//! Original initializer source contracts through the production argument port.
use super::*;

#[test]
fn initializer_text_entry_read_retains_all_incoming_and_original_alias_provenance() {
    use crate::mir::resolved_value_profile::expression_plan::{local_expression_source_plan_v1, ExpressionNodeSourceV1};
    use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
    for (prefix, actual, sibling, accepted) in [
        ("", "\"field\"", "", true),
        ("local alias = key", "\"field\"", "", true),
        ("key = other", "\"field\"", "", false),
        ("", "null", "", true), ("", "true", "", false), ("", "7", "", true),
        ("", "\"field\"", "second() { return Values.caller(true, \"spare\") }", false),
        ("", "\"field\"", "relay(p) { return Values.caller(p, \"spare\") }", false),
    ] {
        let read_name = if prefix.contains("local alias") { "alias" } else { "key" };
        let text = format!(r#"static box Values {{
            caller(key, other) {{ {prefix} local needle = "x" + {read_name} return 0 }}
            seed() {{ return Values.caller({actual}, "spare") }} {sibling}
        }} static box Main {{ main() {{ return 0 }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        package.batch().with_lowering_input(contract.batch_slot, |input| {
            let initializer = input.function().expression_source().initializers().last().unwrap();
            let original = input.forest().callable_source_ledger(input.owner()).unwrap();
            let plan = local_expression_source_plan_v1(&original, input.body_shape(), initializer).unwrap();
            let read = plan.nodes().iter().find_map(|node| match node.source() {
                ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) => Some((node.site(), *binding)), _ => None,
            }).unwrap();
            let site = OwnedExprSiteV1::new(input.owner(), initializer.initializer_site().unwrap().clone());
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let before = pending.clone();
            let mut ask = |plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, root| {
                borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()),
                    &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                    &ledger.borrowed_i64_results, &mut |_| None, &site,
                    BorrowedCallActualRequestV1::InitializerTextEntryRead { input, plan, read: read.0, binding: read.1, root },
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims).unwrap()
            };
            let row = ask(&plan, contract.parameters[0].binding);
            assert_eq!(row.is_some(), accepted, "{prefix:?}/{actual}");
            if let Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan)) = row {
                assert!(Rc::ptr_eq(loan.requirement().source_plan(), &plan));
                assert_eq!(loan.leaf_class(None).is_some(), actual != "null", "source Null still requires the real terminating branch");
                assert_eq!(loan.requirement().formal(), (0, contract.parameters[0].binding));
                assert_eq!(Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan.clone())), ask(&plan, contract.parameters[0].binding));
                let reissued = local_expression_source_plan_v1(&original, input.body_shape(), initializer).unwrap();
                assert_ne!(Some(BorrowedCallArgumentsV1::InitializerTextEntryRead(loan)), ask(&reissued, contract.parameters[0].binding));
            }
            assert_eq!(ask(&plan, contract.parameters[1].binding).is_some(), prefix == "key = other", "only the original current entry can supply provenance");
            assert_eq!(before, pending, "source demand does not arm execution or stage actuals");
        }).unwrap();
    }
}

#[test]
fn text_initializer_frame_requires_original_text_signed_source_and_current_prefix() {
    for (annotation, actual, mutation, accepted) in [
        (": i64", "\"text\"", 0, true), (": usize", "\"text\"", 0, false),
        ("", "\"text\"", 0, true), (": i64", "true", 0, false),
        (": i64", "\"text\"", 1, false), (": i64", "\"text\"", 2, false),
        (": i64", "\"text\"", 3, false), (": i64", "\"text\"", 4, false),
    ] {
        let text = format!(r#"static box Values {{
            caller(line, count{annotation}) {{ local needle = "x" + line + count
                local alias = needle local next = alias + "!" return 0 }}
            seed() {{ return Values.caller({actual}, 7) }}
        }} static box Main {{ main() {{ return Values.seed() }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        package.batch().with_lowering_input(contract.batch_slot, |input| {
            let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let mut batches = 0;
            let mut requests = 0;
            let (plans, homes) = crate::mir::resolved_semantics::home_new_prefix::observe_local_initializers_for_test(
                input, parameters, mutation != 1,
                match mutation { 2 => Some(contract.parameters[0].binding), 4 => Some(contract.parameters[1].binding), _ => None },
                &mut |fields, _| -> Result<_, OrdinaryNewCoSealIssueV1> { batches += 1; assert!(fields.is_empty()); Ok((mutation != 3).then(Vec::new)) },
                &mut |site, request| {
                    if matches!(request, BorrowedCallActualRequestV1::InitializerTextEntryRead { .. }) { requests += 1; }
                    borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &package.parameter_contracts,
                        &[], None, source, &Ok(Default::default()), &mut pending, &ledger.borrowed_i64_results,
                        &mut |_| None, site, request, &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)
                }).unwrap();
            assert_eq!(plans.len(), if accepted { 2 } else { 0 }, "{annotation}/{actual}/{mutation}");
            assert_eq!(homes.len(), if accepted { 2 } else { 0 });
            assert_eq!(batches, if accepted { 2 } else { usize::from(mutation == 3) });
            if mutation == 1 { assert_eq!(requests, 0, "unknown prefix cannot qualify current state"); }
            if accepted {
                let first = plans[0].initializer_frame().unwrap();
                let second = plans[1].initializer_frame().unwrap();
                assert_eq!(first.entry_reads().len(), if annotation.is_empty() { 2 } else { 1 });
                assert_eq!(first.scalar_reads().len(), usize::from(!annotation.is_empty()));
                assert_eq!(first.evaluation().owned_result_nodes().len(), 2);
                assert_eq!(second.residence_reads().len(), 1);
                assert!(Rc::ptr_eq(second.residence_reads()[0].acquisition(), first));
                assert_eq!(second.residence_reads()[0].root(), first.binding());
                assert_ne!(second.residence_reads()[0].binding(), first.binding(), "alias is a read, not a second owner");
                assert_eq!(second.prior_homes(), [first.binding()]);
                crate::mir::resolved_semantics::home_new_prefix::assert_text_residence_identity_for_test(input, Rc::clone(first), second);
                // The actual production scan retains the same complete frame,
                // not just this test's independent pre-evaluation walk.
                let flow = ledger.completion_for_owner(input.owner()).unwrap().cleanup().root_flow().unwrap();
                let retained = flow.expression_plans().iter().find_map(|plan| plan.initializer_frame()).unwrap();
                let selected = ledger.initializer_expression_frame_v1(input.owner(), retained.source().root().statement().node()).unwrap().unwrap();
                assert!(Rc::ptr_eq(retained, &selected));
            }
        }).unwrap();
    }
}

#[test]
fn owned_text_initializer_refuses_execution_until_the_same_physical_consumer_is_complete() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    for (initializer, selected) in [("\"hello\"", false), ("\"he\" + \"llo\"", true)] {
        let text = format!("static box Main {{ main() {{ local text = {initializer} return 0 }} }}");
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            &text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source") };
        let result = crate::mir::MirCompiler::with_options(false).compile_normal(
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()));
        if selected {
            let issue = result.unwrap_err().to_string();
            assert!(issue.contains("text-initializer/execution-pending"), "{issue}");
        } else {
            assert!(result.is_ok(), "existing non-owning literal initializer stays with its owner: {result:?}");
        }
    }
}

#[test]
fn finite_initializer_entry_uses_the_original_terminating_null_arm_and_both_add_envelopes() {
    for (guard, middle, accepted) in [
        ("if value == null { return 0 }", "", true),
        ("if null == value { return 0 }", "local alias = value", true),
        ("if value != null { } else { return 0 }", "", true),
        ("if value == null { local ignored = 0 }", "", false),
        ("if value == null { local ignored = \"\" + true return 0 }", "", false),
        ("if value != null { return 0 }", "", false),
        ("if value == null { return 0 }", "value = other", false),
    ] {
        let read = if middle.starts_with("local alias") { "alias" } else { "value" };
        let text = format!(r#"static box Values {{
            parse(value, other) {{ {guard} {middle} local out = "" + {read} return 0 }}
            textSeed() {{ return Values.parse("bytes", null) }}
            integerSeed() {{ return Values.parse(7, null) }}
            nullSeed() {{ return Values.parse(null, null) }}
        }} static box Main {{ main() {{
            local text = Values.textSeed() local integer = Values.integerSeed()
            local absent = Values.nullSeed() return 0
        }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        let frame = ledger.completion_for_owner(contract.owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| flow.expression_plans().iter().find_map(|plan| plan.initializer_frame()));
        assert_eq!(frame.is_some(), accepted, "{guard}/{middle}; role={:?}; completion={:?}",
            package.selected.role_for_batch_slot(contract.batch_slot),
            ledger.completion_for_owner(contract.owner).map(|completion| completion.cleanup().root_flow()));
        if let Some(frame) = frame {
            assert!(frame.evaluation().reference_coverage_is_exact());
            assert_eq!(frame.entry_reads().len(), 1);
            assert_eq!(frame.entry_refinements().len(), 1);
            assert!(frame.entry_reads()[0].leaf_class(None).is_none(), "original SUM still contains Null");
            let refinements = frame.entry_refinements();
            let proof = refinements.values().next().unwrap();
            assert!(frame.entry_reads()[0].leaf_class(Some(proof)).is_some());
            let operation = frame.evaluation().operations().iter().find(|operation| operation.node() == 0).unwrap();
            assert_eq!(operation.envelopes().len(), 2);
            assert!(operation.single_envelope().is_none(), "physical writer must not select one alternative");
            assert!(operation.envelopes().iter().all(|envelope| envelope.normal_result() ==
                crate::mir::dynamic_operator_contract::DynamicOperatorNormalResultV1::OwnedStringValue));
            assert!(frame.owns_root());
            let selected = ledger.initializer_expression_frame_v1(contract.owner, frame.source().root().statement().node()).unwrap().unwrap();
            assert!(Rc::ptr_eq(frame, &selected), "the real production selector retains this SAME complete frame");
        }
    }
}

#[test]
fn original_text_argument_product_keeps_static_residence_entry_and_core_distinct() {
    for (prefix, argument, incoming, expected, static_reads, residence_reads, entry_reads, owned) in [
        ("", "\"bytes\"", "\"seed\"", true, 0, 0, 0, false),
        ("local key = \"bytes\"", "key", "\"seed\"", true, 1, 0, 0, false),
        ("local key = \"bytes\" local alias = key", "alias", "\"seed\"", true, 1, 0, 0, false),
        ("local key = \"bytes\" key = 7", "key", "\"seed\"", false, 0, 0, 0, false),
        ("local key = \"bytes\" key = \"bytes\"", "key", "\"seed\"", false, 0, 0, 0, false),
        ("local key = \"a\" + \"b\"", "key", "\"seed\"", true, 0, 1, 0, false),
        ("local key = \"a\" + \"b\" local alias = key", "alias", "\"seed\"", true, 0, 1, 0, false),
        ("", "line", "\"seed\"", true, 0, 0, 1, false),
        ("local alias = line", "alias", "\"seed\"", true, 0, 0, 1, false),
        ("", "line", "7", false, 0, 0, 0, false),
        ("", "line", "null", false, 0, 0, 0, false),
        ("local key = \"bytes\"", "key.substring(0,1)", "\"seed\"", true, 1, 0, 0, true),
        ("local key = \"bytes\" local alias = key", "alias.substring(0,1)", "\"seed\"", true, 1, 0, 0, true),
        ("local key = \"a\" + \"b\"", "key.substring(0,1)", "\"seed\"", true, 0, 1, 0, true),
        ("", "line.substring(0,1)", "\"seed\"", true, 0, 0, 1, true),
        ("", "\"aé界z\".substring(0,1)", "\"seed\"", true, 0, 0, 0, true),
        ("", "\"aé界z\".substring(\"aé界z\".indexOf(\"é\"),\"aé界\".length())", "\"seed\"", true, 0, 0, 0, true),
        ("local key = \"bytes\" local needle = \"bytes\"", "key.substring(needle.length(),1)", "\"seed\"", true, 2, 0, 0, true),
        ("local key = \"bytes\"", "key.substring(key.indexOf(\"y\"),key.length())", "\"seed\"", true, 3, 0, 0, true),
    ] {
        let (caller_parameters, caller_actual) = if static_reads > 0 { ("", String::new()) }
            else if entry_reads > 0 { ("line, count: i64", format!("{incoming}, 7")) }
            else { ("line", incoming.to_owned()) };
        let extra_seed = if entry_reads > 0 { "text_seed() { return Values.caller(\"seed\", 7) }" } else { "" };
        let text = format!("static box Values {{ sink(p) {{ return 0 }} caller({caller_parameters}) {{ {prefix} local n = Values.sink({argument}) return 0 }} seed() {{ return Values.caller({caller_actual}) }} {extra_seed} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap();
        let declaration = package.batch().declarations().find(|row|
            package.batch().with_lowering_input(row.batch_slot(), |input|
                input.function().method_calls().any(|(_, call)| call.selector() == "sink")).unwrap()).unwrap();
        package.batch().with_lowering_input(declaration.batch_slot(), |input| {
            let contract = package.parameter_contracts.iter().find(|row| row.owner == input.owner()).unwrap();
            let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
            let mut pending = ledger.borrowed_formal_actuals.clone();
            let (plans, _) = crate::mir::resolved_semantics::home_new_prefix::observe_local_initializers_for_test(
                input, parameters, true, None,
                &mut |fields, _| -> Result<_, OrdinaryNewCoSealIssueV1> { assert!(fields.is_empty()); Ok(Some(Vec::new())) },
                &mut |site, request| borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()),
                    &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                    &ledger.borrowed_i64_results, &mut |_| None, site, request,
                    &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)).unwrap();
            let call_site = input.function().method_calls().find(|(_, call)| call.selector() == "sink")
                .map(|(site, _)| OwnedExprSiteV1::new(input.owner(), site.clone())).unwrap();
            let caller = CanonicalSameModuleCallableKeyV1::static_box_method("Values", "caller", contract.parameters.len() as u32);
            let (claim, _) = package.source_static_claims_for_test.claim_target(&caller, call_site.site()).unwrap();
            let frames: Vec<_> = plans.iter().filter_map(|plan| plan.argument_frame()).collect();
            assert_eq!(frames.len(), usize::from(expected), "{prefix}/{argument}/{incoming}");
            if let Some(frame) = frames.first() {
                let projected = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::project_pending_static_source_arguments_v1(
                    source, &pending, &call_site, claim).unwrap().expect("qualified original Text source projection");
                assert_eq!(projected.as_ref(), &[LocalCallArgumentV1::BorrowedActual { ordinal: 0,
                    site: frame.source().nodes()[0].site().clone() }], "same source tuple across all Text roles");
                assert!(Rc::ptr_eq(frame.source(), frame.source_loan().plan()));
                assert!(frame.evaluation().reference_coverage_is_exact());
                assert_eq!(frame.evaluation().references().len(), static_reads + residence_reads + entry_reads + frame.references().len());
                if entry_reads > 0 && !owned {
                    let mut unqualified = pending.clone();
                    let read = frame.text_reads()[0];
                    let actuals = [BorrowedCallActualCandidateV1 { ordinal: 0, site: read.read().clone(),
                        value: BorrowedCallActualValueV1::SelfRooted { binding: read.binding(), root: read.formal().1 } }];
                    borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()), &package.parameter_contracts,
                        &[], None, source, &Ok(Default::default()), &mut unqualified, &ledger.borrowed_i64_results,
                        &mut |_| None, &call_site, BorrowedCallActualRequestV1::Observe(input, &actuals),
                        &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims).unwrap();
                    assert!(crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::project_pending_static_source_arguments_v1(
                        source, &unqualified, &call_site, claim).unwrap().is_none(), "unqualified SelfRooted does not gain selection");
                }
                assert_eq!(frame.static_reads().len(), static_reads);
                assert_eq!(frame.residence_reads().len(), residence_reads);
                assert_eq!(frame.text_reads().len(), entry_reads);
                assert_eq!(frame.evaluation().owned_result_nodes().contains(&0), owned);
                assert_eq!(frame.source_loan().resolver_source().is_some(), owned);
                assert_eq!(frame.evaluation().cores().len(), if !owned { 0 } else if argument.contains("indexOf") { 3 } else if argument.contains("length") { 2 } else { 1 });
                for (node, core) in frame.evaluation().cores() {
                    assert!(core.matches_plan(frame.source(), *node));
                    let original = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::lend_expression_core_source_v1(
                        input, source.as_ref().unwrap(), frame.source(),
                        &OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[*node].site().clone())).unwrap().unwrap();
                    assert_eq!(core, &original, "SAME canonical Core Rc, plan and node");
                    assert!(!frame.evaluation().transfers_node(*node));
                }
                if owned {
                    assert!(std::ptr::eq(frame.evaluation().cores()[&0].source(), frame.source_loan().resolver_source().unwrap()));
                }
                if owned {
                    crate::mir::MirBuilder::assert_checked_expression_core_for_test(input, package.ordinary_new_claim_ledger_for_test(), frame);
                }
                let parameters = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
                let (repeated, _) = crate::mir::resolved_semantics::home_new_prefix::observe_local_initializers_for_test(
                    input, parameters, true, None,
                    &mut |fields, _| -> Result<_, OrdinaryNewCoSealIssueV1> { assert!(fields.is_empty()); Ok(Some(Vec::new())) },
                    &mut |site, request| borrowed_call_arguments_callback_v1(package.batch(), &package.selected, &Ok(Vec::new()),
                        &package.parameter_contracts, &[], None, source, &Ok(Default::default()), &mut pending,
                        &ledger.borrowed_i64_results, &mut |_| None, site, request,
                        &ledger.callable_result_classes, &ledger.receiver_call_observations, &ledger.field_write_claims)).unwrap();
                let repeated = repeated.iter().find_map(|plan| plan.argument_frame()).unwrap();
                assert!(Rc::ptr_eq(frame.source(), repeated.source()), "source identity survives a fresh Home walk");
                if let Some(read) = repeated.residence_reads().first() {
                    assert!(!Rc::ptr_eq(read.acquisition(), frame.residence_reads()[0].acquisition()),
                        "a new Home walk has its own actual acquisition proof");
                }
                if let Some(read) = frame.static_reads().first() { assert_eq!(read.utf8(), "bytes"); }
            }
        }).unwrap();
    }
}

#[test]
fn initializer_core_i64_reads_require_original_producers_in_the_real_home_walk() {
    use crate::mir::resolved_semantics::home_new_prefix::{CheckedExpressionReferenceV1 as Reference, CurrentScalarProducerV1 as Producer};
    for actual_input in ["\"bytes\"", "true", "1"] {
    for (prefix, operand, annotation, expected) in [
        ("", "count", ": i64", Some("entry")),
        ("local alias = count", "alias", ": i64", Some("entry")),
        ("local start = 1", "start", ": i64", Some("literal")),
        ("local start = 1 local alias = start", "alias", ": i64", Some("literal")),
        ("local start = Values.find()", "start", ": i64", Some("call")),
        ("local start = Values.find() local alias = start", "alias", ": i64", Some("call")),
        ("local start = line.length()", "start", ": i64", Some("initializer")),
        ("local start = line.substring(0, 1).length()", "start", ": i64", Some("initializer")),
        ("local start = count + line.length()", "start", ": i64", Some("initializer")),
        ("local start = count + line.length() local alias = start", "alias", ": i64", Some("initializer")),
        ("local start = 1 start = 2", "start", ": i64", None),
        ("local start = 1 if count > 0 { start = 2 }", "start", ": i64", None),
        ("local start = 1 loop count > 0 { start = start + 1 break }", "start", ": i64", None),
        ("", "count", ": usize", None),
    ] {
        let text = format!(r#"static box Values {{
            find() {{ return 1 }}
            caller(line, count{annotation}) {{ {prefix} local out = {operand} + line.length() return 0 }}
            seed() {{ return Values.caller({actual_input}, 1) }}
        }} static box Main {{ main() {{ return Values.seed() }} }}"#);
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let contract = package.parameter_contracts.iter().find(|row| row.parameters.len() == 2).unwrap();
        package.batch().with_lowering_input(contract.batch_slot, |input| {
            let initializer = input.function().expression_source().initializers().last().unwrap();
            let flow = ledger.completion_for_owner(input.owner()).and_then(|completion| completion.cleanup().root_flow());
            let frame = flow.and_then(|flow| flow.expression_plans().iter().filter_map(|plan| plan.initializer_frame())
                .find(|frame| frame.binding() == initializer.binding()));
            assert_eq!(frame.is_some(), expected.is_some() && actual_input == "\"bytes\"", "{prefix}/{operand}/{annotation}/{actual_input}; completion={:?}", ledger.completion_for_owner(input.owner()));
            let Some(frame) = frame else { return };
            assert!(frame.is_i64());
            assert!(!frame.is_text_residence());
            assert!(!frame.owns_root());
            assert_eq!(frame.evaluation().cores().len(), 1);
            assert_eq!(frame.entry_reads().len(), 1, "the Core receiver remains a mandatory original Text read");
            let selected = ledger.initializer_expression_frame_v1(input.owner(), frame.source().root().statement().node()).unwrap().unwrap();
            assert!(Rc::ptr_eq(frame, &selected));
            let reads: Vec<_> = frame.evaluation().references().values().filter_map(|proof| match proof {
                Reference::Scalar(read) => read.original_read(), _ => None,
            }).collect();
            assert_eq!(reads.len(), 1);
            let producer = reads[0].producer();
            let actual = match producer {
                Producer::SignedEntry(root) => { assert_eq!(*root, contract.parameters[1].binding); "entry" }
                Producer::Integer { value, .. } => { assert_eq!(*value, 1); "literal" }
                Producer::Call(call) => {
                    assert!(flow.unwrap().local_calls().contains(call));
                    assert!(call.local_binding().is_some());
                    "call"
                }
                Producer::Initializer(original) => {
                    let retained = flow.unwrap().expression_plans().iter().filter_map(|plan| plan.initializer_frame())
                        .find(|frame| frame.binding() == original.binding()).unwrap();
                    assert!(Rc::ptr_eq(original, retained));
                    "initializer"
                }
            };
            assert_eq!(Some(actual), expected);
            assert!(frame.prior_homes().is_empty(), "scalar initializer never manufactures a Text Home");
        }).unwrap();
    }
}
}

#[test]
fn initializer_core_i64_reaches_the_normal_compiler_with_original_reads_and_cleanup() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut failures = Vec::new();
        for (methods, body, returned) in [
            ("", "local n = \"abc\".length()", "0"),
            ("", "local text = \"abc\" local n = text.length()", "0"),
            ("", "local start = 1 local alias = start local n = alias + \"abc\".length()", "0"),
            ("", "local n = (\"a\" + \"b\").length()", "0"),
            ("", "local first = \"abc\".length() local next = first + \"d\".length()", "0"),
            ("static box Values { find(): i64 { return 1 } run(count: i64): i64 { local first = Values.find() local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local alias = count local first = Values.find(alias) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count + 1) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(count + (\"a\" + \"b\").length()) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local computed = \"abc\".length() local first = Values.find(computed + 1) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { find(value: i64): i64 { return value } run(count: i64): i64 { local first = Values.find(\"abc\".length()) local next = first + \"d\".length() return 0 } }", "local out = Values.run(1)", "0"),
            ("static box Values { read(count: i64, other: i64): i64 { local n = count + \"d\".length() return 0 } }", "local out = Values.read(1, 2)", "0"),
        ] {
            // Declared Main args selects the existing ordinary body consumer;
            // arity-0 qualified Main has a distinct canonical trivial-body route.
            let main_parameters = if methods.is_empty() { "" } else { "args" };
            let text = format!("{methods} static box Main {{ main({main_parameters}) {{ {body} return {returned} }} }}");
            let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                &text, crate::parser::ParserBuildConfig::default()).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("original source") };
            let mut consumed = false;
            let result = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()).for_closed_app_executable(),
                |view, verification| {
                    verification.as_ref().map_err(|errors| format!("{errors:?}"))?;
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                    let root = view.retained_root_source().ok_or_else(|| "missing original initializer source".to_owned())?;
                    root.visit_finalized_lexical_call_nodes_v1(view.module(), |_, _, _, _, _, _, _| Ok(()))?;
                    if methods.contains("find(") {
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("run")).unwrap();
                        let call_sites: Vec<_> = function.blocks.iter().filter_map(|(block, body)| {
                            match body.terminator.as_ref() {
                                Some(crate::mir::MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::I64, .. },
                                    normal_landing, ..
                                }) => Some((*block, *normal_landing)),
                                _ => None,
                            }
                        }).collect();
                        assert_eq!(call_sites.len(), 1, "original scalar producer must remain in the real child");
                        for mutation in ["callee", "result", "arguments"] {
                            let mut changed = view.module().clone();
                            let (block, normal) = call_sites[0];
                            let changed_function = changed.functions.get_mut(&function.signature.name).unwrap();
                            if mutation == "result" {
                                let result = changed_function.blocks.get_mut(&normal).unwrap().instructions.iter_mut()
                                    .find(|row| matches!(row, crate::mir::MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == block)).unwrap();
                                let crate::mir::MirInstruction::InvokeNormalResult { dst, .. } = result else { unreachable!() };
                                *dst = crate::mir::ValueId(99999);
                            } else {
                                let Some(crate::mir::MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, ..
                                }) = changed_function.blocks.get_mut(&block).unwrap().terminator.as_mut() else { unreachable!() };
                                match mutation {
                                    "callee" => call.callee = crate::mir::Callee::Global(
                                        crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method("Values", "run", 1)
                                            .canonical_global_target_v1().unwrap()),
                                    "arguments" => call.args.push(crate::mir::ValueId(99999)),
                                    _ => unreachable!(),
                                }
                            }
                            assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "{mutation}");
                        }
                    }
                    if methods.contains("find(") {
                        let function = view.module().functions.values().find(|function| function.signature.name.contains("run")).unwrap();
                        for (block, body) in &function.blocks {
                            for (index, instruction) in body.instructions.iter().enumerate() {
                                if !matches!(instruction, crate::mir::MirInstruction::BinOp { op: crate::mir::BinaryOp::Add, .. }) { continue; }
                                let mut changed = view.module().clone();
                                let crate::mir::MirInstruction::BinOp { op, .. } = &mut changed.functions.get_mut(&function.signature.name).unwrap().blocks.get_mut(block).unwrap().instructions[index] else { unreachable!() };
                                *op = crate::mir::BinaryOp::Sub;
                                assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "original integer operation cannot change");
                            }
                            if matches!(body.terminator, Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { .. }), .. })) {
                                let mut changed = view.module().clone();
                                let Some(crate::mir::MirInstruction::Invoke { operation, .. }) = changed.functions.get_mut(&function.signature.name).unwrap().blocks.get_mut(block).unwrap().terminator.as_mut() else { unreachable!() };
                                *operation = crate::mir::instruction::InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value: crate::mir::ValueId(99999) });
                                assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(), "argument child cleanup cannot change");
                            }
                        }
                    }
                    if let Some(function) = view.module().functions.values().find(|function| function.params.len() == 2) {
                        let mut changed = view.module().clone();
                        changed.functions.get_mut(&function.signature.name).unwrap().params.swap(0, 1);
                        let root = view.retained_root_source().ok_or_else(|| "missing original initializer source".to_owned())?;
                        assert!(root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                            "equal-I64 parameters cannot exchange their original ordinal/value identity");
                    }
                    consumed = true;
                    Ok(())
                });
            match result {
                Err(error) => failures.push(format!("{text}: {error}")),
                Ok(_) if !consumed => failures.push(format!("original initializer must reach publication: {text}")),
                Ok(_) => (),
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    });
}
