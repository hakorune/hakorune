use crate::mir::function::MirModule;
use serde_json::Value;

#[path = "mir_json_v0/array_write.rs"]
mod array_write;
#[path = "mir_json_v0/call.rs"]
mod call;
#[path = "mir_json_v0/catalog.rs"]
mod catalog;
#[path = "mir_json_v0/checked_callout.rs"]
mod checked_callout;
#[path = "mir_json_v0/module.rs"]
mod function_lowering;
#[path = "mir_json_v0/helpers.rs"]
mod helpers;
#[path = "mir_json_v0/lifecycle.rs"]
mod lifecycle;
#[cfg(test)]
#[path = "mir_json_v0/tests.rs"]
mod tests;

pub fn parse_mir_v0_to_module(json: &str) -> Result<MirModule, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("invalid JSON: {}", e))?;
    let functions = value
        .get("functions")
        .and_then(|f| f.as_array())
        .ok_or_else(|| "JSON missing functions array".to_string())?;

    let mut module = MirModule::new("mir_json_v0".to_string());
    module.metadata.static_table_contract_specs =
        parse_static_table_contract_specs(&module.name, value.get("static_table_contract_specs"))?;
    module.metadata.static_data_plans = parse_static_data_plans(value.get("static_data_plans"))?;

    function_lowering::lower_functions(functions, &mut module)?;

    // Canonicalize legacy callsites from selfhost JSON route before VM preflight.
    // This keeps runtime acceptance aligned with MCL lane (BoxCall/ExternCall -> Call(callee=...)).
    let _ = crate::mir::passes::callsite_canonicalize::canonicalize_for_site(
        &mut module,
        crate::mir::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirJsonV0Loader,
    );

    Ok(module)
}

fn parse_static_data_plans(
    value: Option<&Value>,
) -> Result<Vec<crate::mir::function::StaticDataPlan>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let rows = value
        .as_array()
        .ok_or_else(|| "static_data_plans must be an array".to_string())?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let source_name = row
            .get("source_name")
            .and_then(Value::as_str)
            .ok_or_else(|| "static_data_plans row missing source_name".to_string())?
            .to_string();
        let symbol = row
            .get("symbol")
            .and_then(Value::as_str)
            .ok_or_else(|| "static_data_plans row missing symbol".to_string())?
            .to_string();
        let element = row
            .get("element")
            .and_then(Value::as_str)
            .ok_or_else(|| "static_data_plans row missing element".to_string())?
            .to_string();
        let align = row
            .get("align")
            .and_then(Value::as_u64)
            .ok_or_else(|| "static_data_plans row missing align".to_string())?
            as u32;
        let linkage = row
            .get("linkage")
            .and_then(Value::as_str)
            .unwrap_or("private")
            .to_string();
        let unnamed_addr = row
            .get("unnamed_addr")
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let values = row
            .get("values")
            .and_then(Value::as_array)
            .ok_or_else(|| "static_data_plans row missing values".to_string())?
            .iter()
            .map(|item| {
                item.as_u64()
                    .ok_or_else(|| "static_data_plans values must be unsigned integers".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;
        out.push(crate::mir::function::StaticDataPlan {
            source_name,
            symbol,
            element,
            align,
            linkage,
            unnamed_addr,
            values,
        });
    }
    Ok(out)
}

fn parse_static_table_contract_specs(
    module_name: &str,
    value: Option<&Value>,
) -> Result<Vec<crate::mir::function::StaticTableContractSpec>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let rows = value
        .as_array()
        .ok_or_else(|| "static_table_contract_specs must be an array".to_string())?;
    rows.iter()
        .map(|row| {
            let row_module = row
                .get("module_name")
                .and_then(Value::as_str)
                .ok_or_else(|| "static table spec missing module_name".to_string())?;
            if row_module != module_name {
                return Err("static table spec module_name drift".to_string());
            }
            let declaration_name = row
                .get("declaration_name")
                .and_then(Value::as_str)
                .ok_or_else(|| "static table spec missing declaration_name".to_string())?
                .to_string();
            let element = row
                .get("element")
                .and_then(Value::as_str)
                .ok_or_else(|| "static table spec missing element".to_string())?;
            if element != "u16" {
                return Err("static table spec supports only u16".to_string());
            }
            let values = row
                .get("values")
                .and_then(Value::as_array)
                .ok_or_else(|| "static table spec missing values".to_string())?
                .iter()
                .map(|value| {
                    value
                        .as_u64()
                        .and_then(|value| u16::try_from(value).ok())
                        .ok_or_else(|| "static table spec value outside u16".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(crate::mir::function::StaticTableContractSpec {
                table_id: crate::mir::function::StaticTableId {
                    module_name: row_module.to_string(),
                    declaration_name: declaration_name.clone(),
                },
                diagnostic_name: row
                    .get("diagnostic_name")
                    .and_then(Value::as_str)
                    .unwrap_or(&declaration_name)
                    .to_string(),
                element: crate::mir::function::StaticElementType::U16,
                values,
            })
        })
        .collect()
}
