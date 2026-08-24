use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{ConstValue, ValueId};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(super) struct JsonV0FunctionCatalog {
    entries: BTreeMap<ValueId, LegacyConstEvidence>,
    parameter_ids: BTreeSet<ValueId>,
}

#[derive(Debug, Clone)]
enum LegacyConstEvidence {
    String(String),
    NonString,
    Ambiguous,
}

impl JsonV0FunctionCatalog {
    pub(super) fn from_function(
        function: &Value,
        function_name: &str,
        parameter_ids: &[u32],
    ) -> Result<Self, String> {
        let blocks = function
            .get("blocks")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("function '{}' missing blocks array", function_name))?;
        let parameter_ids = parameter_ids
            .iter()
            .copied()
            .map(ValueId::new)
            .collect::<BTreeSet<_>>();
        let mut entries = BTreeMap::new();

        for (block_index, block) in blocks.iter().enumerate() {
            let instructions = block
                .get("instructions")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    format!(
                        "function '{}' block {} missing instructions array",
                        function_name, block_index
                    )
                })?;
            for (instruction_index, instruction) in instructions.iter().enumerate() {
                if instruction.get("op").and_then(Value::as_str) != Some("const") {
                    continue;
                }
                let raw_dst = instruction
                    .get("dst")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| {
                        format!(
                            "function '{}' const[{}:{}] missing dst",
                            function_name, block_index, instruction_index
                        )
                    })?;
                let dst = u32::try_from(raw_dst).map_err(|_| {
                    format!(
                        "function '{}' const[{}:{}] dst out of range: {}",
                        function_name, block_index, instruction_index, raw_dst
                    )
                })?;
                let value_obj = instruction.get("value").ok_or_else(|| {
                    format!(
                        "function '{}' const[{}:{}] missing value",
                        function_name, block_index, instruction_index
                    )
                })?;
                let value = super::helpers::parse_const_value(value_obj)?;
                let value_id = ValueId::new(dst);
                if value_id == ValueId::INVALID {
                    return Err(format!(
                        "function '{}' const[{}:{}] uses invalid dst ValueId",
                        function_name, block_index, instruction_index
                    ));
                }
                if parameter_ids.contains(&value_id) {
                    return Err(format!(
                        "function '{}' const dst conflicts with parameter ValueId {}",
                        function_name, value_id
                    ));
                }
                let evidence = match value {
                    ConstValue::String(text) => LegacyConstEvidence::String(text),
                    _ => LegacyConstEvidence::NonString,
                };
                entries
                    .entry(value_id)
                    .and_modify(|existing| *existing = LegacyConstEvidence::Ambiguous)
                    .or_insert(evidence);
            }
        }

        Ok(Self {
            entries,
            parameter_ids,
        })
    }

    pub(super) fn resolve(&self, value_id: ValueId) -> Result<&str, String> {
        if value_id == ValueId::INVALID {
            return Err("legacy call func cannot use ValueId::INVALID".to_string());
        }
        if self.parameter_ids.contains(&value_id) {
            return Err(format!(
                "legacy call func ValueId {} refers to a parameter, not a Const(String)",
                value_id
            ));
        }
        match self.entries.get(&value_id) {
            Some(LegacyConstEvidence::String(text)) => Ok(text.as_str()),
            Some(LegacyConstEvidence::NonString) => Err(format!(
                "legacy call func ValueId {} is not a Const(String)",
                value_id
            )),
            Some(LegacyConstEvidence::Ambiguous) => Err(format!(
                "legacy call func ValueId {} has multiple Const definitions",
                value_id
            )),
            None => Err(format!(
                "legacy call func ValueId {} has no function-local Const(String) definition",
                value_id
            )),
        }
    }
}
