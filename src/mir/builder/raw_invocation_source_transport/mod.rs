//! Function-relative source transport for the live raw invocation port.
//!
//! The selected invocation route owns one shrinking dual-state carrier.  A
//! located row keeps the already-issued callable root receipt plus the exact
//! `SourcePathV1` node.  An unlocated row names one finite migration portal;
//! it is an execute-once compatibility state, never a retry route.

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1, SourceBodyKindV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::ValueId;

use super::callable_declaration_catalog::SelectedTopLevelFunctionKeyV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1;
use super::normal_script_pre_effect_source_observation::CanonicalScriptCPreparedLoweringSourceV1;
use super::normal_script_semantic_lowering_state::ScriptSemanticLoweringState;
use super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
use super::raw_invocation_source_item_site::body_item_site;
use super::raw_invocation_source_statement_classification::{
    is_bare_function_call_statement, is_located_control_or_diagnostic_terminal,
    is_located_lambda_statement, is_located_scalar_statement,
    is_located_zero_child_runtime_completion, reason_for_non_box_statement,
};
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    lower_raw_expression_with_recursion_guard_v1, RawInvocationChildPortV1,
    RecursiveChildLoweringPortV1,
};
use super::recursive_child_lowering_port::ScriptDirectStaticClaimIngressV1;
use super::{CanonicalSameModuleCallableKeyV1, RawSourceLocatorV1};

mod child_lowering_impl;
mod context;
mod lineage;
mod script_scope;
mod transport_port;

pub(in crate::mir::builder) use context::RawInvocationSourceContextV1;
pub(in crate::mir::builder) use lineage::{
    LocatedRawNodeV1, RawInvocationRootLineageV1, RawInvocationSourceTransportV1,
    RawUnlocatedPortalV1,
};
pub(in crate::mir::builder) use transport_port::RawSourceTransportPortV1;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod lineage_witness_tests;
