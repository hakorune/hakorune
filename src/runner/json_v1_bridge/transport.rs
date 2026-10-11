//! Data-only fallible document reader. No module or source authority escapes.
use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, ValueId};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

mod correspondence;
mod instruction;
mod shape;
#[cfg(test)]
mod tests;
mod unique_json;
use shape::*;

pub fn parse_canonical_document_transport_v1(json: &str) -> Result<Value, String> {
    let document = unique_json::parse(json)?;
    validate_canonical_document_transport_v1(&document)?;
    Ok(document)
}

pub(crate) fn validate_canonical_document_transport_v1(document: &Value) -> Result<(), String> {
    if document["schema_version"] != "1.0" {
        return Err(error("schema"));
    }
    let module = &document["canonical_transport_v1"];
    fields(
        module,
        &["version", "module_name", "callable_definitions"],
        &[],
    )?;
    if module["version"] != 1 {
        return Err(error("version"));
    }
    string(module, "module_name")?;
    let functions = array(document, "functions")?;
    let mut by_name = BTreeMap::new();
    for function in functions {
        let name = string(function, "name")?;
        if by_name.insert(name, function).is_some() {
            return Err(error("duplicate-function"));
        }
    }
    let mut definitions = BTreeMap::new();
    let mut symbols = BTreeSet::new();
    for row in array(module, "callable_definitions")? {
        fields(row, &["key", "symbol"], &[])?;
        let key = key(&row["key"])?;
        let symbol = string(row, "symbol")?;
        let function = by_name
            .get(symbol)
            .ok_or_else(|| error("foreign-definition-symbol"))?;
        let count = array(function, "params")?.len();
        let implicit = usize::from(matches!(key.0, "instance_box_method" | "birth_constructor"));
        if count != key.3 as usize + implicit {
            return Err(error("definition-arity"));
        }
        if definitions.insert(key, *function).is_some() || !symbols.insert(symbol) {
            return Err(error("duplicate-definition"));
        }
    }
    for function in functions {
        let name = string(function, "name")?;
        validate_function(function, &definitions)
            .map_err(|reason| format!("{reason} function={name}"))?;
    }
    Ok(())
}

type Definitions<'a> = BTreeMap<(&'a str, &'a str, &'a str, u32), &'a Value>;

