use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
use crate::mir::BasicBlock;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1;

#[test]
fn unavailable_cleanup_preserves_exact_read_state_but_rejects_artifacts() {
    for (text, owned_ready) in [
        ("box Page { slot: i64 birth() { me.slot = 7 } }
         static box Main { main() { local page = new Page() return page.slot } }", false),
        ("box Page { slot: i64 }
         static box Main { main() { local page = new Page() page.slot = 7 return page.slot } }", true),
    ] {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        text,
        crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).unwrap();
    let main = package.declaration_catalog().source_backed_app_main().unwrap();
    let declaration = package.batch().declarations()
        .find(|row| row.identity().same_as(main.parser_identity())).unwrap();
    let mut builder = crate::mir::MirBuilder::new();
    let function = package.batch().with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity|
        builder.lower_map_dependency_for_test(input,
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
            main.parser_identity(), identity.method_source_observation().cloned(),
            Rc::clone(&package.ordinary_new_claim_ledger), None)).unwrap().unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger.local_field_reads.borrow().keys().next().unwrap().clone();
    let original = ledger.expression_field_read_emission_v1(&site,
        ledger.source_field_requirement_v1(&site).unwrap().unwrap().plan(),
        crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger)).unwrap();
    if owned_ready {
        let field = ledger.local_field_reads.borrow()[&site].source.field();
        let fault = function.blocks.values().find_map(|block| match &block.terminator {
            Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::FieldSet { field: actual, .. }, fault_landing, .. })
                if *actual == field => Some(*fault_landing),
            _ => None,
        }).unwrap();
        let mut changed = function.clone();
        let block = changed.blocks.get_mut(&original.0).unwrap();
        let position = block.instructions.iter().position(|row| row == &original.1).unwrap();
        block.instructions.remove(position);
        block.instruction_spans.remove(position);
        changed.blocks.get_mut(&fault).unwrap().add_instruction(original.1.clone());
        for block in changed.blocks.values_mut() {
            block.successors = block.successors_from_terminator().into_iter().collect();
        }
        // Coordinate the retained read position with the altered image, so
        // the failure proves store Normal dominance, not stale row metadata.
        if let LocalReadProgress::Emitted { block, .. } = &mut ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().progress { *block = fault; }
        let issue = ledger.validate_owned_ready_reads_v1(site.owner(), &changed,
            crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger),
            |binding| Ok(Some(binding.clone()))).unwrap_err();
        assert!(issue.contains("owned-ready/store-normal-dominance"), "{issue}");
        if let LocalReadProgress::Emitted { block, .. } = &mut ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().progress { *block = original.0; }
    }
    let exit = ledger.source_field_requirement_v1(&site).unwrap().unwrap().plan().root().statement().clone();
    ledger.root_exits.borrow_mut().insert((site.owner(), exit.into()), local_commit::RootHomeExitProgress::Unavailable);
    assert!(ledger.take_local_field_read(&site, |_| panic!("duplicate read cannot resolve"))
        .unwrap_err().contains("already-taken"));
    let source = ledger.local_field_reads.borrow().get(&site).unwrap().source.clone();
    assert!(ledger.record_local_field_read(&site, original.0, ValueId(9999), ValueId(9998), source.field(), &source)
        .unwrap_err().contains("emission-mismatch"));
    assert!(ledger.field_reads_complete());
    ledger.validate_field_reads(site.owner(), &function).unwrap();
    assert_eq!(ledger.validate_finalized_new_root(&function).unwrap(),
        crate::mir::function::RootOrdinaryNewObservation::Unavailable(
            crate::mir::function::RootOrdinaryNewUnavailable::RootExitUnavailable));
    assert_eq!(ledger.expression_field_read_emission_v1(&site,
        ledger.source_field_requirement_v1(&site).unwrap().unwrap().plan(),
        crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger)).unwrap(), original);
    assert!(ledger.validate_artifact_after_compiler_finishing(&function).unwrap_err()
        .contains("artifact-source-unavailable"));
    }
}

