use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

pub(super) fn issue_with_brand_catalog(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let catalog = crate::analysis::brand_program_declaration_catalog::
        issue_brand_program_declaration_catalog_v1(source.ast())
        .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    super::issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&catalog),
    )
}

#[test]
fn ordinary_new_claims_match_exact_local_initializers_without_effect_discovery() {
    use crate::mir::resolved_semantics::{BindingKindV1, SourceBindingSiteV1};
    for body in [
        "local first = new Page() local second = new Page() return 0",
        "local unused local scalar = 7 local first = new Page() local second = new Page() return scalar",
    ] {
        let source = format!(
            "box Page {{ birth() {{ }} }} static box Main {{ main() {{ {body} }} }}"
        );
        let package = issue_with_brand_catalog(&source).expect("exact local New source");
        assert_eq!(package.ordinary_new_claims.len(), 2);
        let mut bindings = std::collections::BTreeSet::new();
        for claim in &package.ordinary_new_claims {
            let declaration = package.batch().declarations()
                .find(|row| row.owner() == claim.site().owner()).expect("exact owner");
            let binding = package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                let function = input.function();
                let initializer = function.expression_source().initializers()
                    .find(|row| row.initializer_site() == Some(claim.site().site()))
                    .expect("claim retains its source initializer relation");
                assert!(matches!(initializer.declaration_site(), SourceBindingSiteV1::Local { .. }));
                assert_eq!(function.declaration_binding(initializer.declaration_site()),
                    Some(initializer.binding()));
                assert!(matches!(function.binding(initializer.binding()).unwrap().kind(),
                    BindingKindV1::Local { .. }));
                initializer.binding()
            }).expect("same-source initializer loan");
            assert!(bindings.insert(binding), "distinct destinations");
        }
    }
}

#[test]
fn ordinary_new_home_prefix_retains_order_and_requires_prior_installation() {
    use super::ordinary_new_coseal::{OrdinaryNewClaimLedgerV1, OrdinaryNewClaimTakeErrorV1};
    use crate::mir::resolved_semantics::SourceBindingSiteV1;
    use crate::mir::ValueId;
    let package = issue_with_brand_catalog(
        "box Page { birth() { } } static box Main { main() { local first = new Page() local alias = first local second = new Page() local third = new Page() return 0 } }"
    ).unwrap();
    let claims = &package.ordinary_new_claims;
    assert_eq!(claims.len(), 3);
    let prefixes: Vec<_> = claims.iter().map(|claim| claim.home_prefix().unwrap()).collect();
    assert!(prefixes[0].prior_homes().is_empty());
    assert_eq!(prefixes[1].prior_homes(), &[prefixes[0].destination()]);
    assert_eq!(prefixes[2].prior_homes(), &[prefixes[1].destination(), prefixes[0].destination()]);
    assert_eq!(prefixes[2].covered_statements().len(), 4);
    let sites: Vec<_> = claims.iter().map(|claim| claim.site().clone()).collect();
    let destinations: Vec<_> = prefixes.iter().map(|prefix| prefix.destination()).collect();
    let declarations: Vec<_> = sites.iter().map(|site| {
        let owner = package.batch().declarations().find(|row| row.owner() == site.owner()).unwrap();
        package.batch().with_lowering_input(owner.batch_slot(), |input|
            input.function().expression_source().initializers().find(|row|
                row.initializer_site() == Some(site.site())).unwrap().declaration_site().clone()).unwrap()
    }).collect();
    let ledger = OrdinaryNewClaimLedgerV1::issue(package.ordinary_new_claims, vec!["Page".into()].into());
    assert_eq!(ledger.try_take(&sites[1], "Page", 0), Err(OrdinaryNewClaimTakeErrorV1::Mismatch));
    for (index, site) in sites.iter().enumerate() {
        ledger.try_take(site, "Page", 0).unwrap().unwrap();
        let initializer = ValueId(index as u32 * 2);
        let local = ValueId(index as u32 * 2 + 1);
        ledger.complete_new_expression(site, "Page", initializer).unwrap();
        let SourceBindingSiteV1::Local { statement, ordinal } = &declarations[index] else { panic!("local") };
        ledger.complete_local_installation(site.owner(), statement.node(), &[
            (destinations[index], *ordinal, initializer, local),
        ]).unwrap();
    }
    assert!(ledger.is_empty());
}

