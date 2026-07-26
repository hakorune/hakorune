use crate::ast::ASTNode;

use super::{
    NormalMainFunctionPreflightV1, NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1,
    SealedNormalScalarRootV1, SealedNormalSourcePlanV1, VerifiedNormalMainThunkPlanV1,
};

pub(crate) fn with_main_thunk_for_test<R>(
    program: ASTNode,
    inspect: impl FnOnce(VerifiedNormalMainThunkPlanV1<'_>) -> R,
) -> R {
    let input = PreparedNormalSourcePlanInputV1::new(program, "main-thunk-shared-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    let source = main.prepare_function_source().expect("exact Main source");
    let resolved = source
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolution");
    let main = NormalMainFunctionPreflightV1::seal(&resolved).expect("Main F1 plan");
    inspect(VerifiedNormalMainThunkPlanV1::seal(main).expect("Main thunk plan"))
}
