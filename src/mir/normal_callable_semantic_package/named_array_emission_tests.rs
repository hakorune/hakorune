//! Source/package-to-emission transport tests; physical observations are explicit
//! test input, not proof that the production Array route has switched.
use super::*;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, BasicBlockId,
    ConstructionTarget, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};
use std::collections::BTreeMap;

type Rows = BTreeMap<
    crate::mir::builder::SelectedNormalCallableKeyV1,
    BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
>;

fn rows() -> Rows {
    let source = r#"static box Scan { run() {
        local arr = new ArrayBox()
        local i = 0
        loop(i < 1) { arr.push("text") i = i + 1 }
        return i
    } }"#;
    rows_from(source)
}

fn rows_from(source: &str) -> Rows {
    let mut resolver = FunctionSemanticResolverSessionV1::new(982).unwrap();
    let package = issue_normal_callable_semantic_package_v1(
        &mut resolver,
        super::resolved_selected_handoff_tests::final_source(source),
    )
    .unwrap();
    let ast = crate::parser::NyashParser::parse_from_string(source).unwrap();
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(&ast).unwrap();
    super::core_method_source::issue_source_core_method_calls_with_named_arrays_v1(
        &package.catalog,
        &package.batch,
        &package.selected,
        &brands,
    )
    .unwrap()
}

fn port(mut rows: Rows) -> NamedArrayWriteEmissionPortV1 {
    assert_eq!(rows.len(), 1);
    let (_, mut calls) = rows.pop_first().unwrap();
    assert_eq!(calls.len(), 1);
    let (_, mut row) = calls.pop_first().unwrap();
    let requirement = row.contract().named_array_requirement().unwrap();
    let (owner, site) = (requirement.owner(), requirement.construction().clone());
    row.record_named_allocation(owner, &site, ValueId::new(1))
        .unwrap();
    row.into_named_array_emission().unwrap().into_write_port()
}

fn emitted(port: &NamedArrayWriteEmissionPortV1) -> EmittedNamedArrayRequirementV1 {
    port.record_write(ArrayWriteSiteId(0), ValueId::new(1), ValueId::new(2))
        .unwrap();
    port.take_completed().unwrap()
}

fn module(row: &EmittedNamedArrayRequirementV1) -> MirModule {
    let marker = row.marker();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: row.caller().mir_symbol_projection(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::MUT,
        },
        BasicBlockId::new(0),
    );
    function
        .blocks
        .get_mut(&function.entry_block)
        .unwrap()
        .instructions = vec![
        MirInstruction::NewBox {
            dst: marker.allocation,
            target: ConstructionTarget::Named("ArrayBox".into()),
            args: vec![],
        },
        MirInstruction::ArrayElementWrite {
            site_id: marker.write,
            dst: None,
            kind: ArrayElementWriteKind::Push,
            producer: ArrayWriteProducerKind::MethodCall,
            receiver: marker.receiver,
            index: None,
            value: marker.argument,
        },
    ];
    function.metadata.named_array_write_obligations.push(marker);
    let mut module = MirModule::new("emission".into());
    module
        .add_cataloged_box_method(row.caller().clone(), function)
        .unwrap();
    module
}

#[test]
fn package_inventory_rejects_dropped_draft_and_allocation_without_write() {
    let source = rows();
    let collector = NamedArrayEmissionCollectorV1::from_source_rows(&source);
    let pending = port(source);
    assert!(pending
        .take_completed()
        .unwrap_err()
        .contains("write-not-emitted"));
    assert!(collector
        .finish()
        .unwrap_err()
        .contains("unconsumed-package-obligations"));
}

#[test]
fn package_handback_preserves_source_and_rejects_foreign_same_named_owner() {
    let source = rows();
    let collector = NamedArrayEmissionCollectorV1::from_source_rows(&source);
    let row = emitted(&port(source));
    let module = module(&row);
    collector.hand_back(vec![row]).unwrap();
    let retained = collector.finish().unwrap();
    validate_named_array_coverage(&module, &retained).unwrap();
    let foreign_source = rows();
    let foreign_collector = NamedArrayEmissionCollectorV1::from_source_rows(&foreign_source);
    let foreign_row = emitted(&port(foreign_source));
    assert_eq!(foreign_row.caller(), retained[0].caller());
    assert_ne!(foreign_row.owner(), retained[0].owner());
    assert!(validate_named_array_coverage(&module, &[foreign_row]).is_err());
    assert!(foreign_collector
        .hand_back(retained.into_vec())
        .unwrap_err()
        .contains("foreign-emission-handback"));
}

