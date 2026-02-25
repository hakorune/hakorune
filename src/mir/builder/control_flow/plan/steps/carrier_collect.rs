//! Step: Carrier init collection (Pass 2 of carrier analysis).
//! (plan::steps SSOT)
//!
//! Input: carrier_vars (from recipe-specific Pass 1)
//! Output: carrier_inits (BTreeMap<String, ValueId>)
//! Fail-Fast: Missing variable → immediate error

use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Collect carrier init values from variable_map.
///
/// # Contract
/// - All carrier_vars MUST exist in builder.variable_ctx.variable_map
/// - Returns Err immediately if any variable is missing (Fail-Fast)
pub fn collect_carrier_inits<I>(
    builder: &MirBuilder,
    carrier_vars: I,
    err_tag: &str,
) -> Result<BTreeMap<String, ValueId>, String>
where
    I: IntoIterator<Item = String>,
{
    let mut carrier_inits = BTreeMap::new();
    for var in carrier_vars {
        let Some(&init_val) = builder.variable_ctx.variable_map.get(&var) else {
            return Err(format!("{}: carrier {} missing init", err_tag, var));
        };
        carrier_inits.insert(var, init_val);
    }
    Ok(carrier_inits)
}
