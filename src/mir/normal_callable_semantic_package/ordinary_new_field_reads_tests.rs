use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;

#[test]
fn unavailable_cleanup_preserves_exact_read_state_but_rejects_artifacts() {
    // Physical-state test using real source-issued rows, not executable Birth proof.
    let package = issue_with_brand_catalog(
        "box Page { slot: i64 birth() { me.slot = 7 } }
        static box Main { main() { local page = new Page() return page.slot } }",
    )
    .unwrap();
    let ledger = package.ordinary_new_claim_ledger;
    let site = ledger.claims.borrow().keys().next().unwrap().clone();
    let claim = ledger.try_take(&site, "Page", 0).unwrap().unwrap();
    assert!(ledger.prepare_new_emission(&claim).unwrap());
    ledger.begin_new_emission(&site).unwrap();
    let entry = BasicBlockId(0);
    let initializer = MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(0),
    };
    let mut function = MirFunction::new(
        crate::mir::FunctionSignature {
            name: "physical_state_test".into(),
            params: vec![],
            return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::CONTROL,
        },
        entry,
    );
    let block = function.blocks.get_mut(&entry).unwrap();
    for instruction in [
        initializer.clone(),
        MirInstruction::Copy {
            dst: ValueId(2),
            src: ValueId(1),
        },
    ] {
        block.instructions.push(instruction);
        block.instruction_spans.push(crate::ast::Span::unknown());
    }
    ledger
        .record_new_emission(&site, ValueId(1), vec![(entry, initializer)])
        .unwrap();
    ledger
        .complete_new_expression(&site, "Page", ValueId(1))
        .unwrap();
    let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
        &claim.declaration
    else {
        panic!("source local destination")
    };
    ledger
        .complete_local_installation(
            site.owner(),
            statement.node(),
            &[(claim.destination, *ordinal, ValueId(1), ValueId(2))],
        )
        .unwrap();
    ledger
        .complete_new_emissions(site.owner(), &function)
        .unwrap();
    *ledger.root_exit.borrow_mut() = local_commit::RootHomeExitProgress::Unavailable;
    let read_site = ledger.field_reads.borrow().keys().next().unwrap().clone();
    let (base, field) = ledger
        .take_terminal_field_read(&read_site, |binding| {
            assert_eq!(binding, claim.destination);
            Ok(ValueId(2))
        })
        .unwrap()
        .expect("selected read remains exact despite unavailable cleanup");
    assert!(ledger
        .take_terminal_field_read(&read_site, |_| panic!("duplicate take"))
        .unwrap_err()
        .contains("already-taken"));
    assert!(ledger
        .record_terminal_field_read(&read_site, entry, ValueId(3), ValueId(99), field)
        .unwrap_err()
        .contains("emission-mismatch"));
    ledger
        .record_terminal_field_read(&read_site, entry, ValueId(3), base, field)
        .unwrap();
    let block = function.blocks.get_mut(&entry).unwrap();
    block.instructions.push(MirInstruction::ObjectFieldGet {
        dst: ValueId(3),
        base,
        field,
    });
    block.instruction_spans.push(crate::ast::Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    assert!(ledger.field_reads_complete());
    ledger
        .validate_field_reads(site.owner(), &function)
        .unwrap();
    ledger.register_new_root(site.owner()).unwrap();
    assert_eq!(
        ledger.validate_finalized_new_root(&function).unwrap(),
        crate::mir::function::RootOrdinaryNewObservation::Unavailable(
            crate::mir::function::RootOrdinaryNewUnavailable::RootExitUnavailable
        )
    );
    assert!(ledger
        .validate_artifact_after_compiler_finishing(&function)
        .unwrap_err()
        .contains("artifact-source-unavailable"));
}

#[test]
fn terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression() {
    for (suffix, expected, alias) in [
        ("return page.slot + page.slot", 2, false),
        ("local alias = page return alias.slot", 1, true),
        ("return page.slot + true", 0, false),
    ] {
        let source = format!(
            "box Page {{ slot: i64 birth() {{ me.slot = 7 }} }}
            static box Main {{ main() {{ local page = new Page() {suffix} }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let reads = ledger.field_reads.borrow();
        assert_eq!(reads.len(), expected, "{suffix}");
        for (site, row) in reads.iter() {
            assert_eq!(row.receiver.owner(), site.owner());
            assert_eq!(row.receiver != row.home, alias);
            assert_eq!(row.field.declaration_ordinal(), 0);
            assert!(matches!(row.progress, Progress::Pending));
        }
        let selected = reads.keys().next().cloned();
        drop(reads);
        if let Some(site) = selected {
            let error = ledger
                .take_terminal_field_read(&site, |_| {
                    panic!("wrong root phase must reject before receiver resolution")
                })
                .unwrap_err();
            assert!(
                error.contains("ordinary-field-read/root-exit-phase"),
                "{error}"
            );
        }
    }
}
