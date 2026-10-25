//! Lend exact original ordinary Call slots to final module verification.
//! This is not a new semantic issuer. Physical publication still validates
//! producer provenance, full use closure and the immutable module snapshot.
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::finalized_root_handoff::FinalizedRootHandoffV1;
use crate::mir::{BasicBlockId, MirModule, ValueId, MirInstruction};
use crate::mir::instruction::{InvokeOperation, StringInvokeOperand};
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use std::collections::{BTreeMap, BTreeSet};

type CallSlots = BTreeMap<(String, BasicBlockId, usize), BTreeSet<usize>>;
#[derive(Default)]
pub(super) struct OriginalCallSourceImageV1 {
    pub(super) slots: CallSlots,
    pub(super) text: BTreeMap<(String, BasicBlockId, usize, usize), ValueId>,
    pub(super) field_reads: Vec<(String, (BasicBlockId, MirInstruction))>,
    pub(super) field_writes: Vec<(String, (BasicBlockId, MirInstruction))>,
    pub(super) births: BTreeSet<(String, BasicBlockId, usize)>,
}

pub(super) fn original_call_slots(
    module: &MirModule,
    root: Option<&FinalizedRootHandoffV1>,
) -> Result<OriginalCallSourceImageV1, String> {
    // Source-less verification retains its existing sealed scalar policy.
    let Some(root) = root else {
        return Ok(OriginalCallSourceImageV1::default());
    };
    let mut selected: BTreeSet<_> = module
        .functions
        .values()
        .filter(|function| {
            function
                .metadata
                .physical_param_carriers
                .as_deref()
                .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue))
        })
        .map(|function| function.signature.name.clone())
        .collect();
    let Some(source) = root.root_source() else {
        return if selected.is_empty() {
            Ok(OriginalCallSourceImageV1::default())
        } else {
            Err("borrowed-source-missing".into())
        };
    };
    // Validate source-declared entry correspondence before projecting any
    // incoming actuals. Malformed physical parameters should fail at their
    // declaration contract, not first surface as unrelated call/SSA drift.
    let callables = root
        .callables()
        .ok_or_else(|| "borrowed-declaration/owner-missing".to_owned())?;
    let declarations = source.borrowed_declaration_keys_v1(callables)?;
    for key in &declarations {
        let owner = callables
            .owner_for_canonical_key(key)
            .ok_or_else(|| "borrowed-declaration/owner-missing".to_owned())?;
        let symbol = module
            .canonical_callable_definition_symbol(key)
            .ok_or_else(|| "borrowed-declaration/canonical-key-missing".to_owned())?;
        let function = module
            .functions
            .get(symbol)
            .ok_or_else(|| "borrowed-declaration/function-missing".to_owned())?;
        source.borrowed_declaration_for_function_v1(owner, key, function)?;
    }
    let mut image = OriginalCallSourceImageV1::default();
    source.visit_finished_array_field_reads_v1(module, root.named_arrays(), |function, binding, field| {
        if !module.canonical_array_field(field)
            || image.field_reads.contains(&(function.signature.name.clone(), binding.clone())) {
            return Err("borrowed-source-coordinate-duplicate".into());
        }
        image.field_reads.push((function.signature.name.clone(), binding));
        Ok(())
    })?;
    // Demand selects verification only; no opcode or declaration grants a loan.
    // Keep the old exact-slot verifier passive when no Dynamic view is requested.
    let dynamic_fields = module.functions.values().flat_map(|function| function.blocks.values())
        .flat_map(|body| body.all_instructions()).any(|row| match row {
            MirInstruction::Invoke { operation: InvokeOperation::FieldGet { field, .. }
                | InvokeOperation::ObjectFieldBorrow { field, nullable: true, .. }, .. } =>
                module.canonical_field_definition(*field).is_some_and(|field|
                    !field.is_weak && field.declared_type_name.is_none()),
            MirInstruction::Invoke { operation: InvokeOperation::FieldSet {
                mode, .. }, .. } => *mode != crate::mir::instruction::FieldSetModeV1::ExactSlot,
            _ => false,
        });
    if dynamic_fields {
        // These are borrowed final correspondence rows, not another source index.
        source.with_checked_field_read_functions_v1(module, |owner, function| {
            for binding in source.finished_received_field_reads_v1(owner, module, function)? {
                image.field_reads.push((function.signature.name.clone(), binding));
            }
            Ok(())
        })?;
        source.with_owned_field_write_functions_v1(module, |owner, function| {
            source.with_finished_owned_field_writes_v1(owner, function, |row| {
                image.field_writes.push((function.signature.name.clone(), row.invoke.0));
                Ok(())
            })
        })?;
        source.visit_finished_construction_writes_v1(module, |function, row| {
            // Only the retained whole-constructor replay can distinguish this
            // formal from an ordinary borrowed entry with the same carrier.
            if matches!(&row.1, MirInstruction::Invoke { operation: InvokeOperation::FieldSet {
                mode: crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, .. }, .. }) {
                selected.remove(&function.signature.name);
            }
            image.field_writes.push((function.signature.name.clone(), row));
            Ok(())
        })?;
    }
    for function in module.functions.values() {
        for (block, body) in &function.blocks {
            for (index, row) in body.all_instructions().enumerate() {
                let MirInstruction::Invoke { operation: InvokeOperation::Birth(call), .. } = row else { continue };
                if !call.arguments.iter().any(|arg| matches!(arg, crate::mir::instruction::BirthArgumentV1::Text(_))) { continue; }
                let mut actual = None;
                for candidate in root.birth_actuals().into_iter().flatten().filter(|candidate|
                    candidate.target() == &call.key && candidate.receiver() == call.receiver
                        && candidate.arguments().iter().map(|arg| arg.payload()).eq(call.arguments.iter())) {
                    if source.birth_actual_belongs_to_function_v1(candidate, function)?
                        && actual.replace(candidate).is_some() { return Err("borrowed-source-coordinate-duplicate".into()); }
                }
                let actual = actual.ok_or("call-result-contract-not-connected")?;
                let mut abis = root.births().into_iter().flatten().filter(|abi| abi.target() == &call.key);
                let abi = abis.next().ok_or("call-result-contract-not-connected")?;
                if abis.next().is_some() { return Err("borrowed-source-coordinate-duplicate".into()); }
                source.validate_birth_text_actuals_v1(abi, actual, function)?;
                image.births.insert((function.signature.name.clone(), *block, index));
            }
        }
    }
    let mut callees = BTreeSet::new();
    let mut incoming = BTreeSet::new();
    let mut original = BTreeSet::new();
    source.visit_finalized_borrowed_call_nodes_v1(
        module,
        |packet, arguments, caller, coordinate, _| {
            if let Some(read) = source.finished_stored_receiver_read_v1(packet, caller)? {
                let read = (caller.signature.name.clone(), read);
                if image.field_reads.contains(&read) { return Err("borrowed-source-coordinate-duplicate".into()); }
                image.field_reads.push(read);
            }
            let row = packet;
            let symbol = module
                .canonical_callable_definition_symbol(row.target())
                .ok_or_else(|| "borrowed-source-target-missing".to_owned())?;
            let selected_source = packet.selects_borrowed_source(arguments);
            if !selected_source && !selected.contains(symbol) {
                return Ok(());
            }
            let offset = match row.target().namespace() {
                hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => 1,
                hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => 0,
                _ => return Err("borrowed-source-nonordinary-target".into()),
            };
            let callee = module
                .functions
                .get(symbol)
                .ok_or_else(|| "borrowed-source-callee-missing".to_owned())?;
            let entry = source
                .borrowed_ordinary_entry_source_for_function_v1(row.callee_owner(), callee)?;
            let values = source.borrowed_ordinary_entry_values_v1(row.callee_owner())?;
            let carriers = callee
                .metadata
                .physical_param_carriers
                .as_deref()
                .ok_or_else(|| "borrowed-source-carrier-missing".to_owned())?;
            if carriers.len() != callee.params.len()
                || callee.signature.params.len() != callee.params.len()
            {
                return Err("borrowed-source-entry-shape".into());
            }
            let slots: BTreeSet<_> = carriers
                .iter()
                .enumerate()
                .filter_map(|(slot, carrier)| {
                    (*carrier == Carrier::BorrowedTaggedValue).then_some(slot)
                })
                .collect();
            let mut proven = BTreeSet::new();
            for (ordinal, _, value) in values.iter() {
                let slot = *ordinal as usize + offset;
                if callee.params.get(slot) != Some(value)
                    || carriers.get(slot) != Some(&Carrier::BorrowedTaggedValue)
                {
                    return Err("borrowed-source-entry-correspondence".into());
                }
                proven.insert(slot);
            }
            if slots != proven || slots.is_empty() || (offset == 1 && slots.contains(&0)) {
                return Err("borrowed-source-entry-coverage".into());
            }
            for target in entry.incoming_targets() {
                let target = target?;
                if target.target() != row.target() || target.callee_owner() != row.callee_owner() {
                    return Err("borrowed-source-target-drift".into());
                }
                incoming.insert(target.call_site().clone());
            }
            let actuals = source
                .borrowed_finalized_call_actuals_v1(packet)?
                .ok_or_else(|| "borrowed-source-actuals-missing".to_owned())?;
            let actual_slots: BTreeSet<_> = actuals
                .iter()
                .map(|actual| actual.ordinal as usize)
                .collect();
            let argument_slots: BTreeSet<_> = slots.iter().map(|slot| slot - offset).collect();
            if actual_slots != argument_slots || actual_slots.len() != actuals.len() {
                return Err("borrowed-source-actual-coverage".into());
            }
            for actual in actuals {
                let Some(operand) = packet.borrowed_computed_argument_operand_v1(actual.ordinal, CallValidationContextV1::Finalized(source))? else { continue };
                let value = match operand { StringInvokeOperand::OwnedString(value) | StringInvokeOperand::NormalInteger(value) => value,
                    _ => return Err("borrowed-source-text-operand".into()) };
                let instruction = caller.blocks.get(&coordinate.0).and_then(|block| block.all_instructions().nth(coordinate.1))
                    .ok_or_else(|| "borrowed-source-text-coordinate".to_owned())?;
                let MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } = instruction
                    else { return Err("borrowed-source-text-call".into()) };
                if call.args.get(actual.ordinal as usize) != Some(&value)
                    || matches!(operand, StringInvokeOperand::OwnedString(_))
                        && image.text.insert((caller.signature.name.clone(), coordinate.0, coordinate.1, actual.ordinal as usize), value).is_some() {
                    return Err("borrowed-source-text-identity".into());
                }
            }
            if image.slots
                .insert(
                    (caller.signature.name.clone(), coordinate.0, coordinate.1),
                    actual_slots,
                )
                .is_some()
            {
                return Err("borrowed-source-coordinate-duplicate".into());
            }
            callees.insert(symbol.to_owned());
            original.insert(row.call_site().clone());
            Ok(())
        },
    )?;
    source.visit_finished_map_field_reads_v1(module, |function, binding| {
        let row = (function.signature.name.clone(), binding);
        if image.field_reads.contains(&row) { return Err("borrowed-source-coordinate-duplicate".into()); }
        image.field_reads.push(row);
        Ok(())
    })?;
    source.visit_finished_map_field_installs_v1(module, |function, binding| {
        let row = (function.signature.name.clone(), binding);
        if image.field_writes.contains(&row) { return Err("borrowed-source-coordinate-duplicate".into()); }
        image.field_writes.push(row);
        Ok(())
    })?;
    let mut source_only = Vec::new();
    for symbol in selected.difference(&callees) {
        let mut keys = module
            .canonical_callable_definitions
            .iter()
            .filter(|(_, name)| name.as_str() == symbol.as_str());
        let (key, _) = keys
            .next()
            .ok_or_else(|| "borrowed-declaration/canonical-key-missing".to_owned())?;
        if keys.next().is_some() {
            return Err("borrowed-declaration/canonical-key".into());
        }
        if !declarations.contains(key) {
            source_only.push(symbol.clone());
            continue;
        }
    }
    // Uncalled source-only declarations acquire no borrowed-entry protocol.
    // Any actual physical call still owes its original exact call slot below.
    for symbol in source_only { selected.remove(&symbol); }
    // Declarations contribute only owner coverage, never fabricated call slots.
    callees.extend(selected.iter().cloned());
    if callees != selected || incoming != original {
        return Err(format!(
            "borrowed-source-incoming-coverage: missing-callees={:?}, extra-callees={:?}, missing-sites={:?}, extra-sites={:?}, missing-callers={:?}",
            selected.difference(&callees).collect::<Vec<_>>(),
            callees.difference(&selected).collect::<Vec<_>>(),
            incoming.difference(&original).collect::<Vec<_>>(),
            original.difference(&incoming).collect::<Vec<_>>(),
            incoming.difference(&original).map(|site| (
                site,
                module.canonical_callable_definitions.keys().filter(|key|
                    root.callables().and_then(|rows| rows.owner_for_canonical_key(key)) == Some(site.owner())
                ).collect::<Vec<_>>(),
            )).collect::<Vec<_>>(),
        ));
    }
    Ok(image)
}
