use serde_json::Value;
use std::collections::HashMap;

/// Minimal hv1 inline executor for tests (no Nyash parser).
/// Supports a very small subset needed by canaries:
/// - const i64
/// - mir_call(Constructor ArrayBox)
/// - mir_call(Method ArrayBox.push/size/len/length) with optional per-receiver state
/// - ret (by register id)
pub fn run_json_v1_inline(json: &str) -> i32 {
    // Optional execution trace (stderr) for debugging
    if crate::config::env::env_bool("HAKO_TRACE_EXECUTION") {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[trace] executor: hv1_inline (rust)"));
    }
    // Parse JSON
    let v: Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return 1,
    };
    // schema_version 1.x required
    match v.get("schema_version").and_then(Value::as_str) {
        Some(s) if s.starts_with('1') => {}
        _ => return 1,
    }
    // Fetch first function and build block map
    let functions = match v.get("functions").and_then(Value::as_array) {
        Some(a) => a,
        None => return 1,
    };
    let func = match functions.get(0) {
        Some(f) => f,
        None => return 1,
    };
    let blocks = match func.get("blocks").and_then(Value::as_array) {
        Some(a) => a,
        None => return 1,
    };
    let mut bmap: HashMap<i64, &Vec<Value>> = HashMap::new();
    for b in blocks {
        if let (Some(id), Some(insts)) = (
            b.get("id").and_then(Value::as_i64),
            b.get("instructions").and_then(Value::as_array),
        ) {
            bmap.insert(id, insts);
        }
    }

    // Registers and simple method state
    let mut regs: HashMap<i64, i64> = HashMap::new();
    let mut str_regs: HashMap<i64, String> = HashMap::new(); // Track string values loaded via const
    let sizestate = crate::config::env::env_bool("HAKO_VM_MIRCALL_SIZESTATE");
    let per_recv = crate::config::env::env_bool("HAKO_VM_MIRCALL_SIZESTATE_PER_RECV");
    let value_state = crate::config::env::env_bool("HAKO_VM_MIRCALL_VALUESTATE");
    // Keep Array and Map length states separate to avoid cross‑box interference
    let mut len_global_arr: i64 = 0;
    let mut len_by_recv_arr: HashMap<i64, i64> = HashMap::new();
    let mut len_global_map: i64 = 0;
    let mut len_by_recv_map: HashMap<i64, i64> = HashMap::new();
    // Map dup-key detection: track which keys have been set (per-receiver or global)
    let mut map_keys_global: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut map_keys_by_recv: HashMap<i64, std::collections::HashSet<String>> = HashMap::new();
    // Map value storage (when value_state=1)
    let mut map_vals_global: HashMap<String, i64> = HashMap::new();
    let mut map_vals_by_recv: HashMap<i64, HashMap<String, i64>> = HashMap::new();

    fn is_size_alias(name: &str) -> bool {
        matches!(name, "size" | "len" | "length")
    }

    // Simple CFG interpreter with limited ops (const/compare/phi/branch/jump/mir_call/ret)
    let mut curr: i64 = 0; // assume entry block id 0
    let mut prev: i64 = -1;
    let mut steps: i32 = 0;
    'outer: loop {
        if steps > 10000 {
            return 1;
        }
        steps += 1;
        let insts = match bmap.get(&curr) {
            Some(v) => *v,
            None => return 1,
        };
        let mut ip = 0usize;
        while ip < insts.len() {
            let inst = &insts[ip];
            ip += 1;
            let op = match inst.get("op").and_then(Value::as_str) {
                Some(s) => s,
                None => return 1,
            };
            match op {
                "binop" => {
                    // Minimal integer binops
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let lhs = inst
                        .get("lhs")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let rhs = inst
                        .get("rhs")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let opn = inst.get("operation").and_then(Value::as_str).unwrap_or("");
                    let out = match opn {
                        "+" => lhs.wrapping_add(rhs),
                        "-" => lhs.wrapping_sub(rhs),
                        "*" => lhs.wrapping_mul(rhs),
                        "/" => {
                            if rhs == 0 {
                                0
                            } else {
                                lhs.wrapping_div(rhs)
                            }
                        }
                        "%" => {
                            if rhs == 0 {
                                0
                            } else {
                                lhs.wrapping_rem(rhs)
                            }
                        }
                        "&" => lhs & rhs,
                        "|" => lhs | rhs,
                        "^" => lhs ^ rhs,
                        "<<" => lhs.wrapping_shl((rhs as u32).min(63)),
                        ">>" => ((lhs as i64) >> (rhs as u32).min(63)) as i64,
                        _ => 0,
                    };
                    regs.insert(dst, out);
                }
                "unop" => {
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let src = inst
                        .get("src")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let kind = inst.get("kind").and_then(Value::as_str).unwrap_or("");
                    let out = match kind {
                        "neg" => src.wrapping_neg(),
                        "not" => {
                            if src == 0 {
                                1
                            } else {
                                0
                            }
                        }
                        "bitnot" => !src,
                        _ => 0,
                    };
                    regs.insert(dst, out);
                }
                "copy" => {
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let srcv = inst
                        .get("src")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    regs.insert(dst, srcv);
                }
                "typeop" => {
                    // Minimal TypeOp support for PRIMARY reps
                    // Fields: operation ("check"|"is"|"cast"), src (vid), target_type (str), dst (vid)
                    let operation = inst
                        .get("operation")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_lowercase();
                    let src = match inst.get("src").and_then(Value::as_i64) {
                        Some(s) => s,
                        None => return 1,
                    };
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let target = inst
                        .get("target_type")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_lowercase();

                    let sval = regs.get(&src).cloned();
                    let is_integer = sval.is_some(); // hv1 inline stores i64 only → integer
                    if operation == "check" || operation == "is" {
                        let out: i64 = if target == "i64" || target == "int" || target == "integer"
                        {
                            if is_integer {
                                1
                            } else {
                                0
                            }
                        } else if target == "bool" {
                            // Inline model uses integer registers; treat 0/1 as bool when present
                            if let Some(v) = sval {
                                if v == 0 || v == 1 {
                                    1
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        } else if target == "string" {
                            0 // no string registers in inline model
                        } else {
                            0
                        };
                        regs.insert(dst, out);
                    } else {
                        // cast/as: pass-through (MVP)
                        regs.insert(dst, sval.unwrap_or(0));
                    }
                }
                "const" => {
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    // Prefer i64 numeric constants; otherwise stub non-numeric to 0 for inline path
                    if let Some(n) = inst
                        .get("value")
                        .and_then(|vv| vv.get("value"))
                        .and_then(Value::as_i64)
                    {
                        regs.insert(dst, n);
                    } else if let Some(s) = inst
                        .get("value")
                        .and_then(|vv| vv.get("value"))
                        .and_then(Value::as_str)
                    {
                        // Store string value for Map key tracking
                        str_regs.insert(dst, s.to_string());
                        regs.insert(dst, 0); // Also set numeric register to 0 for compatibility
                    } else {
                        regs.insert(dst, 0);
                    }
                }
                "compare" => {
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let lhs = inst
                        .get("lhs")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let rhs = inst
                        .get("rhs")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let cmp = inst.get("cmp").and_then(Value::as_str).unwrap_or("");
                    let res = match cmp {
                        "Gt" => (lhs > rhs) as i64,
                        "Ge" => (lhs >= rhs) as i64,
                        "Lt" => (lhs < rhs) as i64,
                        "Le" => (lhs <= rhs) as i64,
                        "Eq" => (lhs == rhs) as i64,
                        "Ne" => (lhs != rhs) as i64,
                        _ => 0,
                    };
                    regs.insert(dst, res);
                }
                "mir_call" => {
                    // Support both nested shape {"mir_call":{"callee":...}} and flat shape {"callee":...}
                    let mc = inst.get("mir_call");
                    let callee = if let Some(m) = mc {
                        m.get("callee")
                    } else {
                        inst.get("callee")
                    };
                    let Some(callee) = callee else { return 1 };
                    let ctype = callee.get("type").and_then(Value::as_str).unwrap_or("");
                    match ctype {
                        // Constructor: just create and optionally write dst=0
                        "Constructor" => {
                            if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                regs.insert(dst, 0);
                            }
                            continue;
                        }
                        // ArrayBox methods we model inline
                        "Method" => {
                            let bname =
                                callee.get("box_name").and_then(Value::as_str).unwrap_or("");
                            if bname == "ArrayBox" {
                                let method =
                                    callee.get("method").and_then(Value::as_str).unwrap_or("");
                                let recv =
                                    callee.get("receiver").and_then(Value::as_i64).unwrap_or(-1);
                                if method == "push" {
                                    if sizestate {
                                        if per_recv {
                                            let e = len_by_recv_arr.entry(recv).or_insert(0);
                                            *e += 1;
                                        } else {
                                            len_global_arr += 1;
                                        }
                                    }
                                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                        regs.insert(dst, 0);
                                    }
                                    continue;
                                }
                                if is_size_alias(method) {
                                    let value = if sizestate {
                                        if per_recv {
                                            *len_by_recv_arr.get(&recv).unwrap_or(&0)
                                        } else {
                                            len_global_arr
                                        }
                                    } else {
                                        0
                                    };
                                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                        regs.insert(dst, value);
                                    }
                                    continue;
                                }
                                // unsupported method on ArrayBox → stub 0
                                if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                    regs.insert(dst, 0);
                                }
                                continue;
                            }
                            if bname == "MapBox" {
                                let method =
                                    callee.get("method").and_then(Value::as_str).unwrap_or("");
                                let recv =
                                    callee.get("receiver").and_then(Value::as_i64).unwrap_or(-1);
                                if method == "set" {
                                    // Extract key from first arg (register containing string value)
                                    let args = if let Some(mc) = mc {
                                        mc.get("args")
                                    } else {
                                        inst.get("args")
                                    };
                                    let key_str =
                                        if let Some(args_arr) = args.and_then(Value::as_array) {
                                            if let Some(key_reg) =
                                                args_arr.get(0).and_then(Value::as_i64)
                                            {
                                                // Look up string value from str_regs
                                                str_regs.get(&key_reg).cloned().unwrap_or_default()
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            String::new()
                                        };

                                    if sizestate && !key_str.is_empty() {
                                        let is_new_key = if per_recv {
                                            let keys = map_keys_by_recv
                                                .entry(recv)
                                                .or_insert_with(std::collections::HashSet::new);
                                            keys.insert(key_str.clone())
                                        } else {
                                            map_keys_global.insert(key_str.clone())
                                        };

                                        // Only increment size if this is a new key
                                        if is_new_key {
                                            if per_recv {
                                                let e = len_by_recv_map.entry(recv).or_insert(0);
                                                *e += 1;
                                            } else {
                                                len_global_map += 1;
                                            }
                                        }
                                    }

                                    // value_state: store the value
                                    if value_state && !key_str.is_empty() {
                                        if let Some(args_arr) = args.and_then(Value::as_array) {
                                            if let Some(val_reg) =
                                                args_arr.get(1).and_then(Value::as_i64)
                                            {
                                                let val = *regs.get(&val_reg).unwrap_or(&0);
                                                if per_recv {
                                                    let vals = map_vals_by_recv
                                                        .entry(recv)
                                                        .or_insert_with(HashMap::new);
                                                    vals.insert(key_str.clone(), val);
                                                } else {
                                                    map_vals_global.insert(key_str.clone(), val);
                                                }
                                            }
                                        }
                                    }

                                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                        regs.insert(dst, 0);
                                    }
                                    continue;
                                }
                                if method == "get" && value_state {
                                    // Extract key and retrieve value
                                    let args = if let Some(mc) = mc {
                                        mc.get("args")
                                    } else {
                                        inst.get("args")
                                    };
                                    let key_str =
                                        if let Some(args_arr) = args.and_then(Value::as_array) {
                                            if let Some(key_reg) =
                                                args_arr.get(0).and_then(Value::as_i64)
                                            {
                                                str_regs.get(&key_reg).cloned().unwrap_or_default()
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            String::new()
                                        };

                                    if !key_str.is_empty() {
                                        let val = if per_recv {
                                            map_vals_by_recv
                                                .get(&recv)
                                                .and_then(|m| m.get(&key_str))
                                                .cloned()
                                                .unwrap_or(0)
                                        } else {
                                            *map_vals_global.get(&key_str).unwrap_or(&0)
                                        };
                                        if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                            regs.insert(dst, val);
                                        }
                                    }
                                    continue;
                                }
                                if method == "has" && value_state {
                                    // Check if key exists
                                    let args = if let Some(mc) = mc {
                                        mc.get("args")
                                    } else {
                                        inst.get("args")
                                    };
                                    let key_str =
                                        if let Some(args_arr) = args.and_then(Value::as_array) {
                                            if let Some(key_reg) =
                                                args_arr.get(0).and_then(Value::as_i64)
                                            {
                                                str_regs.get(&key_reg).cloned().unwrap_or_default()
                                            } else {
                                                String::new()
                                            }
                                        } else {
                                            String::new()
                                        };

                                    let has = if !key_str.is_empty() {
                                        if per_recv {
                                            map_vals_by_recv
                                                .get(&recv)
                                                .map(|m| m.contains_key(&key_str))
                                                .unwrap_or(false)
                                        } else {
                                            map_vals_global.contains_key(&key_str)
                                        }
                                    } else {
                                        false
                                    };

                                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                        regs.insert(dst, if has { 1 } else { 0 });
                                    }
                                    continue;
                                }
                                if is_size_alias(method) {
                                    let value = if sizestate {
                                        if per_recv {
                                            *len_by_recv_map.get(&recv).unwrap_or(&0)
                                        } else {
                                            len_global_map
                                        }
                                    } else {
                                        0
                                    };
                                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                        regs.insert(dst, value);
                                    }
                                    continue;
                                }
                                // other methods stub 0
                                if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                    regs.insert(dst, 0);
                                }
                                continue;
                            }
                            // Other box methods are not modeled; stub if dst present
                            if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                regs.insert(dst, 0);
                            }
                            continue;
                        }
                        // Extern calls: allow stub (return 0) when provider is enabled via env, else still stub to 0
                        "Extern" => {
                            let _provider_on =
                                crate::config::env::env_bool("HAKO_V1_EXTERN_PROVIDER");
                            // For now, always treat extern as stub → write 0 when dst present
                            if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                                regs.insert(dst, 0);
                            }
                            continue;
                        }
                        _ => {}
                    }
                    // Unsupported callee shape/type → treat as stub if possible, else error
                    if let Some(dst) = inst.get("dst").and_then(Value::as_i64) {
                        regs.insert(dst, 0);
                        continue;
                    }
                    return 1;
                }
                "phi" => {
                    let dst = match inst.get("dst").and_then(Value::as_i64) {
                        Some(d) => d,
                        None => return 1,
                    };
                    let mut val: i64 = 0;
                    let mut matched = false;
                    if let Some(incomings) = inst.get("incoming").and_then(Value::as_array) {
                        for pair in incomings {
                            if let Some(arr) = pair.as_array() {
                                if arr.len() >= 2 {
                                    // v1 schema (inline minimal): [value_reg, pred_block_id]
                                    let r = arr[0].as_i64().unwrap_or(-1);
                                    let b = arr[1].as_i64().unwrap_or(-1);
                                    if b == prev {
                                        val = regs.get(&r).cloned().unwrap_or(0);
                                        matched = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if !matched {
                        return 1;
                    }
                    regs.insert(dst, val);
                }
                "branch" => {
                    let cond = inst
                        .get("cond")
                        .and_then(Value::as_i64)
                        .and_then(|r| regs.get(&r).cloned())
                        .unwrap_or(0);
                    let then_b = inst.get("then").and_then(Value::as_i64).unwrap_or(curr);
                    let else_b = inst.get("else").and_then(Value::as_i64).unwrap_or(curr);
                    prev = curr;
                    curr = if cond != 0 { then_b } else { else_b };
                    continue 'outer; // switch block
                }
                "jump" => {
                    let target = inst.get("target").and_then(Value::as_i64).unwrap_or(curr);
                    prev = curr;
                    curr = target;
                    continue 'outer; // switch block
                }
                "ret" => {
                    let vid = match inst.get("value").and_then(Value::as_i64) {
                        Some(v) => v,
                        None => return 0,
                    };
                    let v = *regs.get(&vid).unwrap_or(&0);
                    return (v as i32) & 0xFF;
                }
                // ignore others for now (compare/branch not used in hv1 per‑recv canaries)
                _ => {}
            }
            // if we completed a block without control flow, stop
        }
        return 0;
    }
}
