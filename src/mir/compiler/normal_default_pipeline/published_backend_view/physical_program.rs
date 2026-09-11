//! Complete physical program projection for the selected lifecycle consumer.
//!
//! The final view lends its finalized handoff and matching MIR bodies.
//! This projection validates retained correspondence and borrows the result;
//! it issues neither source facts nor C execution permission.

use crate::mir::instruction::InvokeCallResultKind;
use std::collections::BTreeSet;

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::definitions::MirCall;
use crate::mir::instruction::InvokeOperation;
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EdgeArgs, MirFunction, MirInstruction, MirModule,
    ValueId,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

use super::{CompiledEntryRootResultV1, PublishedMirBackendView, PublishedStaticMethodRouteV1};
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;

#[path = "physical_program_object_identity.rs"]
mod object_identity;
#[path = "physical_program_call_helpers.rs"]
mod call_helpers;
#[path = "physical_program_field_ref.rs"]
mod field_ref;
pub(crate) use call_helpers::{ordinary_call_receiver, ordinary_callable_key};
use field_ref::prepare_field_ref;

/// One exact selected function in the physical lifecycle program.
#[derive(Debug, Clone)]
pub(crate) enum PublishedLifecyclePhysicalFunctionRoleV1 {
    Root {
        result: CompiledEntryRootResultV1,
    },
    BirthUnit {
        abi: crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1,
    },
    OrdinaryI64 {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
}

impl PublishedLifecyclePhysicalFunctionRoleV1 {
    pub(crate) const fn wire_name(&self) -> &'static str {
        match self {
            Self::Root {
                result: CompiledEntryRootResultV1::I64,
                ..
            } => "root_i64",
            Self::Root {
                result: CompiledEntryRootResultV1::Unit,
                ..
            } => "root_unit",
            Self::BirthUnit { .. } => "birth_unit",
            Self::OrdinaryI64 { .. } => "ordinary_i64",
        }
    }

    pub(crate) fn birth_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::BirthUnit { abi } => Some(abi.target()),
            Self::Root { .. } | Self::OrdinaryI64 { .. } => None,
        }
    }

    pub(crate) fn ordinary_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::OrdinaryI64 { key, .. } => Some(key),
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) const fn receiver_object(&self) -> Option<CanonicalObjectIdV1> {
        match self {
            Self::OrdinaryI64 {
                receiver_object, ..
            } => *receiver_object,
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) fn has_receiver(&self) -> bool {
        match self {
            Self::BirthUnit { .. } => true,
            Self::OrdinaryI64 { key, .. } => {
                key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
            }
            Self::Root { .. } => false,
        }
    }

    pub(crate) fn receiver_value(&self, params: &[ValueId]) -> Option<ValueId> {
        self.has_receiver().then(|| params.first().copied()).flatten()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecyclePhysicalInstructionRefV1<'module> {
    index: u32,
    instruction: &'module MirInstruction,
    field_ref: Option<CanonicalFieldRefV1>,
}

impl<'module> PublishedLifecyclePhysicalInstructionRefV1<'module> {
    pub(crate) const fn index(self) -> u32 {
        self.index
    }

    pub(crate) fn instruction(self) -> &'module MirInstruction {
        self.instruction
    }

    pub(crate) const fn field_ref(self) -> Option<CanonicalFieldRefV1> {
        self.field_ref
    }
}

/// One physical CFG edge copied from the final block terminator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublishedLifecyclePhysicalEdgeV1 {
    target: BasicBlockId,
    args: Option<EdgeArgs>,
}

impl PublishedLifecyclePhysicalEdgeV1 {
    pub(crate) const fn target(&self) -> BasicBlockId {
        self.target
    }

