//! Source-backed physical function-effect projection for the installed S6C row.
//!
//! The resolver/CoreMethod contracts remain the semantic effect authority.
//! This product only projects their already-verified read-only boundary into
//! the MIR function-header effect mask; it owns no MIR instruction or route.

use crate::mir::core_method_result_kind::CoreMethodEffectV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::EffectMask;

use crate::mir::loop_recipe_contract::VerifiedS6CPrephysicalIngressV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum S6CPhysicalFunctionEffectsRejectV1 {
    OwnerMismatch,
    ExternalCallCount,
    NonReadExternalCall,
}

/// One source-backed boundary projection for `FunctionSignature.effects`.
///
/// `EffectMask` is deliberately private to this projection.  The source
/// CoreMethod rows and S6C Facts/Ingress remain the only semantic authorities.
#[derive(Debug)]
pub(crate) struct VerifiedS6CPhysicalFunctionEffectsV1 {
    owner: FunctionOwnerIdV1,
    effect_mask: EffectMask,
    external_call_count: u8,
}

impl VerifiedS6CPhysicalFunctionEffectsV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn effect_mask(&self) -> EffectMask {
        self.effect_mask
    }

    pub(crate) const fn external_call_count(&self) -> u8 {
        self.external_call_count
    }
}

pub(super) fn issue_s6c_physical_function_effects_v1(
    ingress: &VerifiedS6CPrephysicalIngressV2,
    owner: FunctionOwnerIdV1,
) -> Result<VerifiedS6CPhysicalFunctionEffectsV1, S6CPhysicalFunctionEffectsRejectV1> {
    let source_owner = ingress
        .with_ingress(|view| {
            Ok::<_, crate::mir::loop_recipe_contract::S6CPrephysicalIngressRejectV2>(
                view.source_owner(),
            )
        })
        .map_err(|_| S6CPhysicalFunctionEffectsRejectV1::ExternalCallCount)?;
    if source_owner != owner {
        return Err(S6CPhysicalFunctionEffectsRejectV1::OwnerMismatch);
    }
    let effects = ingress
        .with_ingress(|view| {
            Ok::<_, crate::mir::loop_recipe_contract::S6CPrephysicalIngressRejectV2>(
                view.external_call_effects(),
            )
        })
        .map_err(|_| S6CPhysicalFunctionEffectsRejectV1::ExternalCallCount)?;
    if effects.len() != 2 {
        return Err(S6CPhysicalFunctionEffectsRejectV1::ExternalCallCount);
    }
    if effects
        .iter()
        .any(|effect| *effect != CoreMethodEffectV1::PureRead)
    {
        return Err(S6CPhysicalFunctionEffectsRejectV1::NonReadExternalCall);
    }
    Ok(VerifiedS6CPhysicalFunctionEffectsV1 {
        owner,
        effect_mask: EffectMask::READ,
        external_call_count: effects.len() as u8,
    })
}
