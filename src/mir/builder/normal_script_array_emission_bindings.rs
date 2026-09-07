//! Physical bindings beside the source continuation; source classification stays there.
use super::*;
use crate::mir::builder::collection_literals::array_emission::{
    last_instruction, ArrayLiteralEmission,
};
use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
use crate::mir::builder::stmts::LocalInitializerObservationV1;
use crate::mir::resolved_semantics::ResolvedInitializerRelationV1;
use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirFunction, MirInstruction};

#[derive(Debug, Default)]
pub(super) struct ArrayEmissionBindings {
    rows: BTreeMap<BindingRefV1, BoundArray>,
    root_progress: RootBindingProgress,
    terminal: Option<RootReturnEmission>,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RootBindingProgress {
    #[default]
    Unbound,
    Bound,
    Finished,
}

/// Moved source and physical products, not a new semantic issuance.
#[derive(Debug)]
pub(crate) struct FinalizedScriptArrayV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    entry: BasicBlockId,
    source: super::super::normal_script_source_continuation::ArraySourceLifecycleRows,
    emissions: ArrayEmissionBindings,
    frame: super::super::function_fault_frame::FunctionFaultFrameV1,
}

impl FinalizedScriptArrayV1 {
    pub(crate) fn validate_root_binding(&self, root: &MirFunction) -> Result<(), String> {
        if root.entry_block != self.entry {
            return Err(fault("artifact-entry-drift"));
        }
        // The exact owner travels with the same moved source, never from MIR.
        let bindings = self.source.bindings()?;
        if bindings.is_empty() || bindings.iter().any(|binding| binding.owner() != self.owner) {
            return Err(fault("artifact-source-owner"));
        }
        self.frame.validate(root)?;
        self.emissions.check_bindings(&bindings)?;
        self.emissions.validate(root, true, &bindings)
    }

    #[cfg(test)]
    pub(crate) fn acquisition_count(&self) -> usize {
        self.emissions.rows.len()
    }
}

#[derive(Debug)]
struct BoundArray {
    literal: ArrayLiteralEmission,
    local: ValueId,
    local_slot: crate::mir::LocalSlotId,
    local_site: (BasicBlockId, usize),
}

#[derive(Debug)]
struct RootReturnEmission {
    recipe: super::super::normal_script_source_continuation::ArrayReturnRecipeV1,
    block: BasicBlockId,
    instruction: MirInstruction,
    definition: Option<((BasicBlockId, usize), MirInstruction)>,
    releases: Vec<ValueId>,
}

impl ScriptSemanticLoweringState {
    pub(in crate::mir::builder) fn into_array_artifact(
        self,
        entry: BasicBlockId,
    ) -> Result<Option<FinalizedScriptArrayV1>, String> {
        if self.array_emissions.root_progress != RootBindingProgress::Finished {
            return Err(fault("artifact-before-finishing"));
        }
        let bindings = self.continuation.array_bindings()?;
        self.array_emissions.check_bindings(&bindings)?;
        if bindings.is_empty() {
            return Ok(None);
        }
        let (owner, source) = self.continuation.into_array_parts();
        if bindings.iter().any(|binding| binding.owner() != owner) {
            return Err(fault("artifact-source-owner"));
        }
        Ok(Some(FinalizedScriptArrayV1 {
            owner,
            entry,
            source,
            emissions: self.array_emissions,
            frame: self.array_frame,
        }))
    }