    pub(crate) fn args(&self) -> Option<&EdgeArgs> {
        self.args.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalBlockV1<'module> {
    id: BasicBlockId,
    instructions: Box<[PublishedLifecyclePhysicalInstructionRefV1<'module>]>,
    terminator: PublishedLifecyclePhysicalInstructionRefV1<'module>,
    edges: Box<[PublishedLifecyclePhysicalEdgeV1]>,
}

impl<'module> PublishedLifecyclePhysicalBlockV1<'module> {
    pub(crate) const fn id(&self) -> BasicBlockId {
        self.id
    }

    pub(crate) fn instructions(&self) -> &[PublishedLifecyclePhysicalInstructionRefV1<'module>] {
        &self.instructions
    }

    pub(crate) const fn terminator(&self) -> PublishedLifecyclePhysicalInstructionRefV1<'module> {
        self.terminator
    }

    pub(crate) fn edges(&self) -> &[PublishedLifecyclePhysicalEdgeV1] {
        &self.edges
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalFunctionV1<'module> {
    name: &'module str,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    params: &'module [ValueId],
    entry: BasicBlockId,
    blocks: Box<[PublishedLifecyclePhysicalBlockV1<'module>]>,
}

impl<'module> PublishedLifecyclePhysicalFunctionV1<'module> {
    pub(crate) fn name(&self) -> &'module str {
        self.name
    }

    pub(crate) fn role(&self) -> &PublishedLifecyclePhysicalFunctionRoleV1 {
        &self.role
    }

    pub(crate) fn params(&self) -> &'module [ValueId] {
        self.params
    }

    pub(crate) const fn entry(&self) -> BasicBlockId {
        self.entry
    }

    pub(crate) fn blocks(&self) -> &[PublishedLifecyclePhysicalBlockV1<'module>] {
        &self.blocks
    }
}

/// Complete, deterministic physical program issued by the activated final view.
#[derive(Debug, Clone)]
pub(crate) struct PublishedLifecyclePhysicalProgramV1<'module> {
    functions: Box<[PublishedLifecyclePhysicalFunctionV1<'module>]>,
    handoff: &'module FinalizedRootHandoffV1,
}

impl<'module> PublishedLifecyclePhysicalProgramV1<'module> {
    pub(crate) fn handoff(&self) -> &'module FinalizedRootHandoffV1 {
        self.handoff
    }

    pub(crate) fn is_native_array(&self) -> bool {
        self.handoff.script_array().is_some()
    }

    pub(crate) fn functions(&self) -> &[PublishedLifecyclePhysicalFunctionV1<'module>] {
        &self.functions
    }
}