#[test]
fn recipe_clones_share_one_write_and_one_completion() {
    let source = rows();
    let collector = NamedArrayEmissionCollectorV1::from_source_rows(&source);
    let original = port(source);
    let cloned = original.clone();
    original
        .record_write(ArrayWriteSiteId(0), ValueId::new(1), ValueId::new(2))
        .unwrap();
    let row = cloned.take_completed().unwrap();
    assert!(original
        .take_completed()
        .unwrap_err()
        .contains("duplicate-emission-consumption"));
    collector.hand_back(vec![row]).unwrap();
    assert_eq!(collector.finish().unwrap().len(), 1);
    let duplicate = port(rows());
    duplicate
        .record_write(ArrayWriteSiteId(0), ValueId::new(1), ValueId::new(2))
        .unwrap();
    assert!(duplicate
        .clone()
        .record_write(ArrayWriteSiteId(1), ValueId::new(1), ValueId::new(2))
        .unwrap_err()
        .contains("duplicate-physical-write"));
}

#[test]
fn source_call_cannot_prepare_a_write_before_its_allocation() {
    let mut source = rows();
    let (_, mut calls) = source.pop_first().unwrap();
    let (_, row) = calls.pop_first().unwrap();
    assert!(row
        .into_named_array_emission()
        .unwrap_err()
        .contains("allocation-not-emitted"));
}

#[test]
fn two_source_writes_share_one_exact_construction() {
    let mut source = rows_from(
        r#"static box Scan { run() {
        local arr = new ArrayBox()
        local i = 0
        loop(i < 1) { arr.push("first") arr.push("second") i = i + 1 }
        return i
    } }"#,
    );
    let collector = NamedArrayEmissionCollectorV1::from_source_rows(&source);
    let (_, calls) = source.pop_first().unwrap();
    assert_eq!(calls.len(), 2);
    let mut emitted_rows = Vec::new();
    for (index, (_, mut row)) in calls.into_iter().enumerate() {
        let requirement = row.contract().named_array_requirement().unwrap();
        let (owner, site) = (requirement.owner(), requirement.construction().clone());
        row.record_named_allocation(owner, &site, ValueId::new(1))
            .unwrap();
        let port = row.into_named_array_emission().unwrap().into_write_port();
        port.record_write(
            ArrayWriteSiteId(index as u32),
            ValueId::new(1),
            ValueId::new(2),
        )
        .unwrap();
        emitted_rows.push(port.take_completed().unwrap());
    }
    let mut physical = module(&emitted_rows[0]);
    let second = emitted_rows[1].marker();
    let function = physical.functions.values_mut().next().unwrap();
    function
        .blocks
        .get_mut(&function.entry_block)
        .unwrap()
        .instructions
        .push(MirInstruction::ArrayElementWrite {
            site_id: second.write,
            dst: None,
            kind: ArrayElementWriteKind::Push,
            producer: ArrayWriteProducerKind::MethodCall,
            receiver: second.receiver,
            index: None,
            value: second.argument,
        });
    function.metadata.named_array_write_obligations.push(second);
    collector.hand_back(emitted_rows).unwrap();
    let retained = collector.finish().unwrap();
    validate_named_array_coverage(&physical, &retained).unwrap();
    physical
        .functions
        .values_mut()
        .next()
        .unwrap()
        .metadata
        .named_array_write_obligations
        .pop();
    assert!(validate_named_array_coverage(&physical, &retained)
        .unwrap_err()
        .contains("marker-source-mismatch"));
}

/// Source-issued relation with explicit physical observations for boundary tests.
/// This cannot fabricate a finalized callable cohort or authorize publication.
pub(crate) fn physical_observation_fixture() -> (MirModule, Vec<EmittedNamedArrayRequirementV1>) {
    let row = emitted(&port(rows()));
    (module(&row), vec![row])
}
