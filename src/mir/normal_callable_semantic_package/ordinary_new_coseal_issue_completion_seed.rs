//! Plain Completion seed issuance inside the original co-seal source loan.
use super::super::super::physical_header::CallablePhysicalHeaderIssueV1;
use super::*;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_control_flow::{
    FunctionCompletionVerificationErrorV1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::home_new_prefix::issue_terminal_integer_literal_return_from_completion_v1;

pub(super) fn issue_plain_completion(
    input: ResolvedFunctionLoweringInputV1<'_>,
    declaration: VerifiedResolvedCallableSemanticDeclarationRefV1<'_>,
    selected: &VerifiedSelectedCallableBatchMapV1,
    seed_completion: bool,
    app_main_integer_result: bool,
    seeds: &mut super::super::super::result_contract::VerifiedCallableResultContractBuilderV1,
    root_completion: &mut Option<
        Result<Rc<VerifiedFunctionCompletionV1>, FunctionCompletionVerificationErrorV1>,
    >,
    root_terminal_relation: &mut BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    top_level_input: &mut Option<
        super::super::super::result_contract::VerifiedTopLevelScalarInputV1,
    >,
) -> Result<(), OrdinaryNewCoSealIssueV1> {
    let batch_slot = declaration.batch_slot();
    if seed_completion || app_main_integer_result {
        let completion = Rc::new(verify_function_completion_v1(input).map_err(|issue| {
            OrdinaryNewCoSealIssueV1::CompletionSeed(CallablePhysicalHeaderIssueV1::Completion {
                _batch_slot: batch_slot,
                _issue: issue,
            })
        })?);
        if app_main_integer_result {
            if let Some(relation) =
                issue_terminal_integer_literal_return_from_completion_v1(input, completion.as_ref())
                    .map_err(OrdinaryNewCoSealIssueV1::RootTerminalSource)?
            {
                *root_completion = Some(Ok(Rc::clone(&completion)));
                root_terminal_relation.insert(
                    relation.return_site().clone(),
                    TerminalRelationV1::IntegerLiteral(relation),
                );
            }
        }
        if seed_completion {
            seeds
                .push_completion_with_top_level_input(
                    declaration,
                    selected,
                    Rc::clone(&completion),
                    BTreeMap::new(),
                    top_level_input.take(),
                )
                .map_err(OrdinaryNewCoSealIssueV1::CompletionSeed)?;
        }
    }
    Ok(())
}