#[test]
fn ordinary_new_unknown_home_prefix_is_not_an_empty_cleanup_plan() {
    for body in [
        "local n = 0 n = 1 local item = new Page() return 0",
        "local n = new ArrayBox() local item = new Page() return 0",
        "local item = new Page() { value: 1 } local next = new Page() return 0",
    ] {
        let source = format!("box Page {{ value: i64 birth() {{ }} }} static box Main {{ main() {{ {body} }} }}");
        let package = issue_with_brand_catalog(&source).expect("prefix unavailability is not source rejection");
        assert!(!package.ordinary_new_claims.is_empty());
        assert!(package.ordinary_new_claims.iter().all(|claim| claim.home_prefix().is_err()), "{body}");
    }
    let package = issue_with_brand_catalog(
        "box Page { birth() { } } static box Main { helper(value) { local item = new Page() return 0 } main() { return 0 } }"
    ).unwrap();
    assert_eq!(package.ordinary_new_claims.len(), 1);
    assert!(matches!(package.ordinary_new_claims[0].home_prefix(),
        Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::EntryDemandMissing)));

    let package = issue_with_brand_catalog(
        "box Page { birth(value) { } } static box Main { main() { local first = new Page(0) local second = new Page(first) return 0 } }"
    ).unwrap();
    assert_eq!(package.ordinary_new_claims.len(), 2);
    assert!(package.ordinary_new_claims[0].home_prefix().is_ok());
    assert!(matches!(package.ordinary_new_claims[1].home_prefix(),
        Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::ArgumentNotCovered(_))));
}

#[test]
fn ordinary_new_local_completion_reaches_package_finish_for_two_destinations() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "box Page { birth() { } } static box Main { main() { local first = new Page() local second = new Page() return 0 } }",
            ParserBuildConfig::default(),
        ).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else {
                panic!("source authority lost");
            };
        let result = crate::mir::MirCompiler::with_options(false).compile_normal(
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                source, None, Default::default()),
        ).expect("both target takes must complete through their exact locals");
        assert!(result.verification_result.is_ok());
        let main = result.module.get_function("main").unwrap();
        assert_eq!(main.blocks.values().flat_map(|block| &block.instructions).filter(|inst|
            matches!(inst, crate::mir::MirInstruction::Call(call)
                if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }))
        ).count(), 2);
        let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
        assert_eq!(view.route(), crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject);
    });
}

#[test]
fn birth_receiver_non_escape_preserves_field_access_and_local_aliases() {
    for body in [
        "me.value = value",
        "local saved = me.value",
        "local alias = me alias.value = value",
        "local alias = value alias = me alias.value = value",
        "local alias = me local copy = alias copy.value = value",
    ] {
        let source = format!("box Page {{ value: i64 birth(value) {{ {body} }} }}");
        let package = issue_with_brand_catalog(&source).expect("non-escaping Birth body");
        assert_eq!(package.instance_constructors().rows().len(), 1);
    }
}

