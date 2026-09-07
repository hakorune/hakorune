//! Contract-program projection for the lifecycle C frame.

use super::*;

impl PublishedLifecycleCFrameV2 {
    pub(super) fn populate(
        &mut self,
        module: &crate::mir::MirModule,
        contract: &CompiledEntryContractV1<'_>,
    ) -> Result<(), String> {
        module.validate_object_definition_membership()?;
        let [root, births @ ..] = contract.program().functions() else {
            return Err(fault("compiled-entry-root-missing"));
        };
        if births.len() != contract.births().len() {
            return Err(fault("compiled-entry-birth-count"));
        }
        let mut definitions = Vec::with_capacity(births.len());
        for (index, (function, entry_birth)) in births.iter().zip(contract.births()).enumerate() {
            let PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: birth } =
                function.role()
            else {
                return Err(fault("compiled-entry-birth-role"));
            };
            let key = birth.target();
            if key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
                || birth.result()
                    != crate::mir::normal_callable_semantic_package::BirthResultAbiV1::Unit
                || birth.receiver().source_ordinal().is_some()
                || birth.receiver().physical_lane() != 0
                || birth
                    .parameters()
                    .iter()
                    .enumerate()
                    .any(|(index, formal)| {
                        formal.source_ordinal() != Some(index as u32)
                            || formal.physical_lane() != index as u32 + 1
                    })
            {
                return Err(fault("birth-abi-relation-invalid"));
            }
            let symbol = function.name();
            if entry_birth.formals().len() != birth.abi().physical_arity() {
                return Err(fault("birth-physical-lane-count"));
            }
            let object_id = module
                .metadata
                .canonical_object_membership
                .as_ref()
                .and_then(|membership| membership.get(key.owner()))
                .ok_or_else(|| fault("definition-object-missing"))?;
            if *object_id != birth.object() {
                return Err(fault("definition-object-membership-drift"));
            }
            definitions.push((key, symbol));
            let function_name = self.push_string(symbol)?;
            let target_symbol = self.push_string(&key.mir_symbol_projection())?;
            self.definitions.push(PublishedLifecycleDefinitionCRowV2 {
                function_name,
                target_symbol,
                role: DEFINITION_ROLE_BIRTH_UNIT,
                source_arity: as_u32(birth.abi().source_arity(), "source-arity")?,
                receiver_formal: birth.receiver().physical_lane(),
                object_id: birth.object().declaration_index(),
                result_kind: RESULT_KIND_UNIT,
                frame_mode: definition_frame_mode(function)?,
                flags: 0,
            });
            for formal in entry_birth.formals().iter().copied() {
                let input_kind = match formal.kind() {
                    CompiledEntryFormalKindV1::Receiver => 1,
                    CompiledEntryFormalKindV1::Parameter => 2,
                };
                if formal.kind() == CompiledEntryFormalKindV1::Parameter
                    && formal.disposition().is_none()
                {
                    return Err(fault("compiled-entry-formal-disposition-missing"));
                }
                self.formals.push(PublishedLifecycleFormalCRowV2 {
                    definition_index: as_u32(index, "definition-index")?,
                    source_ordinal: formal
                        .source_ordinal()
                        .unwrap_or(PUBLISHED_LIFECYCLE_ABSENT_U32_V2),
                    physical_ordinal: formal.physical_ordinal(),
                    value_id: formal.value().0,
                    wire_revision: 2,
                    input_kind,
                });
            }
        }
        let (root_role, root_result_kind) = match contract.root_result() {
            CompiledEntryRootResultV1::I64 => (DEFINITION_ROLE_ROOT_I64, RESULT_KIND_I64),
            CompiledEntryRootResultV1::Unit => (DEFINITION_ROLE_ROOT_UNIT, RESULT_KIND_UNIT),
        };
        let root_name = self.push_string(root.name())?;
        let root_symbol = self.push_string(root.name())?;
        self.definitions.push(PublishedLifecycleDefinitionCRowV2 {
            function_name: root_name,
            target_symbol: root_symbol,
            role: root_role,
            source_arity: 0,
            receiver_formal: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
            object_id: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
            result_kind: root_result_kind,
            frame_mode: definition_frame_mode(root)?,
            flags: 1,
        });
        if self.definitions.is_empty() {
            return Err(fault("birth-definition-missing"));
        }

