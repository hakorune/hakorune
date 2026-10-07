//! Actual raw source lowering retains original literal append observations.
use super::*;

#[test]
fn borrowed_literal_original_source_records_exact_compare_child_only() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for condition in ["requested > 0", "0 > requested", "alias > 0"] {
            let alias = if condition.starts_with("alias") {
                "local alias = requested"
            } else {
                ""
            };
            let text = format!("box Counter {{ limit: i64 birth() {{ me.limit = 10 }} check(requested): i64 {{ {alias} if {condition} {{ return 7 }} return 3 }} }} static box Main {{ main() {{ local c = new Counter() return c.check(15) }} }}");
            MirCompiler::with_options(false).compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                classify_pretransform_report(verification);
                let handoff = view.retained_handoff.unwrap();
                let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Counter", "check", 1));
                let owner = handoff.callables().unwrap().completed_result(&key).unwrap().owner();
                let function = &view.module().functions["Counter.check/1"];
                let source = handoff.root_source().unwrap();
                let mut values = std::collections::BTreeSet::new();
                let mut sites = std::collections::BTreeSet::new();
                source.with_borrowed_ordinary_compare_integer_literals_v1(owner, function, |binary, site, value, original| {
                    assert_eq!(binary.owner(), owner);
                    assert_eq!(site.owner(), owner);
                    assert!(matches!(&original.1, crate::mir::MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(0) } if *dst == value));
                    assert!(values.insert(value), "one record per actual append");
                    sites.insert(site.clone());
                    Ok(())
                })?;
                assert_eq!(function.blocks.values().flat_map(|block| block.instructions.iter()).filter(|row| matches!(row, crate::mir::MirInstruction::Const { value: crate::mir::ConstValue::Integer(0), .. })).count(), 1, "selected literal is never rematerialized");
                assert_eq!(sites.len(), 1, "one original literal child: {condition}");
                assert_eq!(values.len(), 1, "one original raw literal append: {condition}");
                // The selected Cond consumes the original checked Bool;
                // the opaque operation is not re-executed.
                assert_eq!(function.blocks.values().flat_map(|block| block.instructions.iter())
                    .filter(|instruction| matches!(instruction, crate::mir::MirInstruction::Compare { .. }))
                    .count(), 1, "one original checked execution, reused Bool");
                if condition.starts_with("alias") {
                    // Source observation succeeds; the independently checked
                    // alias rematerialization remains an explicit physical edge.
                    let error = view.issue_lifecycle_physical_abi_input().unwrap_err();
                    assert!(error.contains("borrowed-use/unproved-copy"), "{error}");
                } else {
                    view.issue_lifecycle_physical_abi_input()?;
                }
                let mut comparisons = 0;
                source.with_borrowed_ordinary_compares_v1(owner, function, |loan, children, original| {
                    assert_eq!(loan.owner(), owner);
                    assert_eq!(loan.operator(), crate::mir::CompareOp::Gt);
                    assert_eq!(loan.operand_sites().0.owner(), owner);
                    assert_eq!(loan.operand_sites().1.owner(), owner);
                    assert_ne!(children.0, children.1);
                    assert!(matches!(&original.1, crate::mir::MirInstruction::Compare { op: crate::mir::CompareOp::Gt, .. }));
                    comparisons += 1;
                    Ok(())
                })?;
                assert_eq!(comparisons, 1, "one original source Compare completion: {condition}");
                for remove_compare in [true, false] {
                    let mut drifted = function.clone();
                    if remove_compare {
                        for block in drifted.blocks.values_mut() {
                            block.instructions.retain(|row| !matches!(row, crate::mir::MirInstruction::Compare { .. }));
                        }
                    } else {
                        for block in drifted.blocks.values_mut() {
                            if let Some(crate::mir::MirInstruction::Branch { condition, .. }) = &mut block.terminator {
                                *condition = crate::mir::ValueId(u32::MAX);
                            }
                        }
                    }
                    assert!(source.with_borrowed_ordinary_compares_v1(owner, &drifted,
                        |_, _, _| panic!("final execution/consumer drift must refuse before source loan")).is_err());
                }

                let mut missing_literal = function.clone();
                for block in missing_literal.blocks.values_mut() {
                    block.instructions.retain(|row| !matches!(row, crate::mir::MirInstruction::Const { dst, .. } if values.contains(dst)));
                }
                assert!(source.with_borrowed_ordinary_compares_v1(owner, &missing_literal,
                    |_, _, _| panic!("deleted original literal must refuse before source loan")).is_err());
                let mut foreign = function.clone();
                foreign.signature.name = "foreign/0".into();
                assert!(source.with_borrowed_ordinary_compare_integer_literals_v1(owner, &foreign,
                    |_, _, _, _| panic!("foreign function must not publish observations")).is_err());
                Ok(())
            }).unwrap();
        }
    });
}

#[test]
fn borrowed_compare_two_formals_share_one_original_append_in_both_orders() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for condition in ["p > q", "q > p"] {
            let text = format!("box Counter {{ birth() {{}} check(p,q): i64 {{ if {condition} {{ return 7 }} return 3 }} }} static box Main {{ main() {{ local c = new Counter() return c.check(15,10) }} }}");
            MirCompiler::with_options(false).compile_normal_with_published(request(&text), |view, verification| -> Result<(),String> {
                classify_pretransform_report(verification);
                let handoff=view.retained_handoff.unwrap();
                let key=crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Counter","check",2));
                let owner=handoff.callables().unwrap().completed_result(&key).unwrap().owner();
                let function=&view.module().functions["Counter.check/2"];
                let source=handoff.root_source().unwrap();
                let mut count=0;
                source.with_borrowed_ordinary_compares_v1(owner,function,|loan,children,original| {
                    assert_eq!(loan.owner(),owner);
                    assert_eq!(loan.operator(),crate::mir::CompareOp::Gt);
                    assert_ne!(loan.operand_sites().0,loan.operand_sites().1);
                    assert_ne!(children.0,children.1);
                    assert!(matches!(&original.1,crate::mir::MirInstruction::Compare {op:crate::mir::CompareOp::Gt,..}));
                    count+=1;
                    Ok(())
                })?;
                assert_eq!(count,1,"one original source append for two borrowed operands: {condition}");
                view.issue_lifecycle_physical_abi_input()?;
                let mut foreign=function.clone();
                foreign.signature.name="foreign/0".into();
                assert!(source.with_borrowed_ordinary_compares_v1(owner,&foreign,|_,_,_| panic!("foreign function")).is_err());
                Ok(())
            }).unwrap();
        }
    });
}
