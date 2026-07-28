//! Detached Main0 receipts paired with the retained general-module owner.
//!
//! The bridge borrows the exact `Main.main/0` only while the existing Main0
//! semantic kernels run. The final product stores no AST or lowering input and
//! keeps the complete instance-plan owner intact.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::VerifiedSemanticOwnerForestV1;
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

use super::instance_integer_return_plan::VerifiedNormalInstanceIntegerReturnPlanSetV1;
use super::main_function_plan::{verify_normal_main0_input_v1, NormalMainFunctionPlanErrorV1};
use super::main_resolved_source::{
    resolve_normal_main_loan_v1, NormalMainResolvedSourceErrorV1, VerifiedNormalMainRoleV1,
};
use super::main_source::NormalMainFunctionSourceErrorV1;

#[derive(Debug)]
pub(crate) struct VerifiedNormalMain0BridgePlanV1 {
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
}

impl VerifiedNormalMain0BridgePlanV1 {
    pub(crate) const fn role(&self) -> VerifiedNormalMainRoleV1 {
        self.role
    }

    pub(crate) const fn completion(&self) -> &VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(crate) const fn if_control(&self) -> &VerifiedResolvedFunctionIfControlV1 {
        &self.if_control
    }

    pub(crate) const fn profile(&self) -> &VerifiedTrivialCanonicalOwnerV1 {
        &self.profile
    }

    pub(crate) const fn block_expr_count(&self) -> usize {
        self.block_expr_count
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(crate) const fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalModuleFunctionPlanSetV1 {
    instance: VerifiedNormalInstanceIntegerReturnPlanSetV1,
    main: VerifiedNormalMain0BridgePlanV1,
}

impl VerifiedNormalModuleFunctionPlanSetV1 {
    pub(crate) const fn instance(&self) -> &VerifiedNormalInstanceIntegerReturnPlanSetV1 {
        &self.instance
    }

    pub(crate) const fn main(&self) -> &VerifiedNormalMain0BridgePlanV1 {
        &self.main
    }

    pub(crate) fn source_identity(&self) -> &str {
        self.instance.source_identity()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalMain0BridgeStageV1 {
    SourceLoan,
    Resolution,
    FunctionInput,
    FunctionPlan,
    Pairing,
}

#[derive(Debug)]
pub(crate) enum NormalMain0BridgeErrorV1 {
    SourceLoan(NormalMainFunctionSourceErrorV1),
    Resolution(NormalMainResolvedSourceErrorV1),
    FunctionInput(CanonicalLoweringErrorV1),
    FunctionPlan(NormalMainFunctionPlanErrorV1),
    Pairing { reason: &'static str },
}

impl NormalMain0BridgeErrorV1 {
    pub(crate) const fn stage(&self) -> NormalMain0BridgeStageV1 {
        match self {
            Self::SourceLoan(_) => NormalMain0BridgeStageV1::SourceLoan,
            Self::Resolution(_) => NormalMain0BridgeStageV1::Resolution,
            Self::FunctionInput(_) => NormalMain0BridgeStageV1::FunctionInput,
            Self::FunctionPlan(_) => NormalMain0BridgeStageV1::FunctionPlan,
            Self::Pairing { .. } => NormalMain0BridgeStageV1::Pairing,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMain0BridgeV1 {
    owner: VerifiedNormalInstanceIntegerReturnPlanSetV1,
    error: NormalMain0BridgeErrorV1,
}

impl RejectedNormalMain0BridgeV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.owner.source_identity()
    }

    pub(crate) fn instance_plan_count(&self) -> usize {
        self.owner.len()
    }

    pub(crate) const fn stage(&self) -> NormalMain0BridgeStageV1 {
        self.error.stage()
    }

    pub(crate) const fn error(&self) -> &NormalMain0BridgeErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalInstanceIntegerReturnPlanSetV1 {
    pub(crate) fn seal_main0_bridge(
        self,
    ) -> Result<VerifiedNormalModuleFunctionPlanSetV1, RejectedNormalMain0BridgeV1> {
        let main = match prepare_main0_bridge(&self) {
            Ok(main) => main,
            Err(error) => return Err(RejectedNormalMain0BridgeV1 { owner: self, error }),
        };
        Ok(VerifiedNormalModuleFunctionPlanSetV1 {
            instance: self,
            main,
        })
    }
}

fn prepare_main0_bridge(
    owner: &VerifiedNormalInstanceIntegerReturnPlanSetV1,
) -> Result<VerifiedNormalMain0BridgePlanV1, NormalMain0BridgeErrorV1> {
    let source = owner
        .borrow_exact_main_function()
        .map_err(NormalMain0BridgeErrorV1::SourceLoan)?;
    let (forest, projection, role) =
        resolve_normal_main_loan_v1(&source).map_err(NormalMain0BridgeErrorV1::Resolution)?;

    let (root_owner, if_control, completion, profile, block_expr_count) = {
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            source.function(),
            &forest,
            &projection,
        )
        .map_err(NormalMain0BridgeErrorV1::FunctionInput)?;
        let lowering = verify_normal_main0_input_v1(input, role)
            .map_err(NormalMain0BridgeErrorV1::FunctionPlan)?;
        let (input, if_control, completion, profile, block_expr_count) = lowering.into_parts();
        (
            input.owner(),
            if_control,
            completion,
            profile,
            block_expr_count,
        )
    };

    let [forest_root] = forest.roots() else {
        return Err(NormalMain0BridgeErrorV1::Pairing {
            reason: "main_forest_root_count",
        });
    };
    if *forest_root != root_owner
        || if_control.owner() != root_owner
        || completion.owner() != root_owner
        || profile.owner() != root_owner
    {
        return Err(NormalMain0BridgeErrorV1::Pairing {
            reason: "main_owned_receipt_owner_mismatch",
        });
    }

    Ok(VerifiedNormalMain0BridgePlanV1 {
        forest,
        projection,
        role,
        if_control,
        completion,
        profile,
        block_expr_count,
    })
}