        let object_definitions = module
            .canonical_object_definitions()
            .ok_or_else(|| fault("object-definitions-missing"))?;
        for (index, definition) in object_definitions.iter().enumerate() {
            if definition.destruction_disposition()
                != ObjectDestructionDispositionV1::PlainI64NoHook
            {
                return Err(fault("destruction-unavailable"));
            }
            let layout = definition
                .runtime_layout()
                .ok_or_else(|| fault("layout-not-issued"))?
                .as_ref()
                .map_err(|_| fault("layout-unavailable"))?;
            let object_id = as_u32(index, "object-index")?;
            self.layouts.push(PublishedLifecycleLayoutCRowV2 {
                object_id,
                runtime_type_id: layout.type_id,
                field_count: layout.field_count,
                destruction_kind: 1,
            });
            for (ordinal, field) in layout.fields.iter().enumerate() {
                self.fields.push(PublishedLifecycleFieldCRowV2 {
                    object_id,
                    declaration_ordinal: as_u32(ordinal, "field-ordinal")?,
                    runtime_slot: field.slot,
                    storage_kind: storage_kind(field.storage),
                });
            }
        }

        let mut seen_birth_calls = vec![false; contract.birth_calls().len()];
        let mut seen_cleanup = vec![false; contract.cleanup().len()];
        for (function_index, function) in contract.program().functions().iter().enumerate() {
            for block in function.blocks() {
                for row in block
                    .instructions()
                    .iter()
                    .copied()
                    .chain(std::iter::once(block.terminator()))
                {
                    let name = self.push_string(function.name())?;
                    let instruction = row.instruction();
                    mark_cleanup_coordinate(
                        contract,
                        &mut seen_cleanup,
                        function_index,
                        block.id().0,
                        row.index(),
                        instruction,
                    )?;
                    match instruction {
                        MirInstruction::Invoke {
                            operation,
                            fault_frame,
                            normal_landing,
                            fault_landing,
                        } => {
                            if let InvokeOperation::Call(call) = operation {
                                mark_birth_call(contract, &mut seen_birth_calls, call)?;
                            }
                            let (
                                kind,
                                definition_index,
                                object_id,
                                field_ordinal,
                                base,
                                value,
                                receiver,
                                operands,
                            ) = operation_row(operation, &definitions)?;
                            let operation_index = as_u32(self.operations.len(), "operation-index")?;
                            self.operations.push(PublishedLifecycleOperationCRowV2 {
                                function_name: name,
                                block_id: block.id().0,
                                instruction_index: row.index(),
                                kind,
                                definition_index,
                                fault_frame: fault_frame.0,
                                normal_landing: normal_landing.as_u32(),
                                fault_landing: fault_landing.as_u32(),
                                object_id,
                                field_ordinal,
                                base,
                                value,
                                receiver,
                                operand_count: as_u32(operands.len(), "operand-count")?,
                                flags: 0,
                            });
                            for (ordinal, value_id) in operands.into_iter().enumerate() {
                                self.operands.push(PublishedLifecycleOperandCRowV2 {
                                    operation_index,
                                    ordinal: as_u32(ordinal, "operand-ordinal")?,
                                    value_id,
                                    kind: 1,
                                });
                            }
                            if let InvokeOperation::NewBox { object } = operation {
                                let result = normal_result_for_newbox(
                                    function,
                                    block.id().0,
                                    normal_landing.as_u32(),
                                )?;
                                self.body_sites.push(PublishedLifecycleBodySiteCRowV1 {
                                    function_name: name,
                                    block_id: block.id().0,
                                    instruction_index: row.index(),
                                    normal_result: result.0,
                                    fault_frame: fault_frame.0,
                                    normal_landing: normal_landing.as_u32(),
                                    fault_landing: fault_landing.as_u32(),
                                    object_id: object.declaration_index(),
                                });
                            }
                        }
                        MirInstruction::ObjectFieldGet { dst, base, field } => {
                            let operation_index = as_u32(self.operations.len(), "operation-index")?;
                            self.operations.push(PublishedLifecycleOperationCRowV2 {
                                function_name: name,
                                block_id: block.id().0,
                                instruction_index: row.index(),
                                kind: 6,
                                definition_index: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                fault_frame: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                normal_landing: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                fault_landing: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                object_id: field.object().declaration_index(),
                                field_ordinal: field.declaration_ordinal(),
                                base: base.0,
                                value: dst.0,
                                receiver: base.0,
                                operand_count: 1,
                                flags: 0,
                            });
                            self.operands.push(PublishedLifecycleOperandCRowV2 {
                                operation_index,
                                ordinal: 0,
                                value_id: base.0,
                                kind: 1,
                            });
                        }
                        MirInstruction::InvokeNormalResult { invoke_block, dst } => {
                            self.controls.push(control_row(
                                name,
                                block.id().0,
                                row.index(),
                                1,
                                dst.0,
                                invoke_block.as_u32(),
                                0,
                            ))
                        }
                        MirInstruction::ReturnFault { fault_frame } => {
                            self.controls.push(control_row(
                                name,
                                block.id().0,
                                row.index(),
                                2,
                                fault_frame.0,
                                PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                0,
                            ))
                        }
                        MirInstruction::FaultFrameEnter { dst, mode } => {
                            self.controls.push(control_row(
                                name,
                                block.id().0,
                                row.index(),
                                3,
                                dst.0,
                                PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                match mode {
                                    FaultFrameMode::RootOwned => 1,
                                    FaultFrameMode::Borrowed => 2,
                                },
                            ))
                        }
                        MirInstruction::Return { value } => self.controls.push(control_row(
                            name,
                            block.id().0,
                            row.index(),
                            CONTROL_KIND_RETURN,
                            value.map_or(PUBLISHED_LIFECYCLE_ABSENT_U32_V2, |value| value.0),
                            PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                            u32::from(value.is_some()),
                        )),
                        MirInstruction::Call(call) => {
                            mark_birth_call(contract, &mut seen_birth_calls, call)?;
                            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                                return Err(fault("call-not-birth"));
                            };
                            let definition_index = definitions
                                .iter()
                                .position(|(candidate, _)| **candidate == *key)
                                .ok_or_else(|| fault("birth-definition-missing"))?;
                            let operation_index = as_u32(self.operations.len(), "operation-index")?;
                            self.operations.push(PublishedLifecycleOperationCRowV2 {
                                function_name: name,
                                block_id: block.id().0,
                                instruction_index: row.index(),
                                kind: 1,
                                definition_index: as_u32(definition_index, "definition-index")?,
                                fault_frame: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                normal_landing: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                fault_landing: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                object_id: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                field_ordinal: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                base: PUBLISHED_LIFECYCLE_ABSENT_U32_V2,
                                value: call
                                    .dst
                                    .map_or(PUBLISHED_LIFECYCLE_ABSENT_U32_V2, |value| value.0),
                                receiver: receiver.0,
                                operand_count: as_u32(call.args.len() + 1, "operand-count")?,
                                flags: 0,
                            });
                            self.operands.push(PublishedLifecycleOperandCRowV2 {
                                operation_index,
                                ordinal: 0,
                                value_id: receiver.0,
                                kind: 2,
                            });
                            for (ordinal, value) in call.args.iter().enumerate() {
                                self.operands.push(PublishedLifecycleOperandCRowV2 {
                                    operation_index,
                                    ordinal: as_u32(ordinal + 1, "operand-ordinal")?,
                                    value_id: value.0,
                                    kind: 1,
                                });
                            }
                        }
                        // Scalar/CFG rows are carried by the companion physical-program
                        // JSON. This frame owns only the typed lifecycle row families.
                        _ => {}
                    }
                }
            }
        }
        if seen_birth_calls.iter().any(|seen| !seen) {
            return Err(fault("compiled-entry-call-unconsumed"));
        }
        if seen_cleanup.iter().any(|seen| !seen) {
            return Err(fault("compiled-entry-cleanup-unconsumed"));
        }
        if self.operations.is_empty()
            || self.controls.is_empty()
            || self.layouts.is_empty()
            || self.fields.is_empty()
        {
            return Err(fault("required-row-family-empty"));
        }
        if self.body_sites.is_empty() {
            return Err(fault("newbox-body-site-missing"));
        }
        self.header.definitions = self.definitions.as_ptr();
        self.header.definition_count = self.definitions.len();
        self.header.formals = self.formals.as_ptr();
        self.header.formal_count = self.formals.len();
        self.header.operations = self.operations.as_ptr();
        self.header.operation_count = self.operations.len();
        self.header.operands = self.operands.as_ptr();
        self.header.operand_count = self.operands.len();
        self.header.controls = self.controls.as_ptr();
        self.header.control_count = self.controls.len();
        self.header.layouts = self.layouts.as_ptr();
        self.header.layout_count = self.layouts.len();
        self.header.fields = self.fields.as_ptr();
        self.header.field_count = self.fields.len();
        Ok(())
    }

    fn push_string(&mut self, value: &str) -> Result<*const c_char, String> {
        let value = CString::new(value).map_err(|_| fault("string-nul"))?;
        let pointer = value.as_ptr();
        self.strings.push(value);
        Ok(pointer)
    }
}

