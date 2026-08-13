use crate::mir::{EffectMask, MirInstruction, ValueId};
use serde_json::Value;

use super::helpers::require_u64;

fn id(node: &Value, key: &str, context: &str) -> Result<u32, String> {
    u32::try_from(require_u64(node, key, context)?)
        .map_err(|_| format!("{context} field '{key}' overflows u32"))
}

fn required_args(node: &Value) -> Result<Vec<ValueId>, String> {
    let values = node
        .get("args")
        .and_then(Value::as_array)
        .ok_or_else(|| "checked_callout missing args array".to_owned())?;
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            u32::try_from(
                value
                    .as_u64()
                    .ok_or_else(|| format!("checked_callout args[{index}] must be an integer"))?,
            )
            .map(ValueId::new)
            .map_err(|_| format!("checked_callout args[{index}] overflows u32"))
        })
        .collect()
}

pub(super) fn parse(op: &str, node: &Value) -> Result<MirInstruction, String> {
    match op {
        "checked_callout" => {
            let effects = u16::try_from(require_u64(node, "effects", "checked_callout effects")?)
                .map_err(|_| "checked_callout effects overflow u16".to_owned())?;
            Ok(MirInstruction::CheckedCallOut {
                site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(id(
                    node,
                    "site_id",
                    "checked_callout",
                )?),
                receiver: ValueId::new(id(node, "receiver", "checked_callout receiver")?),
                arguments: required_args(node)?,
                normal_landing: crate::mir::BasicBlockId::new(id(
                    node,
                    "normal",
                    "checked_callout normal landing",
                )?),
                fault_landing: crate::mir::BasicBlockId::new(id(
                    node,
                    "fault",
                    "checked_callout fault landing",
                )?),
                effects: EffectMask::from_bits(effects),
            })
        }
        "checked_callout_normal_result" => Ok(MirInstruction::CheckedCallOutNormalResult {
            site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(id(
                node,
                "site_id",
                "checked_callout normal result",
            )?),
            dst: ValueId::new(id(node, "dst", "checked_callout normal result dst")?),
        }),
        "checked_callout_end" => Ok(MirInstruction::CheckedCallOutEnd {
            site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(id(
                node,
                "site_id",
                "checked_callout end",
            )?),
            lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(id(
                node,
                "lease_slot",
                "checked_callout end lease slot",
            )?),
        }),
        "checked_callout_fault" => Ok(MirInstruction::CheckedCallOutFault {
            site_id: crate::mir::checked_callout::CheckedCallOutSiteIdV1(id(
                node,
                "site_id",
                "checked_callout fault",
            )?),
        }),
        other => Err(format!("unsupported checked callout op '{other}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn missing_checked_callout_landing_is_rejected() {
        let node = serde_json::json!({
            "op": "checked_callout",
            "site_id": 0,
            "receiver": 1,
            "args": [],
            "fault": 2,
            "effects": 1,
        });
        let error = parse("checked_callout", &node).unwrap_err();
        assert!(error.contains("normal landing"));
    }
}