#[test]
fn birth_receiver_non_escape_rejects_unproven_uses_before_row_publication() {
    use super::instance_constructor_semantic::InstanceConstructorSemanticBatchIssueV1;
    for body in [
        "return me",
        "other.value = me",
        "local alias = me other.value = alias",
        "other.accept(me)",
        "local alias = me other.accept(alias)",
        "me.accept(value)",
        "local nested = fn() { return me }",
        "local alias = me local nested = fn() { return alias }",
        "local alias = value if value { alias = me } other.value = alias",
        "local alias = me alias = value other.value = alias",
        "local aggregate = [me]",
    ] {
        let source = format!("box Page {{ value: i64 birth(value, other) {{ {body} }} }}");
        let error = issue_with_brand_catalog(&source).expect_err("unproven Birth receiver use");
        // Direct nested `me` is already stopped by the upstream resolver.
        // Keep it as dependency evidence, not as acceptance for the new check;
        // the separate alias-capture case below must reach ReceiverNonEscape.
        if body == "local nested = fn() { return me }" {
            assert!(matches!(&error,
                super::NormalCallableSemanticPackageIssueV1::InstanceConstructors {
                    _error: InstanceConstructorSemanticBatchIssueV1::Resolver(_),
                }) && format!("{error:?}").contains("body Me shape lacks exact receiver authority"),
                "wrong upstream boundary: {error:?}");
            continue;
        }
        assert!(matches!(error,
            super::NormalCallableSemanticPackageIssueV1::InstanceConstructors {
                _error: InstanceConstructorSemanticBatchIssueV1::ReceiverNonEscape { .. },
            }), "wrong boundary for {body}: {error:?}");
    }
}

#[test]
fn ordinary_new_rejects_value_birth_and_unselected_effect_contract() {
    use super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1;
    for (declaration, value_return) in [
        ("birth(value) { return value }", true),
        ("@rune Contract(no_alloc) birth(value) { local saved = value }", false),
    ] {
        let source = format!(
            "box Page {{ {declaration} }}
             static box Main {{ main() {{ local page = new Page(1) return 0 }} }}"
        );
        let error = issue_with_brand_catalog(&source).expect_err("unselected birth contract");
        match error {
            super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
                _error: OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit { .. },
            } => assert!(value_return),
            super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
                _error: OrdinaryNewCoSealIssueV1::BirthEffectUnsupported { .. },
            } => assert!(!value_return),
            other => panic!("wrong boundary: {other:?}"),
        }
    }
}

#[test]
fn instance_constructor_semantics_keep_parser_identity_and_nested_brand_relations() {
    let package = issue_with_brand_catalog(
        r#"
brand Id: i64

box Holder {
    init(value) {
        local direct = Id(value)
        local nested = fn(x) { Id(x) }
    }
    pack(other) {
        local second = Id(other)
    }
}
"#,
    )
    .expect("constructor semantic package");

    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].box_name(), "Holder");
    assert_eq!(rows[0].key(), "init/1");
    assert!(rows[0].source_id().same_as(rows[0].source_id()));
    assert_eq!(rows[0].forest().owner_count(), 2);
    assert_eq!(rows[1].key(), "pack/1");
    assert!(rows.iter().all(|row| row.published_birth_key().is_none()));
    assert_eq!(
        rows.iter()
            .flat_map(|row| row.forest().owners())
            .map(|(_, owner)| owner.brand_call_relations().count())
            .sum::<usize>(),
        3
    );
}

#[test]
fn birth_semantic_row_issues_distinct_non_global_publication_key() {
    let package = issue_with_brand_catalog("box Page { birth(value) { local saved = value } }")
        .expect("birth package");
    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 1);
    let key = rows[0]
        .published_birth_key()
        .expect("Birth owns a published key");
    assert_eq!(
        key.namespace(),
        hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
    );
    assert_eq!(key.owner(), "Page");
    assert_eq!(key.arity(), rows[0].source_arity());
    assert_eq!(key.mir_symbol_projection(), "Page.birth/1");
    assert!(key.canonical_global_target_v1().is_err());
    let completion = rows[0].birth_completion().expect("source-owned birth completion");
    assert_eq!(completion.owner(), rows[0].forest().roots()[0]);
    assert!(!completion.returns_value());
    package.with_normal_program_source_loan(|loan| {
        let input = rows[0].lowering_input(loan.program()).unwrap();
        let shape = input.body_shape().expect("constructor events survive the loan");
        assert_eq!(shape.owner(), input.owner());
        assert_eq!(shape.body_root(), &input.function().root_profile().body_root());
    }).unwrap();
}