fn operation_row(
    operation: &InvokeOperation,
    definitions: &[(&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, &str)],
) -> Result<(u32, u32, u32, u32, u32, u32, u32, Vec<u32>), String> {
    let absent = PUBLISHED_LIFECYCLE_ABSENT_U32_V2;
    Ok(match operation {
        InvokeOperation::Call(call) => {
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                return Err(fault("invoke-call-not-birth"));
            };
            let definition_index = definitions
                .iter()
                .position(|(candidate, _)| **candidate == *key)
                .ok_or_else(|| fault("operation-definition-missing"))?;
            let mut operands = vec![receiver.0];
            operands.extend(call.args.iter().map(|value| value.0));
            (
                1,
                as_u32(definition_index, "definition-index")?,
                absent,
                absent,
                absent,
                absent,
                receiver.0,
                operands,
            )
        }
        InvokeOperation::NewBox { object } => (
            2,
            absent,
            object.declaration_index(),
            absent,
            absent,
            absent,
            absent,
            Vec::new(),
        ),
        InvokeOperation::FieldSet { field, base, value } => (
            3,
            absent,
            field.object().declaration_index(),
            field.declaration_ordinal(),
            base.0,
            value.0,
            base.0,
            vec![base.0, value.0],
        ),
        InvokeOperation::HomeRelease { object, value } => (
            4,
            absent,
            object.declaration_index(),
            absent,
            absent,
            value.0,
            absent,
            vec![value.0],
        ),
        InvokeOperation::ReclaimUnpublished { object, value } => (
            5,
            absent,
            object.declaration_index(),
            absent,
            absent,
            value.0,
            absent,
            vec![value.0],
        ),
    })
}