impl<'module> PublishedMirBackendView<'module> {
    /// Issues the complete physical image selected by final lifecycle admission.
    ///
    /// The retained source handoff selects functions and ABI.  Final MIR supplies
    /// only their already-published physical bodies; names and JSON are never
    /// consulted to select, repair, or classify the program.
    pub(crate) fn issue_lifecycle_physical_program(
        &self,
    ) -> Result<PublishedLifecyclePhysicalProgramV1<'module>, String> {
        let handoff = self
            .retained_handoff
            .ok_or_else(|| fault("root-handoff-missing"))?;
        let root = self.retained_root().ok_or_else(|| fault("root-missing"))?;
        let ordinary_calls = if handoff.script_array().is_some() {
            Vec::new()
        } else {
            collect_ordinary_calls(root)?
        };
        let (root_result, births) = if let Some(script) = handoff.script_array() {
            script.validate_root_binding(root)?;
            let result = match script.root_result()? {
                crate::mir::builder::ScriptArrayRootResultV1::Integer { .. } => {
                    CompiledEntryRootResultV1::I64
                }
                crate::mir::builder::ScriptArrayRootResultV1::Unit => {
                    CompiledEntryRootResultV1::Unit
                }
            };
            (result, &[][..])
        } else {
            if self.route() != PublishedStaticMethodRouteV1::CanonicalTyped {
                return Err(fault("not-final-lifecycle-view"));
            }
            let result = match handoff.root_result() {
                Some(result) => super::compiled_entry_contract::root_result_category(result),
                None => return Err(fault("root-result-missing")),
            };
            (
                result,
                handoff
                    .births()
                    .ok_or_else(|| fault("birth-handoff-missing"))?,
            )
        };
        let mut names = BTreeSet::new();
        let mut functions = Vec::with_capacity(births.len() + ordinary_calls.len() + 1);
        names.insert(root.signature.name.as_str());
        functions.push(issue_function_with_module(
            Some(self.module()),
            root,
            PublishedLifecyclePhysicalFunctionRoleV1::Root {
                result: root_result,
            },
            handoff.script_array().is_some(),
            &ordinary_calls,
        )?);
        let mut ordinary_keys = BTreeSet::new();
        for call in &ordinary_calls {
            let key = ordinary_callable_key(&call.callee)?;
            if !ordinary_keys.insert(key.clone()) {
                continue;
            }
            let symbol = self
                .module()
                .canonical_callable_definition_symbol(&key)
                .ok_or_else(|| fault("ordinary-definition-missing"))?;
            let function = self
                .module()
                .functions
                .get(symbol)
                .ok_or_else(|| fault("ordinary-function-missing"))?;
            let receiver = ordinary_call_receiver(&call.callee)?;
            let expected_arity = call.args.len() + usize::from(receiver.is_some());
            if function.signature.name != key.mir_symbol_projection()
                || function.signature.params.len() != expected_arity
                || function.signature.return_type != crate::mir::MirType::Integer
                || !names.insert(symbol)
            {
                return Err(fault("ordinary-membership-drift"));
            }
            let receiver_object = object_identity::ordinary_receiver_object(self.module(), &key)?;
            functions.push(issue_function_with_module(
                Some(self.module()),
                function,
                PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
                    key,
                    receiver_object,
                },
                false,
                &[],
            )?);
        }
        for birth in births {
            let key = birth.target();
            if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor {
                return Err(fault("birth-namespace"));
            }
            let symbol = self
                .module()
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| fault("birth-definition-missing"))?;
            let function = self
                .module()
                .functions
                .get(symbol)
                .ok_or_else(|| fault("birth-function-missing"))?;
            if function.signature.name != key.mir_symbol_projection()
                || function.params.len() != birth.abi().physical_arity()
                || !names.insert(symbol)
            {
                return Err(fault("birth-membership-drift"));
            }
            functions.push(issue_function_with_module(
                Some(self.module()),
                function,
                PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: birth.clone() },
                false,
                &[],
            )?);
        }
        Ok(PublishedLifecyclePhysicalProgramV1 {
            functions: functions.into_boxed_slice(),
            handoff,
        })
    }
}

fn collect_ordinary_calls(function: &MirFunction) -> Result<Vec<MirCall>, String> {
    let mut calls = Vec::new();
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    for block_id in block_ids {
        let block = function
            .blocks
            .get(&block_id)
            .ok_or_else(|| fault("function-block-membership"))?;
        for instruction in block.all_instructions() {
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        call,
                        result: InvokeCallResultKind::I64,
                    },
                ..
            } = instruction
            else {
                continue;
            };
            ordinary_callable_key(&call.callee)?;
            if call.dst.is_some() {
                return Err(fault("ordinary-destination"));
            }
            calls.push(call.clone());
        }
    }
    Ok(calls)
}

pub(super) fn issue_function<'module>(
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[MirCall],
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
    issue_function_with_module(None, function, role, script, ordinary_calls)
}