    pub(in crate::mir::builder) fn record_array_local_emission(
        &mut self,
        builder: &MirBuilder,
        relation: &ResolvedInitializerRelationV1,
        local: ValueId,
        observations: Vec<LocalInitializerObservationV1>,
    ) -> Result<(), String> {
        if !observations
            .iter()
            .any(|observation| observation.array.is_some())
        {
            // Required selected Recipe consumption is checked by Local descent.
            return Ok(());
        }
        let [observation]: [LocalInitializerObservationV1; 1] = observations
            .try_into()
            .map_err(|_| fault("initializer-count"))?;
        let initializer = relation
            .initializer_site()
            .ok_or_else(|| fault("initializer-missing"))?;
        if observation.ordinal != 0
            || !matches!(&observation.source,
            PreparedRawChildSourceV1::Exact(context) if context.site() == Some(initializer.node()))
        {
            return Err(fault("initializer-source"));
        }
        let literal = observation
            .array
            .ok_or_else(|| fault("literal-emission-missing"))?;
        if literal.recipe.relation() != relation
            || observation.value != literal.allocation
            || literal.recipe.elements().len() != literal.elements.len()
        {
            return Err(fault("initializer-value-or-children"));
        }
        let (local_site, instruction) = last_instruction(builder)?;
        if !matches!(instruction, MirInstruction::Copy { dst, src }
            if *dst == local && *src == literal.allocation)
        {
            return Err(fault("local-emission"));
        }
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("root-missing"))?;
        let carriers: Vec<_> = function
            .metadata
            .typed_array_contract_sources
            .iter()
            .filter(|source| source.contract_id == literal.claim)
            .collect();
        let [carrier] = carriers.as_slice() else {
            return Err(fault("claim-source-carrier"));
        };
        let crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(local_slot) =
            &carrier.source_identity
        else {
            return Err(fault("claim-local-slot"));
        };
        if self.array_emissions.rows.contains_key(&relation.binding()) {
            return Err(fault("duplicate-local"));
        }
        self.array_emissions.rows.insert(
            relation.binding(),
            BoundArray {
                literal,
                local,
                local_slot: *local_slot,
                local_site,
            },
        );
        Ok(())
    }

    pub(in crate::mir::builder) fn emit_array_root_return(
        &mut self,
        builder: &mut MirBuilder,
        recipe: super::super::normal_script_source_continuation::ArrayReturnRecipeV1,
    ) -> Result<ValueId, String> {
        use super::super::normal_script_source_continuation::{ArrayReleaseRoleV1, RootResult};
        use crate::mir::builder::control_flow::cleanup::{
            ensure_cleanup_exit_allowed_v1, CleanupExitKindV1,
        };
        if self.array_emissions.terminal.is_some() {
            return Err(fault("return-source-or-duplicate"));
        }
        ensure_cleanup_exit_allowed_v1(&builder.function_state, CleanupExitKindV1::Return)?;
        let releases = recipe
            .releases()
            .iter()
            .map(|role| {
                let ArrayReleaseRoleV1::Home(binding) = role else {
                    return Err(fault("return-release-role"));
                };
                self.value(*binding)
                    .ok_or_else(|| fault("return-home-value"))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let value = match recipe.result() {
            RootResult::Unit => crate::mir::builder::emission::constant::emit_void(builder)?,
            RootResult::Integer { value, .. } => {
                crate::mir::builder::emission::constant::emit_integer(builder, *value)?
            }
        };
        let (position, definition) = last_instruction(builder)?;
        let definition = if matches!(recipe.result(), RootResult::Unit) {
            None
        } else {
            Some((position, definition.clone()))
        };
        for value in &releases {
            builder.emit_instruction(MirInstruction::ArrayResidenceRelease { value: *value })?;
        }
        let instruction = MirInstruction::Return {
            value: if matches!(recipe.result(), RootResult::Unit) {
                None
            } else {
                Some(value)
            },
        };
        builder.emit_instruction(instruction.clone())?;
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| fault("return-block"))?;
        self.array_emissions.terminal = Some(RootReturnEmission {
            recipe,
            block,
            instruction,
            definition,
            releases,
        });
        Ok(value)
    }

    pub(in crate::mir::builder) fn bind_array_root(
        &mut self,
        root: &MirFunction,
    ) -> Result<(), String> {
        self.finish_source_claims()?;
        let bindings = self.continuation.array_bindings()?;
        self.array_emissions.check_bindings(&bindings)?;
        if self.array_emissions.root_progress != RootBindingProgress::Unbound {
            return Err(fault("duplicate-root-bind"));
        }
        self.array_frame.validate(root)?;
        self.array_emissions.validate(root, false, &bindings)?;
        self.array_emissions.root_progress = RootBindingProgress::Bound;
        Ok(())
    }

    pub(in crate::mir::builder) fn validate_finished_array_root(
        &mut self,
        root: &MirFunction,
    ) -> Result<(), String> {
        self.finish_source_claims()?;
        let bindings = self.continuation.array_bindings()?;
        self.array_emissions.check_bindings(&bindings)?;
        if self.array_emissions.root_progress == RootBindingProgress::Unbound {
            return Err(fault("root-unbound"));
        }
        self.array_frame.validate(root)?;
        self.array_emissions.validate(root, true, &bindings)?;
        self.array_emissions.root_progress = RootBindingProgress::Finished;
        Ok(())
    }
}

#[path = "normal_script_array_control_validation.rs"]
mod validation;
fn fault(reason: &str) -> String {
    format!("[freeze:contract][script-array/emission/{reason}]")
}
