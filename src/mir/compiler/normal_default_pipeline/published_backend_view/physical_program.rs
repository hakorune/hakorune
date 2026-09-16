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
    /// Caller-owned opaque Map storage result; never an i64 handle.
    OrdinaryMap {
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
            Self::OrdinaryMap { .. } => "ordinary_map",
        }
    }

    pub(crate) fn birth_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::BirthUnit { abi } => Some(abi.target()),
            Self::Root { .. } | Self::OrdinaryI64 { .. } | Self::OrdinaryMap { .. } => None,
        }
    }

    pub(crate) fn ordinary_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::OrdinaryI64 { key, .. } | Self::OrdinaryMap { key, .. } => Some(key),
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) const fn receiver_object(&self) -> Option<CanonicalObjectIdV1> {
        match self {
            Self::OrdinaryI64 {
                receiver_object, ..
            }
            | Self::OrdinaryMap {
                receiver_object, ..
            } => *receiver_object,
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) fn has_receiver(&self) -> bool {
        match self {
            Self::BirthUnit { .. } => true,
            Self::OrdinaryI64 { key, .. } | Self::OrdinaryMap { key, .. } => {
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
        // Ordinary membership is transitive: every emitted function carries
        // its own sealed call rows, and callees discovered mid-walk join the
        // walk. The retained root selects the entry, not the edge set.
        let mut call_sets: std::collections::BTreeMap<&str, Vec<OrdinaryCallSite>> =
            std::collections::BTreeMap::new();
        let mut ordinary_sites: std::collections::BTreeMap<
            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
            OrdinaryCallSite,
        > = std::collections::BTreeMap::new();
        if handoff.script_array().is_none() {
            let mut visited = BTreeSet::from([root.signature.name.as_str()]);
            let mut pending = std::collections::VecDeque::from([root]);
            while let Some(function) = pending.pop_front() {
                let sites = collect_ordinary_calls(function)?;
                for site in &sites {
                    let key = ordinary_callable_key(&site.call.callee)?;
                    match ordinary_sites.entry(key.clone()) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(site.clone());
                        }
                        // One physical result contract per callee key; a key
                        // cannot be both an i64 callee and a Map callee.
                        std::collections::btree_map::Entry::Occupied(entry)
                            if entry.get().result == site.result => {}
                        _ => return Err(fault("ordinary-result-contract-drift")),
                    }
                    let symbol = self
                        .module()
                        .canonical_callable_definition_symbol(&key)
                        .ok_or_else(|| fault("ordinary-definition-missing"))?;
                    if visited.insert(symbol) {
                        pending.push_back(
                            self.module()
                                .functions
                                .get(symbol)
                                .ok_or_else(|| fault("ordinary-function-missing"))?,
                        );
                    }
                }
                call_sets.insert(function.signature.name.as_str(), sites);
            }
        }
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
        let mut functions = Vec::with_capacity(births.len() + ordinary_sites.len() + 1);
        names.insert(root.signature.name.as_str());
        functions.push(issue_function_with_module(
            Some(self.module()),
            root,
            PublishedLifecyclePhysicalFunctionRoleV1::Root {
                result: root_result,
            },
            handoff.script_array().is_some(),
            call_sets
                .get(root.signature.name.as_str())
                .map(Vec::as_slice)
                .unwrap_or(&[]),
        )?);
        for (key, site) in &ordinary_sites {
            let symbol = self
                .module()
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| fault("ordinary-definition-missing"))?;
            let function = self
                .module()
                .functions
                .get(symbol)
                .ok_or_else(|| fault("ordinary-function-missing"))?;
            let receiver = ordinary_call_receiver(&site.call.callee)?;
            let expected_arity = site.call.args.len() + usize::from(receiver.is_some());
            if function.signature.name != key.mir_symbol_projection()
                || function.signature.params.len() != expected_arity
                || !matches!(
                    (site.result, &function.signature.return_type),
                    (InvokeCallResultKind::I64, crate::mir::MirType::Integer)
                        | (
                            InvokeCallResultKind::Map,
                            crate::mir::MirType::Box(_) | crate::mir::MirType::Unknown,
                        )
                )
                || !names.insert(symbol)
            {
                return Err(fault("ordinary-membership-drift"));
            }
            let receiver_object = object_identity::ordinary_receiver_object(self.module(), key)?;
            let role = match site.result {
                InvokeCallResultKind::I64 => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::Map => {
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryMap {
                        key: key.clone(),
                        receiver_object,
                    }
                }
                InvokeCallResultKind::Unit => {
                    return Err(fault("ordinary-result-contract-drift"));
                }
            };
            functions.push(issue_function_with_module(
                Some(self.module()),
                function,
                role,
                false,
                call_sets.get(symbol).map(Vec::as_slice).unwrap_or(&[]),
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

/// One admitted ordinary call site with its physical result kind. The MirCall
/// itself carries no result kind; the invoke operation selects the callee ABI.
#[derive(Debug, Clone)]
pub(super) struct OrdinaryCallSite {
    pub(super) call: MirCall,
    pub(super) result: InvokeCallResultKind,
}

fn collect_ordinary_calls(function: &MirFunction) -> Result<Vec<OrdinaryCallSite>, String> {
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
                        result: result @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Map),
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
            calls.push(OrdinaryCallSite {
                call: call.clone(),
                result: *result,
            });
        }
    }
    Ok(calls)
}

pub(super) fn issue_function<'module>(
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
) -> Result<PublishedLifecyclePhysicalFunctionV1<'module>, String> {
    issue_function_with_module(None, function, role, script, ordinary_calls)
}

