//! Checked LessEqual uses the original comparison and source-owned carriers.
use super::*;
#[test]
fn borrowed_less_equal_publishes_both_sides_shared_carrier_and_original_alias() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for condition in ["p <= 0", "0 <= p", "p <= p", "alias <= 0", "0 <= alias"] {
            let prefix = if condition.contains("alias") {
                "local first = p local alias = first"
            } else {
                ""
            };
            let text = format!("box Counter {{ birth() {{}} check(p): i64 {{ {prefix} if {condition} {{ return 7 }} return 3 }} }} static box Main {{ main() {{ local c = new Counter() return c.check(15) }} }}");
            MirCompiler::with_options(false).compile_normal_with_published(request(&text), |view, verification| -> Result<(), String> {
                classify_pretransform_report(verification);
                let handoff = view.retained_handoff.unwrap();
                let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Counter", "check", 1));
                let owner = handoff.callables().unwrap().completed_result(&key).unwrap().owner();
                let function = &view.module().functions["Counter.check/1"];
                let source = handoff.root_source().unwrap();
                let mut count = 0;
                source.with_borrowed_ordinary_compares_v1(owner, function, |loan, _, original| {
                    assert_eq!(loan.operator(), crate::mir::CompareOp::Le);
                    assert!(matches!(original.1, MirInstruction::Compare { op: crate::mir::CompareOp::Le, .. }));
                    count += 1;
                    Ok(())
                })?;
                assert_eq!(count, 1);
                view.issue_lifecycle_physical_abi_input()?;
                let mut drifted = function.clone();
                for block in drifted.blocks.values_mut() { for row in &mut block.instructions {
                    if let MirInstruction::Compare { op, .. } = row { *op = crate::mir::CompareOp::Gt; }
                } }
                assert!(source.with_borrowed_ordinary_compares_v1(owner, &drifted,
                    |_, _, _| panic!("original Le cannot become Gt")).is_err());
                Ok(())
            }).unwrap_or_else(|error| panic!("{condition}: {error}"));
        }
    });
}

#[test]
fn borrowed_less_equal_normal_consumers_require_executed_dominating_guard() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for consumer in [
            "me.total = me.total + p",
            "me.sizes.set(0, p)",
            "local h = new Item(p, 3)",
        ] {
            for bypass in [false, true] {
                let guard = if bypass {
                    "if me.limit == 0 { if p <= me.limit { return 0 } }"
                } else {
                    "if p <= me.limit { return 0 }"
                };
                let (child, fields, birth) = if consumer.contains("new Item") {
                    ("box Item { id: i64 serial: i64 birth(id, serial) { me.id = id me.serial = serial } }", "limit: i64", "me.limit = 10")
                } else if consumer.contains(".set") {
                    (
                        "",
                        "limit: usize sizes: ArrayBox = new ArrayBox()",
                        "me.limit = 10",
                    )
                } else {
                    (
                        "",
                        "limit: usize total: usize",
                        "me.limit = 10 me.total = 0",
                    )
                };
                let text = format!("{child} box Store {{ {fields} birth() {{ {birth} }} check(p): i64 {{ {guard} {consumer} return 1 }} }} static box Main {{ main() {{ local s = new Store() return s.check(15) }} }}");
                let result = MirCompiler::with_options(false).compile_normal_with_published(
                    request(&text),
                    |view, verification| -> Result<(), String> {
                        classify_pretransform_report(verification);
                        view.issue_lifecycle_physical_abi_input()?;
                        Ok(())
                    },
                );
                if bypass {
                    assert!(result.is_err(), "unexecuted guard: {consumer}");
                } else {
                    result.unwrap_or_else(|error| panic!("Normal Le guard: {consumer}: {error}"));
                }
            }
        }
    });
}
