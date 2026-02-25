use std::collections::{BTreeSet, VecDeque};

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirPrinter};

use super::super::StepTrace;

pub(crate) fn prepare_stepbudget_dumps(
    func: &MirFunction,
    cur: BasicBlockId,
    last_block: Option<BasicBlockId>,
) -> (Option<String>, Option<String>) {
    let fn_name_sanitized = sanitize_for_path(&func.signature.name);
    let pid = std::process::id();
    let path = format!(
        "/tmp/mir_dump_stepbudget_{}_{}.txt",
        fn_name_sanitized, pid
    );
    let dump_text = MirPrinter::new().print_function(func);
    let mir_dump_path = if let Ok(mut f) = std::fs::File::create(&path) {
        let _ = std::io::Write::write_all(&mut f, dump_text.as_bytes());
        Some(path)
    } else {
        Some("write_failed".to_string())
    };
    let snip_path = if mir_dump_path.as_deref() == Some("write_failed") {
        None
    } else {
        let block_ids = collect_snip_block_ids(func, cur, last_block);
        let snip = build_mir_snip(&dump_text, &block_ids);
        if snip.is_empty() {
            None
        } else {
            let snip_path = format!(
                "/tmp/mir_dump_stepbudget_snip_{}_{}.txt",
                fn_name_sanitized, pid
            );
            if let Ok(mut f) = std::fs::File::create(&snip_path) {
                let _ = std::io::Write::write_all(&mut f, snip.as_bytes());
                Some(snip_path)
            } else {
                Some("write_failed".to_string())
            }
        }
    };
    (mir_dump_path, snip_path)
}

pub(crate) fn format_trace_tail(recent_steps: &VecDeque<StepTrace>) -> Option<String> {
    if recent_steps.is_empty() {
        return None;
    }
    let mut parts = Vec::with_capacity(recent_steps.len());
    for step in recent_steps {
        let idx = step
            .inst_idx
            .map(|v| v.to_string())
            .unwrap_or_else(|| "n".to_string());
        let inst = step.inst.as_deref().unwrap_or("none");
        parts.push(format!("{}:{}:{}", step.bb, idx, inst));
    }
    Some(parts.join("|"))
}

pub(crate) fn loop_signature(recent_steps: &VecDeque<StepTrace>) -> Option<String> {
    if recent_steps.len() < 2 {
        return None;
    }
    let bbs: Vec<BasicBlockId> = recent_steps.iter().map(|s| s.bb).collect();
    let n = bbs.len();
    let max_len = std::cmp::min(8, n / 2);
    for len in 1..=max_len {
        let tail = &bbs[n - len..];
        let prev = &bbs[n - 2 * len..n - len];
        if tail == prev {
            let sig = tail
                .iter()
                .map(|bb| bb.to_string())
                .collect::<Vec<_>>()
                .join("->");
            return Some(sig);
        }
    }
    None
}

fn sanitize_for_path(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "unknown".to_string()
    } else {
        out
    }
}

fn collect_snip_block_ids(
    func: &MirFunction,
    cur: BasicBlockId,
    last_block: Option<BasicBlockId>,
) -> BTreeSet<BasicBlockId> {
    let mut set = BTreeSet::new();
    set.insert(cur);
    if let Some(bb) = last_block {
        set.insert(bb);
    }
    let mut to_visit: Vec<BasicBlockId> = set.iter().copied().collect();
    while let Some(bb) = to_visit.pop() {
        let Some(block) = func.blocks.get(&bb) else {
            continue;
        };
        for pred in &block.predecessors {
            if set.insert(*pred) {
                to_visit.push(*pred);
            }
        }
        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Jump { target, .. } => {
                    if set.insert(*target) {
                        to_visit.push(*target);
                    }
                }
                MirInstruction::Branch {
                    then_bb, else_bb, ..
                } => {
                    if set.insert(*then_bb) {
                        to_visit.push(*then_bb);
                    }
                    if set.insert(*else_bb) {
                        to_visit.push(*else_bb);
                    }
                }
                _ => {}
            }
        }
    }
    set
}

fn build_mir_snip(dump: &str, block_ids: &BTreeSet<BasicBlockId>) -> String {
    if block_ids.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut include = false;
    for line in dump.lines() {
        let trimmed = line.trim_start();
        if let Some(bb) = parse_bb_label(trimmed) {
            include = block_ids.contains(&bb);
            if include {
                out.push_str(line);
                out.push('\n');
            }
            continue;
        }
        if include {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn parse_bb_label(line: &str) -> Option<BasicBlockId> {
    if !line.starts_with("bb") {
        return None;
    }
    let rest = &line[2..];
    let num = rest.split(':').next()?;
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let id: u32 = num.parse().ok()?;
    Some(BasicBlockId(id))
}
