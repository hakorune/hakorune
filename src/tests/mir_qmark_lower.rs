use crate::ast::{ASTNode, Span};
use crate::mir::{MirCompiler, MirPrinter};

#[test]
fn mir_lowering_of_qmark_propagate() {
    // Build AST: (new StringBox("ok"))?
    let ast = ASTNode::QMarkPropagate {
        expression: Box::new(ASTNode::New {
            class: "StringBox".to_string(),
            arguments: vec![ASTNode::Literal {
                value: crate::ast::LiteralValue::String("ok".to_string()),
                span: Span::unknown(),
            }],
            type_arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut c = MirCompiler::new();
    let out = c.compile(ast).expect("compile ok");
    let dump = MirPrinter::new().print_module(&out.module);
    assert!(
        dump.contains("call ") || dump.contains("boxcall "),
        "expected call/boxcall isOk/getValue in MIR:\n{}",
        dump
    );
    assert!(dump.contains("br "), "expected branch in MIR:\n{}", dump);
    assert!(
        dump.contains("ret "),
        "expected return on error path in MIR:\n{}",
        dump
    );
}