fn issue_function_with_module<'module>(
    module: Option<&'module MirModule>,
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
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
                        result:
                            result @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Map),
                    },
                ..
            } = row.instruction()
            else {
                return None;
            };
            Some((call, *result))
        })
        .collect::<Vec<_>>();
    let mut consumed = vec![false; ordinary_rows.len()];
    for expected in ordinary_calls {
        let Some(index) = ordinary_rows.iter().enumerate().find_map(|(index, actual)| {
            (!consumed[index]
                && *actual.0 == expected.call
                && actual.1 == expected.result)
                .then_some(index)
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
    ordinary_calls: &[OrdinaryCallSite],
) -> Result<(), String> {
    validate_instruction_with_context(None, instruction, script, ordinary_calls)
}

fn validate_instruction_with_context(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    script: bool,
    ordinary_calls: &[OrdinaryCallSite],
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
                result: result @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Map),
            },
            ..
        } if ordinary_calls
            .iter()
            .any(|expected| expected.call == *call && expected.result == *result)
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

    #[test]
    fn nested_ordinary_call_chain_emits_per_function_call_rows() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let mut compiler = MirCompiler::with_options(false);
            let result = compiler.compile_normal_with_published(
                request(
                    "static function inner(): i64 { return 7 }
                     static function helper(value: i64): i64 { return value }
                     static box Main { main() { return helper(10) } }",
                ),
                |view, _| -> Result<(), String> {
                    // The producer lane cannot yet seal a call edge inside an
                    // ordinary callee; inject the nested edge on a module clone
                    // so the physical projection is pinned at its own layer.
                    let handoff = view.retained_handoff.expect("one borrowed handoff");
                    let mut nested = view.module().clone();
                    let symbol_of = |fragment: &str| {
                        nested
                            .canonical_callable_definitions
                            .keys()
                            .map(|key| key.mir_symbol_projection())
                            .find(|symbol| symbol.contains(fragment))
                            .unwrap_or_else(|| panic!("{fragment} catalog symbol"))
                    };
                    let helper_symbol = symbol_of("helper");
                    let inner_symbol = symbol_of("inner");
                    let helper = nested.functions.get_mut(&helper_symbol).unwrap();
                    let invoke_block = *helper.blocks.keys().next().unwrap();
                    let normal = BasicBlockId::new(90);
                    let fault_block = BasicBlockId::new(91);
                    let nested_result = ValueId::new(900);
                    let nested_call = MirCall::new(
                        None,
                        Callee::Global(
                            hakorune_mir_defs::CanonicalGlobalTargetV1::new_free_function(
                                "inner".into(),
                                0,
                            )
                            .unwrap(),
                        ),
                        vec![],
                    );
                    helper
                        .blocks
                        .get_mut(&invoke_block)
                        .unwrap()
                        .set_terminator(MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call: nested_call.clone(),
                                result: InvokeCallResultKind::I64,
                            },
                            fault_frame: ValueId::INVALID,
                            normal_landing: normal,
                            fault_landing: fault_block,
                        });
                    let mut normal_block = crate::mir::BasicBlock::new(normal);
                    normal_block.add_instruction(MirInstruction::InvokeNormalResult {
                        invoke_block,
                        dst: nested_result,
                    });
                    normal_block.set_terminator(MirInstruction::Return {
                        value: Some(nested_result),
                    });
                    helper.blocks.insert(normal, normal_block);
                    let mut fault_block = crate::mir::BasicBlock::new(fault_block);
                    fault_block.set_terminator(MirInstruction::ReturnFault {
                        fault_frame: ValueId::INVALID,
                    });
                    helper.blocks.insert(fault_block.id, fault_block);

                    let admitted = super::super::super::lifecycle_admission::admit_lifecycle(
                        PublishedMirBackendView::try_new(&nested)
                            .map_err(|error| error.to_string())?
                            .bind_finalized_root_handoff(Some(handoff))?,
                        &super::super::PublishedObjectStorageProfileV1::SafeMutex,
                    )?;
                    let program = admitted.issue_lifecycle_physical_program()?;
                    let contract = admitted.issue_lifecycle_compiled_entry_contract()?;
                    assert_eq!(program.functions().len(), 3, "{:?}", program.functions());
                    let index_of = |fragment: &str| {
                        program
                            .functions()
                            .iter()
                            .position(|function| function.name().contains(fragment))
                            .unwrap_or_else(|| panic!("{fragment} emitted function"))
                    };
                    let helper_index = index_of("helper");
                    let inner_index = index_of("inner");
                    assert!(matches!(
                        program.functions()[0].role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::Root {
                            result: CompiledEntryRootResultV1::I64,
                            ..
                        }
                    ));
                    for (index, symbol) in [(helper_index, "helper"), (inner_index, "inner")] {
                        assert!(
                            matches!(
                                program.functions()[index].role(),
                                PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
                                    key,
                                    ..
                                } if key.mir_symbol_projection().contains(symbol)
                            ),
                            "{symbol} must keep its ordinary i64 role"
                        );
                    }
                    assert_eq!(program.functions()[helper_index].name(), helper_symbol);
                    assert_eq!(program.functions()[inner_index].name(), inner_symbol);
                    // The callee's own edge stays inside its emitted body.
                    let nested_rows: Vec<_> = program.functions()[helper_index]
                        .blocks()
                        .iter()
                        .flat_map(|block| {
                            block
                                .instructions()
                                .iter()
                                .copied()
                                .chain(std::iter::once(block.terminator()))
                        })
                        .filter_map(|row| match row.instruction() {
                            MirInstruction::Invoke {
                                operation: InvokeOperation::Call { call, result },
                                ..
                            } => Some((call, *result)),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(
                        nested_rows,
                        [(&nested_call, InvokeCallResultKind::I64)],
                        "helper must carry exactly its own sealed call row"
                    );
                    // Each contract row names its exact caller.
                    let mut callers: Vec<_> = contract
                        .ordinary_calls()
                        .iter()
                        .map(|call| (call.caller_function_index(), call.function_index()))
                        .collect();
                    callers.sort();
                    assert_eq!(
                        callers,
                        [
                            (0, helper_index as u32),
                            (helper_index as u32, inner_index as u32)
                        ],
                        "{callers:?}"
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
