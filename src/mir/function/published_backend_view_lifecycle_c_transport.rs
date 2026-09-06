//! Versioned physical transport for the selected object lifecycle consumer.
//!
//! V2 carries one backend-owned storage profile and typed row arrays borrowed
//! for one synchronous C call. It does not issue source meaning, object
//! identity, layout, or lifecycle permission; those remain owned by the final
//! published view. V1 stays layout-compatible for its existing cohorts.

use std::ffi::CString;
use std::os::raw::c_char;

use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::function::{ObjectDestructionDispositionV1, TypedObjectFieldStorage};
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};
use crate::mir::{Callee, MirInstruction};

use super::{
    lifecycle_schema::{
        ABSENT_U32, CONTROL_KIND_RETURN, DEFINITION_ROLE_BIRTH_UNIT, DEFINITION_ROLE_ROOT_I64,
        DEFINITION_ROLE_ROOT_UNIT, RESULT_KIND_I64, RESULT_KIND_UNIT,
    },
    CompiledEntryFormalKindV1, PublishedLifecyclePhysicalFunctionRoleV1, PublishedMirBackendView,
    PublishedStaticMethodCFrameV1, PublishedStaticMethodCallCRowV1,
};

pub(crate) const PUBLISHED_LIFECYCLE_ABI_REVISION_V2: u32 = 2;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedObjectStorageProfileV1 {
    SafeMutex = 1,
    SingleThreadExact = 2,
}