fn control_row(
    name: *const c_char,
    block_id: u32,
    instruction_index: u32,
    kind: u32,
    operand: u32,
    origin_block: u32,
    mode: u32,
) -> PublishedLifecycleControlCRowV2 {
    PublishedLifecycleControlCRowV2 {
        function_name: name,
        block_id,
        instruction_index,
        kind,
        operand,
        origin_block,
        mode,
        flags: 0,
    }
}

fn definition_frame_mode(
    function: &crate::mir::compiler::normal_default_pipeline::published_backend_view::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
) -> Result<u32, String> {
    let modes: Vec<_> = function
        .blocks()
        .iter()
        .flat_map(|block| {
            block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
        })
        .filter_map(|row| {
            if let MirInstruction::FaultFrameEnter { mode, .. } = row.instruction() {
                Some(mode)
            } else {
                None
            }
        })
        .collect();
    match modes.as_slice() {
        [FaultFrameMode::RootOwned] => Ok(1),
        [FaultFrameMode::Borrowed] => Ok(2),
        _ => Err(fault("definition-frame-mode")),
    }
}

fn normal_result_for_newbox(
    function: &crate::mir::compiler::normal_default_pipeline::published_backend_view::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    invoke_block: u32,
    normal_landing: u32,
) -> Result<crate::mir::ValueId, String> {
    let results: Vec<_> = function
        .blocks()
        .iter()
        .filter(|block| block.id().0 == normal_landing)
        .flat_map(|block| {
            block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
        })
        .filter_map(|row| match row.instruction() {
            MirInstruction::InvokeNormalResult {
                invoke_block: origin,
                dst,
            } if origin.as_u32() == invoke_block => Some(*dst),
            _ => None,
        })
        .collect();
    match results.as_slice() {
        [result] => Ok(*result),
        _ => Err(fault("newbox-normal-result")),
    }
}

