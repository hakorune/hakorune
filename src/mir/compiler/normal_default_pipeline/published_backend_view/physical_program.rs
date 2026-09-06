//! Complete physical program projection for the selected lifecycle consumer.
//!
//! The activated final view is the only owner permitted to join its retained
//! root/Birth source handoff with final MIR bodies.  This is a physical borrow:
//! it records neither source facts nor C lowering choices.

use std::collections::BTreeSet;

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::{
    BasicBlockId, BinaryOp, Callee, ConstValue, EdgeArgs, MirFunction,
    MirInstruction, ValueId,
};
use crate::mir::instruction::InvokeOperation;
use crate::mir::definitions::MirCall;

use super::{PublishedMirBackendView, PublishedStaticMethodRouteV1};

/// One exact selected function in the physical lifecycle program.
#[derive(Debug, Clone)]
pub(crate) enum PublishedLifecyclePhysicalFunctionRoleV1 {
    RootI64 {
        result: crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1,
    },
    BirthUnit {
        abi: crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1,
    },
}

impl PublishedLifecyclePhysicalFunctionRoleV1 {
    pub(crate) const fn wire_name(&self) -> &'static str {
        match self {
            Self::RootI64 { .. } => "root_i64",
            Self::BirthUnit { .. } => "birth_unit",
        }
    }

    pub(crate) fn birth_target(&self) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::BirthUnit { abi } => Some(abi.target()),
            Self::RootI64 { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecyclePhysicalInstructionRefV1<'module> {
    index: u32,
    instruction: &'module MirInstruction,
}

impl<'module> PublishedLifecyclePhysicalInstructionRefV1<'module> {
    pub(crate) const fn index(self) -> u32 {
        self.index
    }

    pub(crate) fn instruction(self) -> &'module MirInstruction {
        self.instruction
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
}

impl<'module> PublishedLifecyclePhysicalProgramV1<'module> {
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
        if self.route() != PublishedStaticMethodRouteV1::CanonicalTyped {
            return Err(fault("not-final-lifecycle-view"));
        }
        let root = self.retained_root().ok_or_else(|| fault("root-missing"))?;
        let root_result = self
            .retained_root_result()
            .ok_or_else(|| fault("root-result-missing"))?;
        let births = self
            .retained_birth_abi()
            .ok_or_else(|| fault("birth-handoff-missing"))?;
        let mut names = BTreeSet::new();
        let mut functions = Vec::with_capacity(births.len() + 1);
        names.insert(root.signature.name.as_str());
        functions.push(issue_function(
            root,
            PublishedLifecyclePhysicalFunctionRoleV1::RootI64 {
                result: root_result,
            },
        )?);
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
            functions.push(issue_function(
                function,
                PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: birth.clone() },
            )?);
        }
        Ok(PublishedLifecyclePhysicalProgramV1 {
            functions: functions.into_boxed_slice(),
        })
    }
}

fn issue_function<'module>(
    function: &'module MirFunction,
    role: PublishedLifecyclePhysicalFunctionRoleV1,
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
            validate_instruction(instruction)?;
            instructions.push(PublishedLifecyclePhysicalInstructionRefV1 {
                index: as_u32(index, "instruction-index")?,
                instruction,
            });
        }
        validate_instruction(terminator)?;
        let terminator_index = as_u32(block.instructions.len(), "terminator-index")?;
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
            },
            edges,
        });
    }
    Ok(PublishedLifecyclePhysicalFunctionV1 {
        name: function.signature.name.as_str(),
        role,
        params: &function.params,
        entry: function.entry_block,
        blocks: blocks.into_boxed_slice(),
    })
}

fn validate_instruction(instruction: &MirInstruction) -> Result<(), String> {
    let supported = matches!(
        instruction,
        MirInstruction::Const {
            value: ConstValue::Integer(_) | ConstValue::String(_) | ConstValue::Void,
            ..
        } | MirInstruction::BinOp {
            op: BinaryOp::Add,
            ..
        } | MirInstruction::Copy { .. }
            | MirInstruction::Phi { .. }
            | MirInstruction::ObjectFieldGet { .. }
            | MirInstruction::Invoke {
                operation:
                    InvokeOperation::NewBox { .. }
                    | InvokeOperation::FieldSet { .. }
                    | InvokeOperation::HomeRelease { .. }
                    | InvokeOperation::ReclaimUnpublished { .. }
                    | InvokeOperation::Call(MirCall {
                        callee: Callee::BirthConstructor { .. },
                        ..
                    }),
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
    use crate::mir::compiler::normal_default_pipeline::{
        MirCompiler, NormalCompileRequestV1,
    };
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
                request(include_str!("../../../../../apps/typed-object-birth-min/main.hako")),
                |view, _| -> Result<(), String> {
                    let program = view.issue_lifecycle_physical_program()?;
                    let contract = view.issue_lifecycle_compiled_entry_contract()?;
                    let [root, birth] = program.functions() else {
                        panic!("Pair must retain root and one Birth function");
                    };
                    assert!(matches!(
                        root.role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::RootI64 { .. }
                    ));
                    let [entry_birth] = contract.births() else {
                        panic!("Pair must retain one compiled Birth contract");
                    };
                    assert_eq!(entry_birth.function_index(), 1);
                    assert_eq!(entry_birth.formals().len(), 3);
                    assert!(entry_birth.formals()[0].source_ordinal().is_none());
                    assert_eq!(entry_birth.formals()[1].source_ordinal(), Some(0));
                    assert!(entry_birth.formals()[1].disposition().is_some());
                    assert!(matches!(
                        birth.role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi }
                            if abi.abi().source_arity() == 2 && birth.params().len() == 3
                    ));
                    assert!(root.blocks().windows(2).all(|blocks| blocks[0].id() < blocks[1].id()));
                    let all = root.blocks().iter().flat_map(|block| {
                        block.instructions().iter().copied().chain(std::iter::once(block.terminator()))
                    });
                    assert!(all.clone().any(|row| matches!(
                        row.instruction(),
                        MirInstruction::Const { value: ConstValue::Integer(10 | 20), .. }
                    )));
                    assert!(all.clone().any(|row| matches!(
                        row.instruction(),
                        MirInstruction::BinOp { op: BinaryOp::Add, .. }
                    )));
                    assert_eq!(
                        all.filter(|row| matches!(row.instruction(), MirInstruction::ObjectFieldGet { .. })).count(),
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
