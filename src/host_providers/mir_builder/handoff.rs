use super::{
    failfast_error, module_to_mir_json, refresh_bridge_semantic_metadata, with_phase0_mir_json_env,
    Stage1UserBoxDecls, FAILFAST_TAG,
};
use crate::mir::types::ConstValue;
use crate::mir::{Callee, ConstructionTarget, MirInstruction, MirModule};
use crate::runner::json_v0_bridge::Stage1ProgramJsonCallAnchorReceiptV1;
use crate::stage1::program_json_v0::{
    emit_program_json_v0_source_artifact_for_strict_authority_source,
    Stage1ProgramJsonCrosswalkEntryV1, Stage1ProgramJsonCrosswalkV1,
    Stage1ProgramJsonSourceArtifactV1, Stage1StringBoxArgumentKindV1, Stage1StringBoxReceiverV1,
    Stage1StringBoxSourceProductV1, Stage1StringBoxSourceRelationV1,
};
use std::collections::BTreeSet;

pub(super) struct Stage1ProgramJsonInput<'a> {
    program_json: &'a str,
}

pub(super) struct Stage1ProgramJsonValue {
    program_value: serde_json::Value,
}

pub(super) struct Stage1ProgramJsonModuleHandoff {
    module: crate::mir::MirModule,
    user_box_decls: Stage1UserBoxDecls,
}

pub(super) struct Stage1FinalizedMirModule {
    module: crate::mir::MirModule,
}

impl Stage1ProgramJsonModuleHandoff {
    pub(super) fn new(module: crate::mir::MirModule, user_box_decls: Stage1UserBoxDecls) -> Self {
        Self {
            module,
            user_box_decls,
        }
    }

    pub(super) fn parse(program_json: &str) -> Result<Self, String> {
        Stage1ProgramJsonInput::new(program_json).into_module_handoff()
    }

    pub(super) fn from_source_artifact(
        program_json: String,
        source_product: Stage1StringBoxSourceProductV1,
        crosswalk: Stage1ProgramJsonCrosswalkV1,
    ) -> Result<Self, String> {
        let input = Stage1ProgramJsonInput::new(&program_json);
        let (module, receipts) = input.parse_source_module()?;
        admit_stringbox_source(&module, &source_product, &crosswalk, &receipts)?;
        let program_value = input.parse_value()?;
        Ok(program_value.into_module_handoff(module))
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        self.into_finalized_module()?.emit_guarded_mir_json()
    }

    pub(super) fn into_finalized_module(self) -> Result<Stage1FinalizedMirModule, String> {
        let mut module = self.module;
        let (user_box_decls, user_box_field_decls) = self.user_box_decls.into_metadata_maps();
        module.metadata.user_box_decls = user_box_decls;
        module.metadata.user_box_field_decls = user_box_field_decls;
        refresh_bridge_semantic_metadata(&mut module)?;
        Ok(Stage1FinalizedMirModule { module })
    }
}

pub(super) struct SourceProgramJsonOutputHandoff {
    program_json: String,
    source_product: Stage1StringBoxSourceProductV1,
    crosswalk: Stage1ProgramJsonCrosswalkV1,
}

pub(super) struct SourceProgramJsonAuthority;

impl<'a> Stage1ProgramJsonInput<'a> {
    pub(super) fn new(program_json: &'a str) -> Self {
        Self { program_json }
    }

    pub(super) fn into_module_handoff(self) -> Result<Stage1ProgramJsonModuleHandoff, String> {
        let module = self.parse_module()?;
        let program_value = self.parse_value()?;
        Ok(program_value.into_module_handoff(module))
    }

    pub(super) fn parse_value(&self) -> Result<Stage1ProgramJsonValue, String> {
        Stage1ProgramJsonValue::parse(self.program_json)
    }

    fn parse_module(&self) -> Result<crate::mir::MirModule, String> {
        crate::runner::json_v0_bridge::parse_json_v0_to_module(self.program_json)
            .map_err(failfast_error)
    }

    fn parse_source_module(
        &self,
    ) -> Result<
        (
            crate::mir::MirModule,
            Vec<Stage1ProgramJsonCallAnchorReceiptV1>,
        ),
        String,
    > {
        crate::runner::json_v0_bridge::parse_json_v0_to_module_with_source_anchors(
            self.program_json,
        )
        .map_err(failfast_error)
    }
}

