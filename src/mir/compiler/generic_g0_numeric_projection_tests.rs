use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_projection::{
    issue_generic_g0_source_type_bundle_v1, issue_generic_g0_typed_source_bundle_v1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::numeric_substrate::NumericTarget;
use crate::parser::NyashParser;

const TYPED: &str = r#"
function generic_g0(i: i64, j: i64): i64 {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

fn parse_function(source: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture must produce a Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("fixture function")
}

fn typed_bundle(
    source: &str,
) -> Result<
    crate::mir::compiler::generic_g0_projection::VerifiedGenericSourceBundleG0,
    crate::mir::compiler::generic_g0_projection::GenericG0SourceTypeProjectionRejectV1,
> {
    let function = parse_function(source);
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).expect("resolve fixture");
    issue_generic_g0_source_type_bundle_v1(unit.root_function_input().expect("root input"))
}

#[test]
fn natural_plain_literals_issue_one_cumulative_numeric_bundle() {
    let bundle = typed_bundle(TYPED).expect("S0B source bundle");
    let typed = issue_generic_g0_typed_source_bundle_v1(bundle, NumericTarget::host())
        .expect("S0C numeric bundle");
    assert_eq!(typed.source().source_types().literals().len(), 4);
    assert_eq!(typed.numeric().parameters().len(), 2);
    assert_eq!(typed.numeric().literals().len(), 4);
    assert_eq!(typed.numeric().literals()[0].value, 3);
    assert_eq!(typed.return_abi().source_type_name(), "i64");
}
