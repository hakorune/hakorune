use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::instruction::InvokeCallResultKind;
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
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        assert_eq!(claims.len(), 2);
        let mut bindings = std::collections::BTreeSet::new();
        for claim in &claims {
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
fn ordinary_new_retains_unavailable_descriptors_through_candidate_co_seal() {
    let package = issue_with_brand_catalog(
        "box Page { value } static box Main { main() { local page = new Page() return 0 } }",
    )
    .expect("unavailable construction is retained, not a failed lookup");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    assert_eq!(rows.len(), 1);
    let claim = rows.values().next().unwrap();
    assert!(claim.construction().is_err());
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::Unavailable(
            crate::mir::function::ObjectDestructionUnavailableV1::FieldType,
        )
    );
    assert_eq!(
        package
            .instance_constructors
            .destruction_for(claim.box_source())
            .unwrap(),
        (claim.object(), claim.destruction())
    );
}

#[test]
fn ordinary_new_descriptor_errors_follow_candidate_source_order() {
    for (first, second) in [("First", "Second"), ("Second", "First")] {
        let source = format!(
            "box First {{}} box Second {{}} static box Main {{ main() {{
             local a = new {first}(1) local b = new {second}(2) return 0 }} }}"
        );
        match issue_with_brand_catalog(&source) {
            Err(super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
                _error:
                    super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
                        class,
                        arity,
                        ..
                    },
            }) => {
                assert_eq!(class.as_ref(), first);
                assert_eq!(arity, 1);
            }
            other => panic!("expected first candidate's missing Birth: {other:?}"),
        }
    }
}

#[test]
fn ordinary_new_claim_keeps_source_construction_plan_and_override_dependency() {
    use super::instance_construction::ConstructionUnavailableV1;
    let source = "box Page { value: i64\nbirth(value) { me.value = value } }
        static box Main { main() { local page = new Page(7)\nreturn 0 } }";
    let package = issue_with_brand_catalog(source).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one exact New");
    };
    let plan = claim.construction().as_ref().unwrap();
    assert_eq!(plan.stores().len(), 1);
    assert!(plan.reclaims_unpublished_outer_storage());
    let source = source.replace("new Page(7)", "new Page(7) { value: 8 }");
    let package = issue_with_brand_catalog(&source).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(
        claims[0].construction(),
        &Err(ConstructionUnavailableV1::OverrideUnsupported)
    );
    let claim = &claims[0];
    let batch = &package.instance_constructors;
    let projected = batch.destruction_for(claim.box_source()).unwrap();
    assert_eq!(projected, (claim.object(), claim.destruction()));
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
    );
    let foreign = issue_with_brand_catalog(&source).unwrap();
    let foreign_claims = foreign.ordinary_new_claim_ledger.pending_claims_for_test();
    assert!(matches!(batch.destruction_for(foreign_claims.values().next().unwrap().box_source()),
        Err(super::instance_constructor_semantic::InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)));
    let definitions = batch.take_object_definitions().unwrap();
    assert_eq!(
        definitions[claim.object().declaration_index() as usize].destruction_disposition(),
        claim.destruction()
    );
    assert!(matches!(batch.destruction_for(claim.box_source()),
        Err(super::instance_constructor_semantic::InstanceConstructorBirthLookupErrorV1::ObjectDefinitionsTransferred)));
    assert_eq!(
        batch.object_for(claim.box_source()).unwrap(),
        claim.object()
    );
    assert_eq!(
        projected,
        (claim.object(), claim.destruction()),
        "retained claim survives transfer"
    );
    assert!(batch.take_object_definitions().is_none());
}

#[test]
fn ordinary_new_claims_retain_exact_parent_with_and_without_birth() {
    for birth in ["birth() {}", ""] {
        let source = format!(
            "box Page {{ {birth} }} static box Main {{ main() {{
            local first = new Page() local second = new Page() return 0
        }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        let parent = package
            .batch()
            .ordinary_box_coverage()
            .row_for("Page")
            .unwrap()
            .unwrap();
        assert_eq!(claims.len(), 2);
        assert_ne!(claims[0].site(), claims[1].site());
        for claim in &claims {
            assert!(claim.box_source().same_source_as(parent));
        }
    }
}

