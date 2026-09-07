use super::super::tests::source;
use super::*;

#[test]
fn terminal_source_rejects_wrong_exit_target_and_dropped_value_relation() {
    let (product, window) = source("local a: Array<i64> = []\nreturn 30", 0);
    let (foreign, _) = source("local a: Array<i64> = []\nreturn 30", 1);
    let core = product.core().data();
    let shape = product.body_shape();
    let BodyStatementShapeV1::Return { site, value } = shape.statements().last().unwrap() else {
        panic!("Return")
    };
    let exit = core
        .resolved_exits
        .get(&ResolvedExitSiteV1::Statement(site.clone()))
        .unwrap();
    validate_return_target(exit, core.function_region).unwrap();
    assert!(validate_return_target(exit, foreign.core().data().function_region).is_err());
    let wrong = crate::mir::resolved_semantics::ResolvedExitRecordV1::new(
        exit.source_region(),
        ResolvedExitOriginV1::ExplicitBreak,
        ResolvedControlTransferV1::Return {
            target_function: core.function_region,
        },
    );
    assert!(validate_return_target(&wrong, core.function_region).is_err());
    let value = value.as_ref().unwrap();
    let literal = core.expression_source.literal(value);
    let relations = shape
        .relations()
        .iter()
        .filter(|row| row.parent() == site.node())
        .collect::<Vec<_>>();
    assert_eq!(
        validate_return_value(Some(value), literal, &relations).unwrap(),
        RootResult::Integer {
            site: value.clone(),
            value: 30
        }
    );
    assert!(validate_return_value(Some(value), literal, &[]).is_err());
    assert!(validate_return_value(None, None, &relations).is_err());
    assert!(validate_return_value(Some(value), literal, &[relations[0], relations[0]]).is_err());
    let homes = [product
        .expression_source()
        .initializers()
        .next()
        .unwrap()
        .binding()];
    assert!(
        RootTerminalCoverage::issue(&product, &window, 0, true, &homes)
            .require()
            .is_err()
    );
    assert!(
        RootTerminalCoverage::issue(&product, &window, 1, true, &[homes[0], homes[0]])
            .require()
            .is_err()
    );
    let foreign_home = foreign
        .expression_source()
        .initializers()
        .next()
        .unwrap()
        .binding();
    assert!(
        RootTerminalCoverage::issue(&product, &window, 1, true, &[foreign_home])
            .require()
            .is_err()
    );
}
