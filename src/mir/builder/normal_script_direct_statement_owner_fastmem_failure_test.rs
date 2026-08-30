#[test]
fn direct_fastmem_body_failure_discards_candidate_and_reuses_compiler() {
    let request = |body, name| {
        NormalCompileRequestV1::for_mir_mode(
            ASTNode::Program {
                statements: vec![ASTNode::FastMemRegion {
                    contract: "PageMapV0".to_owned(),
                    body,
                    span: Span::new(230, 240, 230, 1),
                }],
                span: Span::unknown(),
            },
            Some(name),
            HashMap::new(),
        )
        .expect("normal FastMem request")
    };
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(request(
            vec![ASTNode::Variable {
                name: "missing_fastmem".to_owned(),
                span: Span::unknown(),
            }],
            "direct-fastmem-failure.hako",
        ))
        .expect_err("missing FastMem child must reject");
    assert!(
        error.contains("Undefined variable: missing_fastmem"),
        "{error}"
    );

    compiler
        .compile_normal(request(vec![integer(9, 250)], "direct-fastmem-reuse.hako"))
        .expect("fresh FastMem candidate after child failure");
}