#[path = "brand_catalog_new_completion_tests.rs"]
mod new_completion_tests;

#[test]
fn ordinary_new_unknown_home_prefix_is_not_an_empty_cleanup_plan() {
    for body in [
        "local n = 0 n = 1 local item = new Page() return 0",
        "local n = new ArrayBox() local item = new Page() return 0",
        "local item = new Page() { value: 1 } local next = new Page() return 0",
    ] {
        let source = format!(
            "box Page {{ value: i64 birth() {{ }} }} static box Main {{ main() {{ {body} }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .expect("prefix unavailability is not source rejection");
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        assert!(!claims.is_empty());
        assert!(
            claims.iter().all(|claim| claim.home_prefix().is_err()),
            "{body}"
        );
    }
    let package = issue_with_brand_catalog(
        "box Page { birth() { } } static box Main { helper(value) { local item = new Page() return 0 } main() { return 0 } }"
    ).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(claims.len(), 1);
    assert!(claims[0].home_prefix().is_ok());

    let package = issue_with_brand_catalog(
        "box Page { birth(value) { } } static box Main { main() { local first = new Page(0) local second = new Page(first) return 0 } }"
    ).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(claims.len(), 2);
    assert!(claims[0].home_prefix().is_ok());
    assert!(claims[1].home_prefix().is_ok());
}

#[test]
fn ordinary_new_claim_records_parameter_handle_argument() {
    use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1;
    use crate::mir::resolved_semantics::SourceBindingSiteV1;
    let package = issue_with_brand_catalog(
        "box Page { birth(value) { } } static box Main { helper(value) { local item = new Page(value) return 0 } main() { return 0 } }"
    ).expect("parameter handle argument claim");
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    assert_eq!(claims.len(), 1);
    assert!(claims[0].home_prefix().is_ok());
    let rows = claims[0]
        .argument_rows()
        .expect("declared parameter installs as a handle observation");
    let [row] = rows else {
        panic!("one argument row, got {rows:?}")
    };
    let OrdinaryNewTrivialArgumentKindV1::Handle { binding } = row.kind() else {
        panic!(
            "parameter argument must be a Handle row, got {:?}",
            row.kind()
        )
    };
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == claims[0].site().owner())
        .expect("exact owner");
    let parameter = package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            input
                .function()
                .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
        })
        .expect("lowering input");
    assert_eq!(Some(*binding), parameter);
}

