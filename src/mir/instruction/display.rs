//! Display implementation for MIR Instructions
//!
//! Provides human-readable string representation of MIR instructions for debugging and analysis.

use crate::mir::instruction::MirInstruction;
use crate::mir::printer_helpers::format_call_target;
use crate::mir::ValueId;
use std::fmt;

impl fmt::Display for MirInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirInstruction::Invoke {
                operation,
                fault_frame,
                normal_landing,
                fault_landing,
            } => {
                write!(
                    f,
                    "invoke {:?} frame={} normal={:?} fault={:?}",
                    operation, fault_frame, normal_landing, fault_landing
                )
            }
            MirInstruction::InvokeNormalResult { invoke_block, dst } => {
                write!(f, "{} = invoke.normal {:?}", dst, invoke_block)
            }
            MirInstruction::ReturnFault { fault_frame } => {
                write!(f, "return.fault {}", fault_frame)
            }
            MirInstruction::Const { dst, value } => {
                write!(f, "{} = const {}", dst, value)
            }
            MirInstruction::BinOp { dst, op, lhs, rhs } => {
                write!(f, "{} = {} {:?} {}", dst, lhs, op, rhs)
            }
            MirInstruction::UnaryOp { dst, op, operand } => {
                write!(f, "{} = {:?} {}", dst, op, operand)
            }
            MirInstruction::Compare { dst, op, lhs, rhs } => {
                write!(f, "{} = {} {:?} {}", dst, lhs, op, rhs)
            }
            MirInstruction::Load { dst, ptr } => {
                write!(f, "{} = load {}", dst, ptr)
            }
            MirInstruction::Store { value, ptr } => {
                write!(f, "store {} -> {}", value, ptr)
            }
            MirInstruction::FieldGet {
                dst, base, field, ..
            } => {
                write!(f, "{} = field.get {} .{}", dst, base, field)
            }
            MirInstruction::FieldSet {
                base, field, value, ..
            } => {
                write!(f, "field.set {} .{} = {}", base, field, value)
            }
            MirInstruction::VariantMake {
                dst,
                enum_name,
                variant,
                tag,
                payload,
                ..
            } => {
                if let Some(payload) = payload {
                    write!(
                        f,
                        "{} = variant.make {}::{} tag={} payload={}",
                        dst, enum_name, variant, tag, payload
                    )
                } else {
                    write!(
                        f,
                        "{} = variant.make {}::{} tag={}",
                        dst, enum_name, variant, tag
                    )
                }
            }
            MirInstruction::VariantTag {
                dst,
                value,
                enum_name,
            } => write!(f, "{} = variant.tag {} as {}", dst, value, enum_name),
            MirInstruction::VariantProject {
                dst,
                value,
                enum_name,
                variant,
                tag,
                ..
            } => write!(
                f,
                "{} = variant.project {} as {}::{} tag={}",
                dst, value, enum_name, variant, tag
            ),
            MirInstruction::Call(call) => {
                let call_display =
                    format_call_target(Some(&call.callee), ValueId::INVALID, &call.args);
                if let Some(dst) = call.dst {
                    write!(f, "{} = {}; effects: {}", dst, call_display, call.effects)
                } else {
                    write!(f, "{}; effects: {}", call_display, call.effects)
                }
            }
            MirInstruction::LegacyCallV0 {
                dst,
                func,
                callee,
                args,
                effects,
            } => {
                let call_display = format_call_target(callee.as_ref(), *func, args);
                if let Some(dst) = dst {
                    write!(f, "{} = {}; effects: {}", dst, call_display, effects)
                } else {
                    write!(f, "{}; effects: {}", call_display, effects)
                }
            }
            MirInstruction::Return { value } => {
                if let Some(value) = value {
                    write!(f, "ret {}", value)
                } else {
                    write!(f, "ret void")
                }
            }
            MirInstruction::CheckedCallOutEnd {
                site_id,
                lease_slot,
            } => write!(
                f,
                "checked_callout.end site={} lease={}",
                site_id.as_u32(),
                lease_slot.as_u32()
            ),
            MirInstruction::CheckedCallOutFault { site_id } => {
                write!(f, "checked_callout.fault site={}", site_id.as_u32())
            }
            // Phase 287: Lifecycle management
            MirInstruction::KeepAlive { values } => {
                write!(f, "keepalive")?;
                for v in values {
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            MirInstruction::CopyOwned { dst, src } => {
                write!(f, "{} = copy_owned {}", dst, src)
            }
            MirInstruction::DestroyOwned { value } => {
                write!(f, "destroy_owned {}", value)
            }
            MirInstruction::ReleaseStrong { values } => {
                write!(f, "release_strong")?;
                for v in values {
                    write!(f, " {}", v)?;
                }
                Ok(())
            }
            MirInstruction::MemOp {
                region,
                kind,
                dst,
                operands,
                access,
                effects,
            } => {
                if let Some(dst) = dst {
                    write!(f, "{} = memop {} r{}", dst, kind.display_name(), region.0)?;
                } else {
                    write!(f, "memop {} r{}", kind.display_name(), region.0)?;
                }
                write!(f, "(")?;
                for (idx, operand) in operands.iter().enumerate() {
                    if idx > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", operand)?;
                }
                if let Some(access) = access {
                    if let Some(table_id) = &access.table_id {
                        write!(f, " table={}", table_id)?;
                    }
                    if let Some(layout_id) = &access.layout_id {
                        write!(f, " layout={}", layout_id)?;
                    }
                    if let Some(field_id) = &access.field_id {
                        write!(f, " field={}", field_id)?;
                    }
                }
                write!(f, "); effects: {}", effects)
            }
            MirInstruction::PinnedTextOp { dst, plan, kind } => {
                write!(
                    f,
                    "{} = pinned_text.{} plan={} stamp={}",
                    dst,
                    kind.tag(),
                    plan.index(),
                    plan.stamp()
                )
            }
            _ => write!(f, "{:?}", self), // Fallback for other instructions
        }
    }
}