fn issue_function_with_module<'module>(
    module: Option<&'module MirModule>,
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[MirCall],
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
    let mut ids: Vec<_> = function.blocks.keys().copied().collect();
    ids.sort();
    if ids.is_empty() || !function.blocks.contains_key(&function.entry_block) {
        return Err(fault("function-block-membership"));
    }
    let mut blocks = Vec::with_capacity(ids.len());
    for id in ids {
        let block = function
            .blocks
            .get(&id)
            .expect("sorted final block id must remain present");
        let terminator = block
            .terminator
            .as_ref()
            .ok_or_else(|| fault("block-terminator-missing"))?;
        let mut instructions = Vec::with_capacity(block.instructions.len());
        for (index, instruction) in block.instructions.iter().enumerate() {
            let field_ref = prepare_field_ref(
                module,
                function,
                block.id,
                index,
                instruction,
            )?;
            validate_instruction_with_context(
                field_ref,
                instruction,
                script,
                ordinary_calls,
            )?;
            instructions.push(PublishedLifecyclePhysicalInstructionRefV1 {
                index: as_u32(index, "instruction-index")?,
                instruction,
                field_ref,
            });
        }
        let terminator_index = block.instructions.len();
        let terminator_field_ref = prepare_field_ref(
            module,
            function,
            block.id,
            terminator_index,
            terminator,
        )?;
        validate_instruction_with_context(
            terminator_field_ref,
            terminator,
            script,
            ordinary_calls,
        )?;
        let terminator_index = as_u32(terminator_index, "terminator-index")?;
        let edges = block
            .out_edges()
            .into_iter()
            .map(|edge| PublishedLifecyclePhysicalEdgeV1 {
                target: edge.target,
                args: edge.args,
            })
            .collect();
        blocks.push(PublishedLifecyclePhysicalBlockV1 {
            id,
            instructions: instructions.into_boxed_slice(),
            terminator: PublishedLifecyclePhysicalInstructionRefV1 {
                index: terminator_index,
                instruction: terminator,
                field_ref: terminator_field_ref,
            },
            edges,
        });
    }
    let ordinary_rows = blocks
        .iter()
        .flat_map(|block| {
            block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
        })
        .filter_map(|row| {
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        call,
                        result: InvokeCallResultKind::I64,
                    },
                ..
            } = row.instruction()
            else {
                return None;
            };
            Some(call)
        })
        .collect::<Vec<_>>();
    let mut consumed = vec![false; ordinary_rows.len()];
    for expected in ordinary_calls {
        let Some(index) = ordinary_rows.iter().enumerate().find_map(|(index, actual)| {
            (!consumed[index] && *actual == expected).then_some(index)
        }) else {
            return Err(fault("ordinary-call-membership"));
        };
        consumed[index] = true;
    }
    if consumed.iter().any(|used| !used) {
        return Err(fault("ordinary-call-membership"));
    }
    Ok(PublishedLifecyclePhysicalFunctionV1 {
        name: function.signature.name.as_str(),
        role,
        params: &function.params,
        entry: function.entry_block,
        blocks: blocks.into_boxed_slice(),
    })
}

fn validate_instruction(
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[MirCall],
) -> Result<(), String> {
    validate_instruction_with_context(None, instruction, script, ordinary_calls)
}