/// `me.<field> = <rhs>` on the self-rooted receiver is a covered prefix
/// statement when the RHS subtree is Home-neutral: `me.<field>` reads are
/// proven by the receiver-side scalar contract (`i64` and `usize` both
/// admit — argument-position `i64` evidence stays a separate authority),
/// and the write itself mints no ledger row.
#[test]
fn ordinary_new_receiver_field_write_is_home_neutral() {
    let package = issue_with_brand_catalog(
        "box Page { left: i64 right: usize birth() { }
        touch() {
            me.left = me.left + 1
            me.right = me.right - me.left
            local item = new Page()
            return 0
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("field-write package");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim inside `touch`, got {claims:?}")
    };
    assert_eq!(
        claim.home_prefix().map(|_| ()),
        Ok(()),
        "self-rooted scalar field writes are covered prefix statements"
    );
}

/// Field-write admission stays fail-closed: an owned-binding RHS, a
/// non-`me` receiver, a `me`-field receiver call, and a `new` inside the
/// RHS each keep `PrefixNotCovered` — no owning-field or receiver-write
/// meaning is minted.
#[test]
fn ordinary_new_receiver_field_write_stays_fail_closed() {
    for (label, write) in [
        // An owned Home in the RHS is the parked owning-field family.
        ("owned-handle-rhs", "local h = new Page() me.left = h"),
        // A non-`me` receiver is not a self field write.
        ("non-self-receiver", "local p = new Page() p.left = 5"),
        // A call inside the RHS is the nested receiver-call family.
        ("rhs-receiver-call", "me.left = me.right.make()"),
        // `new` inside the RHS is a construction, not a scalar store.
        ("rhs-new", "me.left = new Page()"),
        // Compound `op=` reads and writes the same field.
        ("compound", "me.left += 1"),
    ] {
        let source = format!(
            "box Page {{ left: i64 right: Page birth() {{ }}
            make() {{ return 0 }}
            touch() {{ {write} local item = new Page() return 0 }} }}
            static box Main {{ main() {{ return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        // The `local item = new Page()` claim follows the write — its
        // prefix must name the uncovered statement; claims declared before
        // the write legitimately stay `Ok`.
        assert!(
            claims.iter().any(|claim| matches!(
                claim.home_prefix(),
                Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(_))
            )),
            "{label}: a claim past the write keeps PrefixNotCovered, got {claims:?}"
        );
    }
}

/// `me.<ArrayBox field>.m(..)` is a covered prefix statement when the
/// field's declared type is `ArrayBox`, the (selector, arity) pair
/// resolves in the generated `CORE_METHOD_CONTRACT_ROWS_V2` manifest, and
/// every argument subtree is Home-neutral. `NoValue` calls admit in bare
/// statement position; `local x = ..` installs the manifest result class
/// (`Dynamic` → `BoundValue`, scalar kinds → `Trivial`). No ledger row is
/// minted — the raw lane's `Callee::Method{RuntimeData}`/`ArrayElementWrite`
/// emission already owns the instruction.
#[test]
fn ordinary_new_receiver_field_call_is_home_neutral() {
    let package = issue_with_brand_catalog(
        "box Page { left: i64 items: ArrayBox birth() { }
        touch() {
            me.items.set(0, 1)
            me.items.push(me.left)
            local v = me.items.get(me.left)
            local n = me.items.length()
            local b = me.items.has(v)
            me.items.set(n, v)
            if n > 0 {
                me.items.push(n)
                local w = me.items.get(0)
            }
            local item = new Page()
            return 0
        } }
        static box Main { main() { return 0 } }",
    )
    .expect("field-call package");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim inside `touch`, got {claims:?}")
    };
    assert_eq!(
        claim.home_prefix().map(|_| ()),
        Ok(()),
        "manifest-proven ArrayBox field calls are covered prefix statements"
    );
}

/// Field-call admission stays fail-closed: anything outside the declared
/// `ArrayBox`-field + manifest-(selector, arity) + Home-neutral-argument
/// contract keeps `PrefixNotCovered`.
#[test]
fn ordinary_new_receiver_field_call_stays_fail_closed() {
    for (label, prefix) in [
        // A `get` (Dynamic result) in bare statement position silently
        // discards a produced value — only `NoValue` admits there.
        ("discarded-get", "me.items.get(0)"),
        // `local x = ..` can never bind a `NoValue` result.
        ("novalue-in-local", "local x = me.items.set(0, 1)"),
        // Manifest-absent selectors keep the boundary.
        ("pop", "me.items.pop()"),
        ("insert", "me.items.insert(0, 1)"),
        ("clear", "me.items.clear()"),
        ("contains", "local x = me.items.contains(0)"),
        // A non-`me` receiver is not a self field call.
        ("non-self-receiver", "local p = new Page() p.items.set(0, 1)"),
        // A non-`ArrayBox` field has no container contract.
        ("non-arraybox-field", "me.left.set(0, 1)"),
        // `new` inside an argument is a construction, not a scalar store.
        ("new-arg", "me.items.set(0, new Page())"),
        // A nested call inside an argument is unobserved.
        ("nested-call-arg", "me.items.set(0, me.items.get(0))"),
        // A container literal argument is outside scalar admission.
        ("array-arg", "me.items.set(0, [1, 2])"),
        ("map-arg", "me.items.set(0, %{\"a\" => 1})"),
        // An owned Home argument is an untracked ownership transfer.
        ("owned-arg", "local h = new Page() me.items.set(0, h)"),
        // An `ArrayBox` field read is not a scalar argument.
        ("array-field-arg", "me.items.set(0, me.items)"),
    ] {
        let source = format!(
            "box Page {{ left: i64 items: ArrayBox birth() {{ }}
            touch() {{ {prefix} local item = new Page() return 0 }} }}
            static box Main {{ main() {{ return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        assert!(
            claims.iter().any(|claim| matches!(
                claim.home_prefix(),
                Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(_))
            )),
            "{label}: a claim past the call keeps PrefixNotCovered, got {claims:?}"
        );
    }
}

#[test]
fn normal_home_completion_observes_suffix_and_does_not_reuse_last_new_prefix() {
    for (suffix, available) in [
        ("return 0", true),
        ("local answer = 7 return answer", true),
        ("first = second return 0", false),
        ("return first.left + second.right", true),
        ("local alias = first return alias.right + 7", true),
        ("return first.missing", false),
        ("return first.left + true", false),
        ("return false + first.left", false),
        ("return first.left - second.right", false),
        ("return first.left.right", false),
        ("local value = first.left return value", false),
    ] {
        let source = format!(
            "box Page {{ left: i64 right: i64
            birth() {{ me.left = 4 me.right = 9 }} }} static box Main {{ main() {{
            local first = new Page() local second = new Page() {suffix}
        }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        let owner = claims[0].site().owner();
        {
            let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
            assert_eq!(completion.owner(), owner);
            assert!(
                claims.iter().all(|claim| claim.home_prefix().is_ok()),
                "New-failure prefixes are unchanged"
            );
            let homes = completion
                .cleanup()
                .terminal_homes()
                .expect("terminal analysis is explicit");
            if available {
                let expected: Vec<_> = claims
                    .iter()
                    .rev()
                    .map(|claim| claim.home_prefix().unwrap().destination())
                    .collect();
                assert_eq!(homes.unwrap(), expected.as_slice(), "{suffix}");
            } else {
                assert!(
                    homes.is_err(),
                    "unsupported suffix must not become empty cleanup: {suffix}"
                );
            }
        }
    }
    // A returned Home local leaves with the caller: the Value relation records
    // the exact binding, and terminal cleanup keeps only the sibling home.
    for suffix in ["return first", "local alias = first return alias"] {
        let source = format!(
            "box Page {{ left: i64 right: i64
            birth() {{ me.left = 4 me.right = 9 }} }} static box Main {{ main() {{
            local first = new Page() local second = new Page() {suffix}
        }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = claim_rows.values().collect();
        let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
        let homes = completion
            .cleanup()
            .terminal_homes()
            .expect("terminal analysis is explicit")
            .unwrap();
        assert_eq!(
            homes,
            [claims[1].home_prefix().unwrap().destination()].as_slice(),
            "{suffix}"
        );
    }
    for declaration in [
        "slot: i64 birth() {}",
        "slot: i64 birth() { me.slot = 1 me.slot = 2 }",
        "slot: bool birth() { me.slot = true }",
        "slot: i64",
    ] {
        let source = format!(
            "box Page {{ {declaration} }} static box Main {{ main() {{
            local page = new Page() return page.slot
        }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
        assert!(completion.cleanup().terminal_homes().unwrap().is_err(),
            "field declaration without supported initialization is not a return proof: {declaration}");
    }
}

#[test]
fn ordinary_new_fault_continuation_is_source_owned_and_not_normal_completion() {
    use crate::mir::resolved_control_flow::issue_new_fault_continuation_v1;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, OwnedExprSiteV1};
    let package = issue_with_brand_catalog(
        "box Page { birth() { } } static box Main { main() { local scalar = 0 local first = new Page() local second = new Page() return 0 } }"
    ).unwrap();
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = claim_rows.values().collect();
    let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign = owners.issue().unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == claims[0].site().owner())
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            for claim in &claims {
                let fault = claim.home_prefix().unwrap().outward_fault();
                assert_eq!(fault.site(), claim.site());
                assert_eq!(fault.target_function(), input.function().function_region());
                assert_eq!(
                    fault.source_scope(),
                    input.function().lowering_roots().body_pair().scope()
                );
                assert_eq!(
                    issue_new_fault_continuation_v1(
                        input,
                        &OwnedExprSiteV1::new(foreign, claim.site().site().clone())
                    ),
                    Err("foreign-source-owner")
                );
            }
            assert_ne!(
                claims[0].home_prefix().unwrap().outward_fault().site(),
                claims[1].home_prefix().unwrap().outward_fault().site()
            );
            let scalar = input
                .function()
                .expression_source()
                .initializers()
                .find(|row| {
                    row.initializer_site().is_some_and(|site| {
                        input.function().expression_source().literal(site).is_some()
                    })
                })
                .unwrap();
            assert_eq!(
                issue_new_fault_continuation_v1(
                    input,
                    &OwnedExprSiteV1::new(
                        input.owner(),
                        scalar.initializer_site().unwrap().clone()
                    )
                ),
                Err("source-not-new")
            );
        })
        .unwrap();
}

#[test]
fn ordinary_new_local_completion_reaches_package_finish_for_two_destinations() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        for text in [
            "box Page { birth() { } } static box Main { main() { local first = new Page() local second = new Page() return 0 } }",
            "box Page { left: i64\nright: i64\nbirth(a, b) { me.left = a\nme.right = b } } static box Main { main() { local a = 7\nlocal b = 9\nlocal first = new Page(a, b)\nlocal second = new Page(b, a)\nreturn 0 } }",
        ] {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            text,
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
        assert_eq!(main.root_ordinary_new_observation(),
            crate::mir::function::RootOrdinaryNewObservation::SourceCompleteAtFinalization);
        assert_eq!(main.blocks.values().flat_map(|block| block.all_instructions()).filter(|inst|
            matches!(inst, crate::mir::MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Call { call, result: InvokeCallResultKind::Unit }, .. }
                if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }))
        ).count(), 2);
        assert!(!main.blocks.values().flat_map(|block| block.all_instructions()).any(|inst|
            matches!(inst, crate::mir::MirInstruction::NewBox { .. }
                | crate::mir::MirInstruction::Call(_))));
        assert_normal_home_exit_paths(main);
        let view = crate::mir::function::PublishedMirBackendView::try_new(&result.module).unwrap();
        assert_eq!(view.route(), crate::mir::function::PublishedStaticMethodRouteV1::UnsupportedBeforeObject);
        }
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

