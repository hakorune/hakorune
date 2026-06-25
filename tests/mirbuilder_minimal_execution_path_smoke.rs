use nyash_rust::ast::{ASTNode, LiteralValue, Span};
use nyash_rust::mir::{ConstValue, MirBuilder, MirInstruction, MirType};

#[test]
fn mirbuilder_minimal_literal_integer_path_smoke() {
    let mut builder = MirBuilder::new();
    let ast = ASTNode::Literal {
        value: LiteralValue::Integer(0),
        span: Span::unknown(),
    };

    let module = builder
        .build_module(ast)
        .expect("literal integer build_module should succeed");
    let main = module
        .get_function("main")
        .expect("minimal literal path should create main");

    assert_eq!(main.signature.return_type, MirType::Integer);
    assert!(
        module.get_function("condition_fn").is_some(),
        "condition_fn injection is part of the live finalize path"
    );

    let literal_dst = main
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .find_map(|inst| match inst {
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(0),
            } => Some(*dst),
            _ => None,
        })
        .expect("literal integer const should be emitted");

    let returns_literal = main.blocks.values().any(|block| {
        matches!(
            &block.terminator,
            Some(MirInstruction::Return {
                value: Some(value),
            }) if *value == literal_dst
        )
    });
    assert!(
        returns_literal,
        "main should return the emitted literal integer value"
    );
}