impl Stage1ProgramJsonValue {
    fn parse(program_json: &str) -> Result<Self, String> {
        serde_json::from_str(program_json)
            .map(|program_value| Self { program_value })
            .map_err(|error| format!("program json parse error: {}", error))
    }

    pub(super) fn resolve_user_box_decls(&self) -> Stage1UserBoxDecls {
        Stage1UserBoxDecls::from_program_value(&self.program_value)
    }

    fn into_module_handoff(self, module: crate::mir::MirModule) -> Stage1ProgramJsonModuleHandoff {
        let user_box_decls = self.resolve_user_box_decls();
        Stage1ProgramJsonModuleHandoff::new(module, user_box_decls)
    }
}

impl Stage1FinalizedMirModule {
    pub(super) fn emit_mir_json(self) -> Result<String, String> {
        module_to_mir_json(&self.module)
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        with_phase0_mir_json_env(|| {
            let mir_json = self.emit_mir_json()?;
            super::normalize_program_json_bridge_backend_shape(&mir_json)
        })
    }
}

impl SourceProgramJsonAuthority {
    pub(super) fn for_source(source_text: &str) -> Result<SourceProgramJsonOutputHandoff, String> {
        let artifact =
            emit_program_json_v0_source_artifact_for_strict_authority_source(source_text)
                .map_err(|error| format!("{FAILFAST_TAG} {}", error))?;
        Ok(Self::from_artifact(artifact))
    }

    fn from_artifact(
        artifact: Stage1ProgramJsonSourceArtifactV1,
    ) -> SourceProgramJsonOutputHandoff {
        SourceProgramJsonOutputHandoff {
            program_json: artifact.program_json,
            source_product: artifact.product,
            crosswalk: artifact.crosswalk,
        }
    }
}

impl SourceProgramJsonOutputHandoff {
    pub(super) fn emit_guarded_program_and_mir_json(self) -> Result<(String, String), String> {
        let program_json = self.program_json.clone();
        let mir_json = Stage1ProgramJsonModuleHandoff::from_source_artifact(
            self.program_json,
            self.source_product,
            self.crosswalk,
        )?
        .emit_guarded_mir_json()?;
        Ok((program_json, mir_json))
    }

    pub(super) fn emit_guarded_mir_json(self) -> Result<String, String> {
        self.emit_guarded_program_and_mir_json()
            .map(|(_, mir_json)| mir_json)
    }

    #[cfg(test)]
    pub(super) fn emit_plain_program_and_mir_json(self) -> Result<(String, String), String> {
        let program_json = self.program_json.clone();
        let mir_json = Stage1ProgramJsonModuleHandoff::from_source_artifact(
            self.program_json,
            self.source_product,
            self.crosswalk,
        )?
        .into_finalized_module()?
        .emit_mir_json()?;
        Ok((program_json, mir_json))
    }
}

fn admit_stringbox_source(
    module: &MirModule,
    product: &Stage1StringBoxSourceProductV1,
    crosswalk: &Stage1ProgramJsonCrosswalkV1,
    receipts: &[Stage1ProgramJsonCallAnchorReceiptV1],
) -> Result<(), String> {
    if product.relations.len() != crosswalk.entries.len()
        || product.relations.len() != receipts.len()
    {
        return Err(format!(
            "{} stringbox source artifact cardinality mismatch: product={} crosswalk={} receipts={}",
            FAILFAST_TAG,
            product.relations.len(),
            crosswalk.entries.len(),
            receipts.len()
        ));
    }

    let mut consumed = BTreeSet::new();
    for ((relation, entry), receipt) in product
        .relations
        .iter()
        .zip(&crosswalk.entries)
        .zip(receipts)
    {
        validate_crosswalk_relation(relation, entry)?;
        if relation.anchor != receipt.anchor {
            return Err(format!(
                "{} stringbox source anchor mismatch: {}",
                FAILFAST_TAG, relation.path
            ));
        }
        if !consumed.insert(receipt.anchor) {
            return Err(format!(
                "{} stringbox source anchor consumed twice: {}",
                FAILFAST_TAG, relation.path
            ));
        }
        validate_anchor_call(module, entry, receipt)?;
    }
    Ok(())
}