fn assert_normal_home_exit_paths(function: &crate::mir::MirFunction) {
    use crate::mir::instruction::InvokeOperation;
    use crate::mir::{BasicBlockId, MirInstruction, ValueId};
    fn walk(
        function: &crate::mir::MirFunction,
        id: BasicBlockId,
        mut allocations: Vec<ValueId>,
        mut homes: Vec<ValueId>,
        mut released: Vec<ValueId>,
        pending_fault: bool,
        depth: usize,
    ) -> usize {
        assert!(depth < 64, "selected straight-line cleanup cannot cycle");
        for instruction in function.blocks[&id].all_instructions() {
            match instruction {
                MirInstruction::InvokeNormalResult { dst, .. } => allocations.push(*dst),
                MirInstruction::Copy { dst, src } if allocations.contains(src) => homes.push(*dst),
                MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease { value, .. },
                    normal_landing,
                    fault_landing,
                    ..
                } => {
                    released.push(*value);
                    let normal = walk(
                        function,
                        *normal_landing,
                        allocations.clone(),
                        homes.clone(),
                        released.clone(),
                        pending_fault,
                        depth + 1,
                    );
                    let fault = walk(
                        function,
                        *fault_landing,
                        allocations,
                        homes,
                        released,
                        true,
                        depth + 1,
                    );
                    return normal + fault;
                }
                MirInstruction::Invoke {
                    operation: InvokeOperation::ReclaimUnpublished { .. },
                    ..
                } => panic!("successful construction must not reclaim unpublished storage"),
                MirInstruction::Invoke { normal_landing, .. } => {
                    return walk(
                        function,
                        *normal_landing,
                        allocations,
                        homes,
                        released,
                        pending_fault,
                        depth + 1,
                    )
                }
                MirInstruction::Jump { target, .. } => {
                    return walk(
                        function,
                        *target,
                        allocations,
                        homes,
                        released,
                        pending_fault,
                        depth + 1,
                    )
                }
                MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. } => {
                    assert_eq!(homes.len(), 2);
                    homes.reverse();
                    assert_eq!(
                        released, homes,
                        "each completed Home releases once in reverse order"
                    );
                    assert_eq!(
                        matches!(instruction, MirInstruction::ReturnFault { .. }),
                        pending_fault,
                        "later successful cleanup cannot swallow an earlier Fault"
                    );
                    return 1;
                }
                _ => {}
            }
        }
        panic!("cleanup path must end in a typed terminal")
    }
    assert_eq!(
        walk(
            function,
            function.entry_block,
            vec![],
            vec![],
            vec![],
            false,
            0
        ),
        4,
        "both outcomes of both releases must reach a terminal"
    );
}