#[test]
fn birth_fixed_source_retains_definition_through_normal_publication() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default(),
        )
        .expect("birth source with explicitly migrated i64 field contracts");
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed).expect("birth transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("birth fixture must retain source authority")
        };
        let result = crate::mir::MirCompiler::with_options(false)
            .compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    Some("apps/typed-object-birth-min/main.hako"),
                    Default::default(),
                ),
            )
            .expect("normal constructor publication");
        assert!(
            result.verification_result.is_ok(),
            "published source must retain valid field contracts: {:?}",
            result.verification_result
        );
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2);
        assert_eq!(
            result.module.canonical_callable_definition_symbol(&key),
            Some("Pair.birth/2")
        );
        let function = result.module.get_function("Pair.birth/2").unwrap();
        assert_eq!(function.signature.params.len(), 3);
        let contracts = &function.metadata.exact_numeric_runtime_check_contracts;
        assert_eq!(contracts.len(), 2, "unannotated birth values require checks");
        for contract in contracts {
            assert_eq!(contract.declared_type_name, "i64");
            assert_eq!(
                contract.kind,
                crate::mir::function::ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
            );
            let instruction =
                &function.blocks[&contract.block].instructions[contract.instruction_index];
            let crate::mir::MirInstruction::FieldSet { field, value, .. } = instruction else {
                panic!("each check must remain bound to its actual FieldSet")
            };
            assert_eq!(field, &contract.field);
            assert_eq!(value, &contract.value);
        }
        let mut fields: Vec<_> = contracts.iter().map(|row| row.field.as_str()).collect();
        fields.sort_unstable();
        assert_eq!(fields, ["left", "right"]);

        let main = result
            .module
            .get_function("main")
            .expect("published Main entry");
        assert_eq!(main.signature.return_type, crate::mir::MirType::Integer);
        let mut read_count = 0;
        let mut add_count = 0;
        let mut birth_count = 0;
        for instruction in main.blocks.values().flat_map(|block| &block.instructions) {
            if let crate::mir::MirInstruction::Call(call) = instruction {
                if let crate::mir::Callee::BirthConstructor { key: target, receiver } = &call.callee {
                    assert_eq!(target, &key);
                    assert_eq!(call.dst, None);
                    assert_eq!(call.args.len(), 2, "receiver is not a source argument");
                    assert!(main.metadata.value_types.contains_key(receiver));
                    for effect in [crate::mir::Effect::WriteHeap, crate::mir::Effect::ReadHeap,
                        crate::mir::Effect::Panic, crate::mir::Effect::Barrier] {
                        assert!(call.effects.contains(effect));
                    }
                    assert!(!call.effects.contains(crate::mir::Effect::Pure));
                    assert!(!call.effects.is_moveable());
                    birth_count += 1;
                }
            }
            let dst = match instruction {
                crate::mir::MirInstruction::FieldGet {
                    dst, declared_type, ..
                } => {
                    assert_eq!(declared_type, &Some(crate::mir::MirType::Integer));
                    read_count += 1;
                    dst
                }
                crate::mir::MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Add,
                    ..
                } => {
                    add_count += 1;
                    dst
                }
                _ => continue,
            };
            assert_eq!(
                main.metadata.value_types.get(dst),
                Some(&crate::mir::MirType::Integer)
            );
        }
        assert_eq!((read_count, add_count), (2, 1));
        assert_eq!(birth_count, 1, "selected birth uses the exact canonical carrier");
        let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module)
            .expect("birth definition relation remains valid");
        assert_eq!(view.route(), crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject);
        let output_dir = tempfile::tempdir().expect("temporary artifact directory");
        let exe = output_dir.path().join("birth");
        let error = crate::host_providers::llvm_codegen::emit_published_static_method_exe(
            &result.module, exe.to_str().unwrap(), None, None,
        ).expect_err("unimplemented Birth consumer must not retry compatibility");
        assert!(error.contains("UnsupportedBeforeObject"));
        assert_eq!(std::fs::read_dir(output_dir.path()).unwrap().count(), 0);
        let static_key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "Pair", "birth", 2,
        );
        assert!(result
            .module
            .canonical_callable_definition_symbol(&static_key)
            .is_none());
    });
}
