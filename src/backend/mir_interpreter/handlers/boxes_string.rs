use super::string_method_helpers::{
    parse_index_of_args, parse_last_index_of_args, parse_substring_args, ArgParsePolicy,
};
use super::*;
use crate::boxes::string_ops;
use crate::config::env;
use crate::config::env::string_codepoint_mode;
use crate::runtime::get_global_ring0;

pub(super) fn try_handle_string_box(
    this: &mut MirInterpreter,
    dst: Option<ValueId>,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Result<bool, VMError> {
    if env::env_bool("NYASH_VM_TRACE") {
        get_global_ring0().log.debug(&format!(
            "[vm-trace] try_handle_string_box(method={})",
            method
        ));
    }
    let recv = this.reg_load(box_val)?;
    // Ultra-fast path: raw VM string receiver for length/size (no boxing at all)
    if (method == "length" || method == "size") && env::env_bool("NYASH_VM_FAST") {
        if let VMValue::String(ref raw) = recv {
            let use_cp = string_codepoint_mode();
            let n = if use_cp {
                raw.chars().count() as i64
            } else {
                raw.len() as i64
            };
            this.write_result(dst, VMValue::Integer(n));
            return Ok(true);
        }
    }
    // Handle ONLY when the receiver is actually a string.
    // Do NOT coerce arbitrary boxes to StringBox (e.g., ArrayBox.length()).
    let sb_norm_opt: Option<crate::box_trait::StringBox> = match recv.clone() {
        VMValue::String(s) => Some(crate::box_trait::StringBox::new(s)),
        VMValue::BoxRef(b) => {
            if b.as_any()
                .downcast_ref::<crate::box_trait::StringBox>()
                .is_some()
            {
                Some(b.to_string_box())
            } else {
                None
            }
        }
        _ => None,
    };
    let Some(sb_norm) = sb_norm_opt else {
        return Ok(false);
    };

    let trace = crate::config::env::dev_provider_trace()
        && matches!(
            sb_norm.value.as_str(),
            "-42" | "0123456789" | "-" | "4" | "2"
        );
    if trace {
        get_global_ring0().log.debug(&format!(
            "[provider/trace][string_box] enter method={} recv={:?} argc={}",
            method,
            sb_norm.value,
            args.len()
        ));
    }
    // Only handle known string methods here (receiver is confirmed string)
    match method {
        "length" | "size" => {
            // Bench/profile fast path: return VMValue::Integer directly (avoid boxing overhead)
            if env::env_bool("NYASH_VM_FAST") {
                let use_cp = string_codepoint_mode();
                let n = if use_cp {
                    sb_norm.value.chars().count() as i64
                } else {
                    sb_norm.value.len() as i64
                };
                if trace {
                    get_global_ring0().log.debug(&format!(
                        "[provider/trace][string_box] length/size fast use_cp={} -> {}",
                        use_cp, n
                    ));
                }
                this.write_result(dst, VMValue::Integer(n));
                return Ok(true);
            }
            let ret = sb_norm.length();
            if trace {
                let use_cp = string_codepoint_mode();
                let n_dbg = if use_cp {
                    sb_norm.value.chars().count() as i64
                } else {
                    sb_norm.value.len() as i64
                };
                get_global_ring0().log.debug(&format!(
                    "[provider/trace][string_box] length/size slow use_cp={} -> {}",
                    use_cp, n_dbg
                ));
            }
            this.write_result(dst, VMValue::from_nyash_box(ret));
            return Ok(true);
        }
        "replace" => {
            // replace(old, new) -> string with first occurrence replaced (Rust replace = all; match Core minimal
            this.validate_args_exact("replace", args, 2)?;
            let old_s = this.reg_load(args[0])?.to_string();
            let new_s = this.reg_load(args[1])?.to_string();
            // Core policy: replace only the first occurrence
            let out = if let Some(pos) = sb_norm.value.find(&old_s) {
                let mut s = String::with_capacity(sb_norm.value.len() + new_s.len());
                s.push_str(&sb_norm.value[..pos]);
                s.push_str(&new_s);
                s.push_str(&sb_norm.value[pos + old_s.len()..]);
                s
            } else {
                sb_norm.value.clone()
            };
            this.write_result(
                dst,
                VMValue::from_nyash_box(Box::new(crate::box_trait::StringBox::new(out))),
            );
            return Ok(true);
        }
        "trim" => {
            let ret = sb_norm.trim();
            this.write_result(dst, VMValue::from_nyash_box(ret));
            return Ok(true);
        }
        "indexOf" => {
            let (needle, start) = parse_index_of_args(
                this,
                args,
                ArgParsePolicy::STRICT,
                "indexOf expects 1 or 2 args (search [, fromIndex])",
            )?;
            let mode = string_ops::index_mode_from_env();
            let idx = string_ops::index_of(&sb_norm.value, &needle, start, mode);
            if trace {
                get_global_ring0().log.debug(&format!(
                    "[provider/trace][string_box] indexOf needle={:?} start={:?} mode={:?} -> {}",
                    needle, start, mode, idx
                ));
            }
            this.write_result(dst, VMValue::Integer(idx));
            return Ok(true);
        }
        "contains" => {
            // contains(search) -> boolean (true if found, false otherwise)
            // Implemented as indexOf(search) >= 0
            this.validate_args_exact("contains", args, 1)?;
            let needle = this.reg_load(args[0])?.to_string();
            let found = sb_norm.value.contains(&needle);
            this.write_result(dst, VMValue::Bool(found));
            return Ok(true);
        }
        "lastIndexOf" => {
            let needle = parse_last_index_of_args(
                this,
                args,
                ArgParsePolicy::STRICT,
                "lastIndexOf requires 1 argument",
            )?;
            let mode = string_ops::index_mode_from_env();
            let idx = string_ops::last_index_of(&sb_norm.value, &needle, mode);
            if trace {
                get_global_ring0().log.debug(&format!(
                    "[provider/trace][string_box] lastIndexOf needle={:?} mode={:?} -> {}",
                    needle, mode, idx
                ));
            }
            this.write_result(dst, VMValue::Integer(idx));
            return Ok(true);
        }
        "stringify" => {
            // JSON-style stringify for strings: quote and escape common characters
            let mut quoted = String::with_capacity(sb_norm.value.len() + 2);
            quoted.push('"');
            for ch in sb_norm.value.chars() {
                match ch {
                    '"' => quoted.push_str("\\\""),
                    '\\' => quoted.push_str("\\\\"),
                    '\n' => quoted.push_str("\\n"),
                    '\r' => quoted.push_str("\\r"),
                    '\t' => quoted.push_str("\\t"),
                    c if c.is_control() => quoted.push(' '),
                    c => quoted.push(c),
                }
            }
            quoted.push('"');
            this.write_result(
                dst,
                VMValue::from_nyash_box(Box::new(crate::box_trait::StringBox::new(quoted))),
            );
            return Ok(true);
        }
        "substring" => {
            let (start, end) = parse_substring_args(
                this,
                args,
                ArgParsePolicy::STRICT,
                "substring expects 1 or 2 args (start [, end])",
            )?;
            let mode = string_ops::index_mode_from_env();
            let sub = string_ops::substring(&sb_norm.value, start, end, mode);
            if trace {
                get_global_ring0().log.debug(&format!(
                    "[provider/trace][string_box] substring start={} end={:?} mode={:?} -> {:?}",
                    start, end, mode, sub
                ));
            }
            this.write_result(
                dst,
                VMValue::from_nyash_box(Box::new(crate::box_trait::StringBox::new(sub))),
            );
            return Ok(true);
        }
        "concat" => {
            this.validate_args_exact("concat", args, 1)?;
            let rhs = this.reg_load(args[0])?;
            let new_s = format!("{}{}", sb_norm.value, rhs.to_string());
            this.write_result(
                dst,
                VMValue::from_nyash_box(Box::new(crate::box_trait::StringBox::new(new_s))),
            );
            return Ok(true);
        }
        "is_digit_char" => {
            // Accept either 0-arg (use first char of receiver) or 1-arg (string/char to test)
            let ch_opt = if args.is_empty() {
                sb_norm.value.chars().next()
            } else if args.len() == 1 {
                let s = this.reg_load(args[0])?.to_string();
                s.chars().next()
            } else {
                return Err(this.err_invalid("is_digit_char expects 0 or 1 arg"));
            };
            let is_digit = ch_opt.map(|c| c.is_ascii_digit()).unwrap_or(false);
            this.write_result(dst, VMValue::Bool(is_digit));
            return Ok(true);
        }
        "is_hex_digit_char" => {
            let ch_opt = if args.is_empty() {
                sb_norm.value.chars().next()
            } else if args.len() == 1 {
                let s = this.reg_load(args[0])?.to_string();
                s.chars().next()
            } else {
                return Err(this.err_invalid("is_hex_digit_char expects 0 or 1 arg"));
            };
            let is_hex = ch_opt.map(|c| c.is_ascii_hexdigit()).unwrap_or(false);
            this.write_result(dst, VMValue::Bool(is_hex));
            return Ok(true);
        }
        _ => {}
    }
    Ok(false)
}