/// A result claim that declines to `RetainedUnavailable` releases its
/// staged argument field reads at the prepare point — the raw lane owns
/// the physical read, and `field_reads_complete`/`validate_field_reads`
/// treat the released row as discharged without expecting an
/// `ObjectFieldGet`. `Pending` rows still gate on `unconsumed-read`.
#[test]
fn retained_unavailable_claim_releases_staged_argument_reads() {
    for ready in [true, false] {
    let mut text = "box Node {
             value: i64
             tick() { }
             make() { me.value = 3 me.tick() return new Leaf(me.value) }
         }
         box Leaf { value: i64 birth(v) { me.value = v } }
         static box Main { main() { return 0 } }".to_owned();
    if !ready {
        text = text.replace("me.value = v }", "me.value = v + 1 }");
    }
    let package = issue_with_brand_catalog(&text)
    .expect("unavailable-claim package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (staged_sites, staged_objects): (Vec<_>, Vec<_>) = {
        let reads = ledger.argument_field_reads.borrow();
        reads
            .iter()
            .map(|(site, row)| (site.clone(), row.object))
            .unzip()
    };
    assert_eq!(staged_sites.len(), 1, "me.value stages one argument read");
    let site = ledger
        .pending_result_claims_for_test()
        .keys()
        .next()
        .expect("result claim")
        .clone();
    let claim = ledger
        .try_take_result(&site, "Leaf", 1)
        .expect("result take")
        .expect("result claim present");
    assert!(claim.argument_rows().is_ok(), "the same field argument remains staged");
    assert_eq!(claim.construction().is_ok(), ready, "only runtime Birth arithmetic remains unsupported");
    let prepared = ledger.prepare_result_new_emission(&claim).unwrap().is_ok();
    if ready {
        assert!(prepared, "the supported Unit statement no longer declines the claim");
        continue;
    }
    assert!(!prepared, "unsupported runtime Birth arithmetic leaves the claim unavailable");
    {
        let reads = ledger.argument_field_reads.borrow();
        assert!(
            matches!(reads[&staged_sites[0]].progress, Progress::Released),
            "the declined claim's staged read releases"
        );
    }
    assert!(ledger.field_reads_complete());
    // Release is final — the staged site can no longer be taken.
    assert!(ledger
        .take_argument_field_read(&staged_sites[0], staged_objects[0], |_, _| {
            panic!("released rows never take")
        })
        .unwrap_err()
        .contains("already-taken"));
    // And no `ObjectFieldGet` expectation is registered for the owner.
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        crate::mir::FunctionSignature {
            name: "release_validation".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::CONTROL,
        },
        entry,
    );
    let block = function.blocks.get_mut(&entry).unwrap();
    block.set_terminator(MirInstruction::Return { value: None });
    ledger
        .validate_field_reads(site.owner(), &function)
        .unwrap();
    }
}