/// A declaration default `items: ArrayBox = new ArrayBox()` desugars to a
/// birth-side `me.items = new ArrayBox()` provider store; every declared
/// ArrayBox field with that sealed store lands in `array_children` in
/// declaration order under the `OwnedArrayFieldsNoHook` disposition.
#[test]
fn ordinary_new_owned_array_children_seal_in_declaration_order() {
    let package = issue_with_brand_catalog(
        "box Page {
            left: i64
            items: ArrayBox = new ArrayBox()
            children: ArrayBox = new ArrayBox()
            birth() { }
        }
        static box Main { main() { local item = new Page() return 0 } }",
    )
    .expect("owned-array package");
    let rows = package
        .ordinary_new_claim_ledger
        .pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one claim, got {claims:?}")
    };
    assert_eq!(
        claim.destruction(),
        crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
    );
    let ordinals: Vec<u32> = claim
        .array_children()
        .expect("proven owned residences")
        .iter()
        .map(|field| field.declaration_ordinal())
        .collect();
    assert_eq!(ordinals, [1, 2], "declaration order is the sealed order");
}

/// The residence proof stays fail-closed: no provider store, a second
/// `me.` store, or a store outside the constructor path each leave
/// `array_children` empty on an `OwnedArrayFieldsNoHook` claim — the
/// lifecycle gate then refuses the teardown instead of releasing plain.
#[test]
fn ordinary_new_owned_array_children_stay_unproven_on_ambiguous_writes() {
    for (label, members) in [
        // No birth-side provider store at all.
        ("missing", "items: ArrayBox\nbirth() { }"),
        // A second `me.` store — even of `new ArrayBox()` — is re-assignment.
        (
            "rewritten",
            "items: ArrayBox = new ArrayBox()\nbirth() { me.items = new ArrayBox() }",
        ),
        // A store outside the constructor path is not a birth provider.
        (
            "non-birth",
            "items: ArrayBox = new ArrayBox()\nbirth() { }\ntouch() { me.items = new ArrayBox() }",
        ),
    ] {
        let source = format!(
            "box Page {{ {members} }}
            static box Main {{ main() {{ local item = new Page() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source)
            .unwrap_or_else(|issue| panic!("{label} package: {issue:?}"));
        let rows = package
            .ordinary_new_claim_ledger
            .pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        let [claim] = claims.as_slice() else {
            panic!("{label}: one claim, got {claims:?}")
        };
        assert_eq!(
            claim.destruction(),
            crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook,
            "{label}"
        );
        assert!(
            claim.array_children().is_none(),
            "{label}: an unproven residence stays unsealed"
        );
    }
}

include!("brand_catalog_tail_tests.rs");
include!("brand_catalog_selected_new_argument_tests.rs");
include!("brand_catalog_mixed_result_class_tests.rs");