fn validate_instruction_with_context(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[MirCall],
) -> Result<(), String> {
    if script {
        return if matches!(
            instruction,
            MirInstruction::Const {
                value: ConstValue::Integer(_)
                    | ConstValue::Bool(_)
                    | ConstValue::Float(_)
                    | ConstValue::Void,
                ..
            } | MirInstruction::Copy { .. }
                | MirInstruction::FaultFrameEnter { .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::IntrinsicArrayNew
                        | InvokeOperation::ArrayStateContractClaim { .. }
                        | InvokeOperation::ArrayElementWrite { .. },
                    ..
                }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::ArrayResidenceRelease { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::Return { .. }
        ) {
            Ok(())
        } else {
            Err(fault("script-instruction-unsupported"))
        };
    }
    let ordinary = matches!(
        instruction,
        MirInstruction::Invoke {
            operation: InvokeOperation::Call {
                call,
                result: InvokeCallResultKind::I64,
            },
            ..
        } if ordinary_calls.iter().any(|expected| expected == call)
    );
    let field_get = matches!(instruction, MirInstruction::FieldGet { .. }) && field_ref.is_some();
    let supported = ordinary
        || field_get
        || matches!(
            instruction,
            MirInstruction::Const {
                value: ConstValue::Integer(_)
                    | ConstValue::Bool(_)
                    | ConstValue::String(_)
                    | ConstValue::Void,
                ..
            } | MirInstruction::BinOp {
                op: BinaryOp::Add,
                ..
            } | MirInstruction::Compare { .. }
                | MirInstruction::Copy { .. }
                | MirInstruction::Phi { .. }
                | MirInstruction::ObjectFieldGet { .. }
                | MirInstruction::Invoke {
                    operation: InvokeOperation::Map(_)
                        | InvokeOperation::NewBox { .. }
                        | InvokeOperation::FieldSet { .. }
                        | InvokeOperation::HomeRelease { .. }
                        | InvokeOperation::ReclaimUnpublished { .. }
                        | InvokeOperation::Call {
                            call: MirCall {
                                callee: Callee::BirthConstructor { .. },
                                ..
                            },
                            result: InvokeCallResultKind::Unit
                        },
                    ..
                }
                | MirInstruction::InvokeNormalResult { .. }
                | MirInstruction::ReturnFault { .. }
                | MirInstruction::FaultFrameEnter { .. }
                | MirInstruction::Branch { .. }
                | MirInstruction::Jump { .. }
                | MirInstruction::Return { .. }
                | MirInstruction::Call(MirCall {
                    callee: Callee::BirthConstructor { .. },
                    ..
                })
        );
    if supported {
        Ok(())
    } else {
        Err(fault("instruction-unsupported"))
    }
}

fn as_u32(value: usize, reason: &str) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| fault(reason))
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-program/{reason}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};
    use crate::parser::NyashParser;
    use std::collections::HashMap;

    fn request(source: &str) -> NormalCompileRequestV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("exact callable parse");
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("source identity must remain intact");
        };
        NormalCompileRequestV1::for_mir_mode_callable_source(source, None, HashMap::new())
    }

    #[test]
    fn final_view_issues_complete_pair_program_in_root_then_birth_order() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let mut compiler = MirCompiler::with_options(false);
            let result = compiler.compile_normal_with_published(
                request(include_str!(
                    "../../../../../apps/typed-object-birth-min/main.hako"
                )),
                |view, _| -> Result<(), String> {
                    let program = view.issue_lifecycle_physical_program()?;
                    let contract = view.issue_lifecycle_compiled_entry_contract()?;
                    let [root, birth] = program.functions() else {
                        panic!("Pair must retain root and one Birth function");
                    };
                    assert!(matches!(
                        root.role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::Root {
                            result: CompiledEntryRootResultV1::I64,
                            ..
                        }
                    ));
                    let [entry_birth] = contract.births() else {
                        panic!("Pair must retain one compiled Birth contract");
                    };
                    assert_eq!(entry_birth.function_index(), 1);
                    assert_eq!(entry_birth.formals().len(), 3);
                    assert!(entry_birth.formals()[0].source_ordinal().is_none());
                    assert_eq!(entry_birth.formals()[1].source_ordinal(), Some(0));
                    assert!(entry_birth.formals()[1].disposition().is_some());
                    let [birth_call] = contract.birth_calls() else {
                        panic!("Pair must retain one exact Birth call");
                    };
                    assert_eq!(birth_call.function_index(), 1);
                    assert_eq!(birth_call.arguments().len(), 2);
                    assert!(!contract.cleanup().is_empty());
                    assert!(matches!(
                        birth.role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi }
                            if abi.abi().source_arity() == 2 && birth.params().len() == 3
                    ));
                    assert!(root
                        .blocks()
                        .windows(2)
                        .all(|blocks| blocks[0].id() < blocks[1].id()));
                    let all = root.blocks().iter().flat_map(|block| {
                        block
                            .instructions()
                            .iter()
                            .copied()
                            .chain(std::iter::once(block.terminator()))
                    });
                    assert!(all.clone().any(|row| matches!(
                        row.instruction(),
                        MirInstruction::Const {
                            value: ConstValue::Integer(10 | 20),
                            ..
                        }
                    )));
                    assert!(all.clone().any(|row| matches!(
                        row.instruction(),
                        MirInstruction::BinOp {
                            op: BinaryOp::Add,
                            ..
                        }
                    )));
                    assert_eq!(
                        all.filter(|row| matches!(
                            row.instruction(),
                            MirInstruction::ObjectFieldGet { .. }
                        ))
                        .count(),
                        2,
                    );
                    Err("[freeze:contract][published-lifecycle/consumer-pending]".into())
                },
            );
            match result {
                Err(error) if error.contains("consumer-pending") => {}
                Err(error) => panic!("unexpected selected consumer error: {error}"),
                Ok(_) => panic!("selected consumer must propagate pending terminal"),
            }
        });
    }
}

#[cfg(test)]
#[path = "physical_program_call_tests.rs"]
mod call_result_tests;

#[cfg(test)]
#[path = "physical_program_order_tests.rs"]
mod order_tests;