fn mark_birth_call(
    contract: &CompiledEntryContractV1<'_>,
    seen: &mut [bool],
    call: &crate::mir::definitions::MirCall,
) -> Result<(), String> {
    let Callee::BirthConstructor { key, receiver } = &call.callee else {
        return Err(fault("call-not-birth"));
    };
    let index = contract
        .program()
        .functions()
        .iter()
        .enumerate()
        .skip(1)
        .find_map(|(index, function)| {
            matches!(function.role(), PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } if abi.target() == key)
                .then_some(index)
        })
        .ok_or_else(|| fault("compiled-entry-call-target"))?;
    let matches = contract.birth_calls().iter().enumerate().filter(|(_, expected)|
        expected.function_index() == index as u32 && expected.receiver() == *receiver
            && expected.arguments().eq(call.args.iter().copied()))
        .map(|(i, _)| i).collect::<Vec<_>>();
    let [call_index] = matches.as_slice() else {
        return Err(fault("compiled-entry-call-drift"));
    };
    let consumed = seen.get_mut(*call_index)
        .ok_or_else(|| fault("compiled-entry-call-index"))?;
    if std::mem::replace(consumed, true) {
        return Err(fault("compiled-entry-call-duplicate"));
    }
    Ok(())
}

fn mark_cleanup_coordinate(
    contract: &CompiledEntryContractV1<'_>,
    seen: &mut [bool],
    function_index: usize,
    block_id: u32,
    instruction_index: u32,
    instruction: &MirInstruction,
) -> Result<(), String> {
    let kind = match instruction {
        MirInstruction::Invoke {
            operation: InvokeOperation::HomeRelease { .. },
            ..
        } => Some(CompiledEntryCleanupKindV1::HomeRelease),
        MirInstruction::Invoke {
            operation: InvokeOperation::ReclaimUnpublished { .. },
            ..
        } => Some(CompiledEntryCleanupKindV1::ReclaimUnpublished),
        MirInstruction::FaultFrameEnter { .. } => Some(CompiledEntryCleanupKindV1::FaultFrameEnter),
        MirInstruction::ReturnFault { .. } => Some(CompiledEntryCleanupKindV1::ReturnFault),
        _ => None,
    };
    let Some(kind) = kind else {
        return Ok(());
    };
    let index = contract
        .cleanup()
        .iter()
        .position(|coordinate| {
            coordinate.function_index() == function_index as u32
                && coordinate.block_id() == block_id
                && coordinate.instruction_index() == instruction_index
                && coordinate.kind() == kind
        })
        .ok_or_else(|| fault("compiled-entry-cleanup-drift"))?;
    let consumed = seen
        .get_mut(index)
        .ok_or_else(|| fault("compiled-entry-cleanup-index"))?;
    if std::mem::replace(consumed, true) {
        return Err(fault("compiled-entry-cleanup-duplicate"));
    }
    Ok(())
}

fn storage_kind(storage: TypedObjectFieldStorage) -> u32 {
    match storage {
        TypedObjectFieldStorage::I8 => 1,
        TypedObjectFieldStorage::I16 => 2,
        TypedObjectFieldStorage::I32 => 3,
        TypedObjectFieldStorage::I64 => 4,
        TypedObjectFieldStorage::ISize => 5,
        TypedObjectFieldStorage::U8 => 6,
        TypedObjectFieldStorage::U16 => 7,
        TypedObjectFieldStorage::U32 => 8,
        TypedObjectFieldStorage::U64 => 9,
        TypedObjectFieldStorage::USize => 10,
        TypedObjectFieldStorage::Handle => 11,
    }
}

fn as_u32(value: usize, reason: &str) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| fault(reason))
}
fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle/{reason}]")
}