fn validate_crosswalk_relation(
    relation: &Stage1StringBoxSourceRelationV1,
    entry: &Stage1ProgramJsonCrosswalkEntryV1,
) -> Result<(), String> {
    if relation.owner != entry.owner
        || relation.path != entry.path
        || relation.anchor != entry.anchor
        || relation.span != entry.span
        || relation.receiver != entry.receiver
        || relation.selector != entry.expected_method
        || relation.arity != entry.expected_arity
    {
        return Err(format!(
            "{} stringbox source/crosswalk relation mismatch: {}",
            FAILFAST_TAG, relation.path
        ));
    }
    if relation.contract.selector != relation.selector
        || relation.contract.arity != relation.arity
        || relation.contract.canonical_selector != "length"
        || relation.contract.result_kind != "I64Value"
        || relation.contract.effect_kind != "pure_read"
        || relation.constructor_arg_count
            != usize::from(matches!(
                relation.receiver,
                Stage1StringBoxReceiverV1::NewStringBox
            ))
        || relation.constructor_argument_kinds
            != match relation.receiver {
                Stage1StringBoxReceiverV1::DirectStringLiteral => Vec::new(),
                Stage1StringBoxReceiverV1::NewStringBox => {
                    vec![Stage1StringBoxArgumentKindV1::DirectStringLiteral]
                }
            }
        || relation.field_initializer_count != 0
        || !relation.type_arguments.is_empty()
    {
        return Err(format!(
            "{} stringbox source contract mismatch: {}",
            FAILFAST_TAG, relation.path
        ));
    }
    Ok(())
}

fn validate_anchor_call(
    module: &MirModule,
    entry: &Stage1ProgramJsonCrosswalkEntryV1,
    receipt: &Stage1ProgramJsonCallAnchorReceiptV1,
) -> Result<(), String> {
    let function = module
        .functions
        .get(&receipt.function_name)
        .ok_or_else(|| {
            format!(
                "{} stringbox source anchor function missing: {}",
                FAILFAST_TAG, receipt.function_name
            )
        })?;
    let block = function.blocks.get(&receipt.block).ok_or_else(|| {
        format!(
            "{} stringbox source anchor block missing: {} {}",
            FAILFAST_TAG, receipt.function_name, receipt.block
        )
    })?;
    let instruction = block
        .instructions
        .get(receipt.instruction_index)
        .ok_or_else(|| {
            format!(
                "{} stringbox source anchor instruction missing: {} {}:{}",
                FAILFAST_TAG, receipt.function_name, receipt.block, receipt.instruction_index
            )
        })?;
    let MirInstruction::Call(call) = instruction else {
        return Err(format!(
            "{} stringbox source anchor is not a Call: {}",
            FAILFAST_TAG, entry.path
        ));
    };
    if call.dst != receipt.dst {
        return Err(format!(
            "{} stringbox source anchor destination mismatch: {}",
            FAILFAST_TAG, entry.path
        ));
    }
    let Callee::Method {
        box_name,
        method,
        receiver: Some(receiver),
        ..
    } = &call.callee
    else {
        return Err(format!(
            "{} stringbox source anchor callee mismatch: {}",
            FAILFAST_TAG, entry.path
        ));
    };
    if box_name != entry.expected_box_name
        || method != &entry.expected_method
        || call.args.len() != entry.expected_arity
        || !receiver_matches(function, *receiver, &entry.receiver)
    {
        return Err(format!(
            "{} stringbox source anchor shape mismatch: {}",
            FAILFAST_TAG, entry.path
        ));
    }
    Ok(())
}

fn receiver_matches(
    function: &crate::mir::MirFunction,
    receiver: crate::mir::ValueId,
    expected: &Stage1StringBoxReceiverV1,
) -> bool {
    let mut new_stringbox_args = None;
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::Const {
                    dst,
                    value: ConstValue::String(_),
                } if *dst == receiver => {
                    return matches!(expected, Stage1StringBoxReceiverV1::DirectStringLiteral);
                }
                MirInstruction::NewBox {
                    dst,
                    target: ConstructionTarget::Named(name),
                    args,
                } if *dst == receiver && name == "StringBox" => {
                    new_stringbox_args = Some(args);
                }
                _ => {}
            }
        }
    }
    let Some(args) = new_stringbox_args else {
        return false;
    };
    matches!(expected, Stage1StringBoxReceiverV1::NewStringBox)
        && args.len() == 1
        && function.blocks.values().any(|block| {
            block.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::String(_)
                    } if *dst == args[0]
                )
            })
        })
}