fn validate_function(value: &Value, definitions: &Definitions<'_>) -> Result<(), String> {
    let name = string(value, "name")?;
    let metadata = &value["metadata"];
    let contract = &metadata["canonical_transport_v1"];
    fields(
        contract,
        &[
            "signature",
            "entry_block",
            "next_value_id",
            "locals",
            "physical_param_carriers",
            "source_file",
            "line_number",
            "declared_params",
            "block_spans",
        ],
        &[],
    )?;
    let signature = &contract["signature"];
    fields(
        signature,
        &["name", "params", "return_type", "effects_bits"],
        &[],
    )?;
    if string(signature, "name")? != name {
        return Err(error("signature-name"));
    }
    let types = array(signature, "params")?
        .iter()
        .map(ty)
        .collect::<Result<Vec<_>, _>>()?;
    let params = array(value, "params")?
        .iter()
        .map(id_value)
        .collect::<Result<Vec<_>, _>>()?;
    if params.len() != types.len() {
        return Err(error("signature-arity"));
    }
    validate_carriers(contract, &types)?;
    nullable_string(contract, "source_file")?;
    if !contract["line_number"].is_null() {
        id(contract, "line_number")?;
    }
    for declaration in array(contract, "declared_params")? {
        fields(
            declaration,
            &["name", "declared_type_name", "implicit_receiver"],
            &[],
        )?;
        string(declaration, "name")?;
        nullable_string(declaration, "declared_type_name")?;
        boolean(declaration, "implicit_receiver")?;
    }
    let entry = BasicBlockId::new(id(contract, "entry_block")?);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.into(),
            params: types,
            return_type: ty(&signature["return_type"])?,
            effects: effects(signature)?,
        },
        entry,
    );
    function.blocks.clear();
    function.params = params.iter().copied().map(ValueId::new).collect();
    function.next_value_id = id(contract, "next_value_id")?;
    function.locals = array(contract, "locals")?
        .iter()
        .map(ty)
        .collect::<Result<_, _>>()?;
    let mut defined = BTreeSet::new();
    for param in &params {
        if !defined.insert(*param) {
            return Err(error("duplicate-parameter"));
        }
    }
    let mut used = BTreeSet::new();
    for block in array(value, "blocks")? {
        fields(block, &["id", "instructions"], &[])?;
        let block_id = BasicBlockId::new(id(block, "id")?);
        let mut bb = BasicBlock::new(block_id);
        let rows = array(block, "instructions")?;
        if rows.is_empty() {
            return Err(error("empty-block"));
        }
        for (index, row) in rows.iter().enumerate() {
            let instruction = instruction::parse(row, definitions)?;
            if matches!(
                instruction,
                crate::mir::MirInstruction::Return { .. }
                    | crate::mir::MirInstruction::ReturnFault { .. }
                    | crate::mir::MirInstruction::Invoke { .. }
                    | crate::mir::MirInstruction::Branch { .. }
                    | crate::mir::MirInstruction::Jump { .. }
            ) != (index + 1 == rows.len())
            {
                return Err(error("terminator-order"));
            }
            if let Some(dst) = instruction.dst_value() {
                if !defined.insert(dst.as_u32()) {
                    return Err(error("duplicate-destination"));
                }
            }
            used.extend(
                instruction
                    .used_values()
                    .iter()
                    .map(|operand| operand.as_u32()),
            );
            bb.add_instruction(instruction);
        }
        if function.blocks.insert(block_id, bb).is_some() {
            return Err(error("duplicate-block"));
        }
    }
    if !function.blocks.contains_key(&entry) {
        return Err(error("missing-entry"));
    }
    if defined
        .iter()
        .chain(&used)
        .any(|id| *id >= function.next_value_id)
        || used.iter().any(|id| !defined.contains(id))
    {
        return Err(error("value-bound-or-definition"));
    }
    let type_rows = metadata["value_types"]
        .as_object()
        .ok_or_else(|| error("value-types"))?;
    for (key, value) in type_rows {
        let id: u32 = key.parse().map_err(|_| error("value-type-id"))?;
        if key != &id.to_string() || id >= function.next_value_id {
            return Err(error("foreign-value-type"));
        }
        function
            .metadata
            .value_types
            .insert(ValueId::new(id), ty(value)?);
    }
    correspondence::validate(value, &function, definitions)?;
    validate_spans(contract, &function)?;
    correspondence::validate_return_env(contract, &function)?;
    crate::mir::MirVerifier::new_strict()
        .verify_function(&function)
        .map_err(|errors| format!("{}: {errors:?}", error("structural-verification")))?;
    Ok(())
}

fn validate_carriers(contract: &Value, types: &[crate::mir::MirType]) -> Result<(), String> {
    let Some(carriers) = contract["physical_param_carriers"].as_array() else {
        if contract["physical_param_carriers"].is_null() {
            return Ok(());
        }
        return Err(error("missing-physical-carriers"));
    };
    if carriers.len() != types.len() {
        return Err(error("carrier-arity"));
    }
    for (carrier, ty) in carriers.iter().zip(types) {
        if !matches!(
            ty,
            crate::mir::MirType::Integer | crate::mir::MirType::Unknown
        ) || !matches!(
            carrier.as_str(),
            Some("existing_callable_i64" | "u64_bits_on_i64" | "borrowed_tagged_value")
        ) {
            return Err(error("carrier-type"));
        }
    }
    Ok(())
}

fn validate_spans(contract: &Value, function: &MirFunction) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for row in array(contract, "block_spans")? {
        fields(
            row,
            &[
                "block",
                "instructions",
                "terminator",
                "return_env",
                "return_env_layout",
            ],
            &[],
        )?;
        let block_id = BasicBlockId::new(id(row, "block")?);
        if !seen.insert(block_id) {
            return Err(error("duplicate-span-block"));
        }
        let block = function
            .blocks
            .get(&block_id)
            .ok_or_else(|| error("foreign-span-block"))?;
        if array(row, "instructions")?.len() != block.instructions.len() {
            return Err(error("span-alignment"));
        }
        if array(row, "instructions")?.iter().any(Value::is_null) {
            return Err(error("missing-instruction-span"));
        }
        for span in array(row, "instructions")?
            .iter()
            .chain(std::iter::once(&row["terminator"]))
        {
            if span.is_null() {
                continue;
            }
            fields(span, &["start", "end", "line", "column"], &[])?;
            let start = span["start"].as_u64().ok_or_else(|| error("span-start"))?;
            let end = span["end"].as_u64().ok_or_else(|| error("span-end"))?;
            if start > end || span["line"].as_u64().is_none() || span["column"].as_u64().is_none() {
                return Err(error("span-range"));
            }
        }
    }
    if seen.len() != function.blocks.len() {
        return Err(error("missing-span-block"));
    }
    Ok(())
}
