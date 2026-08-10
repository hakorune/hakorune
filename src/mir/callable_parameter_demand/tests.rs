use crate::mir::resolved_semantics::{
    BindingKindV1, FunctionSemanticResolverSessionV1, HomeDemandV1, SourceBindingSiteV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{issue_callable_parameter_demands_v1, CallableParameterDeclarationModeV1};

fn issue(source: &str, compilation: u32) -> super::VerifiedCallableParameterDemandCatalogV1 {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("callable parameter source parses");
    let mut resolver =
        FunctionSemanticResolverSessionV1::new(compilation).expect("resolver session opens");
    issue_callable_parameter_demands_v1(&mut resolver, parsed)
        .expect("complete callable parameter demand catalog")
}

#[test]
fn seals_static_instance_and_zero_parameter_declarations_atomically() {
    let catalog = issue(
        r#"
static box StaticApi {
    run(source, count: i64) { return count }
    ping() { return 0 }
}
box InstanceApi {
    read(offset) { return offset }
}
"#,
        7,
    );
    let declarations = catalog.declarations().collect::<Vec<_>>();

    assert_eq!(declarations.len(), 3);
    assert_eq!(
        declarations[0].mode(),
        CallableParameterDeclarationModeV1::StaticBoxMethod
    );
    assert_eq!(declarations[0].parameters().len(), 2);
    assert!(declarations[0]
        .parameters()
        .iter()
        .all(|row| row.demand() == HomeDemandV1::Handle));
    assert_eq!(declarations[1].parameters().len(), 0);
    assert_eq!(
        declarations[2].mode(),
        CallableParameterDeclarationModeV1::InstanceBoxMethod
    );
    assert_eq!(declarations[2].parameters().len(), 1);

    for declaration in declarations {
        let function = declaration
            .resolved_forest()
            .owner(declaration.owner())
            .expect("catalog retains its resolved declaration");
        assert_eq!(function.function_origin(), declaration.function_origin());
        for parameter in declaration.parameters() {
            assert_eq!(parameter.binding().owner(), declaration.owner());
            assert_eq!(
                function
                    .declaration_binding(&SourceBindingSiteV1::Parameter {
                        index: parameter.ordinal(),
                    })
                    .expect("exact parameter declaration binding"),
                parameter.binding()
            );
            assert_eq!(
                function
                    .binding(parameter.binding())
                    .expect("exact resolved parameter")
                    .kind(),
                BindingKindV1::Parameter {
                    index: parameter.ordinal(),
                }
            );
        }
    }
}

#[test]
fn parser_scan_loop_box_seals_all_fifteen_ordinary_demands() {
    let catalog = issue(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        11,
    );
    let declarations = catalog.declarations().collect::<Vec<_>>();

    assert_eq!(
        declarations
            .iter()
            .map(|row| row.parameters().len())
            .collect::<Vec<_>>(),
        [4, 3, 4, 4]
    );
    assert_eq!(
        declarations
            .iter()
            .map(|row| row.parameters().len())
            .sum::<usize>(),
        15
    );
    assert!(declarations.iter().all(|declaration| declaration
        .parameters()
        .iter()
        .all(|row| row.demand() == HomeDemandV1::Handle)));

    let skip_while = declarations[0];
    let position = &skip_while.parameters()[1];
    let function = skip_while
        .resolved_forest()
        .owner(skip_while.owner())
        .expect("skip_while resolved root");
    assert_eq!(
        function
            .binding(position.binding())
            .expect("position binding")
            .diagnostic_name(),
        "pos"
    );
}

#[test]
fn independent_resolver_sessions_cannot_reuse_parameter_identity() {
    let source = "static box Source { run(value) { return value } }";
    let first = issue(source, 13);
    let second = issue(source, 13);
    let first_row = first.declarations().next().expect("first declaration");
    let second_row = second.declarations().next().expect("second declaration");

    assert_ne!(first_row.owner(), second_row.owner());
    assert_ne!(
        first_row.parameters()[0].binding(),
        second_row.parameters()[0].binding()
    );
}