#[test]
fn terminal_read_rows_retain_alias_sites_and_commit_only_complete_expression() {
    for (suffix, expected, alias) in [
        ("return page.slot + page.slot", 2, false),
        ("local alias = page return alias.slot", 1, true),
        (
            "local alias = page local copy = alias return copy.slot",
            1,
            true,
        ),
        ("return page.slot + true", 0, false),
    ] {
        let source = format!(
            "box Page {{ slot: i64 birth() {{ me.slot = 7 }} }}
            static box Main {{ main() {{ local page = new Page() {suffix} }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let reads = ledger.local_field_reads.borrow();
        assert_eq!(reads.len(), expected, "{suffix}");
        for (site, row) in reads.iter() {
            assert_eq!(row.receiver.owner(), site.owner());
            assert_eq!(row.receiver != row.home, alias);
            assert_eq!(row.source.field().declaration_ordinal(), 0);
            assert!(matches!(row.progress, LocalReadProgress::Pending));
        }
        let selected = reads.keys().next().cloned();
        drop(reads);
        if let Some(site) = selected {
            let error = ledger
                .take_local_field_read(&site, |_| {
                    panic!("wrong root phase must reject before receiver resolution")
                })
                .unwrap_err();
            assert!(
                error.contains("ordinary-new/local-commit/root-home-not-installed"),
                "{error}"
            );
        }
    }
}

#[test]
fn local_field_declaration_descriptor_survives_emission_and_rejects_replacement() {
    for declared in ["i64", "usize", "u64"] {
        let package = issue_with_brand_catalog(&format!(
            "box Pool {{ size: {declared} = 0 birth() {{ }} }}
             static box Main {{ main() {{ local pool = new Pool() local n = pool.size return n }} }}",
        )).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let site = ledger.local_field_reads.borrow().keys().next().unwrap().clone();
        let source = ledger.staged_local_field_source(&site).unwrap();
        assert_eq!(source.declared_type_name(), Some(declared));
        let (base, taken) = ledger.take_local_field_read(&site, |_| Ok(ValueId(10))).unwrap().unwrap();
        assert!(Rc::ptr_eq(&source, &taken));
        assert!(ledger.take_local_field_read(&site, |_| panic!("duplicate take")).is_err());
        let field = taken.field();
        let foreign = Rc::new((*taken).clone());
        assert!(ledger.record_local_field_read(&site, BasicBlockId(0), ValueId(11), base, field, &foreign).is_err());
        ledger.record_local_field_read(&site, BasicBlockId(0), ValueId(11), base, field, &taken).unwrap();
        let mut function = MirFunction::new(crate::mir::FunctionSignature {
            name: "field_source_state".into(), params: vec![], return_type: crate::mir::MirType::Integer,
            effects: crate::mir::EffectMask::PURE,
        }, BasicBlockId(0));
        function.blocks.get_mut(&BasicBlockId(0)).unwrap().add_instruction(
            MirInstruction::ObjectFieldGet { dst: ValueId(11), base, field },
        );
        ledger.validate_field_reads(site.owner(), &function).unwrap();
        // Same field and class, but replacing the declaration descriptor
        // after emission must fail the independent source correspondence.
        let replacement = LocalFieldReadSourceV1::from_declaration(
            field, crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar, Some("i32".into()),
        );
        ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source = replacement;
        assert!(ledger.validate_field_reads(site.owner(), &function).unwrap_err()
            .contains("source-descriptor-drift"));
        ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source = source;
        ledger.validate_field_reads(site.owner(), &function).unwrap();
    }
    let package = issue_with_brand_catalog("box Pool { values: ArrayBox = new ArrayBox() birth() {} seed() {
        local a = me.values local i = 0 loop(i < 1) { a.push(i) i = i + 1 }
    } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger.local_field_reads.borrow().keys().next().unwrap().clone();
    let (base, source) = ledger.take_local_field_read(&site, |_| Ok(ValueId(10))).unwrap().unwrap();
    let crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(array) = source.result()
        else { panic!("original field-resident Array source"); };
    assert!(ledger.record_local_field_read(&site, BasicBlockId(0), ValueId(11), base, source.field(), &source)
        .unwrap_err().contains("source-emission-kind"), "Array must never use ObjectFieldGet");
    let instruction = MirInstruction::FieldGet { dst: ValueId(11), base,
        field: array.integer_requirement_v1().unwrap().field_name().into(), declared_type: Some(crate::mir::MirType::Box("ArrayBox".into())) };
    let foreign = Rc::new((*source).clone());
    assert!(ledger.record_local_field_read_binding(&site, BasicBlockId(0), instruction.clone(), &foreign).is_err());
    ledger.record_local_field_read_binding(&site, BasicBlockId(0), instruction.clone(), &source).unwrap();
    let mut function = MirFunction::new(crate::mir::FunctionSignature {
        name: "array_field_source".into(), params: vec![], return_type: crate::mir::MirType::Void,
        effects: crate::mir::EffectMask::PURE,
    }, BasicBlockId(0));
    function.blocks.get_mut(&BasicBlockId(0)).unwrap().add_instruction(instruction.clone());
    ledger.validate_field_reads(site.owner(), &function).unwrap();
    let mut lent = 0;
    ledger.visit_array_field_read_originals_v1(site.owner(), |actual_site, actual_source, actual| {
        assert_eq!(actual_site, &site);
        assert!(Rc::ptr_eq(actual_source, &source));
        assert_eq!(actual, &(BasicBlockId(0), instruction.clone()));
        lent += 1;
        Ok(())
    }).unwrap();
    assert_eq!(lent, 1);
    let foreign = Rc::new((*source).clone());
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source = foreign;
    assert!(ledger.visit_array_field_read_originals_v1(site.owner(), |_, _, _| Ok(())).unwrap_err()
        .contains("source-descriptor-drift"));
    ledger.local_field_reads.borrow_mut().get_mut(&site).unwrap().source = source;
    for mutation in 0..5 {
        let mut changed = instruction.clone();
        let MirInstruction::FieldGet { dst, base, field, declared_type } = &mut changed else { unreachable!() };
        match mutation {
            0 => *dst = ValueId(12), 1 => *base = ValueId(12), 2 => *field = "foreign".into(),
            3 => *declared_type = Some(crate::mir::MirType::Integer), _ => {},
        }
        let mut altered = function.clone();
        altered.blocks.get_mut(&BasicBlockId(0)).unwrap().instructions[0] = changed;
        if mutation == 4 { altered.blocks.get_mut(&BasicBlockId(0)).unwrap().add_instruction(instruction.clone()); }
        assert!(ledger.validate_field_reads(site.owner(), &altered).is_err(), "Array physical read drift {mutation}");
    }


    // A fixed provider may identify a borrowed local without granting the
    // Integer operation or either unverified physical field-read protocol.
    let package = issue_with_brand_catalog(r#"box Pool { init { values } birth() { me.values = new ArrayBox() }
        seed() { local a = me.values print("alias") return 0 }
    } static box Main { main() { local pool = new Pool() return pool.seed() } }"#).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger.local_field_reads.borrow().iter().find_map(|(site, row)| matches!(
        row.source.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(_)
    ).then(|| site.clone())).expect("original fixed field alias");
    assert!(ledger.completion_source_for_owner_v1(site.owner())
        .and_then(|completion| completion.cleanup().root_flow()).is_some(), "the borrowed local prefix completes without a new Home");
    let (base, source) = ledger.take_local_field_read(&site, |_| Ok(ValueId(10))).unwrap().unwrap();
    let crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(alias) = source.result() else { unreachable!() };
    assert!(alias.integer_requirement_v1().is_none());
    let original = ledger.staged_local_field_source(&site).unwrap();
    assert!(Rc::ptr_eq(&source, &original));
    for instruction in [
        MirInstruction::ObjectFieldGet { dst: ValueId(11), base, field: source.field() },
        MirInstruction::FieldGet { dst: ValueId(11), base, field: "values".into(), declared_type: Some(crate::mir::MirType::Box("ArrayBox".into())) },
    ] {
        assert!(ledger.record_local_field_read_binding(&site, BasicBlockId(0), instruction, &source).is_err(),
            "source alias alone must not issue a physical Array read");
    }

    // Real Count/Get calls lend only their exact initializer alias. Merely
    // declaring another Array field does not transfer that operation evidence.
    let package = issue_with_brand_catalog(r#"box Pool { init { values, other }
        birth() { me.values = new ArrayBox() me.other = new ArrayBox() }
        seed() { local a = me.values local b = me.other
            local n = a.length() local x = a.get(0) return n }
    } static box Main { main() { local pool = new Pool() return pool.seed() } }"#).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let rows = ledger.local_field_reads.borrow();
    let aliases: Vec<_> = rows.values().filter_map(|row| match row.source.result() {
        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(alias)
            if alias.fixed_source_v1().is_some() => Some((&row.source, alias)),
        _ => None,
    }).collect();
    assert_eq!(aliases.len(), 2);
    for (source, alias) in aliases {
        let name = alias.fixed_source_v1().unwrap().0.name();
        if name == "values" {
            assert_eq!(source.array_reads.len(), 2);
            assert!(source.array_reads.iter().any(|read| read.is_count()));
            assert!(source.array_reads.iter().any(|read| read.is_dynamic_get()));
            assert!(source.array_reads.iter().all(|read| read.receiver_alias_v1() == Some(alias)
                && read.field() == source.field()));
        } else {
            assert_eq!(name, "other");
            assert!(source.array_reads.is_empty());
        }
    }

    // Direct Text append borrows the original canonical field, whereas the
    // Loop Integer origin above keeps its established FieldGet protocol.
    let package = issue_with_brand_catalog(r#"box Pool { values: ArrayBox = new ArrayBox() birth() {} seed() {
        me.values.push("text") return 0
    } } static box Main { main() { local pool = new Pool() return pool.seed() } }"#).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let frame = package.parameter_contracts.iter().find_map(|contract|
        ledger.completion_source_for_owner_v1(contract.owner)
            .and_then(|completion| completion.cleanup().root_flow())
            .and_then(|flow| flow.named_arrays().iter().find_map(|row| match row {
                crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::FieldTextAppend(frame) => Some(Rc::clone(frame)),
                _ => None,
            }))).expect("original Text append source");
    let site = frame.source().receiver();
    let contract = package.parameter_contracts.iter().find(|row| row.owner == site.owner()).unwrap();
    package.batch().with_lowering_input(contract.batch_slot, |input|
        ledger.stage_source_array_receiver_v1(input, site, frame.source().field_name())).unwrap().unwrap();
    let (base, source) = ledger.take_local_field_read(site, |_| Ok(ValueId(10))).unwrap().unwrap();
    let instruction = MirInstruction::ObjectFieldGet { dst: ValueId(11), base, field: source.field() };
    let legacy = MirInstruction::FieldGet { dst: ValueId(11), base, field: frame.source().field_name().into(),
        declared_type: Some(crate::mir::MirType::Box("ArrayBox".into())) };
    assert!(ledger.record_local_field_read_binding(site, BasicBlockId(0), legacy, &source).is_err(),
        "original direct source cannot switch to the legacy field reader");
    ledger.record_local_field_read_binding(site, BasicBlockId(0), instruction.clone(), &source).unwrap();
    let mut function = MirFunction::new(crate::mir::FunctionSignature {
        name: "text_array_field_source".into(), params: vec![], return_type: crate::mir::MirType::Void,
        effects: crate::mir::EffectMask::PURE,
    }, BasicBlockId(0));
    function.blocks.get_mut(&BasicBlockId(0)).unwrap().add_instruction(instruction);
    ledger.validate_field_reads(site.owner(), &function).unwrap();
    ledger.visit_array_field_read_originals_v1(site.owner(), |_, _, _| Ok(())).unwrap();

}

