//! POST-FAILURE0-NATURAL-P0: disconnected natural postprocess failures.
//!
//! These fixtures use the existing optimizer policy and Static Table contract
//! validator.  They do not add a production fault authority or public caller.

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::module_postprocess::{
    ModulePostprocessErrorV1, ModulePostprocessOwnerV1, PostprocessFailureStageV1,
};
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::definitions::Callee;
use crate::mir::function::StaticDataPlan;
use crate::mir::{EffectMask, MirInstruction, ValueId};
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard, OnceLock};

const POLICY_KEYS: [&str; 7] = [
    "NYASH_OPT_DIAG_FAIL",
    "NYASH_OPT_DIAG_FORBID_LEGACY",
    "NYASH_MIR_DISABLE_OPT",
    "HAKO_MIR_DISABLE_OPT",
    "HAKO_JOINIR_STRICT",
    "NYASH_JOINIR_STRICT",
    "HAKO_JOINIR_PLANNER_REQUIRED",
];

fn source() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "postprocess_failure_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    })
    .expect("POST-FAILURE0 source must resolve")
}

fn finalized_trivial<'a>(
    compiler: &'a mut MirCompiler,
    source: &'a VerifiedResolvedSourceUnitV1,
) -> super::canonical_finalization::FinalizedModuleInvocationV1<'a> {
    let plan = match CanonicalLoweringPreflightV1::verify(source).unwrap() {
        CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
            ExactCanonicalPreflightPlanV1::BindingSsaTrivial(plan)
        }
        _ => panic!("POST-FAILURE0 fixture must remain trivial SSA"),
    };
    let package = compiler.bind_canonical_source(plan).unwrap();
    let finalization_input = compiler
        .begin_canonical_invocation(
            package,
            Some("postprocess-failure.hako"),
            "postprocess_failure".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain()
        .prepare_finalization()
        .unwrap();
    super::canonical_finalization::CanonicalModuleFinalizerV1::finalize(
        finalization_input,
    )
    .unwrap()
}

struct ExistingOptimizerPolicyScopeV1 {
    saved: Vec<(&'static str, Option<OsString>)>,
    _lock: MutexGuard<'static, ()>,
}

impl ExistingOptimizerPolicyScopeV1 {
    fn acquire() -> Self {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = LOCK.get_or_init(|| Mutex::new(()));
        let guard = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let saved = POLICY_KEYS
            .into_iter()
            .map(|key| (key, std::env::var_os(key)))
            .collect();
        std::env::set_var("NYASH_OPT_DIAG_FAIL", "1");
        std::env::remove_var("NYASH_OPT_DIAG_FORBID_LEGACY");
        std::env::remove_var("NYASH_MIR_DISABLE_OPT");
        std::env::remove_var("HAKO_MIR_DISABLE_OPT");
        std::env::remove_var("HAKO_JOINIR_STRICT");
        std::env::remove_var("NYASH_JOINIR_STRICT");
        std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
        Self {
            saved,
            _lock: guard,
        }
    }
}

impl Drop for ExistingOptimizerPolicyScopeV1 {
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

#[test]
fn optimizer_natural_failure_retains_discard_only_owner() {
    let _policy = ExistingOptimizerPolicyScopeV1::acquire();
    let source = source();
    let mut compiler = MirCompiler::new();
    let mut finalized = finalized_trivial(&mut compiler, &source);
    let super::canonical_finalization::CanonicalFinalizationInputV1::Single(input) =
        &mut finalized.input
    else {
        panic!("POST-FAILURE0 optimizer fixture changed route shape")
    };
    let function = input
        .physical
        .module
        .functions
        .values_mut()
        .next()
        .expect("optimizer fixture function");
    let receiver = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .find_map(MirInstruction::dst_value)
        .expect("optimizer fixture defined receiver");
    function
        .get_block_mut(function.entry_block)
        .expect("optimizer fixture entry block")
        .add_instruction_before_terminator(MirInstruction::Call {
            dst: None,
            func: receiver,
            callee: Some(Callee::Method {
                box_name: "IntegerBox".into(),
                method: "is".into(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: Vec::new(),
            effects: EffectMask::IO,
        });

    let mut verifier = super::super::verification::MirVerifier::new();
    let rejected = ModulePostprocessOwnerV1::new(&mut verifier, true)
        .run(finalized)
        .expect_err("optimizer diagnostics must reject");
    assert_eq!(rejected.stage(), PostprocessFailureStageV1::Optimizer);
    assert!(matches!(
        rejected.error(),
        ModulePostprocessErrorV1::OptimizerDiagnostics { count } if *count >= 1
    ));
    rejected.discard();
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn orphan_static_plan_natural_failure_retains_discard_only_owner() {
    let source = source();
    let mut compiler = MirCompiler::with_options(false);
    let mut finalized = finalized_trivial(&mut compiler, &source);
    let super::canonical_finalization::CanonicalFinalizationInputV1::Single(input) =
        &mut finalized.input
    else {
        panic!("POST-FAILURE0 contract fixture changed route shape")
    };
    input.physical.module.metadata.static_data_plans.push(StaticDataPlan {
        source_name: "ORPHAN".into(),
        symbol: ".hako.static.ORPHAN".into(),
        element: "u16".into(),
        align: 2,
        linkage: "private".into(),
        unnamed_addr: true,
        values: vec![1],
    });

    let mut verifier = super::super::verification::MirVerifier::new();
    let rejected = ModulePostprocessOwnerV1::new(&mut verifier, false)
        .run(finalized)
        .expect_err("orphan static plan must reject");
    assert_eq!(rejected.stage(), PostprocessFailureStageV1::ContractRefresh);
    assert!(matches!(
        rejected.error(),
        ModulePostprocessErrorV1::ContractRefresh(error)
            if error.contains("[type/static_table_contract_spec_missing]")
    ));
    rejected.discard();
    assert!(compiler.builder.current_module.is_none());
}