impl PublishedObjectStorageProfileV1 {
    pub(crate) fn from_runtime_name(value: Option<&str>) -> Result<Self, String> {
        match value {
            None | Some("") | Some("safe_mutex") => Ok(Self::SafeMutex),
            Some("single_thread_exact") => Ok(Self::SingleThreadExact),
            Some(other) => Err(format!(
                "[freeze:contract][published-lifecycle/storage-profile] unsupported profile: {other}"
            )),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleDefinitionCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) target_symbol: *const c_char,
    pub(crate) role: u32,
    pub(crate) source_arity: u32,
    pub(crate) receiver_formal: u32,
    pub(crate) object_id: u32,
    pub(crate) result_kind: u32,
    pub(crate) frame_mode: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleFormalCRowV2 {
    pub(crate) definition_index: u32,
    pub(crate) source_ordinal: u32,
    pub(crate) physical_ordinal: u32,
    pub(crate) value_id: u32,
    pub(crate) wire_revision: u32,
    pub(crate) input_kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleOperationCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) kind: u32,
    pub(crate) definition_index: u32,
    pub(crate) fault_frame: u32,
    pub(crate) normal_landing: u32,
    pub(crate) fault_landing: u32,
    pub(crate) object_id: u32,
    pub(crate) field_ordinal: u32,
    pub(crate) base: u32,
    pub(crate) value: u32,
    pub(crate) receiver: u32,
    pub(crate) operand_count: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleOperandCRowV2 {
    pub(crate) operation_index: u32,
    pub(crate) ordinal: u32,
    pub(crate) value_id: u32,
    pub(crate) kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleControlCRowV2 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) kind: u32,
    pub(crate) operand: u32,
    pub(crate) origin_block: u32,
    pub(crate) mode: u32,
    pub(crate) flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleLayoutCRowV2 {
    pub(crate) object_id: u32,
    pub(crate) runtime_type_id: u32,
    pub(crate) field_count: u32,
    pub(crate) destruction_kind: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleFieldCRowV2 {
    pub(crate) object_id: u32,
    pub(crate) declaration_ordinal: u32,
    pub(crate) runtime_slot: u32,
    pub(crate) storage_kind: u32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleBodySiteCRowV1 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) normal_result: u32,
    pub(crate) fault_frame: u32,
    pub(crate) normal_landing: u32,
    pub(crate) fault_landing: u32,
    pub(crate) object_id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedLifecycleCFrameHeaderV2 {
    pub(crate) abi_revision: u32,
    pub(crate) storage_profile: u32,
    pub(crate) call_rows: *const PublishedStaticMethodCallCRowV1,
    pub(crate) call_row_count: usize,
    pub(crate) definitions: *const PublishedLifecycleDefinitionCRowV2,
    pub(crate) definition_count: usize,
    pub(crate) formals: *const PublishedLifecycleFormalCRowV2,
    pub(crate) formal_count: usize,
    pub(crate) operations: *const PublishedLifecycleOperationCRowV2,
    pub(crate) operation_count: usize,
    pub(crate) operands: *const PublishedLifecycleOperandCRowV2,
    pub(crate) operand_count: usize,
    pub(crate) controls: *const PublishedLifecycleControlCRowV2,
    pub(crate) control_count: usize,
    pub(crate) layouts: *const PublishedLifecycleLayoutCRowV2,
    pub(crate) layout_count: usize,
    pub(crate) fields: *const PublishedLifecycleFieldCRowV2,
    pub(crate) field_count: usize,
}

/// Owns every pointer reachable from the V2 header for one synchronous call.
#[derive(Debug)]
pub(crate) struct PublishedLifecycleCFrameV2 {
    calls: PublishedStaticMethodCFrameV1,
    strings: Vec<CString>,
    definitions: Vec<PublishedLifecycleDefinitionCRowV2>,
    formals: Vec<PublishedLifecycleFormalCRowV2>,
    operations: Vec<PublishedLifecycleOperationCRowV2>,
    operands: Vec<PublishedLifecycleOperandCRowV2>,
    controls: Vec<PublishedLifecycleControlCRowV2>,
    layouts: Vec<PublishedLifecycleLayoutCRowV2>,
    fields: Vec<PublishedLifecycleFieldCRowV2>,
    body_sites: Vec<PublishedLifecycleBodySiteCRowV1>,
    header: PublishedLifecycleCFrameHeaderV2,
}

impl PublishedLifecycleCFrameV2 {
    /// S0 constructor. Empty lifecycle arrays never admit execution; the same
    /// series populates them from the published view before V2 is callable.
    pub(crate) fn from_call_frame(
        profile: PublishedObjectStorageProfileV1,
        calls: PublishedStaticMethodCFrameV1,
    ) -> Self {
        let header = PublishedLifecycleCFrameHeaderV2 {
            abi_revision: PUBLISHED_LIFECYCLE_ABI_REVISION_V2,
            storage_profile: profile as u32,
            call_rows: calls.as_ptr(),
            call_row_count: calls.len(),
            definitions: std::ptr::null(),
            definition_count: 0,
            formals: std::ptr::null(),
            formal_count: 0,
            operations: std::ptr::null(),
            operation_count: 0,
            operands: std::ptr::null(),
            operand_count: 0,
            controls: std::ptr::null(),
            control_count: 0,
            layouts: std::ptr::null(),
            layout_count: 0,
            fields: std::ptr::null(),
            field_count: 0,
        };
        Self {
            calls,
            strings: Vec::new(),
            definitions: Vec::new(),
            formals: Vec::new(),
            operations: Vec::new(),
            operands: Vec::new(),
            controls: Vec::new(),
            layouts: Vec::new(),
            fields: Vec::new(),
            body_sites: Vec::new(),
            header,
        }
    }

    /// Projects only the lifecycle coordinates retained by the final view.
    /// It never rescans the module or derives source identity from names.
    pub(crate) fn from_view(view: &PublishedMirBackendView<'_>) -> Result<Self, String> {
        let profile = view
            .lifecycle_storage_profile()
            .ok_or_else(|| fault("profile-not-issued"))?;
        let calls =
            PublishedStaticMethodCFrameV1::from_view(view).map_err(|error| error.to_string())?;
        let mut frame = Self::from_call_frame(profile, calls);
        frame.populate(view)?;
        Ok(frame)
    }

    pub(crate) const fn header(&self) -> &PublishedLifecycleCFrameHeaderV2 {
        &self.header
    }

    pub(crate) fn call_rows(&self) -> &[PublishedStaticMethodCallCRowV1] {
        self.calls.as_slice()
    }
    pub(crate) fn body_sites(&self) -> &[PublishedLifecycleBodySiteCRowV1] {
        &self.body_sites
    }

    pub(crate) fn definition_rows(&self) -> &[PublishedLifecycleDefinitionCRowV2] {
        &self.definitions
    }

    fn populate(&mut self, view: &PublishedMirBackendView<'_>) -> Result<(), String> {
        let module = view.module();
        module.validate_object_definition_membership()?;
        let contract = view.issue_lifecycle_compiled_entry_contract()?;
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
            let function = module
                .functions
                .get(symbol)
                .ok_or_else(|| fault("definition-missing"))?;
            if function.params.len() != birth.abi().physical_arity() {
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
                    source_ordinal: formal.source_ordinal().unwrap_or(ABSENT_U32),
                    physical_ordinal: formal.physical_ordinal(),
                    value_id: formal.value().0,
                    wire_revision: 2,
                    input_kind,
                });
            }
        }
        let (root_role, root_result_kind) = match contract.root_result() {
            crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64AddReturn { .. } =>
                (DEFINITION_ROLE_ROOT_I64, RESULT_KIND_I64),
            crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::UnitReturn { .. } =>
                (DEFINITION_ROLE_ROOT_UNIT, RESULT_KIND_UNIT),
            crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::IntegerLiteralReturn { .. } =>
                (DEFINITION_ROLE_ROOT_I64, RESULT_KIND_I64),
            crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::I64FieldReturn { .. } =>
                (DEFINITION_ROLE_ROOT_I64, RESULT_KIND_I64),
        };
        let root_function = module
            .functions
            .get(root.name())
            .ok_or_else(|| fault("root-result-root-missing"))?;
        let root_name = self.push_string(root.name())?;
        let root_symbol = self.push_string(root.name())?;
        self.definitions.push(PublishedLifecycleDefinitionCRowV2 {
            function_name: root_name,
            target_symbol: root_symbol,
            role: root_role,
            source_arity: 0,
            receiver_formal: ABSENT_U32,
            object_id: ABSENT_U32,
            result_kind: root_result_kind,
            frame_mode: definition_frame_mode(root_function)?,
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

        for row in view.lifecycle_instructions() {
            let name = self.push_string(row.function_name())?;
            match row.instruction() {
                MirInstruction::Invoke {
                    operation,
                    fault_frame,
                    normal_landing,
                    fault_landing,
                } => {
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
                        block_id: row.block_id(),
                        instruction_index: row.instruction_index(),
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
                }
                MirInstruction::ObjectFieldGet { dst, base, field } => {
                    let operation_index = as_u32(self.operations.len(), "operation-index")?;
                    self.operations.push(PublishedLifecycleOperationCRowV2 {
                        function_name: name,
                        block_id: row.block_id(),
                        instruction_index: row.instruction_index(),
                        kind: 6,
                        definition_index: u32::MAX,
                        fault_frame: u32::MAX,
                        normal_landing: u32::MAX,
                        fault_landing: u32::MAX,
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
                MirInstruction::InvokeNormalResult { invoke_block, dst } => self
                    .controls
                    .push(control_row(name, *row, 1, dst.0, invoke_block.as_u32(), 0)),
                MirInstruction::ReturnFault { fault_frame } => {
                    self.controls
                        .push(control_row(name, *row, 2, fault_frame.0, u32::MAX, 0))
                }
                MirInstruction::FaultFrameEnter { dst, mode } => self.controls.push(control_row(
                    name,
                    *row,
                    3,
                    dst.0,
                    u32::MAX,
                    match mode {
                        FaultFrameMode::RootOwned => 1,
                        FaultFrameMode::Borrowed => 2,
                    },
                )),
                MirInstruction::Return { value } => self.controls.push(control_row(
                    name,
                    *row,
                    CONTROL_KIND_RETURN,
                    value.map_or(ABSENT_U32, |value| value.0),
                    ABSENT_U32,
                    u32::from(value.is_some()),
                )),
                MirInstruction::Call(call) => {
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
                        block_id: row.block_id(),
                        instruction_index: row.instruction_index(),
                        kind: 1,
                        definition_index: as_u32(definition_index, "definition-index")?,
                        fault_frame: u32::MAX,
                        normal_landing: u32::MAX,
                        fault_landing: u32::MAX,
                        object_id: u32::MAX,
                        field_ordinal: u32::MAX,
                        base: u32::MAX,
                        value: call.dst.map_or(u32::MAX, |value| value.0),
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
                _ => return Err(fault("retained-instruction-drift")),
            }
        }
        for row in view.lifecycle_instructions() {
            let MirInstruction::Invoke {
                operation: InvokeOperation::NewBox { object },
                fault_frame,
                normal_landing,
                fault_landing,
            } = row.instruction()
            else {
                continue;
            };
            let results: Vec<_> = view.lifecycle_instructions().iter().filter_map(|candidate| {
                (candidate.function_name() == row.function_name()
                    && candidate.block_id() == normal_landing.as_u32()
                    && matches!(candidate.instruction(), MirInstruction::InvokeNormalResult { invoke_block, .. } if invoke_block.as_u32() == row.block_id()))
                    .then(|| match candidate.instruction() {
                        MirInstruction::InvokeNormalResult { dst, .. } => *dst,
                        _ => unreachable!(),
                    })
            }).collect();
            let [result] = results.as_slice() else {
                return Err(fault("newbox-normal-result"));
            };
            let function_name = self.push_string(row.function_name())?;
            self.body_sites.push(PublishedLifecycleBodySiteCRowV1 {
                function_name,
                block_id: row.block_id(),
                instruction_index: row.instruction_index(),
                normal_result: result.0,
                fault_frame: fault_frame.0,
                normal_landing: normal_landing.as_u32(),
                fault_landing: fault_landing.as_u32(),
                object_id: object.declaration_index(),
            });
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
    let absent = u32::MAX;
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
    row: super::lifecycle::PublishedLifecycleInstructionRef<'_>,
    kind: u32,
    operand: u32,
    origin_block: u32,
    mode: u32,
) -> PublishedLifecycleControlCRowV2 {
    PublishedLifecycleControlCRowV2 {
        function_name: name,
        block_id: row.block_id(),
        instruction_index: row.instruction_index(),
        kind,
        operand,
        origin_block,
        mode,
        flags: 0,
    }
}

fn definition_frame_mode(function: &crate::mir::MirFunction) -> Result<u32, String> {
    let modes: Vec<_> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| {
            if let MirInstruction::FaultFrameEnter { mode, .. } = instruction {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_profile_is_finite_and_rejects_legacy_backends() {
        assert_eq!(
            PublishedObjectStorageProfileV1::from_runtime_name(None).unwrap(),
            PublishedObjectStorageProfileV1::SafeMutex
        );
        assert_eq!(
            PublishedObjectStorageProfileV1::from_runtime_name(Some("single_thread_exact"))
                .unwrap(),
            PublishedObjectStorageProfileV1::SingleThreadExact
        );
        for rejected in ["direct_slot_exact", "pinned_arena_exact", "unknown"] {
            assert!(PublishedObjectStorageProfileV1::from_runtime_name(Some(rejected)).is_err());
        }
    }

    #[test]
    fn v2_rows_have_the_fixed_lp64_c_layout() {
        use std::mem::{align_of, size_of};

        assert_eq!(size_of::<PublishedLifecycleDefinitionCRowV2>(), 48);
        assert_eq!(size_of::<PublishedLifecycleFormalCRowV2>(), 24);
        assert_eq!(size_of::<PublishedLifecycleOperationCRowV2>(), 64);
        assert_eq!(size_of::<PublishedLifecycleOperandCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleControlCRowV2>(), 40);
        assert_eq!(size_of::<PublishedLifecycleLayoutCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleFieldCRowV2>(), 16);
        assert_eq!(size_of::<PublishedLifecycleBodySiteCRowV1>(), 40);
        assert_eq!(size_of::<PublishedLifecycleCFrameHeaderV2>(), 136);
        assert_eq!(align_of::<PublishedLifecycleCFrameHeaderV2>(), 8);
    }
}