#[test]
fn returned_field_argument_descriptor_retains_exact_use_and_refuses_source_loss() {
    let text = "box Token {} box Wrapper { child: Token birth(child) { me.child = child } }
        box Maker { make() { return new Token() }
            wrap() { local h = me.make() return new Wrapper(h) }
            consume(h) { if h == null { return 0 } return 1 } }
        static box Main { main() { local maker = new Maker() local w = maker.wrap()
            local sibling = maker.wrap() maker.consume(w.child) return 0 } }";
    let mut package = issue_with_brand_catalog(text).unwrap();
    {
        let ledger = &package.ordinary_new_claim_ledger;
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let slots = ledger.lexical_instance_calls.borrow();
        let wrappers: Vec<_> = slots.values().filter_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row) if row.source_target().target().name() == "wrap" => Some(row.source_target()),
            _ => None,
        }).collect();
        assert_eq!(wrappers.len(), 2, "both real LOCAL acquisitions finish packets");
        for target in &wrappers {
            assert!(ingress.recheck_field_producer_callee_v1(target,
                &ledger.callable_result_classes, &ledger.field_write_claims).unwrap());
            let observation = ledger.handle_call_source(target.call_site()).unwrap();
            let destination = observation.local_binding().unwrap().1;
            ledger.callable_result_classes.check_owning_local_object_source_v1(
                package.batch(), &package.selected, target, destination).unwrap();
            let foreign = wrappers.iter().find(|other| other.call_site() != target.call_site()).unwrap();
            let wrong_destination = ledger.handle_call_source(foreign.call_site()).unwrap().local_binding().unwrap().1;
            assert!(ledger.callable_result_classes.check_owning_local_object_source_v1(
                package.batch(), &package.selected, target, wrong_destination).is_err());
            assert!(ingress.recheck_field_producer_callee_v1(target, &ledger.callable_result_classes,
                &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1::default()).is_err());
        }
        assert_eq!(wrappers.iter().filter(|target|
            ingress.original_field_producer_source_v1(target).unwrap().is_some()).count(), 1,
            "a sibling does not inherit the demanded Field hop loan");
    }
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger.local_field_reads.borrow().values()
        .find(|row| row.source.returned_argument_v1().is_some())
        .map(|row| Rc::clone(&row.source)).unwrap_or_else(|| panic!(
            "original argument reconstruction must stage its descriptor; root={:?}; pending={:?}",
            ledger.root_completion.as_ref().map(|row| row.as_ref().map(|completion| completion.owner())),
            ledger.borrowed_formal_actuals));
    let (call, ordinal, loan) = source.returned_argument_v1().unwrap();
    let (call, ordinal, loan) = (call.clone(), ordinal, loan.clone());
    {
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let target = ingress.object_source_target_at_v1(loan.acquisition()).unwrap().unwrap();
        let (producer, destination) = ingress.recheck_field_producer_source_v1(target,
            &ledger.callable_result_classes, &ledger.field_write_claims).unwrap().unwrap();
        assert_eq!(producer.source(), target);
        assert_eq!(destination, loan.parent());
        assert!(std::ptr::eq(ingress.original_object_producer_source_v1(target).unwrap().unwrap(), producer));
        let parent = ledger.handle_call_source(loan.acquisition()).expect("original Field parent must issue real Handle LOCAL");
        assert_eq!(parent.local_binding().map(|(_, binding)| binding), Some(loan.parent()));
        assert!(ingress.recheck_field_producer_source_v1(target, &ledger.callable_result_classes,
            &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1::default()).is_err(),
            "passive capsule cannot hide loss of the current field writer");
    }
    {
        let slots = ledger.lexical_instance_calls.borrow();
        assert!(matches!(slots.get(&call), Some(LexicalInstanceCallDispositionSlotV1::Ready(_))),
            "original field input must finish the real Call disposition");
    }
    {
        let row = ledger.take_lexical_instance_call(call.owner(), call.site()).unwrap().unwrap();
        let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
        let actual = actuals.iter().find(|actual| actual.ordinal == ordinal).unwrap();
        let crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1::Read(read) = &actual.source
            else { panic!("original Call must retain its checked read actual"); };
        assert!(read.corroborates_field_input_v1(&call, ordinal, &loan));
    }
    let reborrow = ledger.retained_returned_field_read_source_v1(&call, ordinal, loan.parent(), loan.acquisition())
        .unwrap().unwrap();
    assert!(Rc::ptr_eq(&source, &reborrow));
    ledger.recheck_returned_field_argument_source_v1(&source).unwrap();
    let retained_row = ledger.local_field_reads.borrow_mut().remove(loan.read()).unwrap();
    assert!(ledger.recheck_returned_field_argument_source_v1(&source).is_err());
    assert!(!ledger.local_field_reads.borrow().contains_key(loan.read()),
        "final source recheck cannot restage a missing read");
    ledger.local_field_reads.borrow_mut().insert(loan.read().clone(), retained_row);
    let foreign = Rc::new((*source).clone());
    assert!(ledger.recheck_returned_field_argument_source_v1(&foreign).is_err(),
        "matching metadata cannot replace original source identity");
    assert!(matches!(ledger.local_field_reads.borrow()[loan.read()].progress, LocalReadProgress::Pending));
    assert!(ledger.retained_returned_field_read_source_v1(&call, ordinal + 1, loan.parent(), loan.acquisition()).is_err());
    assert!(ledger.retained_returned_field_read_source_v1(&call, ordinal, loan.parent(), &call).is_err());
    let (_, taken) = ledger.take_local_field_read(loan.read(), |_| Ok(ValueId(10))).unwrap().unwrap();
    assert!(Rc::ptr_eq(&source, &taken));
    assert!(Rc::ptr_eq(&source, &ledger.retained_returned_field_read_source_v1(
        &call, ordinal, loan.parent(), loan.acquisition()).unwrap().unwrap()));
    assert!(matches!(ledger.local_field_reads.borrow()[loan.read()].progress, LocalReadProgress::Taken { .. }),
        "source reconstruction cannot reset affine progress");
    ledger.local_field_reads.borrow_mut().get_mut(loan.read()).unwrap().source =
        LocalFieldReadSourceV1::from_declaration(loan.field(),
            crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias("Token".into()), Some("Token".into()));
    assert!(ledger.retained_returned_field_read_source_v1(&call, ordinal, loan.parent(), loan.acquisition()).is_err(),
        "erasing original argument metadata cannot select generic declaration proof");
    ledger.local_field_reads.borrow_mut().get_mut(loan.read()).unwrap().source = source;
    ledger.field_write_claims = super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1::default();
    assert!(ledger.retained_returned_field_read_source_v1(&call, ordinal, loan.parent(), loan.acquisition()).is_err());
}
